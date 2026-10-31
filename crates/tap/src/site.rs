//! The COM half of the injection: `TAPSite` (`IObjectWithSite`) receives the
//! XAML diagnostics object from `InitializeXamlDiagnosticsEx`, and
//! `VisualTreeWatcher` (`IVisualTreeServiceCallback2`) feeds every reported
//! element into `styler` — the native taskbar/Start Menu theme engine.

use crate::bindings::*;
use crate::shared;
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

/// `ixde` failures are sticky: injecting into a just-started Explorer lands
/// before the taskbar's DXamlCore registers with the diagnostics service,
/// and the first `0x80070490` poisons **every** later attempt for the
/// process's life (observed: 80+ consecutive failures; every success landed
/// on attempt 1). So the first attempt waits for the host to be past boot —
/// the command window's `WM_TIMER` retries `ensure_watching`, which calls
/// `install` again once the age gate opens.
fn host_warm_enough() -> bool {
    host_warm_enough_ms(30_000)
}

/// `warmup_ms` — the WUX session needs 30s (the taskbar's DXamlCore
/// registers ~10–25s after explorer spawns); the WinUI3 session only sees
/// cabinet processes, which exist to host their island — injection lands
/// after the window already renders, so `FrameworkUdk` presence plus a
/// short floor covers the diagnostics registration. A poisoned attempt in a
/// cabinet process is also self-limiting: the next window is a new process.
fn host_warm_enough_ms(warmup_ms: u64) -> bool {
    unsafe {
        let (mut c, mut e, mut k, mut u) = (
            win32::FILETIME::default(),
            win32::FILETIME::default(),
            win32::FILETIME::default(),
            win32::FILETIME::default(),
        );
        if !win32::GetProcessTimes(win32::GetCurrentProcess(), &mut c, &mut e, &mut k, &mut u)
            .as_bool()
        {
            return true; // can't measure — attempt anyway
        }
        let mut now = win32::FILETIME::default();
        win32::GetSystemTimeAsFileTime(&mut now);
        now.ticks().saturating_sub(c.ticks()) / 10_000 >= warmup_ms
    }
}

static WARMUP_LOGGED: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);

/// Which diagnostics session the in-flight `install_*` call belongs to —
/// `SetSite` reads this to pick the shared-state slot. `0` = none,
/// `1` = WUX (Windows.UI.Xaml), `2` = WinUI3 (Microsoft.UI.Xaml via
/// `Microsoft.Internal.FrameworkUdk.dll` — File Explorer's CabinetWClass
/// islands). `INSTALLING` serializes installs, so the tag is unambiguous
/// for the `SetSite` the ixde handshake delivers.
static PENDING_SESSION: std::sync::atomic::AtomicU8 = std::sync::atomic::AtomicU8::new(0);

/// `InitializeXamlDiagnosticsEx` on the **WinUI3** framework — File
/// Explorer's modern chrome is a Microsoft.UI.Xaml island, invisible to the
/// WUX session the taskbar uses. Same one-shot-per-thread rule applies to
/// `install()` below; the Windhawk mod calls this `InjectWindhawkTAP`.
///
/// Unlike WUX, the connection endpoint must be *guessed*: the framework
/// registers exactly one `WinUIVisualDiagConnection{N}` per session and the
/// number isn't discoverable, so we walk candidates until one accepts
/// (any result other than `ERROR_NOT_FOUND` stops the walk) — the mod does
/// the same, up to 10000.
pub fn install_winui3() {
    if INSTALLING.swap(true, Ordering::AcqRel) {
        return;
    }
    let result = std::panic::catch_unwind(core::panic::AssertUnwindSafe(install_winui3_inner));
    INSTALLING.store(false, Ordering::Release);
    crate::tap_log!("winui3 install exited: {result:?}");
}

fn install_winui3_inner() {
    // Attach only once the host loaded the framework itself: `LoadLibraryW`
    // here would bootstrap WinUI3 on *our* worker thread instead.
    let udk =
        unsafe { win32::GetModuleHandleW(windows_core::w!("Microsoft.Internal.FrameworkUdk.dll")) };
    if udk.is_null() {
        return;
    }
    let ixde = unsafe {
        win32::GetProcAddress(
            udk,
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
    PENDING_SESSION.store(2, Ordering::Release);
    // One thread runs the whole endpoint walk: ixde marks its calling
    // thread, and joining keeps the TEB alive until the framework has
    // recorded our callback address.
    std::thread::scope(|s| {
        s.spawn(|| unsafe {
            let _ = win32::RoInitialize(0);
            let mut hr = HRESULT(0x80070490_u32 as _); // ERROR_NOT_FOUND
            for i in 1..=4096u32 {
                let mut name = [0u16; 64];
                let text = format!("WinUIVisualDiagConnection{i}\0");
                let w: Vec<u16> = text.encode_utf16().collect();
                name[..w.len()].copy_from_slice(&w);
                hr = ixde(
                    windows_core::PCWSTR(name.as_ptr()),
                    pid,
                    windows_core::PCWSTR::null(),
                    windows_core::PCWSTR(location.as_ptr()),
                    CLSID_PECO_TAP_SITE,
                    windows_core::PCWSTR::null(),
                );
                if hr != HRESULT(0x80070490_u32 as _) {
                    break;
                }
            }
            PENDING_SESSION.store(0, Ordering::Release);
            crate::tap_log!("winui3 IXDEx walk done: hr={hr:?}");
        })
        .join()
        .unwrap_or(())
    });
}

/// Re-arms the **WinUI3** visual-tree watcher — the mux engine's session.
/// Mirrors `ensure_watching` against the `*_winui3` shared slots: a missing
/// diagnostics object means `install_winui3` hasn't landed the session yet
/// (the endpoint exists only after the first WinUI3 island initializes).
pub fn ensure_watching_winui3() {
    if crate::command::is_unloaded() {
        return;
    }
    {
        let shared = shared::lock();
        if shared.watcher_winui3.is_none() {
            let diagnostics = shared.diagnostics_winui3.clone();
            drop(shared);
            match diagnostics {
                Some(diagnostics) => start_advise_winui3(diagnostics),
                None => {
                    std::thread::spawn(install_winui3);
                }
            }
        }
    }
}

fn install_inner() {
    if !host_warm_enough() {
        if !WARMUP_LOGGED.swap(true, Ordering::Relaxed) {
            crate::tap_log!("IXDEx deferred — host still warming up");
        }
        return;
    }
    // Attach only once the host loaded the framework itself: `LoadLibraryW`
    // here would bootstrap `Windows.UI.Xaml` on *our* worker thread instead.
    let wux = unsafe { win32::GetModuleHandleW(windows_core::w!("Windows.UI.Xaml.dll")) };
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
    PENDING_SESSION.store(1, Ordering::Release);
    // One call per thread; joining keeps the thread's TEB alive until the
    // framework has recorded our callback address (same trick as TTB).
    let connection: Vec<u16> = format!("VisualDiagConnection{attempt}\0")
        .encode_utf16()
        .collect();
    let hr = std::thread::scope(|s| {
        s.spawn(|| unsafe {
            let _ = win32::RoInitialize(0);
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
    PENDING_SESSION.store(0, Ordering::Release);
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
        let _ = std::panic::catch_unwind(core::panic::AssertUnwindSafe(move || {
            let site = punksite.ok().ok().cloned();
            *self.site.lock().unwrap_or_else(|p| p.into_inner()) = site.clone();
            if site.is_none() {
                return Ok(());
            }
            // Which session this SetSite belongs to: `PENDING_SESSION` was
            // stamped by the in-flight install (WUX=1, WinUI3=2). A late
            // SetSite after the tag cleared picks the unoccupied slot —
            // the WUX install always runs first at host boot, so an empty
            // WUX slot belongs to it.
            let mut winui3 = PENDING_SESSION.load(Ordering::Acquire) == 2;
            {
                let shared = shared::lock();
                if PENDING_SESSION.load(Ordering::Acquire) == 0 {
                    winui3 = shared.watcher.is_some();
                }
                let occupied = if winui3 {
                    shared.watcher_winui3.is_some()
                } else {
                    shared.watcher.is_some()
                };
                if occupied {
                    return Ok::<(), windows_core::Error>(()); // one watcher per session
                }
            }
            let diagnostics: Option<IXamlDiagnostics> =
                site.and_then(|s| windows_core::Interface::cast::<IXamlDiagnostics>(&s).ok());
            crate::tap_log!(
                "SetSite: winui3={winui3} diagnostics={}",
                diagnostics.is_some()
            );
            if let Some(diagnostics) = diagnostics {
                if winui3 {
                    start_advise_winui3(diagnostics);
                } else {
                    start_advise(diagnostics);
                }
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
        let watcher: IVisualTreeServiceCallback2 = VisualTreeWatcher { session: 1 }.into();
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

/// `start_advise` for the **WinUI3** session — same ordering contract,
/// separate slots, watcher tagged `session: 2` so reported elements route
/// to the mux engine.
fn start_advise_winui3(diagnostics: IXamlDiagnostics) {
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
        let watcher: IVisualTreeServiceCallback2 = VisualTreeWatcher { session: 2 }.into();
        let Ok(service) = windows_core::Interface::cast::<IVisualTreeService3>(&diagnostics) else {
            crate::tap_log!("advise(winui3): IVisualTreeService3 cast failed");
            return;
        };
        {
            let mut shared = shared::lock();
            shared.diagnostics_winui3 = Some(diagnostics);
            shared.tree_service_winui3 = Some(service.clone());
            shared.watcher_winui3 = Some(watcher.clone());
        }
        let advise_hr = unsafe { service.AdviseVisualTreeChange(&*watcher) };
        crate::tap_log!("advise(winui3) hr={advise_hr:?}");
        if advise_hr.is_err() {
            shared::lock().watcher_winui3 = None;
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
    // An unloaded resident stays inert — re-advising here would steal the
    // diagnostics channel back from the live co-resident.
    if crate::command::is_unloaded() {
        return;
    }
    {
        let shared = shared::lock();
        if shared.watcher.is_none() {
            let diagnostics = shared.diagnostics.clone();
            drop(shared);
            match diagnostics {
                Some(diagnostics) => start_advise(diagnostics),
                None => {
                    std::thread::spawn(install);
                }
            }
        }
    }
}

#[implement(IVisualTreeServiceCallback2)]
struct VisualTreeWatcher {
    /// Which diagnostics session delivers this callback: `1` = WUX
    /// (taskbar/Start Menu hosts), `2` = WinUI3 (File Explorer mux engine).
    session: u8,
}

impl IVisualTreeServiceCallback_Impl for VisualTreeWatcher_Impl {
    /// Invoked on the XAML UI thread by the diagnostics service.
    fn OnVisualTreeChange(
        &self,
        relation: &ParentChildRelation,
        element: &VisualElement,
        mutationtype: VisualMutationType,
    ) -> windows_core::Result<()> {
        let _ = std::panic::catch_unwind(core::panic::AssertUnwindSafe(|| {
            on_visual_tree_change(self.session, relation, element, mutationtype)
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
    session: u8,
    relation: &ParentChildRelation,
    element: &VisualElement,
    mutationtype: VisualMutationType,
) {
    // DIAG: element-feed telemetry while bringing up the taskbar styler —
    // counts callbacks and logs the first element types seen per resident.
    {
        static SEEN: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);
        let n = SEEN.fetch_add(1, Ordering::Relaxed) + 1;
        if n <= 40 || n.is_multiple_of(200) {
            crate::tap_log!(
                "vtw s{session} #{n} {:?} type={:?} handle={}",
                mutationtype,
                String::from_utf16_lossy(&element.Type[..]),
                element.Handle.0
            );
        }
    }
    // Every element feeds the styler for its own session — WUX (taskbar/
    // Start Menu hosts) or WinUI3 (File Explorer islands, mux engine);
    // the theme's own matchers decide what to touch.
    match mutationtype {
        t if t == Remove => {
            if session == 2 {
                crate::styler::mux::on_element_removed(element.Handle.0);
            } else {
                crate::styler::wux::on_element_removed(element.Handle.0);
            }
        }
        _ => {
            let diagnostics = {
                let shared = shared::lock();
                if session == 2 {
                    shared.diagnostics_winui3.clone()
                } else {
                    shared.diagnostics.clone()
                }
            };
            if let Some(diagnostics) = diagnostics {
                let ty = String::from_utf16_lossy(&element.Type[..]);
                if session == 2 {
                    crate::styler::mux::on_element(&diagnostics, element, relation.Parent.0, &ty);
                } else {
                    crate::styler::wux::on_element(&diagnostics, element, relation.Parent.0, &ty);
                }
            }
        }
    }
    unsafe { free_element_strings(element) };
}
