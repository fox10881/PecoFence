//! Host-side controller for the File Explorer glass effect — the native
//! port of [ExplorerBlurMica](https://github.com/Maplespe/ExplorerBlurMica)'s
//! `config.ini`:
//!
//! ```ini
//! [config]  effect=0..4  clearAddress  clearBarBg  clearWinUIBg  showLine
//! [light]   r g b a        (blend color while the system is light)
//! [dark]    r g b a        (blend color while the system is dark)
//! ```
//!
//! The effect rides two channels into the explorer-resident
//! `pecofence_tap.dll`:
//! - `FEBD` COPYDATA — the DWM material: `[backdrop:i32, accent_state:u32,
//!   accent_flags:u32, gradient_abgr:u32, options:u32]` (little-endian,
//!   20 bytes). `fe_glass` applies it via `DwmExtendFrameIntoClientArea`,
//!   `DwmSetWindowAttribute`, `DwmEnableBlurBehindWindow` and
//!   `WCA_ACCENT_POLICY` — the exact pipeline EBM runs in-process.
//! - `FEST` COPYDATA — a `PECO-FES1` controlStyles wire carrying the
//!   `clearAddress`/`clearWinUIBg` island-transparency rules into the
//!   WinUI 3 diagnostics session. `clearBarBg`/`showLine` are Win32
//!   behaviors instead — they ride `options` bits in the FEBD.

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
/// Backdrop/accent material for the resident's `fe_glass` detours —
/// `[backdrop:i32, accent_state:u32, accent_flags:u32, gradient:u32,
/// options:u32]` little-endian (20 bytes).
const CDS_FE_BACKDROP: usize = u32::from_be_bytes(*b"FEBD") as usize;

// DWMWA_SYSTEMBACKDROP_TYPE (38) values.
const DWMWA_SYSTEMBACKDROP_TYPE: u32 = 38;
const DWMSBT_AUTO: i32 = 0; // system default — accent path wants this
const DWMSBT_MAINWINDOW: i32 = 2; // Mica
const DWMSBT_TABBEDWINDOW: i32 = 4; // Mica Alt / explorer's stock request

// ACCENT_STATE values (`WCA_ACCENT_POLICY`).
const ACCENT_DISABLED: u32 = 0;
const ACCENT_TRANSPARENTGRADIENT: u32 = 2; // EBM "Blur (Clear)"
const ACCENT_BLURBEHIND: u32 = 3; // EBM "Blur" (removed on Win11 22H2+)
const ACCENT_ACRYLIC: u32 = 4; // EBM "Acrylic"

/// `FE_OPTIONS` bits — Win32-side behaviors that can't ride the XAML
/// wire (must match `fe_glass.rs`).
const FEOPT_CLEAR_BAR_BG: u32 = 1 << 0;
const FEOPT_SHOW_LINE: u32 = 1 << 1;

/// The `clearAddress` rule set — the address/search bar containers.
/// `S Background=Transparent` lets the material show through.
const CLEAR_ADDRESS_RULES: &[&str] = &[
    "FileExplorerExtensions.NavigationBarControl",
    "Microsoft.UI.Xaml.Controls.Grid#NavigationBarControlGrid",
];

/// The `clearWinUIBg` rule set — the WinUI 3 island's own backgrounds
/// (tab strip, command bar, root containers). The nav bar is governed by
/// `clearAddress` instead.
const CLEAR_WINUI_BG_RULES: &[&str] = &[
    "FileExplorerExtensions.FileExplorerTabControl",
    "FileExplorerExtensions.FileExplorerTabControl > Grid",
    "Microsoft.UI.Xaml.Controls.TabView",
    "Microsoft.UI.Xaml.Controls.Grid#TabContainerGrid",
    "FileExplorerExtensions.CommandBarControl_Wave1",
    "FileExplorerExtensions.CommandBarControl_Wave1 > Grid, Grid#CommandBarControlRootGrid",
    "Microsoft.UI.Xaml.Controls.CommandBar#FileExplorerCommandBar",
    "Microsoft.UI.Xaml.Controls.CommandBar#FileExplorerCommandBar > Grid#LayoutRoot > Grid#ContentRoot",
    "Microsoft.UI.Xaml.Controls.Grid#PART_LayoutRoot",
    "Microsoft.UI.Xaml.Controls.Grid#ContentRoot",
    "Microsoft.UI.Xaml.Controls.Grid#RootContainer",
    "Microsoft.UI.Xaml.Controls.Grid#DetailsViewControlRootGrid",
    "Microsoft.UI.Xaml.Controls.Grid#HomeViewRootGrid",
    "FileExplorerExtensions.GalleryViewControl#GalleryViewControl > Grid",
];

/// UI-facing EBM configuration — mirrors `config.ini`'s `[config]` flags
/// plus the `[light]`/`[dark]` RGBA strings (`#AARRGGBB`).
pub struct FeGlassConfig<'a> {
    /// `""` = off; `"blur"`, `"acrylic"`, `"mica"`, `"blurclear"`,
    /// `"micaalt"` — EBM `effect=` 0..4.
    pub effect: &'a str,
    pub clear_address: bool,
    pub clear_bar_bg: bool,
    pub clear_winui_bg: bool,
    pub show_line: bool,
    pub light_rgba: &'a str,
    pub dark_rgba: &'a str,
}

struct Fe {
    /// The EBM `effect` string currently stored ("" = off).
    effect: String,
    /// The flag bits currently resolved into `options`.
    options: u32,
    /// The resolved `GradientColor` (light/dark adaptive).
    gradient: u32,
    /// The FEST wire last built — cached so `ensure` re-delivers after an
    /// explorer restart without re-serializing.
    wire: String,
    /// The resident command window the wire was delivered to.
    cmd_hwnd: usize,
    /// Whether `CDS_FE_HELLO` reached the current DLL residency.
    hello_sent: bool,
    /// The DWM material values currently pushed to cabinets —
    /// `apply_glass` skips untouched windows unless these change.
    applied: (i32, u32, u32),
    /// Cabinet hwnds the current `applied` material was pushed to.
    seen_hwnds: std::collections::HashSet<isize>,
}

static FE: Mutex<Option<Fe>> = Mutex::new(None);
/// Whether a File Explorer glass effect is armed — read lock-free.
static ARMED: AtomicBool = AtomicBool::new(false);

/// `true` while a File Explorer effect is armed — `tap` keeps the DLL resident.
pub fn is_armed() -> bool {
    ARMED.load(Ordering::Acquire)
}

fn lock() -> std::sync::MutexGuard<'static, Option<Fe>> {
    FE.lock().unwrap_or_else(|p| p.into_inner())
}

fn widestr(s: &str) -> Vec<u16> {
    s.encode_utf16().chain(core::iter::once(0)).collect()
}

/// Every `CabinetWClass` that is a real File Explorer window — frames
/// with a `WorkerW` child are the legacy Control Panel shell, which has
/// no XAML islands and isn't targeted.
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

/// `#AARRGGBB` (or `#RRGGBB`, alpha `FF` assumed) → `0xAABBGGRR`, the
/// ABGR layout `ACCENT_POLICY.GradientColor` consumes.
fn parse_argb(s: &str) -> Option<u32> {
    let hex = s.strip_prefix('#').unwrap_or(s);
    let v = match hex.len() {
        8 => u32::from_str_radix(hex, 16).ok()?,
        6 => u32::from_str_radix(hex, 16).ok()? | 0xFF00_0000,
        _ => return None,
    };
    let (a, r, g, b) = (v >> 24, (v >> 16) & 0xFF, (v >> 8) & 0xFF, v & 0xFF);
    Some(a << 24 | b << 16 | g << 8 | r)
}

/// `HKCU\...\Personalize\AppsUseLightTheme` — `false` means the system
/// runs dark mode and the `[dark]` RGBA applies (EBM's adaptive rule).
fn apps_use_light_theme() -> bool {
    const HKCU: isize = 0x8000_0001;
    const KEY_READ: u32 = 0x0002_0019;
    const REG_DWORD: u32 = 4;
    let subkey = widestr("Software\\Microsoft\\Windows\\CurrentVersion\\Themes\\Personalize");
    let name = widestr("AppsUseLightTheme");
    unsafe {
        let mut key = 0isize;
        if RegOpenKeyExW(HKCU, PCWSTR(subkey.as_ptr()), 0, KEY_READ, &mut key) != 0 {
            return false; // assume dark — the Explorer glass default
        }
        let mut val = 0u32;
        let mut ty = 0u32;
        let mut cb = core::mem::size_of::<u32>() as u32;
        let _ = RegQueryValueExW(
            key,
            PCWSTR(name.as_ptr()),
            core::ptr::null(),
            &mut ty,
            &mut val as *mut u32 as *mut u8,
            &mut cb,
        );
        let _ = RegCloseKey(key);
        ty == REG_DWORD && val != 0
    }
}

/// EBM `effect=` → `(backdrop, accent_state)`. Accent materials keep
/// `DWMSBT_AUTO` so no system backdrop fights `WCA_ACCENT_POLICY`;
/// mica/mica-alt ride the DWMSBT path with the accent off.
fn desired_material(effect: &str) -> (i32, u32) {
    match effect {
        "blur" => (DWMSBT_AUTO, ACCENT_BLURBEHIND),
        "acrylic" => (DWMSBT_AUTO, ACCENT_ACRYLIC),
        "mica" => (DWMSBT_MAINWINDOW, ACCENT_DISABLED),
        "blurclear" => (DWMSBT_AUTO, ACCENT_TRANSPARENTGRADIENT),
        "micaalt" => (DWMSBT_TABBEDWINDOW, ACCENT_DISABLED),
        _ => (DWMSBT_TABBEDWINDOW, ACCENT_DISABLED),
    }
}

/// Builds the `PECO-FES1` wire from the `clearAddress`/`clearWinUIBg`
/// flags — `Background=Transparent` on the matching island targets.
/// Always at least `magic\nE\n` so the resident arms + restores cleanly.
fn glass_wire(clear_address: bool, clear_winui_bg: bool) -> String {
    let mut wire = String::with_capacity(1024);
    wire.push_str("PECO-FES1\n");
    for target in CLEAR_ADDRESS_RULES
        .iter()
        .filter(|_| clear_address)
        .chain(CLEAR_WINUI_BG_RULES.iter().filter(|_| clear_winui_bg))
    {
        wire.push_str("T ");
        wire.push_str(target);
        wire.push_str("\nS Background=Transparent\n");
    }
    wire.push_str("E\n");
    wire
}

fn options_of(cfg: &FeGlassConfig) -> u32 {
    let mut o = 0u32;
    if cfg.clear_bar_bg {
        o |= FEOPT_CLEAR_BAR_BG;
    }
    if cfg.show_line {
        o |= FEOPT_SHOW_LINE;
    }
    o
}

/// The resolved `GradientColor` for `cfg` under the CURRENT system
/// scheme — light RGBA when `AppsUseLightTheme`, dark otherwise
/// (EBM's `[light]`/`[dark]` adaptive palettes).
fn gradient_of(cfg: &FeGlassConfig) -> u32 {
    let s = if apps_use_light_theme() {
        cfg.light_rgba
    } else {
        cfg.dark_rgba
    };
    // EBM defaults: light #A0DCDCDC / dark #78000000.
    parse_argb(s).unwrap_or(if apps_use_light_theme() {
        0xA0DCDCDC
    } else {
        0x78000000
    })
}

/// Applies an EBM `effect` + flags + adaptive RGBA. `effect=""` →
/// `reset()` (Windows Default). Returns `false` for an unknown effect id
/// so settings reverts.
pub fn apply_effect(cfg: &FeGlassConfig) -> bool {
    if cfg.effect.is_empty() {
        return reset();
    }
    if !matches!(
        cfg.effect,
        "blur" | "acrylic" | "mica" | "blurclear" | "micaalt"
    ) {
        return false;
    }
    let wire = glass_wire(cfg.clear_address, cfg.clear_winui_bg);
    let mut guard = lock();
    let fe = guard.get_or_insert_with(|| Fe {
        effect: String::new(),
        options: 0,
        gradient: 0,
        wire: String::new(),
        cmd_hwnd: 0,
        hello_sent: false,
        applied: (0, 0, 0),
        seen_hwnds: std::collections::HashSet::new(),
    });
    fe.effect = cfg.effect.to_string();
    fe.options = options_of(cfg);
    fe.gradient = gradient_of(cfg);
    fe.wire = wire;
    ARMED.store(true, Ordering::Release);
    ensure_transparency_effects();
    if let Some((hwnd, _)) = crate::tap::ensure_resident()
        && !deliver(hwnd, fe)
    {
        fe.cmd_hwnd = 0; // delivery pending — `ensure` retries next tick
    }
    drop(guard);
    apply_glass();
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
        fe.effect.clear();
        fe.wire.clear();
    }
    ARMED.store(false, Ordering::Release);
    drop(guard);
    restore_glass();
    true
}

/// Periodic liveness driven by the app's anchor tick: re-injects and
/// re-delivers the wire when explorer restarted under an armed effect,
/// and re-asserts the DWM material on cabinet windows.
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
    drop(guard);
    apply_glass();
}

/// App shutdown — restore stock styling; `tap::shutdown` unloads the
/// resident itself.
pub fn shutdown() {
    let _ = reset();
}

/// Sends the stored wire to the resident DLL — FEHI hello, then the FEBD
/// material payload (the detours need the desired values BEFORE the
/// theme wire arms them), then the FEST controlStyles wire.
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
    let (bd, st) = desired_material(&fe.effect);
    let mut payload = [0u8; 20];
    payload[0..4].copy_from_slice(&bd.to_le_bytes());
    payload[4..8].copy_from_slice(&st.to_le_bytes());
    payload[8..12].copy_from_slice(&0u32.to_le_bytes()); // accent_flags
    payload[12..16].copy_from_slice(&fe.gradient.to_le_bytes());
    payload[16..20].copy_from_slice(&fe.options.to_le_bytes());
    let _ = crate::tap::send_copydata(hwnd, CDS_FE_BACKDROP, &payload);
    fe.cmd_hwnd = if crate::tap::send_copydata(hwnd, CDS_FE_THEME, fe.wire.as_bytes()) {
        hwnd.0 as usize
    } else {
        0
    };
    fe.cmd_hwnd != 0
}

/// Pushes the DWM material onto every `CabinetWClass` — direct
/// `DwmSetWindowAttribute`/`SetWindowCompositionAttribute` calls for the
/// cross-process half (the resident's in-process copy does the same via
/// its own `apply_glass_to`; the belt-and-suspenders pair keeps windows
/// styled even before injection lands). Only touches windows when the
/// material changed or a new cabinet appears.
fn apply_glass() {
    let hwnds = cabinet_windows();
    let (material, options, gradient, mut seen) = {
        let mut guard = lock();
        let Some(fe) = guard.as_mut() else {
            return;
        };
        let (bd, st) = desired_material(&fe.effect);
        let changed = fe.applied != (bd, st, fe.options | fe.gradient);
        if changed {
            fe.applied = (bd, st, fe.options | fe.gradient);
            fe.seen_hwnds.clear();
        }
        (
            (bd, st),
            fe.options,
            fe.gradient,
            std::mem::take(&mut fe.seen_hwnds),
        )
    };
    let (backdrop, accent_state) = material;
    let _ = options; // resident-side bits — not needed cross-process
    let glass = is_armed();
    for hwnd in &hwnds {
        if seen.contains(&(hwnd.0 as usize as isize)) {
            continue;
        }
        unsafe {
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
        // EBM/TW `EnableBlurBehind` — the accent path blurs the whole
        // client region; the SYSTEMBACKDROP path disables blur-behind
        // (mutually exclusive pairings render black).
        blur_behind(*hwnd, glass && accent_state != ACCENT_DISABLED);
        set_accent_policy(*hwnd, accent_state, 0, gradient);
        // The system backdrop only blurs while DWM treats the window as
        // active — force that appearance + fake an activation.
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
        seen.retain(|h| hwnds.iter().any(|w| w.0 as usize as isize == *h));
        fe.seen_hwnds = seen;
    }
}

/// Restores every cabinet to stock — the disarm counterpart of
/// `apply_glass`.
fn restore_glass() {
    for hwnd in cabinet_windows() {
        extend_frame(hwnd, false);
        let stock: i32 = DWMSBT_TABBEDWINDOW;
        unsafe {
            let _ = DwmSetWindowAttribute(
                hwnd,
                DWMWA_SYSTEMBACKDROP_TYPE,
                &stock as *const i32 as *const core::ffi::c_void,
                core::mem::size_of::<i32>() as u32,
            );
            let dark: i32 = 0;
            let _ = DwmSetWindowAttribute(
                hwnd,
                20,
                &dark as *const i32 as *const core::ffi::c_void,
                core::mem::size_of::<i32>() as u32,
            );
        }
        blur_behind(hwnd, false);
        set_accent_policy(hwnd, ACCENT_DISABLED, 0, 0);
        set_force_active(hwnd, false);
        trigger_composition_update(hwnd);
    }
}

#[repr(C)]
struct DwmBlurBehind {
    flags: u32,
    enable: windows_core::BOOL,
    rgn: *mut core::ffi::c_void,
    transition_on_max: windows_core::BOOL,
}

/// `DwmEnableBlurBehindWindow` — Translucent Windows `EnableBlurBehind`
/// verbatim: the accent-acrylic path wants `fEnable=TRUE` over a
/// whole-client blur region plus `TRANSITIONONMAXIMIZED`; the
/// SYSTEMBACKDROP path disables it — blur-behind plus a DWMSBT material
/// is a mutually-exclusive pairing that renders black instead of glass.
fn blur_behind(hwnd: crate::bindings::HWND, on: bool) {
    let rgn = if on {
        unsafe { CreateRectRgn(0, 0, -1, -1) }
    } else {
        core::ptr::null_mut()
    };
    let bb = DwmBlurBehind {
        // DWM_BB_ENABLE | DWM_BB_BLURREGION | DWM_BB_TRANSITIONONMAXIMIZED
        flags: 1 | if on { 2 | 4 } else { 0 },
        enable: windows_core::BOOL(on as i32),
        rgn,
        transition_on_max: windows_core::BOOL(on as i32),
    };
    unsafe {
        let _ = DwmEnableBlurBehindWindow(hwnd, &bb);
        if !rgn.is_null() {
            let _ = DeleteObject(rgn);
        }
    }
}

/// `DwmExtendFrameIntoClientArea` — `{ -1, -1, -1, -1 }` pulls the glass
/// frame over the whole client area so a `Transparent` XAML/Win32
/// background actually shows the material; `{0,0,0,0}` restores. The
/// resident detour keeps explorer from resetting it while armed.
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
/// every backdrop opaque while this is 0. Arming glass needs it — set it
/// once and broadcast `WM_SETTINGCHANGE` (`ImmersiveColorSet`) so DWM
/// re-reads it without a sign-out.
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
/// Keeps DWM evaluating the backdrop as if `CabinetWClass` is always the
/// active window — without it the material drops to a flat `#585858`
/// sheet the moment focus moves elsewhere.
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

/// `SetWindowCompositionAttribute` — `WCA_ACCENT_POLICY`. The `mica`/
/// `micaalt` paths keep it strictly `ACCENT_DISABLED` (mutual exclusion
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

/// `TriggerWindowCompositionUpdate` — `SetWindowPos` with
/// `SWP_FRAMECHANGED` generates a real `WM_WINDOWPOSCHANGED`/
/// `WM_NCCALCSIZE` inside explorer — the nudge EBM does in-process.
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

const RDW_INVALIDATE: u32 = 0x0001;
const RDW_ERASE: u32 = 0x0004;
const RDW_FRAME: u32 = 0x0400;
const RDW_ALLCHILDREN: u32 = 0x0080;
const SWP_NOMOVE: u32 = 0x0002;
const SWP_NOSIZE: u32 = 0x0001;
const SWP_NOZORDER: u32 = 0x0004;
const SWP_NOACTIVATE: u32 = 0x0010;
const SWP_FRAMECHANGED: u32 = 0x0020;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn effect_maps_to_material() {
        assert_eq!(desired_material("acrylic"), (DWMSBT_AUTO, ACCENT_ACRYLIC));
        assert_eq!(desired_material("blur"), (DWMSBT_AUTO, ACCENT_BLURBEHIND));
        assert_eq!(
            desired_material("blurclear"),
            (DWMSBT_AUTO, ACCENT_TRANSPARENTGRADIENT)
        );
        assert_eq!(desired_material("mica"), (DWMSBT_MAINWINDOW, 0));
        assert_eq!(desired_material("micaalt"), (DWMSBT_TABBEDWINDOW, 0));
    }

    #[test]
    fn argb_parses_to_abgr() {
        assert_eq!(parse_argb("#A0DCDCDC"), Some(0xA0DCDCDC));
        assert_eq!(parse_argb("#78000000"), Some(0x78000000));
        assert_eq!(parse_argb("78000000"), Some(0x78000000));
        assert_eq!(parse_argb("#0078D4"), Some(0xFFD47800));
        assert!(parse_argb("#FFF").is_none());
        assert!(parse_argb("garbage").is_none());
    }

    #[test]
    fn clear_wire_lists_targets() {
        let w = glass_wire(true, true);
        assert!(w.starts_with("PECO-FES1\n"));
        assert!(w.ends_with("E\n"));
        assert!(w.contains("T Microsoft.UI.Xaml.Controls.Grid#NavigationBarControlGrid\n"));
        assert_eq!(w.matches("S Background=Transparent").count(), 16);

        let none = glass_wire(false, false);
        assert_eq!(none, "PECO-FES1\nE\n");
    }
}
