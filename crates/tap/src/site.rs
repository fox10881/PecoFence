//! The COM half of the injection: `TAPSite` (`IObjectWithSite`) receives the
//! XAML diagnostics object from `InitializeXamlDiagnosticsEx`, and
//! `VisualTreeWatcher` (`IVisualTreeServiceCallback2`) tracks the taskbar's
//! `BackgroundFill`/`BackgroundStroke` rectangles — the Rust equivalent of
//! TranslucentTB's `tapsite.cpp` + `visualtreewatcher.cpp`.

use crate::bindings::*;
use crate::shared::{self, RectSlot};
use crate::win32;
use std::sync::Mutex;
use std::sync::atomic::Ordering;
use windows_core::{GUID, HRESULT, implement};

/// CLSID the XAML framework instantiates in `InitializeXamlDiagnosticsEx`.
/// Ours — not TranslucentTB's — so both can coexist.
pub const CLSID_PECO_TAP_SITE: GUID = GUID::from_u128(0x9E4A7C31_B5D2_4F8A_A3E6_1C7B2D5F9A04);

const CLASS_E_CLASSNOTAVAILABLE: HRESULT = HRESULT(0x8004_0111_u32 as _);
const E_POINTER: HRESULT = HRESULT(0x8000_4003_u32 as _);

type InitializeXamlDiagnosticsEx = unsafe extern "system" fn(
    endpointname: windows_core::PCWSTR,
    pid: u32,
    wszdllxamldiagnostics: windows_core::PCWSTR,
    wsztapdllname: windows_core::PCWSTR,
    tapclsid: GUID,
    wszinitializationdata: windows_core::PCWSTR,
) -> HRESULT;

/// `tapsite.cpp` `TAPSite::Install`: resolves `InitializeXamlDiagnosticsEx` on
/// `Windows.UI.Xaml.dll` and calls it on a fresh thread per attempt — the API
/// only initializes once per thread, later calls on the same thread are no-ops.
/// Guard so command-thread and timer-driven retries never run installs
/// concurrently (`InitializeXamlDiagnosticsEx` is once-per-thread anyway).
static INSTALLING: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);
static CONN_ATTEMPT: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);
/// `ensure_watching` fires on every host tick; without this guard two ticks
/// landing while `watcher` is still `None` would each advise — registering
/// duplicate callbacks for the same tree.
static ADVISING: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);

/// One install pass. Retries are *not* done in a loop here: after a failed
/// `ixde` the calling thread can never execute a blocking wait again (sleep,
/// yield, join — all hang, likely a stray framework APC), so cadence lives in
/// the command window's `WM_TIMER`, which never touches `ixde`.
pub fn install() {
    if INSTALLING.swap(true, Ordering::AcqRel) {
        return;
    }
    let result = std::panic::catch_unwind(core::panic::AssertUnwindSafe(install_inner));
    INSTALLING.store(false, Ordering::Release);
    crate::tap_log!("install exited: {result:?}");
}

fn install_inner() {
    let wux = unsafe { win32::LoadLibraryW(windows_core::w!("Windows.UI.Xaml.dll")) };
    if wux.is_null() {
        return;
    }
    let ixde = unsafe {
        win32::GetProcAddress(
            wux,
            windows_core::PCSTR(c"InitializeXamlDiagnosticsEx".as_ptr() as *const u8),
        )
    };
    if ixde.is_null() {
        return;
    }
    let ixde: InitializeXamlDiagnosticsEx = unsafe { core::mem::transmute(ixde) };
    let mut location = [0u16; 512];
    let len = unsafe {
        win32::GetModuleFileNameW(
            crate::dll_instance(),
            windows_core::PWSTR(location.as_mut_ptr()),
            location.len() as u32,
        )
    };
    if len == 0 {
        return;
    }
    let pid = unsafe { win32::GetCurrentProcessId() };
    let attempt = CONN_ATTEMPT.fetch_add(1, Ordering::Relaxed) + 1;
    // One call per thread; joining keeps the thread's TEB alive until the
    // framework has recorded our callback address (same trick as TTB).
    let connection: Vec<u16> = format!("VisualDiagConnection{attempt}\0")
        .encode_utf16()
        .collect();
    let hr = std::thread::scope(|s| {
        s.spawn(|| unsafe {
            ixde(
                windows_core::PCWSTR(connection.as_ptr()),
                pid,
                windows_core::PCWSTR::null(),
                windows_core::PCWSTR(location.as_ptr()),
                CLSID_PECO_TAP_SITE,
                windows_core::PCWSTR::null(),
            )
        })
        .join()
        .unwrap_or(HRESULT(0x8000_4005_u32 as _))
    });
    crate::tap_log!("IXDEx attempt {attempt}: hr={hr:?}");
}

#[implement(IClassFactory)]
struct TapSiteFactory;

impl IClassFactory_Impl for TapSiteFactory_Impl {
    fn CreateInstance(
        &self,
        _punkouter: windows_core::Ref<windows_core::IUnknown>,
        riid: *const GUID,
        ppvobject: *mut *mut core::ffi::c_void,
    ) -> windows_core::Result<()> {
        unsafe {
            if ppvobject.is_null() || riid.is_null() {
                return Err(windows_core::Error::from_hresult(E_POINTER));
            }
            *ppvobject = core::ptr::null_mut();
            let site: IObjectWithSite = TapSite {
                site: Mutex::new(None),
            }
            .into();
            let unknown = windows_core::Interface::cast::<windows_core::IUnknown>(&site)?;
            (windows_core::Interface::vtable(&unknown).QueryInterface)(
                windows_core::Interface::as_raw(&unknown),
                riid,
                ppvobject,
            )
            .ok()
        }
    }

    fn LockServer(&self, _flock: windows_core::BOOL) -> windows_core::Result<()> {
        Ok(())
    }
}

/// `module.cpp`'s `DllGetClassObject` — the framework calls into this after
/// `InitializeXamlDiagnosticsEx` hands it our DLL path.
#[unsafe(no_mangle)]
pub unsafe extern "system" fn DllGetClassObject(
    rclsid: *const GUID,
    riid: *const GUID,
    ppv: *mut *mut core::ffi::c_void,
) -> HRESULT {
    unsafe {
        if ppv.is_null() || rclsid.is_null() || riid.is_null() {
            return E_POINTER;
        }
        *ppv = core::ptr::null_mut();
        crate::tap_log!("DllGetClassObject rclsid={:?} riid={:?}", *rclsid, *riid);
        if *rclsid != CLSID_PECO_TAP_SITE {
            return CLASS_E_CLASSNOTAVAILABLE;
        }
        let factory: IClassFactory = TapSiteFactory.into();
        (windows_core::Interface::vtable(&factory)
            .base__
            .QueryInterface)(windows_core::Interface::as_raw(&factory), riid, ppv)
    }
}

#[implement(IObjectWithSite)]
struct TapSite {
    site: Mutex<Option<windows_core::IUnknown>>,
}

impl IObjectWithSite_Impl for TapSite_Impl {
    /// Called by the XAML framework with the diagnostics object.
    fn SetSite(
        &self,
        punksite: windows_core::Ref<windows_core::IUnknown>,
    ) -> windows_core::Result<()> {
        crate::tap_log!("SetSite entry");
        let _ = std::panic::catch_unwind(core::panic::AssertUnwindSafe(|| {
            let site = punksite.ok().ok().cloned();
            *self.site.lock().unwrap_or_else(|p| p.into_inner()) = site.clone();
            if site.is_none() {
                return Ok(());
            }
            if shared::lock().watcher.is_some() {
                return Ok::<(), windows_core::Error>(()); // only ever one watcher
            }
            let diagnostics: Option<IXamlDiagnostics> =
                site.and_then(|s| windows_core::Interface::cast::<IXamlDiagnostics>(&s).ok());
            crate::tap_log!("SetSite: diagnostics={}", diagnostics.is_some());
            if let Some(diagnostics) = diagnostics {
                start_advise(diagnostics);
            }
            Ok(())
        }));
        Ok(())
    }

    fn GetSite(
        &self,
        riid: *const GUID,
        ppvsite: *mut *mut core::ffi::c_void,
    ) -> windows_core::Result<()> {
        unsafe {
            if ppvsite.is_null() || riid.is_null() {
                return Err(windows_core::Error::from_hresult(E_POINTER));
            }
            *ppvsite = core::ptr::null_mut();
            if let Some(site) = self.site.lock().unwrap_or_else(|p| p.into_inner()).as_ref() {
                (windows_core::Interface::vtable(site).QueryInterface)(
                    windows_core::Interface::as_raw(site),
                    riid,
                    ppvsite,
                )
                .ok()
            } else {
                Err(windows_core::Error::from_hresult(HRESULT(
                    0x8000_4005_u32 as _,
                )))
            }
        }
    }
}

/// Spawns the `AdviseVisualTreeChange` thread — the call marshals callbacks
/// onto the UI thread (TTB's hang fix). The diagnostics object is agile (the
/// XAML framework gives the same pointer VS debugger threads call
/// cross-thread), so moving it into the advise thread is safe.
fn start_advise(diagnostics: IXamlDiagnostics) {
    if ADVISING.swap(true, Ordering::AcqRel) {
        return;
    }
    struct SendDiag(IXamlDiagnostics);
    unsafe impl Send for SendDiag {}
    impl SendDiag {
        fn into_inner(self) -> IXamlDiagnostics {
            self.0
        }
    }
    let diagnostics = SendDiag(diagnostics);
    std::thread::spawn(move || {
        let _guard = AdvisingGuard;
        let diagnostics = diagnostics.into_inner();
        let watcher: IVisualTreeServiceCallback2 = VisualTreeWatcher.into();
        let Ok(service) = windows_core::Interface::cast::<IVisualTreeService3>(&diagnostics) else {
            crate::tap_log!("advise: IVisualTreeService3 cast failed");
            return;
        };
        // Store diagnostics/service/watcher BEFORE `AdviseVisualTreeChange`:
        // the service synchronously replays the existing visual tree inside
        // this call, and each `OnVisualTreeChange` needs `diagnostics` to
        // resolve handles — storing afterwards meant every replayed rectangle
        // was skipped (the "no diagnostics" race).
        {
            let mut shared = shared::lock();
            shared.diagnostics = Some(diagnostics);
            shared.tree_service = Some(service.clone());
            shared.watcher = Some(watcher.clone());
        }
        let advise_hr = unsafe { service.AdviseVisualTreeChange(&*watcher) };
        crate::tap_log!("advise hr={advise_hr:?}");
        if advise_hr.is_err() {
            // Rolled back so a later `ensure_watching` can retry the advise.
            shared::lock().watcher = None;
        }
    });
}

/// Clears `ADVISING` when the advise thread exits — including on panic, so a
/// failed advise never wedges `ensure_watching` permanently.
struct AdvisingGuard;
impl Drop for AdvisingGuard {
    fn drop(&mut self) {
        ADVISING.store(false, Ordering::Release);
    }
}

/// Re-arms the visual-tree watcher after an unload unadvised it. The
/// diagnostics object and service survive unload (the framework holds the
/// TAPSite alive), so a fresh callback re-advises without re-running
/// `InitializeXamlDiagnosticsEx`. If the diagnostics object never arrived
/// (island wasn't ready at first inject), a fresh install pass is spawned —
/// that is the retry channel the timer drives.
pub fn ensure_watching() {
    if shared::lock().watcher.is_some() {
        return;
    }
    let diagnostics = shared::lock().diagnostics.clone();
    match diagnostics {
        Some(diagnostics) => start_advise(diagnostics),
        None => {
            std::thread::spawn(install);
        }
    }
}

#[implement(IVisualTreeServiceCallback2)]
struct VisualTreeWatcher;

impl IVisualTreeServiceCallback_Impl for VisualTreeWatcher_Impl {
    /// Invoked on the XAML UI thread by the diagnostics service.
    fn OnVisualTreeChange(
        &self,
        relation: &ParentChildRelation,
        element: &VisualElement,
        mutationtype: VisualMutationType,
    ) -> windows_core::Result<()> {
        let _ = std::panic::catch_unwind(core::panic::AssertUnwindSafe(|| {
            on_visual_tree_change(relation, element, mutationtype)
        }));
        Ok(())
    }
}

impl IVisualTreeServiceCallback2_Impl for VisualTreeWatcher_Impl {
    fn OnElementStateChanged(
        &self,
        _element: InstanceHandle,
        _elementstate: VisualElementState,
        _context: &windows_core::PCWSTR,
    ) -> windows_core::Result<()> {
        Ok(())
    }
}

/// The callback owns the `VisualElement` strings (xamlOM.h contract): take the
/// `BSTR`s out of the borrowed struct so dropping frees them once.
unsafe fn free_element_strings(element: &VisualElement) {
    unsafe {
        drop(core::mem::ManuallyDrop::into_inner(core::ptr::read(
            &element.Type,
        )));
        drop(core::mem::ManuallyDrop::into_inner(core::ptr::read(
            &element.Name,
        )));
        drop(core::mem::ManuallyDrop::into_inner(core::ptr::read(
            &element.SrcInfo.FileName,
        )));
        drop(core::mem::ManuallyDrop::into_inner(core::ptr::read(
            &element.SrcInfo.Hash,
        )));
    }
}

fn on_visual_tree_change(
    relation: &ParentChildRelation,
    element: &VisualElement,
    mutationtype: VisualMutationType,
) {
    if mutationtype == Remove {
        // Take the slot out under the lock, then release COM refs outside it.
        let (removed, removed_bg) = {
            let mut shared = shared::lock();
            let rect_slot =
                shared
                    .slot_handles
                    .remove(&element.Handle.0)
                    .and_then(|(frame, is_fill)| {
                        shared.rects.get_mut(&frame).and_then(|set| {
                            if is_fill {
                                set.fill.take()
                            } else {
                                set.stroke.take()
                            }
                        })
                    });
            let bg_slot = shared
                .bg_handles
                .remove(&element.Handle.0)
                .and_then(|frame| {
                    shared
                        .rects
                        .get_mut(&frame)
                        .and_then(|set| set.background.take())
                });
            shared.xaml_sources.remove(&element.Handle.0);
            (rect_slot, bg_slot)
        };
        drop(removed);
        drop(removed_bg);
        unsafe { free_element_strings(element) };
        return;
    }

    let diagnostics = shared::lock().diagnostics.clone();
    let Some(diagnostics) = diagnostics else {
        unsafe { free_element_strings(element) };
        return;
    };

    let is_xaml_source =
        element.Type[..] == **windows_core::h!("Windows.UI.Xaml.Hosting.DesktopWindowXamlSource");
    if is_xaml_source {
        shared::lock().xaml_sources.insert(element.Handle.0);
        unsafe { free_element_strings(element) };
        return;
    }

    let is_taskbar_frame = element.Type[..] == **windows_core::h!("Taskbar.TaskbarFrame");
    if is_taskbar_frame {
        register_taskbar_frame(&diagnostics, relation, element);
        unsafe { free_element_strings(element) };
        return;
    }

    let is_taskbar_bg = element.Type[..] == **windows_core::h!("Taskbar.TaskbarBackground")
        && element.Name[..] == **windows_core::h!("BackgroundControl");
    if is_taskbar_bg {
        register_taskbar_background(&diagnostics, relation, element);
        unsafe { free_element_strings(element) };
        return;
    }

    let is_rectangle = element.Type[..] == **windows_core::h!("Windows.UI.Xaml.Shapes.Rectangle");
    let is_fill = element.Name[..] == **windows_core::h!("BackgroundFill");
    let is_stroke = element.Name[..] == **windows_core::h!("BackgroundStroke");
    if !is_rectangle || (!is_fill && !is_stroke) {
        unsafe { free_element_strings(element) };
        return;
    }

    // Only rectangles inside a `TaskbarFrame` — Start and friends reuse the
    // same element names elsewhere in the shell (same check TTB does).
    let Some(frame) = find_taskbar_frame(&diagnostics, relation) else {
        crate::tap_log!(
            "rect {:?}: TaskbarFrame not found",
            String::from_utf16_lossy(&element.Name[..])
        );
        unsafe { free_element_strings(element) };
        return;
    };
    let Ok(frame_handle) = (unsafe { diagnostics.GetHandleFromIInspectable(&frame) }) else {
        crate::tap_log!("rect: frame handle failed");
        unsafe { free_element_strings(element) };
        return;
    };
    let Ok(rect) = unsafe { diagnostics.GetIInspectableFromHandle(element.Handle) }
        .and_then(|o| windows_core::Interface::cast::<Rectangle>(&o))
    else {
        crate::tap_log!("rect: IInspectable->Rectangle failed");
        unsafe { free_element_strings(element) };
        return;
    };

    // Explorer re-styles the taskbar's Fill on theme/accent changes; the
    // property-changed callback (TTB's `OnTaskbarBackgroundUpdated`) puts our
    // brush straight back instead of waiting for the next state command. It
    // re-posts through the dispatcher rather than painting inside the
    // callback: `SetFill` runs this synchronously, so painting here would
    // nest `SetFill` inside itself.
    let on_fill_changed = DependencyPropertyChangedCallback::new(|_, _| {
        // `painting()` filters our own SetFill — otherwise every apply would
        // queue another apply forever (each fresh brush is a real change).
        if !shared::painting() && shared::TAP_STATE.load(Ordering::Acquire) != shared::STATE_NORMAL
        {
            shared::invalidate_applied();
            shared::dispatch_apply(0);
        }
        Ok(())
    });
    if let Ok(dp) = Shape::FillProperty() {
        let _ = rect.RegisterPropertyChangedCallback(&dp, &on_fill_changed);
    }

    // XAML getters happen before the lock so `shared` is never held across a
    // call that can run callbacks on this thread.
    let dispatcher = rect.Dispatcher().ok();
    let original = rect.Fill().ok();
    {
        let mut shared = shared::lock();
        if shared.dispatcher.is_none() {
            shared.dispatcher = dispatcher;
        }
        let slot = RectSlot {
            original,
            element: rect.clone(),
            _on_fill_changed: Some(on_fill_changed),
        };
        shared
            .slot_handles
            .insert(element.Handle.0, (frame_handle.0, is_fill));
        let set = shared.rects.entry(frame_handle.0).or_default();
        if is_fill {
            set.fill = Some(slot);
        } else {
            set.stroke = Some(slot);
        }
        crate::tap_log!(
            "rect registered: frame={} fill={is_fill} sets={}",
            frame_handle.0,
            shared.rects.len()
        );
    }
    // New rectangles appear already styled by Explorer; repaint this one when
    // a non-Normal mode is active so freshly spawned taskbars match.
    shared::paint_rect(&rect, is_fill);
    unsafe { free_element_strings(element) };
}

/// Matches a new `Taskbar.TaskbarFrame` to the `DesktopWindowXamlSource` that
/// hosts it (TTB's `m_NonMatchingXamlSources` scan), then stores the island's
/// real window handle on the frame's set — the accent's actual target.
fn register_taskbar_frame(
    diagnostics: &IXamlDiagnostics,
    relation: &ParentChildRelation,
    element: &VisualElement,
) {
    let Ok(root_grid) = unsafe { diagnostics.GetIInspectableFromHandle(relation.Parent) }
        .and_then(|o| windows_core::Interface::cast::<UIElement>(&o))
    else {
        crate::tap_log!("frame: parent->UIElement failed");
        return;
    };
    let root_unknown = windows_core::Interface::cast::<windows_core::IUnknown>(&root_grid).ok();
    let sources: Vec<u64> = shared::lock().xaml_sources.iter().copied().collect();
    let mut matched = None;
    for handle in sources {
        let Ok(source) = unsafe { diagnostics.GetIInspectableFromHandle(InstanceHandle(handle)) }
            .and_then(|o| windows_core::Interface::cast::<DesktopWindowXamlSource>(&o))
        else {
            continue;
        };
        // `Content` may throw hresult_wrong_thread for a source whose content
        // is still being set up — TTB skips those the same way.
        let Ok(content) = source.Content() else {
            continue;
        };
        let content_unknown =
            windows_core::Interface::cast::<windows_core::IUnknown>(&content).ok();
        match (&root_unknown, &content_unknown) {
            (Some(a), Some(b))
                if windows_core::Interface::as_raw(a) == windows_core::Interface::as_raw(b) =>
            {
                let Ok(hwnd) =
                    windows_core::Interface::cast::<IDesktopWindowXamlSourceNative>(&source)
                        .and_then(|native| unsafe { native.WindowHandle() })
                else {
                    continue;
                };
                crate::tap_log!("frame {} -> island {:?}", element.Handle.0, hwnd.0);
                shared::lock()
                    .rects
                    .entry(element.Handle.0)
                    .or_default()
                    .island_hwnd = Some(hwnd);
                matched = Some(handle);
                break;
            }
            _ => {}
        }
    }
    if let Some(handle) = matched {
        shared::lock().xaml_sources.remove(&handle);
        // A frame arriving while Clear/Blur is active needs its rects painted
        // immediately — same late-arrival rule as rects and backgrounds.
        if shared::TAP_STATE.load(Ordering::Acquire) != shared::STATE_NORMAL {
            shared::invalidate_applied();
            shared::dispatch_apply(0);
        }
    }
}

/// Registers a `Taskbar.TaskbarBackground` control against its frame and
/// applies the current state to it. On builds where the plate is the control
/// itself (not the rects), `Opacity` is the switch that hides it.
fn register_taskbar_background(
    diagnostics: &IXamlDiagnostics,
    relation: &ParentChildRelation,
    element: &VisualElement,
) {
    let Some(frame) = find_taskbar_frame(diagnostics, relation) else {
        crate::tap_log!("bg: TaskbarFrame not found");
        return;
    };
    let Ok(frame_handle) = (unsafe { diagnostics.GetHandleFromIInspectable(&frame) }) else {
        return;
    };
    let Ok(bg) = unsafe { diagnostics.GetIInspectableFromHandle(element.Handle) }
        .and_then(|o| windows_core::Interface::cast::<UIElement>(&o))
    else {
        crate::tap_log!("bg: IInspectable->UIElement failed");
        return;
    };
    let control = windows_core::Interface::cast::<Control>(&bg).ok();
    crate::tap_log!("bg: control={}", control.is_some());
    {
        let mut shared = shared::lock();
        if shared.dispatcher.is_none() {
            shared.dispatcher = bg.Dispatcher().ok();
        }
        shared.bg_handles.insert(element.Handle.0, frame_handle.0);
        shared.rects.entry(frame_handle.0).or_default().background = Some(shared::BgSlot {
            element: bg.clone(),
            control,
        });
        crate::tap_log!("bg registered: frame={}", frame_handle.0);
    }
    // Same late-arrival paint as rects: a background control that appears
    // while Clear/Blur is active must take our brush immediately. Invalidate
    // `applied` first — `apply_current_state` skips repainting when it already
    // matches, and this control still has Explorer's default background.
    if shared::TAP_STATE.load(Ordering::Acquire) != shared::STATE_NORMAL {
        shared::invalidate_applied();
        shared::dispatch_apply(0);
    }
}

/// Walks parents from `relation.Parent` until an element named `TaskbarFrame`
/// is found (`visualtreewatcher.cpp`'s `FindParent`).
fn find_taskbar_frame(
    diagnostics: &IXamlDiagnostics,
    relation: &ParentChildRelation,
) -> Option<FrameworkElement> {
    let mut element = unsafe {
        diagnostics
            .GetIInspectableFromHandle(relation.Parent)
            .and_then(|o| windows_core::Interface::cast::<FrameworkElement>(&o))
            .ok()?
    };
    loop {
        let name = element.Name().ok()?;
        crate::tap_log!("  walk: {}", String::from_utf16_lossy(&name[..]));
        if name[..] == **windows_core::h!("TaskbarFrame") {
            return Some(element);
        }
        element = VisualTreeHelper::GetParent(&element)
            .and_then(|p| windows_core::Interface::cast::<FrameworkElement>(&p))
            .ok()?;
    }
}
