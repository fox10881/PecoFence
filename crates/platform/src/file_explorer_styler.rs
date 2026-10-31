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
windows_core::link!("user32.dll" "system" fn GetClassNameW(hwnd: crate::bindings::HWND, lpclassname: windows_core::PWSTR, nmaxcount: i32) -> i32);
windows_core::link!("user32.dll" "system" fn RedrawWindow(hwnd: crate::bindings::HWND, lprcupdate: *const core::ffi::c_void, hrgnupdate: *mut core::ffi::c_void, flags: u32) -> windows_core::BOOL);

/// `COPYDATASTRUCT.dwData` values — must match `crates/tap/src/command.rs`.
const CDS_FE_THEME: usize = u32::from_be_bytes(*b"FEST") as usize;
const CDS_FE_RESET: usize = u32::from_be_bytes(*b"FESR") as usize;
const CDS_FE_HELLO: usize = u32::from_be_bytes(*b"FEHI") as usize;

// DWMWA_SYSTEMBACKDROP_TYPE (38) values — the same DWMSBT_* mapping the mod
// applies through its DwmSetWindowAttribute hook.
const DWMWA_SYSTEMBACKDROP_TYPE: u32 = 38;
const DWMSBT_MAINWINDOW: i32 = 2; // Mica
const DWMSBT_TRANSIENTWINDOW: i32 = 3; // Acrylic
const DWMSBT_TABBEDWINDOW: i32 = 4; // Mica Alt

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
    let backdrop = match effect {
        "blur" => DWMSBT_TABBEDWINDOW, // blur rides the accent policy below
        "acrylic" => DWMSBT_TRANSIENTWINDOW,
        "mica" => DWMSBT_MAINWINDOW,
        "micaAlt" => DWMSBT_TABBEDWINDOW,
        _ => DWMSBT_TABBEDWINDOW,
    };
    let hwnds = cabinet_windows();
    let (changed, mut seen) = {
        let mut guard = lock();
        let Some(fe) = guard.as_mut() else {
            return;
        };
        let changed = fe.applied_effect != effect;
        if changed {
            fe.applied_effect = effect.to_string();
            fe.seen_hwnds.clear();
        }
        (changed, std::mem::take(&mut fe.seen_hwnds))
    };
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
        }
        set_accent_blur(*hwnd, effect == "blur");
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

/// `SetAccentBlurBehind` — `SetWindowCompositionAttribute` ACCENT_POLICY
/// blur for the `blur` backdrop (the mod's fallback + its blur path).
fn set_accent_blur(hwnd: crate::bindings::HWND, on: bool) {
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
    const ACCENT_ENABLE_BLURBEHIND: u32 = 3;
    const ACCENT_DISABLED: u32 = 0;
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
        accent_state: if on {
            ACCENT_ENABLE_BLURBEHIND
        } else {
            ACCENT_DISABLED
        },
        accent_flags: 2,
        gradient_color: 0,
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
    if crate::tap::send_copydata(hwnd, CDS_FE_THEME, fe.wire.as_bytes()) {
        fe.cmd_hwnd = hwnd.0 as usize;
        true
    } else {
        false
    }
}

/// Applies the selected File Explorer style. `id` is a catalog id (`""` =
/// Windows Default → reset). The selection is armed first, then delivered
/// best-effort: when the resident is missing (explorer restarting) the
/// periodic `ensure` keeps retrying — mirroring `taskbar_styler` semantics.
/// Returns `false` only for an unknown id so settings reverts.
pub fn apply(id: &str) -> bool {
    if id.is_empty() {
        return reset();
    }
    let Some(wire) = pecofence_core::file_explorer_themes::theme_wire(id) else {
        return false;
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
    });
    fe.style = id.to_string();
    fe.wire = wire;
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
