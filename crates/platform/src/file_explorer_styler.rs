//! Host-side controller for the File Explorer XAML styler — the native
//! replacement for the Windhawk `windows-11-file-explorer-styler` mod.
//!
//! Same delivery path as the taskbar styler (`tap::ensure_resident` +
//! `WM_COPYDATA` with a `PECO-FES1` wire), but the resident routes it to
//! explorer.exe's **WinUI3** diagnostics session — the one
//! `Microsoft.Internal.FrameworkUdk.dll` hosts — which sees the
//! `CabinetWClass` XAML islands. The taskbar's WUX session is untouched.
//!
//! The theme's `BackgroundTranslucentEffect` maps to a direct
//! `DwmSetWindowAttribute(DWMWA_SYSTEMBACKDROP_TYPE)` call on every
//! `CabinetWClass` — the same attribute the mod rewrites through its
//! `DwmSetWindowAttribute` hook. We don't hook; explorer can re-assert its
//! own backdrop later, so `ensure` re-applies ours on each tick.

use std::sync::Mutex;
use std::sync::atomic::{AtomicBool, Ordering};
use windows_core::{PCSTR, PCWSTR};

windows_core::link!("kernel32.dll" "system" fn GetModuleHandleW(name: PCWSTR) -> *mut core::ffi::c_void);
windows_core::link!("kernel32.dll" "system" fn GetProcAddress(hmodule: *mut core::ffi::c_void, lpprocname: PCSTR) -> *mut core::ffi::c_void);
windows_core::link!("dwmapi.dll" "system" fn DwmSetWindowAttribute(hwnd: crate::bindings::HWND, dwattribute: u32, pvattribute: *const core::ffi::c_void, cbattribute: u32) -> i32);
windows_core::link!("dwmapi.dll" "system" fn DwmEnableBlurBehindWindow(hwnd: crate::bindings::HWND, bb: *const DwmBlurBehind) -> i32);
windows_core::link!("gdi32.dll" "system" fn CreateRectRgn(left: i32, top: i32, right: i32, bottom: i32) -> *mut core::ffi::c_void);
windows_core::link!("gdi32.dll" "system" fn DeleteObject(obj: *mut core::ffi::c_void) -> windows_core::BOOL);
windows_core::link!("user32.dll" "system" fn GetClassNameW(hwnd: crate::bindings::HWND, lpclassname: windows_core::PWSTR, nmaxcount: i32) -> i32);
windows_core::link!("user32.dll" "system" fn RedrawWindow(hwnd: crate::bindings::HWND, lprcupdate: *const core::ffi::c_void, hrgnupdate: *mut core::ffi::c_void, flags: u32) -> windows_core::BOOL);

/// `COPYDATASTRUCT.dwData` values — must match `crates/tap/src/command.rs`.
const CDS_FE_THEME: usize = u32::from_be_bytes(*b"FEST") as usize;
const CDS_FE_RESET: usize = u32::from_be_bytes(*b"FESR") as usize;
const CDS_FE_HELLO: usize = u32::from_be_bytes(*b"FEHI") as usize;
/// Backdrop/accent state for the resident's `fe_glass` detours —
/// `[backdrop:i32, accent_state:u32, accent_flags:u32, gradient:u32]` LE.
const CDS_FE_BACKDROP: usize = u32::from_be_bytes(*b"FEBD") as usize;

// DWMWA_SYSTEMBACKDROP_TYPE (38) values — the same DWMSBT_* mapping the mod
// applies through its DwmSetWindowAttribute hook.
const DWMWA_SYSTEMBACKDROP_TYPE: u32 = 38;
const DWMSBT_AUTO: i32 = 0; // let the system decide — used on the accent path
const DWMSBT_MAINWINDOW: i32 = 2; // Mica
const DWMSBT_TRANSIENTWINDOW: i32 = 3; // Acrylic
const DWMSBT_TABBEDWINDOW: i32 = 4; // Mica Alt

const ACCENT_DISABLED: u32 = 0;
const ACCENT_ENABLE_ACRYLICBLURBEHIND: u32 = 4;

/// Translucent-Windows `HandleEffects` mutual exclusion:
/// `DWMWA_SYSTEMBACKDROP_TYPE` (2/3/4) and `WCA_ACCENT_POLICY`
/// (`ACCENT_STATE_ENABLE_ACRYLICBLURBEHIND`) are never both active — an
/// extended `-1` frame with a conflicting pair renders a solid black void.
/// Returns `(backdrop, accent_state, accent_flags, gradient_abgr)`.
/// - Case B (`blur` effect, or a Color Tint swatch selected): backdrop
///   drops to `DWMSBT_AUTO`, accent acrylic carries the tint.
/// - Case A (acrylic/mica/micaAlt, no tint): accent off, system backdrop
///   carries the material.
fn desired_glass(effect: &str, tint: &str) -> (i32, u32, u32, u32) {
    if effect == "blur" || !tint.is_empty() {
        (
            DWMSBT_AUTO,
            ACCENT_ENABLE_ACRYLICBLURBEHIND,
            0,
            tint_abgr(tint),
        )
    } else {
        let backdrop = match effect {
            "acrylic" => DWMSBT_TRANSIENTWINDOW,
            "mica" => DWMSBT_MAINWINDOW,
            "micaAlt" => DWMSBT_TABBEDWINDOW,
            _ => DWMSBT_TABBEDWINDOW,
        };
        (backdrop, ACCENT_DISABLED, 0, 0)
    }
}

const RDW_INVALIDATE: u32 = 0x0001;
const RDW_ERASE: u32 = 0x0004;
const RDW_FRAME: u32 = 0x0400;
const RDW_ALLCHILDREN: u32 = 0x0080;
const SWP_NOMOVE: u32 = 0x0002;
const SWP_NOSIZE: u32 = 0x0001;
const SWP_NOZORDER: u32 = 0x0004;
const SWP_NOACTIVATE: u32 = 0x0010;
const SWP_FRAMECHANGED: u32 = 0x0020;

struct Fe {
    /// The catalog id last delivered ("" = Windows Default — nothing armed).
    style: String,
    /// The wire last built — cached so `ensure` re-delivers after an
    /// explorer restart without re-serializing the catalog.
    wire: String,
    /// The resident command window the wire was delivered to.
    cmd_hwnd: usize,
    /// Whether `CDS_FE_HELLO` reached the current DLL residency.
    hello_sent: bool,
    /// The `backgroundTranslucentEffect` currently pushed to cabinets —
    /// `apply_backdrop` only touches windows when this changes or a new
    /// `CabinetWClass` appears, so the 2 s ensure tick stays silent.
    applied_effect: String,
    /// Cabinet hwnds the current `applied_effect` was pushed to.
    seen_hwnds: std::collections::HashSet<isize>,
    /// Selected `color_tint` (`#RRGGBB`, `""` = off) — drives the accent
    /// blur's `GradientColor` so the glass takes the overlay color.
    tint: String,
    /// Tint currently pushed to cabinets — `apply_backdrop` rewrites
    /// when it changes, same change-detection as `applied_effect`.
    applied_tint: String,
}

static FE: Mutex<Option<Fe>> = Mutex::new(None);
/// Whether a File Explorer style is armed — read lock-free.
static ARMED: AtomicBool = AtomicBool::new(false);

/// `true` while a File Explorer style is armed — `tap` keeps the DLL resident.
pub fn is_armed() -> bool {
    ARMED.load(Ordering::Acquire)
}

fn lock() -> std::sync::MutexGuard<'static, Option<Fe>> {
    FE.lock().unwrap_or_else(|p| p.into_inner())
}

fn widestr(s: &str) -> Vec<u16> {
    s.encode_utf16().chain(core::iter::once(0)).collect()
}

/// Every `CabinetWClass` that is a real File Explorer window — the mod's
/// `GetTargetWindowType`: frames with a `WorkerW` child are the legacy
/// Control Panel shell, which has no XAML islands and isn't targeted.
fn cabinet_windows() -> Vec<crate::bindings::HWND> {
    struct Ctx {
        out: Vec<crate::bindings::HWND>,
    }
    unsafe extern "system" fn cb(
        hwnd: crate::bindings::HWND,
        lparam: crate::bindings::LPARAM,
    ) -> windows_core::BOOL {
        let _ = std::panic::catch_unwind(core::panic::AssertUnwindSafe(|| {
            let ctx = unsafe { &mut *(lparam.0 as *mut Ctx) };
            let mut cls = [0u16; 64];
            let n = unsafe { GetClassNameW(hwnd, windows_core::PWSTR(cls.as_mut_ptr()), 64) };
            if n <= 0 || String::from_utf16_lossy(&cls[..n as usize]) != "CabinetWClass" {
                return;
            }
            let worker = widestr("WorkerW");
            let legacy = unsafe {
                crate::bindings::FindWindowExW(
                    Some(hwnd),
                    None,
                    PCWSTR(worker.as_ptr()),
                    PCWSTR::null(),
                )
            };
            if !legacy.0.is_null() {
                return;
            }
            ctx.out.push(hwnd);
        }));
        windows_core::BOOL(1)
    }
    let mut ctx = Ctx { out: Vec::new() };
    unsafe {
        let _ = crate::bindings::EnumWindows(
            Some(cb),
            crate::bindings::LPARAM(&mut ctx as *mut Ctx as isize),
        );
    }
    ctx.out
}

/// `ApplyBackgroundTranslucentEffect` — direct `DwmSetWindowAttribute` on
/// each cabinet window (`""` restores the stock `DWMSBT_TABBEDWINDOW`
/// backdrop File Explorer requests for itself, and drops the accent blur
/// the blur path sets). Runs only when the effect changes or a new
/// cabinet appears — the ensure tick calls this every 2 s, so painting
/// everything unconditionally would be a constant repaint storm.
fn apply_backdrop(effect: &str) {
    let hwnds = cabinet_windows();
    let (changed, mut seen, tint) = {
        let mut guard = lock();
        let Some(fe) = guard.as_mut() else {
            return;
        };
        let changed = fe.applied_effect != effect || fe.applied_tint != fe.tint;
        if changed {
            fe.applied_effect = effect.to_string();
            fe.applied_tint = fe.tint.clone();
            fe.seen_hwnds.clear();
        }
        (changed, std::mem::take(&mut fe.seen_hwnds), fe.tint.clone())
    };
    let (backdrop, accent_state, accent_flags, gradient) = desired_glass(effect, &tint);
    let accent_on = accent_state == ACCENT_ENABLE_ACRYLICBLURBEHIND;
    // Any glass path — a named effect OR a tint overlay — needs the
    // extended frame + dark caption + blur-behind region.
    let glass = !effect.is_empty() || accent_on;
    for hwnd in &hwnds {
        if !changed && seen.contains(&(hwnd.0 as usize as isize)) {
            continue;
        }
        unsafe {
            let _ = DwmSetWindowAttribute(
                *hwnd,
                DWMWA_SYSTEMBACKDROP_TYPE,
                &backdrop as *const i32 as *const core::ffi::c_void,
                core::mem::size_of::<i32>() as u32,
            );
            // Immersive dark caption while a glass theme is armed.
            let dark: i32 = glass as i32;
            let _ = DwmSetWindowAttribute(
                *hwnd,
                20, // DWMWA_USE_IMMERSIVE_DARK_MODE
                &dark as *const i32 as *const core::ffi::c_void,
                core::mem::size_of::<i32>() as u32,
            );
        }
        extend_frame(*hwnd, glass);
        blur_behind(*hwnd, accent_on);
        set_accent_policy(*hwnd, accent_state, accent_flags, gradient);
        trigger_composition_update(*hwnd);
        seen.insert(hwnd.0 as usize as isize);
    }
    let mut guard = lock();
    if let Some(fe) = guard.as_mut() {
        // Keep only live cabinets — dead hwnds can be reused by the OS.
        seen.retain(|h| hwnds.iter().any(|w| w.0 as usize as isize == *h));
        fe.seen_hwnds = seen;
    }
}

#[repr(C)]
struct DwmBlurBehind {
    flags: u32,
    enable: windows_core::BOOL,
    rgn: *mut core::ffi::c_void,
    transition_on_max: windows_core::BOOL,
}

/// `DwmEnableBlurBehindWindow` — the accent path blurs the whole client
/// region (`CreateRectRgn(0,0,-1,-1)`), the SYSTEMBACKDROP path disables
/// it: blur-behind plus a system backdrop is another mutually-exclusive
/// pairing that renders black instead of glass.
fn blur_behind(hwnd: crate::bindings::HWND, on: bool) {
    let rgn = if on {
        unsafe { CreateRectRgn(0, 0, -1, -1) }
    } else {
        core::ptr::null_mut()
    };
    let bb = DwmBlurBehind {
        flags: 1 | if on { 2 } else { 0 }, // DWM_BB_ENABLE | DWM_BB_BLURREGION
        enable: windows_core::BOOL(on as i32),
        rgn,
        transition_on_max: windows_core::BOOL(0),
    };
    unsafe {
        let _ = DwmEnableBlurBehindWindow(hwnd, &bb);
        if !rgn.is_null() {
            let _ = DeleteObject(rgn);
        }
    }
}

/// `DwmExtendFrameIntoClientArea` — the prompt's mandatory Step 3 call:
/// `{ -1, -1, -1, -1 }` pulls the glass frame over the whole client area
/// so a `Transparent` XAML/Win32 background actually shows the backdrop;
/// `{0,0,0,0}` restores. The resident detour keeps explorer from
/// resetting it while a theme is armed.
fn extend_frame(hwnd: crate::bindings::HWND, on: bool) {
    #[repr(C)]
    struct Margins {
        left: i32,
        right: i32,
        top: i32,
        bottom: i32,
    }
    let dwm = unsafe { GetModuleHandleW(PCWSTR(widestr("dwmapi.dll").as_ptr())) };
    if dwm.is_null() {
        return;
    }
    let f = unsafe {
        GetProcAddress(
            dwm,
            PCSTR(c"DwmExtendFrameIntoClientArea".as_ptr() as *const u8),
        )
    };
    if f.is_null() {
        return;
    }
    let f: unsafe extern "system" fn(crate::bindings::HWND, *const Margins) -> i32 =
        unsafe { core::mem::transmute(f) };
    let m = if on {
        Margins {
            left: -1,
            right: -1,
            top: -1,
            bottom: -1,
        }
    } else {
        Margins {
            left: 0,
            right: 0,
            top: 0,
            bottom: 0,
        }
    };
    unsafe {
        let _ = f(hwnd, &m);
    }
}

/// `#RRGGBB` → ABGR `GradientColor` (`alpha<<24 | blue<<16 | green<<8 | red`)
/// for the accent policy — the selected swatch color, or the prompt's
/// subtle dark-glass default `0x40101010` when no tint is picked.
fn tint_abgr(tint: &str) -> u32 {
    let hex = tint.strip_prefix('#').unwrap_or(tint);
    if hex.len() == 6
        && let Ok(rgb) = u32::from_str_radix(hex, 16)
    {
        let r = rgb >> 16 & 0xFF;
        let g = rgb >> 8 & 0xFF;
        let b = rgb & 0xFF;
        return 0x80 << 24 | b << 16 | g << 8 | r;
    }
    0x40101010
}

/// `SetWindowCompositionAttribute` — `WCA_ACCENT_POLICY` acrylic blur
/// (the `blur` effect and the Color Tint overlay path). `state` is
/// `ACCENT_STATE_*` (4 = acrylic blur behind, 0 = disabled); `gradient`
/// is the ABGR `GradientColor` (`alpha<<24 | blue<<16 | green<<8 | red`).
/// Never combined with a non-AUTO `DWMWA_SYSTEMBACKDROP_TYPE` — see
/// `desired_glass`.
fn set_accent_policy(hwnd: crate::bindings::HWND, state: u32, flags: u32, gradient: u32) {
    #[repr(C)]
    struct AccentPolicy {
        accent_state: u32,
        accent_flags: u32,
        gradient_color: u32,
        animation_id: u32,
    }
    #[repr(C)]
    struct WindowCompositionAttribData {
        attrib: u32,
        pv_data: *const core::ffi::c_void,
        cb_data: usize,
    }
    const WCA_ACCENT_POLICY: u32 = 19;

    let user32 = unsafe { GetModuleHandleW(PCWSTR(widestr("user32.dll").as_ptr())) };
    if user32.is_null() {
        return;
    }
    let f = unsafe {
        GetProcAddress(
            user32,
            PCSTR(c"SetWindowCompositionAttribute".as_ptr() as *const u8),
        )
    };
    if f.is_null() {
        return;
    }
    let f: unsafe extern "system" fn(
        crate::bindings::HWND,
        *const WindowCompositionAttribData,
    ) -> windows_core::BOOL = unsafe { core::mem::transmute(f) };
    let policy = AccentPolicy {
        accent_state: state,
        accent_flags: flags,
        gradient_color: gradient,
        animation_id: 0,
    };
    let data = WindowCompositionAttribData {
        attrib: WCA_ACCENT_POLICY,
        pv_data: &policy as *const AccentPolicy as *const core::ffi::c_void,
        cb_data: core::mem::size_of::<AccentPolicy>(),
    };
    unsafe {
        let _ = f(hwnd, &data);
    }
}

/// `TriggerWindowCompositionUpdate` — the mod sends `WM_WINDOWPOSCHANGED`
/// in-process; cross-process that message's `WINDOWPOS*` is not marshalled,
/// so the target would dereference our stack. `SetWindowPos` with
/// `SWP_FRAMECHANGED` is the equivalent that generates a real
/// `WM_WINDOWPOSCHANGED`/`WM_NCCALCSIZE` inside explorer — same nudge the
/// mod is after, no pointer hand-off.
fn trigger_composition_update(hwnd: crate::bindings::HWND) {
    unsafe {
        let _ = crate::bindings::SetWindowPos(
            hwnd,
            Some(crate::bindings::HWND(core::ptr::null_mut())),
            0,
            0,
            0,
            0,
            SWP_NOMOVE | SWP_NOSIZE | SWP_NOZORDER | SWP_NOACTIVATE | SWP_FRAMECHANGED,
        );
        let _ = RedrawWindow(
            hwnd,
            core::ptr::null(),
            core::ptr::null_mut(),
            RDW_INVALIDATE | RDW_ERASE | RDW_FRAME | RDW_ALLCHILDREN,
        );
    }
}

/// Sends the stored wire to the resident DLL — same hello/wire pattern as
/// the taskbar styler so the death watch re-arms per injection.
fn deliver(hwnd: crate::bindings::HWND, fe: &mut Fe) -> bool {
    if fe.cmd_hwnd != hwnd.0 as usize {
        fe.hello_sent = false;
    }
    if !fe.hello_sent {
        let pid = std::process::id();
        if crate::tap::send_copydata(hwnd, CDS_FE_HELLO, &pid.to_ne_bytes()) {
            fe.hello_sent = true;
        }
    }
    // The resident's fe_glass detours need our desired backdrop BEFORE the
    // theme wire arms them — send FEBD first so `set_armed`'s in-process
    // apply already pushes the right values.
    let effect = pecofence_core::file_explorer_themes::backdrop(&fe.style).unwrap_or_default();
    let (bd, st, fl, gr) = desired_glass(effect, &fe.tint);
    let mut payload = [0u8; 16];
    payload[0..4].copy_from_slice(&bd.to_le_bytes());
    payload[4..8].copy_from_slice(&st.to_le_bytes());
    payload[8..12].copy_from_slice(&fl.to_le_bytes());
    payload[12..16].copy_from_slice(&gr.to_le_bytes());
    let _ = crate::tap::send_copydata(hwnd, CDS_FE_BACKDROP, &payload);
    if crate::tap::send_copydata(hwnd, CDS_FE_THEME, fe.wire.as_bytes()) {
        fe.cmd_hwnd = hwnd.0 as usize;
        true
    } else {
        false
    }
}

/// Applies the selected File Explorer style + color tint. `id` is a
/// catalog id (`""` = no theme — tint-only when `tint` is set, `Windows
/// Default` → reset when both are empty). The tint rides the same wire as
/// appended `controlStyles` rules, so a theme switch or tint clear
/// restores cleanly through the resident's atomic re-apply path — no
/// stale layer can stack. Returns `false` only for an unknown id so
/// settings reverts.
pub fn apply(id: &str, tint: &str) -> bool {
    let wire = if id.is_empty() {
        match pecofence_core::color_tint::tint_wire(
            "PECO-FES1",
            pecofence_core::color_tint::FILE_EXPLORER_TARGETS,
            tint,
        ) {
            Some(w) => w,
            None => return reset(),
        }
    } else {
        let Some(mut w) = pecofence_core::file_explorer_themes::theme_wire(id) else {
            return false;
        };
        pecofence_core::color_tint::apply_to_wire(
            &mut w,
            pecofence_core::color_tint::FILE_EXPLORER_TARGETS,
            tint,
        );
        w
    };
    let effect = pecofence_core::file_explorer_themes::backdrop(id)
        .unwrap_or_default()
        .to_string();
    let mut guard = lock();
    let fe = guard.get_or_insert_with(|| Fe {
        style: String::new(),
        wire: String::new(),
        cmd_hwnd: 0,
        hello_sent: false,
        applied_effect: String::new(),
        seen_hwnds: std::collections::HashSet::new(),
        tint: String::new(),
        applied_tint: String::new(),
    });
    fe.style = id.to_string();
    fe.wire = wire;
    fe.tint = tint.to_string();
    ARMED.store(true, Ordering::Release);
    if let Some((hwnd, _)) = crate::tap::ensure_resident()
        && !deliver(hwnd, fe)
    {
        fe.cmd_hwnd = 0; // delivery pending — `ensure` retries next tick
    }
    drop(guard);
    apply_backdrop(&effect);
    true
}

/// `Windows Default` — restore originals, return cabinet windows to the
/// stock backdrop, stop keeping the DLL alive on our account.
pub fn reset() -> bool {
    let mut guard = lock();
    if let Some(hwnd) = crate::tap::resident_window() {
        let _ = crate::tap::send_copydata(hwnd, CDS_FE_RESET, &[]);
    }
    if let Some(fe) = guard.as_mut() {
        fe.style.clear();
        fe.wire.clear();
    }
    ARMED.store(false, Ordering::Release);
    drop(guard);
    apply_backdrop("");
    true
}

/// Periodic liveness driven by the app's anchor tick: re-injects and
/// re-delivers the wire when explorer restarted under an armed style, and
/// re-asserts the DWM backdrop (explorer may rewrite its own attribute —
/// without the mod's `DwmSetWindowAttribute` hook ours is a best-effort
/// value, so refresh it here). Otherwise sends a `HELLO`.
pub fn ensure() {
    if !is_armed() {
        return;
    }
    let Some((hwnd, _)) = crate::tap::ensure_resident() else {
        return;
    };
    let mut guard = lock();
    let Some(fe) = guard.as_mut() else {
        return;
    };
    if fe.cmd_hwnd != hwnd.0 as usize {
        let _ = deliver(hwnd, fe);
    }
    let effect = pecofence_core::file_explorer_themes::backdrop(&fe.style)
        .unwrap_or_default()
        .to_string();
    drop(guard);
    apply_backdrop(&effect);
}

/// App shutdown — restore stock styling; `tap::shutdown` unloads the
/// resident itself.
pub fn shutdown() {
    let _ = reset();
}
