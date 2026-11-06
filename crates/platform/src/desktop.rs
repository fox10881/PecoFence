//! Desktop shell window discovery and z-order anchoring (plan §5.1 / §5.2).
//!
//! Two separate decisions, per the review:
//! * **OS generation** — detected once by capability, never by window shape.
//! * **Icon host** — the top-level window that owns `SHELLDLL_DefView`; re-resolved on every
//!   anchor / show-desktop check because Explorer restructures it at runtime on ≤23H2.

use crate::bindings::*;
use crate::wide::{from_wide, to_wide};
use windows_core::{PCSTR, PCWSTR, PWSTR, Result};

pub use crate::bindings::HWND as RawHwnd;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Generation {
    /// Windows 11 24H2+ : `SHELLDLL_DefView` lives permanently under `Progman`.
    Modern,
    /// ≤ 23H2 : Explorer moves DefView into a top-level `WorkerW` when the desktop is raised.
    Legacy,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HostKind {
    /// 24H2+: host is Progman (always).
    Progman,
    /// Legacy, right after logon: DefView still under Progman; expect a WorkerW takeover later.
    ProgmanNotYetRaised,
    /// Legacy, raised: a top-level WorkerW contains DefView.
    WorkerW,
    /// Fallback: root ancestor of the FolderView list view.
    ListViewRoot,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct IconHost {
    pub host: HWND,
    pub def_view: HWND,
    pub kind: HostKind,
}

/// Windows build number via `RtlGetVersion` (not subject to manifest lies).
pub fn os_build() -> u32 {
    let mut info = OSVERSIONINFOW {
        dwOSVersionInfoSize: size_of::<OSVERSIONINFOW>() as u32,
        ..Default::default()
    };
    // SAFETY: out-pointer to a properly sized local.
    unsafe {
        let _ = RtlGetVersion(&mut info);
    }
    info.dwBuildNumber
}

pub fn shell_window() -> HWND {
    // SAFETY: plain FFI call.
    unsafe { GetShellWindow() }
}

pub fn class_name(hwnd: HWND) -> String {
    let mut buf = [0u16; 128];
    // SAFETY: buffer length is passed; result is NUL-terminated within it.
    let len = unsafe { GetClassNameW(hwnd, PWSTR(buf.as_mut_ptr()), buf.len() as i32) };
    from_wide(&buf[..len.max(0) as usize])
}

pub fn window_pid(hwnd: HWND) -> u32 {
    let mut pid = 0u32;
    // SAFETY: out-pointer to a local.
    unsafe {
        let _ = GetWindowThreadProcessId(hwnd, Some(&mut pid));
    }
    pid
}

pub fn ex_style(hwnd: HWND) -> u32 {
    // SAFETY: plain FFI call.
    unsafe { GetWindowLongW(hwnd, GWL_EXSTYLE) as u32 }
}

pub fn is_visible(hwnd: HWND) -> bool {
    // SAFETY: plain FFI call.
    unsafe { IsWindowVisible(hwnd).as_bool() }
}

pub fn is_iconic(hwnd: HWND) -> bool {
    // SAFETY: plain FFI call.
    unsafe { IsIconic(hwnd).as_bool() }
}

pub fn is_zoomed(hwnd: HWND) -> bool {
    const WS_MAXIMIZE: i32 = 0x0100_0000;
    // SAFETY: plain FFI call — `IsZoomed` is just this style-bit test.
    unsafe { GetWindowLongW(hwnd, GWL_STYLE) & WS_MAXIMIZE != 0 }
}

pub fn is_window(hwnd: HWND) -> bool {
    // SAFETY: plain FFI call.
    unsafe { IsWindow(Some(hwnd)).as_bool() }
}

pub fn root_ancestor(hwnd: HWND) -> HWND {
    // SAFETY: plain FFI call.
    unsafe { GetAncestor(hwnd, GA_ROOT as u32) }
}

pub fn foreground_window() -> HWND {
    // SAFETY: plain FFI call.
    unsafe { GetForegroundWindow() }
}

pub fn window_from_point(x: i32, y: i32) -> HWND {
    // SAFETY: plain FFI call.
    unsafe { WindowFromPoint(POINT { x, y }) }
}

/// Detects the desktop window generation by capability (Lively + Rainmeter probes).
pub fn detect_generation() -> Generation {
    let shell = shell_window();
    let noredir = !shell.0.is_null() && (ex_style(shell) & WS_EX_NOREDIRECTIONBITMAP as u32) != 0;
    // SAFETY: querying an export by name; both handles are process-lifetime.
    let topology_export = unsafe {
        let user32 = GetModuleHandleW(PCWSTR(to_wide("user32.dll").as_ptr()));
        !user32.0.is_null()
            && GetProcAddress(
                user32,
                PCSTR(c"GetCurrentMonitorTopologyId".as_ptr().cast()),
            )
            .is_some()
    };
    if noredir != topology_export {
        tracing::warn!(
            noredir,
            topology_export,
            "generation probes disagree; taking OR"
        );
    }
    if noredir || topology_export {
        Generation::Modern
    } else {
        Generation::Legacy
    }
}

fn find_child(parent: HWND, class: &str) -> HWND {
    let class = to_wide(class);
    // SAFETY: strings outlive the call.
    unsafe { FindWindowExW(Some(parent), None, PCWSTR(class.as_ptr()), PCWSTR::null()) }
}

/// Direct child windows of `parent` (class, rect, visible), for diagnostics.
pub fn child_windows(parent: HWND) -> Vec<(HWND, String, RECT, bool)> {
    let mut out = Vec::new();
    let mut prev: Option<HWND> = None;
    loop {
        // SAFETY: plain FFI enumeration.
        let child = unsafe { FindWindowExW(Some(parent), prev, PCWSTR::null(), PCWSTR::null()) };
        if child.0.is_null() {
            break;
        }
        let mut rect = RECT::default();
        // SAFETY: out-pointer to a local.
        unsafe {
            let _ = GetWindowRect(child, &mut rect);
        }
        out.push((child, class_name(child), rect, is_visible(child)));
        prev = Some(child);
        if out.len() > 64 {
            break;
        }
    }
    out
}

/// The top-level windows of `class`, in z-order (top to bottom): what filtering
/// `top_level_windows` by `class_name` gives, without a class lookup per window.
pub fn windows_of_class(class: &str) -> Vec<HWND> {
    let class = to_wide(class);
    let mut out = Vec::new();
    let mut prev: Option<HWND> = None;
    loop {
        // SAFETY: plain FFI enumeration; the string outlives the call.
        let next = unsafe { FindWindowExW(None, prev, PCWSTR(class.as_ptr()), PCWSTR::null()) };
        if next.0.is_null() {
            break;
        }
        out.push(next);
        prev = Some(next);
    }
    out
}

/// Enumerates top-level windows in z-order (top to bottom).
pub fn top_level_windows() -> Vec<HWND> {
    let mut out: Vec<HWND> = Vec::new();
    // SAFETY: the callback only runs during this call with a pointer to `out`.
    unsafe {
        let _ = EnumWindows(Some(enum_proc), LPARAM(&mut out as *mut Vec<HWND> as isize));
    }
    out
}

unsafe extern "system" fn enum_proc(hwnd: HWND, data: LPARAM) -> windows_core::BOOL {
    // SAFETY: `data` is the Vec pointer from `top_level_windows`.
    unsafe { (*(data.0 as *mut Vec<HWND>)).push(hwnd) };
    windows_core::BOOL(1)
}

/// Resolves the current desktop icon host. Never cache the result across events.
pub fn resolve_icon_host(generation: Generation) -> Option<IconHost> {
    let progman = shell_window();
    if progman.0.is_null() || class_name(progman) != "Progman" {
        return None;
    }
    let def_view_under_progman = find_child(progman, "SHELLDLL_DefView");

    if generation == Generation::Modern {
        // 24H2+: DefView may be absent for a moment during Explorer restart; still anchor to
        // Progman.
        return Some(IconHost {
            host: progman,
            def_view: def_view_under_progman,
            kind: HostKind::Progman,
        });
    }

    if !def_view_under_progman.0.is_null() {
        return Some(IconHost {
            host: progman,
            def_view: def_view_under_progman,
            kind: HostKind::ProgmanNotYetRaised,
        });
    }

    let explorer_pid = window_pid(progman);
    for hwnd in top_level_windows() {
        if class_name(hwnd) != "WorkerW" || window_pid(hwnd) != explorer_pid {
            continue;
        }
        let def_view = find_child(hwnd, "SHELLDLL_DefView");
        if !def_view.0.is_null() {
            return Some(IconHost {
                host: hwnd,
                def_view,
                kind: HostKind::WorkerW,
            });
        }
    }

    // Last resort: locate the FolderView list view anywhere and take its root.
    let list_view = {
        let class = to_wide("SysListView32");
        let title = to_wide("FolderView");
        // SAFETY: strings outlive the call.
        unsafe { FindWindowExW(None, None, PCWSTR(class.as_ptr()), PCWSTR(title.as_ptr())) }
    };
    if !list_view.0.is_null() {
        let root = root_ancestor(list_view);
        return Some(IconHost {
            host: root,
            def_view: find_child(root, "SHELLDLL_DefView"),
            kind: HostKind::ListViewRoot,
        });
    }
    None
}

/// `0x052C` to `Progman` — the undocumented message that makes Explorer spawn
/// a `WorkerW` behind the desktop icons (the classic wallpaper-injection
/// trigger used by Rainmeter/Lively).
const WM_SPAWN_WORKER: u32 = 0x052C;
const SMTO_NORMAL: u32 = 0x0000;

/// The `CoreDeskEngine.DesktopEffects` overlay's window class — kept here so the
/// layer plumbing can find it as a sibling without importing the app crate.
pub const EFFECTS_OVERLAY_CLASS: &str = "CoreDeskEngine.DesktopEffects";

/// The window hosting the desktop background layer, physically below
/// `SHELLDLL_DefView`: children `SetParent`-ed into it render behind the
/// desktop icons and context menus and can never see a click.
pub struct DesktopLayer {
    /// What `SetParent` should target — `Progman` only when no `WorkerW`
    /// exists at all (a last-resort fallback that keeps the old behavior).
    pub host: HWND,
    pub progman: HWND,
    /// `SHELLDLL_DefView` when it is a direct `Progman` child (modern shell).
    pub def_view: HWND,
    /// `Progman`'s `WorkerW` child — the modern-shell background layer.
    pub worker_under_progman: HWND,
    /// The unhosted top-level `WorkerW` — legacy or `0x052C`-spawned layer.
    pub top_worker: HWND,
    /// True when this call created the layer via `0x052C`.
    pub spawned: bool,
}

/// `(SHELLDLL_DefView under Progman, WorkerW under Progman, unhosted top-level WorkerW)`.
fn probe_background_layer(progman: HWND) -> (HWND, HWND, HWND) {
    // SAFETY: read-only window queries.
    unsafe {
        let def_under_progman = FindWindowExW(
            Some(progman),
            None,
            windows_core::w!("SHELLDLL_DefView"),
            PCWSTR::null(),
        );
        let worker_under_progman = FindWindowExW(
            Some(progman),
            None,
            windows_core::w!("WorkerW"),
            PCWSTR::null(),
        );
        let mut top_worker = HWND::default();
        if worker_under_progman.0.is_null() {
            // Legacy or spawned top-level layer: the `WorkerW` sibling that
            // follows the `SHELLDLL_DefView` host in the z-order is unhosted —
            // that one is the background layer.
            for top in top_level_windows() {
                if FindWindowExW(
                    Some(top),
                    None,
                    windows_core::w!("SHELLDLL_DefView"),
                    PCWSTR::null(),
                )
                .0
                .is_null()
                {
                    continue;
                }
                let w = FindWindowExW(None, Some(top), windows_core::w!("WorkerW"), PCWSTR::null());
                if !w.0.is_null() {
                    top_worker = w;
                    break;
                }
            }
        }
        (def_under_progman, worker_under_progman, top_worker)
    }
}

/// Resolves the desktop background layer, spawning it via `0x052C` only when
/// no `WorkerW` layer exists at all — `0x052C` creates a new `WorkerW` every
/// time, so firing it on every attach would leak zombie windows (and their
/// redirection surfaces) inside explorer.exe.
pub fn background_layer() -> Option<DesktopLayer> {
    // SAFETY: standard Win32 window hierarchy calls on shell windows.
    unsafe {
        let progman = {
            let p = FindWindowW(windows_core::w!("Progman"), PCWSTR::null());
            if p.0.is_null() { shell_window() } else { p }
        };
        if progman.0.is_null() {
            return None;
        }
        let (mut def_view, mut worker_under_progman, mut top_worker) =
            probe_background_layer(progman);
        let mut spawned = false;
        if worker_under_progman.0.is_null() && top_worker.0.is_null() {
            // No background layer — have Explorer spawn one behind the icons.
            let r1 = SendMessageTimeoutW(
                progman,
                WM_SPAWN_WORKER,
                WPARAM(0xD),
                LPARAM(1),
                SMTO_NORMAL,
                1000,
                None,
            );
            let r2 = SendMessageTimeoutW(
                progman,
                WM_SPAWN_WORKER,
                WPARAM(0),
                LPARAM(0),
                SMTO_NORMAL,
                1000,
                None,
            );
            spawned = true;
            tracing::debug!(
                spawn_r1 = r1.0,
                spawn_r2 = r2.0,
                "desktop: spawned WorkerW via 0x052C"
            );
            (def_view, worker_under_progman, top_worker) = probe_background_layer(progman);
        }
        // Prefer the real layer over Progman: parenting into Progman lands the
        // window above `SHELLDLL_DefView`, which swallows desktop clicks.
        let host = if !worker_under_progman.0.is_null() {
            worker_under_progman
        } else if !top_worker.0.is_null() {
            top_worker
        } else {
            progman
        };
        Some(DesktopLayer {
            host,
            progman,
            def_view,
            worker_under_progman,
            top_worker,
            spawned,
        })
    }
}

/// `GetAncestor(GA_PARENT)` — the true reparented parent. `GetParent` is not
/// usable here: on a `WS_POPUP` window it reads the *owner* slot, which stays
/// null even after `SetParent` has moved the window into a foreign tree
/// (`GA_PARENT` always reflects the real parent).
pub fn parent_of(hwnd: HWND) -> HWND {
    // SAFETY: plain FFI call.
    unsafe { GetAncestor(hwnd, GA_PARENT as u32) }
}

/// True when `hwnd` was reparented into a foreign window — i.e. it lives in
/// the desktop background layer, whose child z-order is the layer's business
/// and must not be touched by top-level anchoring. `GA_ROOT != hwnd` is the
/// test that survives `SetParent`'s popup/parent quirks: any window inside
/// another window's tree has a root above itself.
pub fn is_child(hwnd: HWND) -> bool {
    root_ancestor(hwnd) != hwnd
}

/// Reparents `hwnd` into the desktop background layer, covering the screen
/// rect `(vx, vy, w, h)` and sitting at the top of the layer's child z-order
/// (above a video-wallpaper sibling, still below `SHELLDLL_DefView` — the
/// icons own every click now). `WS_EX_TRANSPARENT` is OR-ed onto the ex-style
/// as belt-and-braces. Returns the layer host, `None` when no shell window
/// exists to attach to — the caller then keeps the window top-level, where
/// `HTTRANSPARENT` hit-testing already makes it click-through.
///
/// SAFETY: `hwnd` must be a live window owned by the calling thread.
pub unsafe fn attach_to_background_layer(
    hwnd: HWND,
    vx: i32,
    vy: i32,
    w: i32,
    h: i32,
) -> Option<HWND> {
    // SAFETY: window-hierarchy calls on our own HWND and shell windows.
    unsafe {
        let layer = background_layer()?;
        crate::wmf_bindings::SetParent(
            crate::wmf_bindings::HWND(hwnd.0),
            Some(crate::wmf_bindings::HWND(layer.host.0)),
        );
        if parent_of(hwnd) != layer.host {
            let err = windows_core::Error::from_thread();
            let actual = parent_of(hwnd);
            tracing::warn!(
                ?hwnd,
                host = ?layer.host,
                ?actual,
                error = %err,
                "desktop: reparent into background layer failed"
            );
            return None;
        }
        SetWindowLongPtrW(hwnd, GWL_STYLE, (WS_CHILD | WS_CLIPSIBLINGS) as isize);
        SetWindowLongPtrW(
            hwnd,
            GWL_EXSTYLE,
            (ex_style(hwnd) | WS_EX_TRANSPARENT as u32) as isize,
        );
        let mut wr = RECT::default();
        let _ = GetWindowRect(layer.host, &mut wr);
        let ok = SetWindowPos(
            hwnd,
            Some(HWND_TOP),
            vx - wr.left,
            vy - wr.top,
            w,
            h,
            (SWP_NOACTIVATE | SWP_FRAMECHANGED) as u32,
        );
        tracing::info!(
            ?hwnd,
            host = ?layer.host,
            spawned = layer.spawned,
            ok = ok.as_bool(),
            x = vx - wr.left,
            y = vy - wr.top,
            w,
            h,
            "desktop: surface attached to the WorkerW background layer"
        );
        Some(layer.host)
    }
}

const ANCHOR_FLAGS: u32 = (SWP_NOMOVE | SWP_NOSIZE | SWP_NOACTIVATE | SWP_NOOWNERZORDER) as u32;

/// A foreign window that can actually occlude the desktop. Minimized terminal
/// windows and zero-sized pseudo-console hosts may keep WS_VISIBLE set.
pub fn has_visible_surface(hwnd: HWND) -> bool {
    if !is_visible(hwnd) || is_iconic(hwnd) || is_cloaked(hwnd) {
        return false;
    }
    let mut rect = RECT::default();
    // SAFETY: caller-supplied HWND and a local, correctly sized output rectangle.
    (unsafe { GetWindowRect(hwnd, &mut rect).as_bool() })
        && rect.right > rect.left
        && rect.bottom > rect.top
}

/// `HRESULT_FROM_WIN32(ERROR_ACCESS_DENIED)`: what `SetWindowPos` reports when UIPI refuses it.
const E_ACCESS_DENIED: windows_core::HRESULT = windows_core::HRESULT(0x8007_0005_u32 as i32);

/// Inserts `hwnd` directly above `host` in the z-order: below the lowest window above `host`
/// that paints a surface, or at the top of the normal band when there is none (HWND_BOTTOM
/// would put `hwnd` under a raised `host`) or when that window is topmost while `host` is not:
/// with no app window open it is the taskbar, and a window inserted after a topmost one becomes
/// topmost itself. A topmost `host` (a fence during Peek) keeps `hwnd` right above it.
///
/// Placing a window below one of an elevated process (an app run as administrator, lowest
/// above the desktop) is refused by UIPI, which would leave `hwnd` wherever it is — over every
/// app. When `host` is the bottom-most window (Progman on current Windows), `hwnd` then goes to
/// the bottom and `host` back under it, which references no foreign window; `hwnd` spends well
/// under a millisecond behind the desktop.
pub fn insert_above(hwnd: HWND, host: HWND) -> Result<()> {
    // SAFETY: plain FFI calls on caller-supplied handles.
    unsafe {
        let above = surface_window_above(host, hwnd);
        if above == hwnd {
            return Ok(());
        }
        let insert_after = if above.0.is_null() || (is_topmost(above) && !is_topmost(host)) {
            HWND_TOP
        } else {
            above
        };
        match SetWindowPos(hwnd, Some(insert_after), 0, 0, 0, 0, ANCHOR_FLAGS).ok() {
            Err(e)
                if e.code() == E_ACCESS_DENIED
                    && GetWindow(host, GW_HWNDNEXT as u32).0.is_null() =>
            {
                tracing::debug!(
                    ?above,
                    "placement under an elevated window refused; anchoring from the bottom"
                );
                SetWindowPos(hwnd, Some(HWND_BOTTOM), 0, 0, 0, 0, ANCHOR_FLAGS).ok()?;
                SetWindowPos(host, Some(HWND_BOTTOM), 0, 0, 0, 0, ANCHOR_FLAGS).ok()
            }
            placed => placed,
        }
    }
}

/// The lowest window above `host` that paints a surface, which `insert_above` places `hwnd`
/// under; `hwnd` itself when it comes first, a null HWND when there is none.
pub fn surface_window_above(host: HWND, hwnd: HWND) -> HWND {
    // SAFETY: plain FFI calls on caller-supplied handles.
    unsafe {
        let mut above = GetWindow(host, GW_HWNDPREV as u32);
        // Nonvisual windows can sit directly above Progman. A minimized elevated
        // terminal may reject relative placement even though it paints no surface.
        while !above.0.is_null() && above != hwnd && !has_visible_surface(above) {
            above = GetWindow(above, GW_HWNDPREV as u32);
        }
        above
    }
}

/// The window directly above `hwnd` in the z-order (null HWND if none).
pub fn window_above(hwnd: HWND) -> HWND {
    // SAFETY: plain FFI call.
    unsafe { GetWindow(hwnd, GW_HWNDPREV as u32) }
}

/// Inserts `hwnd` directly below `reference` in the z-order.
pub fn insert_after(hwnd: HWND, reference: HWND) -> Result<()> {
    // SAFETY: plain FFI call.
    unsafe { SetWindowPos(hwnd, Some(reference), 0, 0, 0, 0, ANCHOR_FLAGS).ok() }
}

pub fn is_topmost(hwnd: HWND) -> bool {
    ex_style(hwnd) & WS_EX_TOPMOST as u32 != 0
}

/// Moves `hwnd` into (or out of) the topmost band without activating it.
pub fn set_topmost(hwnd: HWND, on: bool) -> Result<()> {
    let after = if on { HWND_TOPMOST } else { HWND_NOTOPMOST };
    // SAFETY: plain FFI call.
    unsafe { SetWindowPos(hwnd, Some(after), 0, 0, 0, 0, ANCHOR_FLAGS).ok() }
}

/// Raises `hwnd` to the top of whichever band (topmost / normal) it is in.
pub fn raise_in_band(hwnd: HWND) -> Result<()> {
    // SAFETY: plain FFI call.
    unsafe { SetWindowPos(hwnd, Some(HWND_TOP), 0, 0, 0, 0, ANCHOR_FLAGS).ok() }
}

/// Pushes `hwnd` to the bottom of the non-topmost band.
pub fn send_to_bottom(hwnd: HWND) -> Result<()> {
    // SAFETY: plain FFI call.
    unsafe { SetWindowPos(hwnd, Some(HWND_BOTTOM), 0, 0, 0, 0, ANCHOR_FLAGS).ok() }
}

/// True when a window of class `class` exists *below* `host` in the z-order — i.e. Explorer
/// has raised the desktop above our (normally just-above-host) windows.
pub fn is_below(host: HWND, class: &str) -> bool {
    let class = to_wide(class);
    // SAFETY: strings outlive the call.
    let found = unsafe { FindWindowExW(None, Some(host), PCWSTR(class.as_ptr()), PCWSTR::null()) };
    !found.0.is_null()
}

/// The lowest visible `WS_EX_TOPMOST` window, i.e. the boundary between the topmost band and
/// the normal band. Inserting after it puts a window on top of all normal windows.
pub fn lowest_topmost_window() -> Option<HWND> {
    top_level_windows()
        .into_iter()
        .filter(|&h| is_visible(h) && (ex_style(h) & WS_EX_TOPMOST as u32) != 0)
        .last()
}

pub fn is_cloaked(hwnd: HWND) -> bool {
    let mut cloaked: u32 = 0;
    // SAFETY: out-pointer sized for a DWORD.
    unsafe {
        let _ = DwmGetWindowAttribute(
            hwnd,
            DWMWA_CLOAKED as u32,
            (&mut cloaked as *mut u32).cast(),
            size_of::<u32>() as u32,
        );
    }
    cloaked != 0
}

pub fn set_cloak(hwnd: HWND, cloak: bool) -> Result<()> {
    let value: i32 = cloak as i32;
    // SAFETY: value outlives the call.
    unsafe {
        DwmSetWindowAttribute(
            hwnd,
            DWMWA_CLOAK as u32,
            (&value as *const i32).cast(),
            size_of::<i32>() as u32,
        )
        .ok()
    }
}

pub fn hide_window(hwnd: HWND) {
    // SAFETY: plain FFI call.
    unsafe {
        let _ = ShowWindow(hwnd, SW_HIDE);
    }
}

/// Injects a left double-click at screen coordinates (test tooling only).
pub fn send_double_click(x: i32, y: i32) {
    // SAFETY: synthesizes input for the current session.
    unsafe {
        let _ = SetCursorPos(x, y);
        for _ in 0..2 {
            mouse_event(MOUSEEVENTF_LEFTDOWN as u32, 0, 0, 0, 0);
            mouse_event(MOUSEEVENTF_LEFTUP as u32, 0, 0, 0, 0);
            std::thread::sleep(std::time::Duration::from_millis(60));
        }
    }
}

pub fn show_no_activate(hwnd: HWND) {
    // SAFETY: plain FFI call.
    unsafe {
        let _ = ShowWindow(hwnd, SW_SHOWNOACTIVATE);
    }
}

/// Registers (or looks up) the `TaskbarCreated` broadcast message id.
pub fn taskbar_created_message() -> u32 {
    let name = to_wide("TaskbarCreated");
    // SAFETY: string outlives the call.
    unsafe { RegisterWindowMessageW(PCWSTR(name.as_ptr())) }
}

/// Injects a Win+D keystroke (test tooling only).
pub fn send_win_d() {
    const VK_D: u8 = 0x44;
    // SAFETY: plain FFI calls; synthesizes keyboard input for the current session.
    unsafe {
        keybd_event(VK_LWIN as u8, 0, 0, 0);
        keybd_event(VK_D, 0, 0, 0);
        keybd_event(VK_D, 0, KEYEVENTF_KEYUP as u32, 0);
        keybd_event(VK_LWIN as u8, 0, KEYEVENTF_KEYUP as u32, 0);
    }
}

/// Taps the unassigned virtual key 0xE8 while Alt is held, so that releasing Alt is not a lone
/// Alt press for the foreground window (which would open its menu bar). AutoHotkey's
/// `#MenuMaskKey` does the same.
pub fn mask_alt_release() {
    const VK_UNASSIGNED: u8 = 0xE8;
    // SAFETY: plain FFI calls; synthesizes keyboard input for the current session.
    unsafe {
        keybd_event(VK_UNASSIGNED, 0, 0, 0);
        keybd_event(VK_UNASSIGNED, 0, KEYEVENTF_KEYUP as u32, 0);
    }
}

/// Human-readable z-order dump around the fences, for spike logs.
pub fn describe_zorder(interesting: &[HWND], all_visible: bool) -> String {
    let mut lines = Vec::new();
    for (i, h) in top_level_windows().into_iter().enumerate() {
        let cls = class_name(h);
        let tagged = interesting.contains(&h);
        let is_shell = cls == "Progman" || (cls == "WorkerW" && is_visible(h));
        if tagged || is_shell || (all_visible && is_visible(h) && !is_cloaked(h)) {
            lines.push(format!(
                "{i:4} {:>18} {:#x} vis={} cloak={} iconic={}{}",
                cls,
                h.0 as usize,
                is_visible(h) as u8,
                is_cloaked(h) as u8,
                is_iconic(h) as u8,
                if tagged { "  <== ours" } else { "" }
            ));
        }
    }
    lines.join("\n")
}

#[cfg(test)]
mod anchor_tests {
    use super::*;
    use crate::window::{ClassOptions, WindowBuilder, WindowClass, ZOrder, style, swp};

    #[test]
    fn desktop_placement_skips_hidden_popups() -> Result<()> {
        let class =
            WindowClass::register("CoreDeskEngine.Test.VisibleAnchor", ClassOptions::default())?;
        let make = || {
            WindowBuilder::new(&class)
                .style(style::POPUP)
                .ex_style(style::EX_TOOLWINDOW | style::EX_NOACTIVATE)
                .bounds(-32000, -32000, 1, 1)
                .create(Box::new(|_, _, _, _| None))
        };
        let host = make()?;
        let reference = make()?;
        let hidden_popup = make()?;
        let target = make()?;
        host.show_no_activate();
        reference.show_no_activate();
        target.show_no_activate();
        hidden_popup.set_bounds_z(
            0,
            0,
            0,
            0,
            ZOrder::After(reference.hwnd()),
            swp::NOMOVE | swp::NOSIZE,
        )?;
        host.set_bounds_z(
            0,
            0,
            0,
            0,
            ZOrder::After(hidden_popup.hwnd()),
            swp::NOMOVE | swp::NOSIZE,
        )?;
        assert_eq!(window_above(host.hwnd()), hidden_popup.hwnd());
        insert_above(target.hwnd(), host.hwnd())?;
        assert_eq!(window_above(target.hwnd()), reference.hwnd());
        // Already in the correct visible position: do not try inserting after itself.
        insert_above(target.hwnd(), host.hwnd())?;
        assert_eq!(window_above(target.hwnd()), reference.hwnd());
        Ok(())
    }

    #[test]
    fn desktop_placement_skips_empty_minimized_and_cloaked_windows() -> Result<()> {
        let class = WindowClass::register(
            "CoreDeskEngine.Test.NonvisualAnchor",
            ClassOptions::default(),
        )?;
        let make = || {
            WindowBuilder::new(&class)
                .style(style::POPUP)
                .ex_style(style::EX_TOOLWINDOW | style::EX_NOACTIVATE)
                .bounds(-32000, -32000, 1, 1)
                .create(Box::new(|_, _, _, _| None))
        };
        for mode in ["empty", "minimized", "cloaked"] {
            let host = make()?;
            let reference = make()?;
            let phantom = make()?;
            let target = make()?;
            for w in [&host, &reference, &phantom, &target] {
                w.show_no_activate();
            }
            match mode {
                "empty" => phantom.set_bounds(-32000, -32000, 0, 0)?,
                "minimized" => {
                    // SAFETY: our own off-screen fixture, without activation.
                    unsafe {
                        windows_sys::Win32::UI::WindowsAndMessaging::ShowWindow(
                            phantom.hwnd().0,
                            windows_sys::Win32::UI::WindowsAndMessaging::SW_SHOWMINNOACTIVE,
                        );
                    }
                }
                "cloaked" => set_cloak(phantom.hwnd(), true)?,
                _ => unreachable!(),
            }
            assert!(!has_visible_surface(phantom.hwnd()), "{mode}");
            phantom.set_bounds_z(
                0,
                0,
                0,
                0,
                ZOrder::After(reference.hwnd()),
                swp::NOMOVE | swp::NOSIZE,
            )?;
            host.set_bounds_z(
                0,
                0,
                0,
                0,
                ZOrder::After(phantom.hwnd()),
                swp::NOMOVE | swp::NOSIZE,
            )?;
            insert_above(target.hwnd(), host.hwnd())?;
            assert_eq!(window_above(target.hwnd()), reference.hwnd(), "{mode}");
        }
        Ok(())
    }
}
