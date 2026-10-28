//! State shared between the SWCA detour, the command window and the XAML
//! visual-tree watcher. Mirrors TranslucentTB's TaskbarAppearanceService state:
//! the discovered `BackgroundFill`/`BackgroundStroke` rectangles per taskbar
//! frame plus their original brushes so "Normal" can restore them exactly.

use crate::bindings::*;
use crate::win32::{HANDLE, HWND};
use std::collections::{BTreeMap, BTreeSet};
use std::sync::Mutex;
use std::sync::atomic::{AtomicU32, Ordering};

pub const STATE_NORMAL: u32 = 0;
pub const STATE_CLEAR: u32 = 1;
pub const STATE_BLUR: u32 = 2;
pub const STATE_OPAQUE: u32 = 3;

/// Current requested mode. The detour reads this on every call — it must stay a
/// plain atomic so the hook path never blocks on a lock.
pub static TAP_STATE: AtomicU32 = AtomicU32::new(STATE_NORMAL);

/// Set while *we* are writing brushes. The `Fill` property-changed callback
/// can't tell our `SetFill` from Explorer's — without it, every apply spawns
/// a fresh brush (a real property change) which re-triggers the callback,
/// which invalidates and re-dispatches apply: a perpetual repaint loop on the
/// XAML UI thread that starves the compositor.
static PAINTING: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);

/// True while `apply_current_state`/`paint_rect` hold the brush-write section.
pub fn painting() -> bool {
    PAINTING.load(Ordering::Acquire)
}

/// RAII guard for the brush-write section; clears `PAINTING` on drop.
struct PaintingGuard;
impl PaintingGuard {
    fn begin() -> Self {
        PAINTING.store(true, Ordering::Release);
        PaintingGuard
    }
}
impl Drop for PaintingGuard {
    fn drop(&mut self) {
        PAINTING.store(false, Ordering::Release);
    }
}

/// One XAML `Shapes.Rectangle` we may repaint, with the brush it had when we
/// first saw it (captured once the system has populated it, like TTB's
/// `originalFill`).
pub struct RectSlot {
    pub element: Rectangle,
    pub original: Option<Brush>,
    /// `Fill` property-changed registration; only held so Explorer re-styling
    /// the taskbar re-applies our brush (TTB's `OnTaskbarBackgroundUpdated`).
    pub _on_fill_changed: Option<DependencyPropertyChangedCallback>,
}

/// The `Taskbar.TaskbarBackground` control — on Windows builds where the
/// taskbar plate is drawn by the control itself (not by the rectangles),
/// `Fill` changes do nothing visually and the control's own `Opacity` is what
/// hides the plate.
pub struct BgSlot {
    /// The `Taskbar.TaskbarBackground` element (kept so the COM ref lives).
    #[allow(dead_code)]
    pub element: UIElement,
    /// As a `Control` when the cast works — its `Background` is the plate on
    /// builds where the rects' `Fill` does not render.
    pub control: Option<Control>,
}

/// The taskbar chrome elements belonging to one `Taskbar.TaskbarFrame`.
#[derive(Default)]
pub struct RectSet {
    pub fill: Option<RectSlot>,
    pub stroke: Option<RectSlot>,
    pub background: Option<BgSlot>,
    /// The `DesktopWindowXamlSource` hwnd hosting this taskbar's XAML island —
    /// the window the accent policy actually has to target (TTB's
    /// `IDesktopWindowXamlSourceNative::get_WindowHandle`). `Shell_TrayWnd`
    /// accent writes do not reach this surface on XAML taskbars.
    pub island_hwnd: Option<HWND>,
}

pub struct Shared {
    /// The XAML diagnostics object handed to `IObjectWithSite::SetSite`.
    pub diagnostics: Option<IXamlDiagnostics>,
    /// `IVisualTreeService3` on the same object; kept to `Unadvise` on unload.
    pub tree_service: Option<IVisualTreeService3>,
    /// Our registered callback; held so the framework never sees a dangling ref.
    pub watcher: Option<IVisualTreeServiceCallback2>,
    /// The XAML UI thread's dispatcher, captured inside the first tree callback.
    pub dispatcher: Option<CoreDispatcher>,
    /// `TaskbarFrame` handle -> its chrome rectangles.
    pub rects: BTreeMap<u64, RectSet>,
    /// Element handles of the rectangles themselves, for `Remove` matching.
    pub slot_handles: BTreeMap<u64, (u64, bool)>, // rect handle -> (frame, is_fill)
    /// Element handle of each `Taskbar.TaskbarBackground` -> its frame.
    pub bg_handles: BTreeMap<u64, u64>,
    /// `DesktopWindowXamlSource` element handles not yet matched to a taskbar
    /// (the source is created empty before its content arrives — TTB's
    /// `m_NonMatchingXamlSources`).
    pub xaml_sources: BTreeSet<u64>,
    /// The state the brushes currently show (re-apply is skipped when equal).
    pub applied: u32,
    /// Wait handle from `RegisterWaitForSingleObject` watching the host process.
    pub host_wait: HANDLE,
    /// Open handle to the host process — kept alive for the wait's duration.
    pub host_process: HANDLE,
    /// Handle of our command window, once created.
    pub command_window: HWND,
}

// The Mutex serializes every access; individual COM objects are only invoked
// on the thread that owns them (rects/dispatcher through the UI dispatcher,
// diagnostics calls from the watcher thread), so sharing the pointers is safe.
unsafe impl Send for Shared {}

static SHARED: Mutex<Shared> = Mutex::new(Shared {
    diagnostics: None,
    tree_service: None,
    watcher: None,
    dispatcher: None,
    rects: BTreeMap::new(),
    slot_handles: BTreeMap::new(),
    bg_handles: BTreeMap::new(),
    xaml_sources: BTreeSet::new(),
    applied: STATE_NORMAL,
    host_wait: core::ptr::null_mut(),
    host_process: core::ptr::null_mut(),
    command_window: HWND(core::ptr::null_mut()),
});

pub fn lock() -> std::sync::MutexGuard<'static, Shared> {
    match SHARED.lock() {
        Ok(g) => g,
        Err(p) => p.into_inner(),
    }
}

/// Builds the appearance brush for `state` on one `BackgroundFill` rect, the
/// Rust port of TTB's `TaskbarAppearanceService::SetTaskbarAppearance` /
/// `SetTaskbarBlur`. Pure XAML: on Windows 11 the host writes **no** accent
/// policy while a mode is armed — TTB only uses `SetWindowCompositionAttribute`
/// when the TAP service is absent, so the taskbar window keeps Explorer's own
/// transparency and the backdrop can be sampled.
///
/// - Clear: `SolidColorBrush` alpha 0 (TTB `brush == SolidColor`, tint A=0).
/// - Opaque: `SolidColorBrush` #000000 alpha 0xFF (TTB `GRADIENT` forces
///   `color.A = 0xFF` on the same solid brush; default tint is black).
/// - Blur: the `XamlBlurBrush` port — a Composition `BackdropBrush` through
///   GaussianBlur + Flood + Composite SOURCE_OVER (`effects.rs`).
///
/// Must run on a XAML UI thread.
fn appearance_brush(state: u32, element: &Rectangle) -> windows_core::Result<Brush> {
    if state == STATE_BLUR {
        // BlurRadius 9 / 3 = 3.0 and a transparent flood = pure frosting —
        // TTB's defaults in `SetTaskbarBlur`.
        return crate::effects::make_blur_brush(element, 3.0, [0.0, 0.0, 0.0, 0.0]);
    }
    let color = if state == STATE_OPAQUE {
        Color {
            A: 0xFF,
            R: 0,
            G: 0,
            B: 0,
        }
    } else {
        Color {
            A: 0,
            R: 0,
            G: 0,
            B: 0,
        }
    };
    let brush = SolidColorBrush::CreateInstanceWithColor(color)?;
    windows_core::Interface::cast(&brush)
}

/// The border hiding brush — TTB's `SetTaskbarBorderVisibility(false)`:
/// a `SolidColorBrush` at `Opacity` 0 on `BackgroundStroke`.
fn hidden_brush() -> windows_core::Result<Brush> {
    let brush = SolidColorBrush::CreateInstanceWithColor(Color {
        A: 0,
        R: 0,
        G: 0,
        B: 0,
    })?;
    brush.SetOpacity(0.0)?;
    windows_core::Interface::cast(&brush)
}

/// Paints one rectangle for `TAP_STATE` — used when a rectangle registers
/// *after* a sweep already ran (a taskbar that spawned while Clear/Blur was
/// active still has Explorer's brush). UI-thread only.
///
/// The lock is released before `SetFill`: property-changed callbacks run on
/// this same thread and take `shared` — painting under the guard deadlocks.
pub fn paint_rect(rect: &Rectangle, is_fill: bool) {
    let state = TAP_STATE.load(Ordering::Acquire);
    if state == STATE_NORMAL {
        return;
    }
    let _guard = PaintingGuard::begin();
    let brush = if is_fill {
        appearance_brush(state, rect)
    } else {
        hidden_brush()
    };
    if let Ok(brush) = brush {
        let _ = rect.SetFill(&brush);
    }
}

/// Forces the next `apply_current_state` to repaint even when `applied`
/// already matches — Explorer mutated a `Fill` underneath us.
pub fn invalidate_applied() {
    lock().applied = u32::MAX;
}

/// Paints every registered rectangle for `TAP_STATE`. UI-thread only — call it
/// from `OnVisualTreeChange` or through `dispatch_apply`.
///
/// Snapshots the brush/element refs under the lock, then releases it before
/// the `SetFill` loop: `SetFill` can synchronously run our `Fill`
/// property-changed callback, which needs `shared` — same-thread re-entry on a
/// `std::Mutex` is a deadlock (this bit on the re-arm path where registration
/// already runs with `TAP_STATE != NORMAL`).
pub fn apply_current_state() {
    let state = TAP_STATE.load(Ordering::Acquire);
    let (fills, strokes, backgrounds) = {
        let mut shared = lock();
        if shared.applied == state {
            return;
        }
        shared.applied = state;
        let mut fills: Vec<(Rectangle, Option<Brush>)> = Vec::new();
        let mut strokes: Vec<(Rectangle, Option<Brush>)> = Vec::new();
        let mut backgrounds: Vec<Control> = Vec::new();
        for set in shared.rects.values() {
            if let Some(slot) = &set.fill {
                fills.push((slot.element.clone(), slot.original.clone()));
            }
            if let Some(slot) = &set.stroke {
                strokes.push((slot.element.clone(), slot.original.clone()));
            }
            if let Some(bg) = &set.background
                && let Some(control) = &bg.control
            {
                backgrounds.push(control.clone());
            }
        }
        (fills, strokes, backgrounds)
    };
    crate::tap_log!(
        "apply state={state} fills={} strokes={}",
        fills.len(),
        strokes.len()
    );
    // Held across every setter: the Fill-changed callback re-enters from here
    // and must see PAINTING so our own writes don't loop the apply.
    let _painting = PaintingGuard::begin();
    // `Taskbar.TaskbarBackground` is never mutated (TTB only drives the two
    // rectangles) — the ClearValue on restore just unwinds any brush a stale
    // build left behind, falling back to the template's theme brush.
    if state == STATE_NORMAL {
        for control in backgrounds {
            let _ = Control::BackgroundProperty().and_then(|dp| control.ClearValue(&dp));
        }
    }
    // Each element gets its OWN brush instance: a composition-backed brush
    // carries per-element compositor state, and sharing one across SetFill
    // calls renders black on re-apply.
    for (element, original) in fills {
        let _ = if state == STATE_NORMAL {
            match &original {
                Some(original) => element.SetFill(original).map(|_| ()),
                None => Ok(()),
            }
        } else {
            match appearance_brush(state, &element) {
                Ok(t) => element.SetFill(&t).map(|_| ()),
                Err(e) => {
                    crate::tap_log!("appearance_brush state={state} err={e:?}");
                    Err(e)
                }
            }
        };
    }
    for (element, original) in strokes {
        let _ = if state == STATE_NORMAL {
            match &original {
                Some(original) => element.SetFill(original).map(|_| ()),
                None => Ok(()),
            }
        } else {
            match hidden_brush() {
                Ok(t) => element.SetFill(&t).map(|_| ()),
                Err(_) => Ok(()),
            }
        };
    }
}

/// Runs `apply_current_state` on the XAML UI thread through the captured
/// `CoreDispatcher`. When `wait` is set, blocks (bounded) until the UI thread
/// confirms the brushes were painted — used by the synchronous unload path so
/// the host's `SendMessage` only returns once the taskbar is restored.
pub fn dispatch_apply(wait_ms: u32) {
    let dispatcher = lock().dispatcher.clone();
    let Some(dispatcher) = dispatcher else {
        return;
    };
    let done = if wait_ms > 0 {
        unsafe {
            crate::win32::CreateEventW(
                core::ptr::null(),
                windows_core::BOOL(1),
                windows_core::BOOL(0),
                windows_core::PCWSTR::null(),
            )
        }
    } else {
        core::ptr::null_mut()
    };
    // The handler must be `Send`, so the event handle crosses as `usize`.
    let done_addr = done as usize;
    let result = dispatcher.RunAsync(
        CoreDispatcherPriority::Normal,
        &DispatchedHandler::new(move || {
            apply_current_state();
            if done_addr != 0 {
                unsafe {
                    let _ = crate::win32::SetEvent(done_addr as HANDLE);
                }
            }
            Ok(())
        }),
    );
    if result.is_err() {
        if !done.is_null() {
            unsafe {
                let _ = crate::win32::CloseHandle(done);
            }
        }
        return;
    }
    if !done.is_null() {
        unsafe {
            let _ = crate::win32::WaitForSingleObject(done, wait_ms);
            let _ = crate::win32::CloseHandle(done);
        }
    }
}
