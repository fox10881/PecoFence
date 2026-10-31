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
windows_core::link!("user32.dll" "system" fn DefWindowProcW(hwnd: crate::bindings::HWND, msg: u32, wparam: usize, lparam: isize) -> isize);
windows_core::link!("user32.dll" "system" fn SendMessageTimeoutW(hwnd: crate::bindings::HWND, msg: u32, wparam: usize, lparam: isize, flags: u32, timeout: u32, result: *mut usize) -> isize);
windows_core::link!("advapi32.dll" "system" fn RegOpenKeyExW(hkey: isize, subkey: PCWSTR, options: u32, sam: u32, result: *mut isize) -> i32);
windows_core::link!("advapi32.dll" "system" fn RegQueryValueExW(hkey: isize, name: PCWSTR, reserved: *const u32, ty: *mut u32, data: *mut u8, cb: *mut u32) -> i32);
windows_core::link!("advapi32.dll" "system" fn RegSetValueExW(hkey: isize, name: PCWSTR, reserved: u32, ty: u32, data: *const u8, cb: u32) -> i32);
windows_core::link!("advapi32.dll" "system" fn RegCloseKey(hkey: isize) -> i32);

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
const DWMSBT_MAINWINDOW: i32 = 2; // Mica
const DWMSBT_TRANSIENTWINDOW: i32 = 3; // system Acrylic — the working 23H2 glass
const DWMSBT_TABBEDWINDOW: i32 = 4; // Mica Alt / explorer's own stock request

const ACCENT_DISABLED: u32 = 0;

/// Translucent-Windows `HandleEffects`: on Windows 11 23H2 `CabinetWClass`
/// the working glass is `DWMWA_SYSTEMBACKDROP_TYPE` =
/// `DWMSBT_TRANSIENTWINDOW` (system acrylic) — or `DWMSBT_MAINWINDOW` for
/// the Mica presets — with `WCA_ACCENT_POLICY` strictly DISABLED: accent
/// blur-behind and a system backdrop on the same window conflict, and
/// `DWMSBT_NONE` + accent does not compose on this window class. The
/// unfocused `#585858` fallback is killed separately by
/// `WCA_FORCE_ACTIVEWINDOW_APPEARANCE` + `WM_NCACTIVATE`, and the Color
/// Tint overlay rides the XAML ruleset instead of the accent gradient.
/// Returns `(backdrop, accent_state, accent_flags, gradient_abgr)`.
fn desired_glass(effect: &str) -> (i32, u32, u32, u32) {
    let backdrop = if effect == "mica" {
        DWMSBT_MAINWINDOW
    } else {
        DWMSBT_TRANSIENTWINDOW
    };
    (backdrop, ACCENT_DISABLED, 0, 0)
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
    let (changed, mut seen) = {
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
        (changed, std::mem::take(&mut fe.seen_hwnds))
    };
    let (backdrop, accent_state, accent_flags, gradient) = if is_armed() {
        desired_glass(effect)
    } else {
        // Disarmed — hand the stock tabbed backdrop explorer asks for
        // itself back; everything else below restores too.
        (DWMSBT_TABBEDWINDOW, ACCENT_DISABLED, 0, 0)
    };
    // Glass is active for any armed state — a named effect or a
    // tint-only overlay (the tint rides the XAML ruleset on top of the
    // same TRANSIENTWINDOW acrylic; the swatch never re-selects the DWM
    // path).
    let glass = is_armed();
    if glass && changed {
        ensure_transparency_effects();
    }
    for hwnd in &hwnds {
        if !changed && seen.contains(&(hwnd.0 as usize as isize)) {
            continue;
        }
        unsafe {
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
        unsafe {
            let _ = DwmSetWindowAttribute(
                *hwnd,
                DWMWA_SYSTEMBACKDROP_TYPE,
                &backdrop as *const i32 as *const core::ffi::c_void,
                core::mem::size_of::<i32>() as u32,
            );
        }
        blur_behind(*hwnd, false);
        set_accent_policy(*hwnd, accent_state, accent_flags, gradient);
        // Kill the unfocused `#585858` fallback: the system backdrop only
        // blurs while DWM treats the window as active — force that
        // appearance, then lie about activation so DWM re-evaluates now.
        set_force_active(*hwnd, glass);
        if glass {
            unsafe {
                let _ = DefWindowProcW(*hwnd, 0x0086 /* WM_NCACTIVATE */, 1, 0);
            }
        }
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

/// `HKCU\...\Themes\Personalize\EnableTransparency`: DWM silently renders
/// every system backdrop opaque while this is 0. Arming glass needs it —
/// set it once and broadcast `WM_SETTINGCHANGE` (`ImmersiveColorSet`) so
/// DWM re-reads it without a sign-out.
fn ensure_transparency_effects() {
    const HKCU: isize = 0x8000_0001;
    const KEY_READ: u32 = 0x0002_0019;
    const KEY_SET_VALUE: u32 = 0x0002;
    const REG_DWORD: u32 = 4;
    let subkey = widestr("Software\\Microsoft\\Windows\\CurrentVersion\\Themes\\Personalize");
    let name = widestr("EnableTransparency");
    unsafe {
        let mut key = 0isize;
        if RegOpenKeyExW(
            HKCU,
            PCWSTR(subkey.as_ptr()),
            0,
            KEY_READ | KEY_SET_VALUE,
            &mut key,
        ) != 0
        {
            return;
        }
        let mut val = 1u32;
        let mut ty = 0u32;
        let mut cb = core::mem::size_of::<u32>() as u32;
        let read = RegQueryValueExW(
            key,
            PCWSTR(name.as_ptr()),
            core::ptr::null(),
            &mut ty,
            &mut val as *mut u32 as *mut u8,
            &mut cb,
        );
        if read != 0 || ty != REG_DWORD || val != 0 {
            let _ = RegCloseKey(key);
            return;
        }
        let one = 1u32;
        let _ = RegSetValueExW(
            key,
            PCWSTR(name.as_ptr()),
            0,
            REG_DWORD,
            &one as *const u32 as *const u8,
            4,
        );
        let _ = RegCloseKey(key);
        let param = widestr("ImmersiveColorSet");
        let mut result = 0usize;
        let _ = SendMessageTimeoutW(
            crate::bindings::HWND(0xFFFF as *mut core::ffi::c_void), // HWND_BROADCAST
            0x001A,                                                  // WM_SETTINGCHANGE
            0,
            param.as_ptr() as isize,
            0x0002, // SMTO_ABORTIFHUNG
            2000,
            &mut result,
        );
    }
}

#[repr(C)]
struct WindowCompositionAttribData {
    attrib: u32,
    pv_data: *const core::ffi::c_void,
    cb_data: usize,
}

const WCA_ACCENT_POLICY: u32 = 19;
/// Keeps DWM evaluating the system backdrop as if `CabinetWClass` is
/// always the active window — without it `DWMSBT_TRANSIENTWINDOW` drops
/// to a flat `#585858` sheet the moment focus moves elsewhere.
const WCA_FORCE_ACTIVEWINDOW_APPEARANCE: u32 = 15;

fn composition_attr(hwnd: crate::bindings::HWND, data: &WindowCompositionAttribData) {
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
    unsafe {
        let _ = f(hwnd, data);
    }
}

/// `WCA_FORCE_ACTIVEWINDOW_APPEARANCE` — `1` while glass is armed, `0` on
/// restore so DWM's real focus tracking resumes.
fn set_force_active(hwnd: crate::bindings::HWND, on: bool) {
    let value: i32 = on as i32;
    composition_attr(
        hwnd,
        &WindowCompositionAttribData {
            attrib: WCA_FORCE_ACTIVEWINDOW_APPEARANCE,
            pv_data: &value as *const i32 as *const core::ffi::c_void,
            cb_data: core::mem::size_of::<i32>(),
        },
    );
}

/// `SetWindowCompositionAttribute` — `WCA_ACCENT_POLICY`. The system
/// backdrop path keeps this strictly `ACCENT_DISABLED` (mutual exclusion
/// with `DWMWA_SYSTEMBACKDROP_TYPE`); it is still written on every apply
/// so a previously-armed accent can't linger.
fn set_accent_policy(hwnd: crate::bindings::HWND, state: u32, flags: u32, gradient: u32) {
    #[repr(C)]
    struct AccentPolicy {
        accent_state: u32,
        accent_flags: u32,
        gradient_color: u32,
        animation_id: u32,
    }
    let policy = AccentPolicy {
        accent_state: state,
        accent_flags: flags,
        gradient_color: gradient,
        animation_id: 0,
    };
    composition_attr(
        hwnd,
        &WindowCompositionAttribData {
            attrib: WCA_ACCENT_POLICY,
            pv_data: &policy as *const AccentPolicy as *const core::ffi::c_void,
            cb_data: core::mem::size_of::<AccentPolicy>(),
        },
    );
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
    let (bd, st, fl, gr) = desired_glass(effect);
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
