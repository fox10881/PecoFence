//! In-process glass hooks for `CabinetWClass` — the translucent-windows
//! half of the File Explorer styler. Armed while the mux engine holds a
//! `FileExplorer` theme, disarmed on `FESR`/host death.
//!
//! What this adds beyond the cross-process `DwmSetWindowAttribute` path:
//! - `DwmExtendFrameIntoClientArea` detour: forces `{-1,-1,-1,-1}` so the
//!   glass sheet covers the whole client area and explorer can't reset it
//!   when switching tabs/folders.
//! - `DwmSetWindowAttribute` detour: swallows explorer-internal rewrites of
//!   `DWMWA_SYSTEMBACKDROP_TYPE`/`DWMWA_USE_HOSTBACKDROPBRUSH` on cabinets
//!   (our own writes come from pecofence.exe's dwmapi — unaffected).
//! - `OpenThemeData` detour: records `HTHEME` → theme-class so the paint
//!   hooks can scope to the DirectUI classes that own the opaque panels.
//! - `GetThemeColor`/`DrawThemeBackground(Ex)` detours: return/fill
//!   transparent black (`RGB(0,0,0)` / `BLACK_BRUSH`) for the background
//!   parts of ItemsView/TreeView/Header/CommandModule/PreviewPane/Rebar/
//!   Status/AddressBand/SearchBox — on cabinet DCs only.
//! - `GetSysColorBrush`/`FillRect` detours: `COLOR_WINDOW` inside a
//!   cabinet context becomes `BLACK_BRUSH`, so `SysTreeView32` and
//!   `DirectUIHWND` stop painting an opaque panel and reveal the DWM
//!   backdrop underneath.
//!
//! - `ExtTextOutW` detour: `ETO_OPAQUE` background boxes (SysTreeView32
//!   row fills) are repainted transparent black and re-emitted without
//!   the opaque flag. Text alpha correction is still not attempted —
//!   a wrong implementation renders text invisible.

use crate::win32;
use core::sync::atomic::{AtomicBool, Ordering};
use std::collections::HashMap;
use std::sync::{LazyLock, Mutex};
use windows_core::{PCSTR, PCWSTR};

type Hwnd = *mut core::ffi::c_void;
type Hdc = *mut core::ffi::c_void;
type HBrush = *mut core::ffi::c_void;
type HTheme = *mut core::ffi::c_void;
type HResult = i32;
type WndEnumProc = Option<unsafe extern "system" fn(Hwnd, isize) -> windows_core::BOOL>;

#[repr(C)]
struct Margins {
    left: i32,
    right: i32,
    top: i32,
    bottom: i32,
}
#[repr(C)]
struct Rect {
    left: i32,
    top: i32,
    right: i32,
    bottom: i32,
}
#[repr(C)]
struct DtbgOpts {
    size: u32,
    flags: u32,
    clip: Rect,
}

// ---------------------------------------------------------------- FFI ----
windows_core::link!("user32.dll" "system" fn WindowFromDC(hdc: Hdc) -> Hwnd);
windows_core::link!("user32.dll" "system" fn GetAncestor(hwnd: Hwnd, gaflags: u32) -> Hwnd);
windows_core::link!("user32.dll" "system" fn EnumThreadWindows(dwthreadid: u32, lpfn: WndEnumProc, lparam: isize) -> windows_core::BOOL);
windows_core::link!("user32.dll" "system" fn EnumWindows(lpfn: WndEnumProc, lparam: isize) -> windows_core::BOOL);
windows_core::link!("user32.dll" "system" fn EnumChildWindows(parent: Hwnd, lpfn: WndEnumProc, lparam: isize) -> windows_core::BOOL);
windows_core::link!("user32.dll" "system" fn FindWindowExW(parent: Hwnd, childafter: Hwnd, class: PCWSTR, title: PCWSTR) -> Hwnd);
windows_core::link!("user32.dll" "system" fn FillRect(hdc: Hdc, lprc: *const Rect, hbr: HBrush) -> i32);
windows_core::link!("user32.dll" "system" fn SendMessageTimeoutW(hwnd: Hwnd, msg: u32, wparam: usize, lparam: isize, flags: u32, timeout: u32, result: *mut usize) -> usize);
windows_core::link!("user32.dll" "system" fn RedrawWindow(hwnd: Hwnd, lprcupdate: *const core::ffi::c_void, hrgnupdate: *mut core::ffi::c_void, flags: u32) -> windows_core::BOOL);
windows_core::link!("user32.dll" "system" fn SetWindowCompositionAttribute(hwnd: Hwnd, data: *const core::ffi::c_void) -> windows_core::BOOL);
windows_core::link!("user32.dll" "system" fn DefWindowProcW(hwnd: Hwnd, msg: u32, wparam: usize, lparam: isize) -> isize);
windows_core::link!("gdi32.dll" "system" fn GetStockObject(i: i32) -> *mut core::ffi::c_void);
windows_core::link!("gdi32.dll" "system" fn GetObjectW(obj: *mut core::ffi::c_void, cb: i32, pv: *mut core::ffi::c_void) -> i32);
windows_core::link!("gdi32.dll" "system" fn StretchDIBits(hdc: Hdc, x_dest: i32, y_dest: i32, dest_w: i32, dest_h: i32, x_src: i32, y_src: i32, src_w: i32, src_h: i32, bits: *const core::ffi::c_void, bmi: *const core::ffi::c_void, usage: u32, rop: u32) -> i32);
windows_core::link!("gdi32.dll" "system" fn CreateRectRgn(left: i32, top: i32, right: i32, bottom: i32) -> *mut core::ffi::c_void);
windows_core::link!("gdi32.dll" "system" fn DeleteObject(obj: *mut core::ffi::c_void) -> windows_core::BOOL);
windows_core::link!("gdi32.dll" "system" fn GetTextExtentPoint32W(hdc: Hdc, lpstring: PCWSTR, c: i32, psizl: *mut Size) -> windows_core::BOOL);
windows_core::link!("gdi32.dll" "system" fn GetTextExtentPointI(hdc: Hdc, pgi_in: *const u16, cgi: i32, psize: *mut Size) -> windows_core::BOOL);
windows_core::link!("gdi32.dll" "system" fn GetTextAlign(hdc: Hdc) -> u32);
windows_core::link!("gdi32.dll" "system" fn SetTextAlign(hdc: Hdc, align: u32) -> u32);
windows_core::link!("gdi32.dll" "system" fn GetLayout(hdc: Hdc) -> u32);
windows_core::link!("gdi32.dll" "system" fn SetLayout(hdc: Hdc, l: u32) -> u32);
windows_core::link!("gdi32.dll" "system" fn SetBkMode(hdc: Hdc, mode: i32) -> i32);
windows_core::link!("gdi32.dll" "system" fn GetTextColor(hdc: Hdc) -> u32);
windows_core::link!("gdi32.dll" "system" fn SetTextColor(hdc: Hdc, color: u32) -> u32);
windows_core::link!("gdi32.dll" "system" fn SetBkColor(hdc: Hdc, color: u32) -> u32);
windows_core::link!("gdi32.dll" "system" fn GetCurrentObject(hdc: Hdc, ty: u32) -> *mut core::ffi::c_void);
windows_core::link!("gdi32.dll" "system" fn SelectObject(hdc: Hdc, h: *mut core::ffi::c_void) -> *mut core::ffi::c_void);
windows_core::link!("gdi32.dll" "system" fn GetTextMetricsW(hdc: Hdc, lptm: *mut TextMetricW) -> windows_core::BOOL);
windows_core::link!("user32.dll" "system" fn DrawTextW(hdc: Hdc, lpchtext: PCWSTR, cchtext: i32, lprc: *mut Rect, format: u32) -> i32);
windows_core::link!("uxtheme.dll" "system" fn BeginBufferedPaint(hdc_target: Hdc, prc_target: *const Rect, dw_format: i32, p_paint_params: *const BpPaintParams, phdc: *mut Hdc) -> isize);
windows_core::link!("uxtheme.dll" "system" fn EndBufferedPaint(hbuffered_paint: isize, f_update_target: windows_core::BOOL) -> HResult);
windows_core::link!("uxtheme.dll" "system" fn GetBufferedPaintBits(hbuffered_paint: isize, ppb_buffer: *mut *mut u32, pcx_row: *mut i32) -> HResult);
windows_core::link!("dwmapi.dll" "system" fn DwmEnableBlurBehindWindow(hwnd: Hwnd, bb: *const DwmBlurBehind) -> HResult);
windows_core::link!("kernel32.dll" "system" fn GetCurrentThreadId() -> u32);

#[repr(C)]
struct DwmBlurBehind {
    flags: u32,
    enable: windows_core::BOOL,
    rgn: *mut core::ffi::c_void,
    transition_on_max: windows_core::BOOL,
}

#[repr(C)]
struct AccentPolicy {
    state: u32,
    flags: u32,
    gradient: u32,
    animation: u32,
}

#[repr(C)]
struct WcaData {
    attrib: u32,
    pv: *const core::ffi::c_void,
    cb: usize,
}

#[repr(C)]
struct Size {
    cx: i32,
    cy: i32,
}

/// `BLENDFUNCTION` — `AC_SRC_OVER`, `0`, `255`, `AC_SRC_ALPHA`.
#[repr(C)]
struct BlendFunction {
    op: u8,
    flags: u8,
    alpha: u8,
    format: u8,
}

/// `BP_PAINTPARAMS` for `BeginBufferedPaint`.
#[repr(C)]
struct BpPaintParams {
    cb_size: u32,
    dw_flags: u32,
    prc_exclude: *const Rect,
    p_blend_function: *const BlendFunction,
}

/// `TEXTMETRICW` — only `ascent` is read (TA_BASELINE rect adjustment).
#[repr(C)]
struct TextMetricW {
    height: i32,
    ascent: i32,
    descent: i32,
    internal_leading: i32,
    external_leading: i32,
    ave_char_width: i32,
    max_char_width: i32,
    weight: i32,
    overhang: i32,
    digitized_x: i32,
    digitized_y: i32,
    first_char: u16,
    last_char: u16,
    default_char: u16,
    break_char: u16,
    italic: u8,
    underlined: u8,
    struck_out: u8,
    pitch_and_family: u8,
    char_set: u8,
}

/// `DTTOPTS` — `DrawThemeTextEx` options block.
#[repr(C)]
#[derive(Clone, Copy)]
struct DttOpts {
    size: u32,
    flags: u32,
    cr_text: u32,
    cr_border: u32,
    cr_shadow: u32,
    shadow_type: i32,
    shadow_offset: [i32; 2],
    border_size: i32,
    font_prop: i32,
    color_prop: i32,
    state: i32,
    apply_overlay: i32,
    glow_size: i32,
    pfn_callback: usize,
    lparam: isize,
}

const GA_ROOT: u32 = 2;
const BLACK_BRUSH: i32 = 4;
const COLOR_WINDOW: i32 = 5;
const TMT_FILLCOLOR: i32 = 3802;
const TMT_TEXTCOLOR: i32 = 3803;
const TMT_FILLCOLORHINT: i32 = 3821;
const DWMWA_USE_HOSTBACKDROPBRUSH: u32 = 17;
const DWMWA_USE_IMMERSIVE_DARK_MODE: u32 = 20;
const DWMWA_SYSTEMBACKDROP_TYPE: u32 = 38;
const DWM_BB_ENABLE: u32 = 1;
const DWM_BB_BLURREGION: u32 = 2;
const WCA_ACCENT_POLICY: u32 = 19;
/// Keeps DWM evaluating the system backdrop as if `CabinetWClass` is
/// always active — otherwise `DWMSBT_TRANSIENTWINDOW` drops to a flat
/// `#585858` sheet on focus loss.
const WCA_FORCE_ACTIVEWINDOW_APPEARANCE: u32 = 15;
const WM_NCACTIVATE: u32 = 0x0086;
const WM_THEMECHANGED: u32 = 0x031A;
const SMTO_ABORTIFHUNG: u32 = 0x0002;
const RDW_INVALIDATE: u32 = 0x0001;
const RDW_ERASE: u32 = 0x0004;
const RDW_FRAME: u32 = 0x0400;
const RDW_ALLCHILDREN: u32 = 0x0080;

/// Theme classes whose background parts get punched to transparent black —
/// the mod's exact target list for the DirectUI/Win32 surfaces.
const BG_CLASSES: &[&str] = &[
    "ItemsView",
    "ListView",
    "TreeView",
    "Header",
    "CommandModule",
    "PreviewPane",
    "Toolbar",
    "Rebar",
    "Status",
    "AddressBand",
    "SearchBox",
];

/// Theme classes whose text is forced white while the glass is dark —
/// keeps folder/tree/header text readable on the glass sheet.
const TEXT_CLASSES: &[&str] = &[
    "ItemsView",
    "ListView",
    "TreeView",
    "Header",
    "CommandModule",
    "PreviewPane",
    "Toolbar",
];

static ARMED: AtomicBool = AtomicBool::new(false);
static DETOURS: Mutex<Vec<retour::RawDetour>> = Mutex::new(Vec::new());
static TRAMPS: LazyLock<Mutex<HashMap<&'static str, usize>>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));
/// `HTHEME` → class list (semicolon-separated, as passed to
/// `OpenThemeData`). Populated by the open-theme detour.
static THEME_CLASSES: LazyLock<Mutex<HashMap<usize, String>>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));

/// The backdrop mode the platform wants on cabinets — pushed via the
/// `FEBD` COPYDATA (BUG 2). Defaults to `DWMSBT_TRANSIENTWINDOW` (3) so a
/// `set_armed` that races ahead of the `FEBD` still applies the working
/// 23H2 system acrylic instead of explorer's opaque tabbed backdrop.
static DESIRED_BACKDROP: core::sync::atomic::AtomicI32 = core::sync::atomic::AtomicI32::new(3);
/// Accent policy the platform wants — `ACCENT_STATE_*` (0 = disabled,
/// 4 = acrylic blur behind), flags, and the ABGR `GradientColor`.
/// Defaults to DISABLED: `WCA_ACCENT_POLICY` conflicts with the
/// `DWMSBT_TRANSIENTWINDOW` system backdrop and cannot compose on
/// `CabinetWClass`.
static ACCENT_STATE: core::sync::atomic::AtomicU32 = core::sync::atomic::AtomicU32::new(0);
static ACCENT_FLAGS: core::sync::atomic::AtomicU32 = core::sync::atomic::AtomicU32::new(0);
/// `FEBD` slot 4: `0` = transparent `BLACK_BRUSH` punch; `0xAABBGGRR`
/// otherwise — the default smoked-glass veil `0x50161210` (~31% dark
/// charcoal, keeps white file names readable over bright windows) or the
/// darkened swatch `0x48BBGGRR`.
static ACCENT_GRADIENT: core::sync::atomic::AtomicU32 =
    core::sync::atomic::AtomicU32::new(0x50161210);

/// uxtheme **Ordinal 74** — `GetThemeClass`: resolves a class name for
/// any `HTHEME`, including handles explorer opened before we armed (the
/// `OpenThemeData` detour can only record post-arm opens).
type GetThemeClassFn = unsafe extern "system" fn(HTheme, *mut u16, i32) -> HResult;
static GET_THEME_CLASS: core::sync::atomic::AtomicUsize = core::sync::atomic::AtomicUsize::new(0);

pub fn set_armed(on: bool) {
    ARMED.store(on, Ordering::Release);
    if on {
        install();
        apply_to_cabinets();
    } else {
        restore_cabinets();
    }
    refresh_cabinets();
}

pub fn is_armed() -> bool {
    ARMED.load(Ordering::Acquire)
}

/// The platform's desired backdrop/accent state — `FEBD` payload:
/// `[backdrop:i32, accent_state:u32, accent_flags:u32, gradient:u32]`
/// little-endian. Re-applies to every live cabinet while armed.
pub fn set_backdrop_params(bytes: &[u8]) {
    if bytes.len() < 16 {
        return;
    }
    let i32le = |o: usize| i32::from_le_bytes(bytes[o..o + 4].try_into().unwrap());
    let u32le = |o: usize| u32::from_le_bytes(bytes[o..o + 4].try_into().unwrap());
    DESIRED_BACKDROP.store(i32le(0), Ordering::Release);
    ACCENT_STATE.store(u32le(4), Ordering::Release);
    ACCENT_FLAGS.store(u32le(8), Ordering::Release);
    ACCENT_GRADIENT.store(u32le(12), Ordering::Release);
    if is_armed() {
        apply_to_cabinets();
        refresh_cabinets();
    }
}

fn tramp<F>(name: &'static str) -> Option<F> {
    let p = *TRAMPS.lock().unwrap_or_else(|p| p.into_inner()).get(name)?;
    if p == 0 {
        return None;
    }
    Some(unsafe { core::mem::transmute_copy::<usize, F>(&p) })
}

// ------------------------------------------------------------ scoping ----
fn is_cabinet(hwnd: Hwnd) -> bool {
    if hwnd.is_null() {
        return false;
    }
    // "CabinetWClass" — 13 UTF-16 units.
    const EXPECT: [u16; 13] = [67, 97, 98, 105, 110, 101, 116, 87, 67, 108, 97, 115, 115];
    let mut cls = [0u16; 32];
    let n = unsafe {
        win32::GetClassNameW(
            crate::bindings::HWND(hwnd),
            windows_core::PWSTR(cls.as_mut_ptr()),
            32,
        )
    };
    n == 13 && cls[..13] == EXPECT
}

/// DirectUI/SysTreeView32 paint through `BeginBufferedPaint` — a memory
/// DC where `WindowFromDC` returns NULL. The DC's window, if resolvable,
/// is checked; for buffered DCs we fall back to the calling thread owning
/// a cabinet top window (the DirectUI paint runs on the cabinet UI
/// thread).
fn cabinet_dc(hdc: Hdc) -> bool {
    if hdc.is_null() {
        return false;
    }
    let w = unsafe { WindowFromDC(hdc) };
    if w.is_null() {
        return thread_has_cabinet();
    }
    is_cabinet(unsafe { GetAncestor(w, GA_ROOT) })
}

/// uxtheme calls carry no HWND — the cabinet's DirectUI paints on the UI
/// thread that owns the `CabinetWClass` top window.
fn thread_has_cabinet() -> bool {
    struct Ctx {
        found: bool,
    }
    unsafe extern "system" fn cb(hwnd: Hwnd, lparam: isize) -> windows_core::BOOL {
        let ctx = unsafe { &mut *(lparam as *mut Ctx) };
        if is_cabinet(hwnd) {
            ctx.found = true;
        }
        windows_core::BOOL(!ctx.found as i32)
    }
    let mut ctx = Ctx { found: false };
    unsafe {
        let _ = EnumThreadWindows(
            GetCurrentThreadId(),
            Some(cb),
            &mut ctx as *mut Ctx as isize,
        );
    }
    ctx.found
}

fn class_of(htheme: HTheme) -> Option<String> {
    if htheme.is_null() {
        return None;
    }
    // Ordinal 74 (`GetThemeClass`) FIRST — it returns the clean base
    // class (`TreeView`, `ItemsView`, …) for every handle, even ones
    // opened before we armed. The `OpenThemeData` detour's recorded
    // string can carry `DarkMode_Explorer::TreeView`-style prefixes,
    // which would poison the cache and never match `BG_CLASSES`.
    let f = GET_THEME_CLASS.load(Ordering::Acquire);
    if f != 0 {
        let f: GetThemeClassFn = unsafe { core::mem::transmute(f) };
        let mut buf = [0u16; 128];
        if unsafe { f(htheme, buf.as_mut_ptr(), buf.len() as i32) } == 0 {
            let len = buf.iter().position(|&c| c == 0).unwrap_or(buf.len());
            let cls = String::from_utf16_lossy(&buf[..len]);
            if !cls.is_empty() {
                THEME_CLASSES
                    .lock()
                    .unwrap_or_else(|p| p.into_inner())
                    .insert(htheme as usize, cls.clone());
                return Some(cls);
            }
        }
    }
    // Fallback: whatever the `OpenThemeData` detour recorded.
    THEME_CLASSES
        .lock()
        .unwrap_or_else(|p| p.into_inner())
        .get(&(htheme as usize))
        .cloned()
}

fn class_in(htheme: HTheme, list: &[&str]) -> bool {
    class_of(htheme)
        .map(|c| {
            list.iter().any(|t| {
                c.split(';').any(|p| {
                    // Strip `SubApp::` prefixes — explorer asks for e.g.
                    // `DarkMode_Explorer::TreeView`; the base class after
                    // the last `::` is what the part list uses.
                    let base = p.rsplit("::").next().unwrap_or(p).trim();
                    base.eq_ignore_ascii_case(t)
                })
            })
        })
        .unwrap_or(false)
}

fn bg_class(htheme: HTheme) -> bool {
    class_in(htheme, BG_CLASSES)
}

// ------------------------------------------------- in-process cabinet push
/// Live `CabinetWClass` count — `site::ensure_watching_winui3` watches
/// this to detect NEW islands (each new File Explorer window registers
/// the next `WinUIVisualDiagConnection{N}` endpoint).
pub fn cabinet_count() -> usize {
    cabinet_windows().len()
}

/// Every live `CabinetWClass` top window (the platform's `WorkerW`-child
/// exclusion: legacy Control Panel frames are skipped).
fn cabinet_windows() -> Vec<Hwnd> {
    struct Ctx {
        out: Vec<Hwnd>,
    }
    unsafe extern "system" fn cb(hwnd: Hwnd, lparam: isize) -> windows_core::BOOL {
        let ctx = unsafe { &mut *(lparam as *mut Ctx) };
        if is_cabinet(hwnd) {
            let mut worker = [0u16; 8];
            for (i, c) in "WorkerW".encode_utf16().enumerate() {
                worker[i] = c;
            }
            if unsafe {
                FindWindowExW(
                    hwnd,
                    core::ptr::null_mut(),
                    PCWSTR(worker.as_ptr()),
                    PCWSTR::null(),
                )
            }
            .is_null()
            {
                ctx.out.push(hwnd);
            }
        }
        windows_core::BOOL(1)
    }
    let mut ctx = Ctx { out: Vec::new() };
    unsafe {
        let _ = EnumWindows(Some(cb), &mut ctx as *mut Ctx as isize);
    }
    ctx.out
}

/// Pushes the stored backdrop/accent state onto one cabinet in-process —
/// the same calls the platform makes cross-process, but from inside
/// explorer where no marshalling or attribute-spoofing concerns apply.
/// Explorer's own later rewrites are intercepted by `dwm_setattr_hook`,
/// which substitutes `DESIRED_BACKDROP`.
fn apply_glass_to(hwnd: Hwnd) {
    let backdrop = DESIRED_BACKDROP.load(Ordering::Acquire).max(0);
    let accent_state = ACCENT_STATE.load(Ordering::Acquire);
    // Immersive dark caption while glass is armed.
    let dark: i32 = 1;
    let _ = unsafe {
        dwm_setattr_call(
            hwnd,
            DWMWA_USE_IMMERSIVE_DARK_MODE,
            &dark as *const i32 as *const core::ffi::c_void,
            4,
        )
    };
    // Glass sheet over the full client area — the detour pins it so
    // explorer can't collapse the margins later.
    let m = Margins {
        left: -1,
        right: -1,
        top: -1,
        bottom: -1,
    };
    if let Some(f) = tramp::<DwmExtendFn>("DwmExtendFrameIntoClientArea") {
        let _ = unsafe { f(hwnd, &m) };
    }
    let _ = unsafe {
        dwm_setattr_call(
            hwnd,
            DWMWA_SYSTEMBACKDROP_TYPE,
            &backdrop as *const i32 as *const core::ffi::c_void,
            4,
        )
    };
    // DwmEnableBlurBehindWindow: acrylic-blur path wants the whole client
    // region blurred; the SYSTEMBACKDROP path wants it off.
    let rgn = unsafe { CreateRectRgn(0, 0, -1, -1) };
    let bb = DwmBlurBehind {
        flags: DWM_BB_ENABLE | DWM_BB_BLURREGION,
        enable: windows_core::BOOL((accent_state == 4) as i32),
        rgn,
        transition_on_max: windows_core::BOOL(0),
    };
    let _ = unsafe { DwmEnableBlurBehindWindow(hwnd, &bb) };
    if !rgn.is_null() {
        let _ = unsafe { DeleteObject(rgn) };
    }
    // Accent policy — mutually exclusive with SYSTEMBACKDROP (BUG 2):
    // acrylic blur rides ACCENT_STATE=4 with the tint GradientColor,
    // otherwise ACCENT_DISABLED.
    let policy = AccentPolicy {
        state: accent_state,
        flags: ACCENT_FLAGS.load(Ordering::Acquire),
        gradient: ACCENT_GRADIENT.load(Ordering::Acquire),
        animation: 0,
    };
    let data = WcaData {
        attrib: WCA_ACCENT_POLICY,
        pv: &policy as *const AccentPolicy as *const core::ffi::c_void,
        cb: core::mem::size_of::<AccentPolicy>(),
    };
    let _ = unsafe {
        SetWindowCompositionAttribute(hwnd, &data as *const WcaData as *const core::ffi::c_void)
    };
    // WCA_FORCE_ACTIVEWINDOW_APPEARANCE — the TRANSIENTWINDOW backdrop
    // only blurs while DWM treats the window as active; force the
    // appearance, then feed a fake activation so DWM re-evaluates now.
    set_force_active(hwnd, true);
    let _ = unsafe { DefWindowProcW(hwnd, WM_NCACTIVATE, 1, 0) };
}

/// `WCA_FORCE_ACTIVEWINDOW_APPEARANCE` (`1` armed / `0` restore) — without
/// it DWM renders the unfocused window's system backdrop as opaque gray.
fn set_force_active(hwnd: Hwnd, on: bool) {
    let value: i32 = on as i32;
    let data = WcaData {
        attrib: WCA_FORCE_ACTIVEWINDOW_APPEARANCE,
        pv: &value as *const i32 as *const core::ffi::c_void,
        cb: core::mem::size_of::<i32>(),
    };
    let _ = unsafe {
        SetWindowCompositionAttribute(hwnd, &data as *const WcaData as *const core::ffi::c_void)
    };
}

/// Raw `DwmSetWindowAttribute` — bypasses our own detour via the
/// trampoline so in-process pushes always use the real entry point.
unsafe fn dwm_setattr_call(
    hwnd: Hwnd,
    attr: u32,
    pv: *const core::ffi::c_void,
    cb: u32,
) -> HResult {
    match tramp::<DwmSetAttrFn>("DwmSetWindowAttribute") {
        Some(f) => unsafe { f(hwnd, attr, pv, cb) },
        None => -1,
    }
}

/// Restores stock window composition on disarm: collapsed margins, the
/// stock tabbed-window backdrop explorer requests for itself, accent off.
fn restore_glass_on(hwnd: Hwnd) {
    let m = Margins {
        left: 0,
        right: 0,
        top: 0,
        bottom: 0,
    };
    if let Some(f) = tramp::<DwmExtendFn>("DwmExtendFrameIntoClientArea") {
        let _ = unsafe { f(hwnd, &m) };
    }
    let stock_backdrop: i32 = 4; // DWMSBT_TABBEDWINDOW — explorer's default
    let _ = unsafe {
        dwm_setattr_call(
            hwnd,
            DWMWA_SYSTEMBACKDROP_TYPE,
            &stock_backdrop as *const i32 as *const core::ffi::c_void,
            4,
        )
    };
    let bb = DwmBlurBehind {
        flags: DWM_BB_ENABLE,
        enable: windows_core::BOOL(0),
        rgn: core::ptr::null_mut(),
        transition_on_max: windows_core::BOOL(0),
    };
    let _ = unsafe { DwmEnableBlurBehindWindow(hwnd, &bb) };
    let policy = AccentPolicy {
        state: 0,
        flags: 0,
        gradient: 0,
        animation: 0,
    };
    let data = WcaData {
        attrib: WCA_ACCENT_POLICY,
        pv: &policy as *const AccentPolicy as *const core::ffi::c_void,
        cb: core::mem::size_of::<AccentPolicy>(),
    };
    let _ = unsafe {
        SetWindowCompositionAttribute(hwnd, &data as *const WcaData as *const core::ffi::c_void)
    };
    set_force_active(hwnd, false);
}

fn apply_to_cabinets() {
    for hwnd in cabinet_windows() {
        apply_glass_to(hwnd);
    }
}

fn restore_cabinets() {
    for hwnd in cabinet_windows() {
        restore_glass_on(hwnd);
    }
}

const TVM_SETBKCOLOR: u32 = 0x111D; // TV_FIRST + 29
const TVM_SETEXTENDEDSTYLE: u32 = 0x112C; // TV_FIRST + 44
const TVS_EX_DOUBLEBUFFER: isize = 0x0004;

/// `WM_THEMECHANGED` + a full redraw — forces DirectUI to repaint through
/// our (now armed/disarmed) hooks immediately, instead of leaving the
/// stock/opaque pixels until the next navigation. The top window gets the
/// theme message FIRST — it re-themes descendants itself, so sending it
/// after `EnumChildWindows` would let explorer overwrite our
/// `TVM_SETBKCOLOR`/`TVM_SETEXTENDEDSTYLE` pushes on `SysTreeView32`.
/// The tree also caches a `COLOR_WINDOW`-colored background and paints
/// through an offscreen `TVS_EX_DOUBLEBUFFER`: both are switched while
/// armed (`RGB(0,0,0)` / double-buffer off) and restored on disarm.
fn refresh_cabinets() {
    let armed = is_armed();
    for hwnd in cabinet_windows() {
        // ---- top window first: it re-themes its own descendants ----
        unsafe {
            let mut result = 0usize;
            let _ = SendMessageTimeoutW(
                hwnd,
                WM_THEMECHANGED,
                0,
                0,
                SMTO_ABORTIFHUNG,
                200,
                &mut result,
            );
        }
        // ---- every descendant: WM_THEMECHANGED + the tree's settings ----
        unsafe extern "system" fn child_cb(hwnd: Hwnd, lparam: isize) -> windows_core::BOOL {
            let armed = lparam != 0;
            let mut result = 0usize;
            unsafe {
                let _ = SendMessageTimeoutW(
                    hwnd,
                    WM_THEMECHANGED,
                    0,
                    0,
                    SMTO_ABORTIFHUNG,
                    200,
                    &mut result,
                );
                // "SysTreeView32" — 13 UTF-16 units.
                const TREE: [u16; 13] =
                    [83, 121, 115, 84, 114, 101, 101, 86, 105, 101, 119, 51, 50];
                let mut cls = [0u16; 32];
                let n = win32::GetClassNameW(
                    crate::bindings::HWND(hwnd),
                    windows_core::PWSTR(cls.as_mut_ptr()),
                    32,
                );
                if n == 13 && cls[..13] == TREE {
                    let color: isize = if armed { 0 } else { -1 };
                    let _ = SendMessageTimeoutW(
                        hwnd,
                        TVM_SETBKCOLOR,
                        0,
                        color,
                        SMTO_ABORTIFHUNG,
                        200,
                        &mut result,
                    );
                    // Kill the offscreen double-buffer while armed — it
                    // composites the tree opaque before our hooks can see
                    // the background fill. Restored on disarm.
                    let style: isize = if armed { 0 } else { TVS_EX_DOUBLEBUFFER };
                    let _ = SendMessageTimeoutW(
                        hwnd,
                        TVM_SETEXTENDEDSTYLE,
                        TVS_EX_DOUBLEBUFFER as usize,
                        style,
                        SMTO_ABORTIFHUNG,
                        200,
                        &mut result,
                    );
                    let _ = RedrawWindow(
                        hwnd,
                        core::ptr::null(),
                        core::ptr::null_mut(),
                        RDW_INVALIDATE | RDW_ERASE,
                    );
                }
            }
            windows_core::BOOL(1)
        }
        unsafe {
            let _ = EnumChildWindows(hwnd, Some(child_cb), armed as isize);
            let _ = RedrawWindow(
                hwnd,
                core::ptr::null(),
                core::ptr::null_mut(),
                RDW_INVALIDATE | RDW_ERASE | RDW_FRAME | RDW_ALLCHILDREN,
            );
        }
    }
}

// -------------------------------------------------------------- hooks ----
type DwmExtendFn = unsafe extern "system" fn(Hwnd, *const Margins) -> HResult;
unsafe extern "system" fn dwm_extend_hook(hwnd: Hwnd, margins: *const Margins) -> HResult {
    let orig: DwmExtendFn = match tramp("DwmExtendFrameIntoClientArea") {
        Some(f) => f,
        None => return -1,
    };
    if is_armed() && is_cabinet(hwnd) {
        let m = Margins {
            left: -1,
            right: -1,
            top: -1,
            bottom: -1,
        };
        return unsafe { orig(hwnd, &m) };
    }
    unsafe { orig(hwnd, margins) }
}

type DwmSetAttrFn = unsafe extern "system" fn(Hwnd, u32, *const core::ffi::c_void, u32) -> HResult;
unsafe extern "system" fn dwm_setattr_hook(
    hwnd: Hwnd,
    attr: u32,
    pv: *const core::ffi::c_void,
    cb: u32,
) -> HResult {
    // Explorer re-asserts its own backdrop/hostbrush on navigation —
    // substitute our desired backdrop instead of swallowing silently:
    // SYSTEMBACKDROP and accent are mutually exclusive, so a dropped
    // rewrite would leave a solid-black extended frame. `FEBD` sets the
    // desired value; unset means just swallow.
    if is_armed()
        && is_cabinet(hwnd)
        && (attr == DWMWA_SYSTEMBACKDROP_TYPE || attr == DWMWA_USE_HOSTBACKDROPBRUSH)
    {
        let desired = DESIRED_BACKDROP.load(Ordering::Acquire);
        if desired < 0 {
            return 0;
        }
        let orig: DwmSetAttrFn = match tramp("DwmSetWindowAttribute") {
            Some(f) => f,
            None => return -1,
        };
        return unsafe {
            orig(
                hwnd,
                DWMWA_SYSTEMBACKDROP_TYPE,
                &desired as *const i32 as *const core::ffi::c_void,
                4,
            )
        };
    }
    let orig: DwmSetAttrFn = match tramp("DwmSetWindowAttribute") {
        Some(f) => f,
        None => return -1,
    };
    unsafe { orig(hwnd, attr, pv, cb) }
}

type OpenThemeFn = unsafe extern "system" fn(Hwnd, PCWSTR) -> HTheme;
unsafe extern "system" fn open_theme_hook(hwnd: Hwnd, classlist: PCWSTR) -> HTheme {
    let orig: OpenThemeFn = match tramp("OpenThemeData") {
        Some(f) => f,
        None => return core::ptr::null_mut(),
    };
    let h = unsafe { orig(hwnd, classlist) };
    if !h.is_null() && !classlist.is_null() {
        let s = unsafe { classlist.to_string().unwrap_or_default() };
        if !s.is_empty() {
            THEME_CLASSES
                .lock()
                .unwrap_or_else(|p| p.into_inner())
                .insert(h as usize, s);
        }
    }
    h
}

type GetThemeColorFn = unsafe extern "system" fn(HTheme, i32, i32, i32, *mut u32) -> HResult;
unsafe extern "system" fn get_theme_color_hook(
    htheme: HTheme,
    part: i32,
    state: i32,
    prop: i32,
    pcolor: *mut u32,
) -> HResult {
    if is_armed() && !pcolor.is_null() && thread_has_cabinet() {
        // Translucent-Windows lines 920–929: on a cabinet thread EVERY
        // fill/fill-hint color goes transparent black — the opaque panel
        // classes open under prefixed names (`DarkMode_*::`) that class
        // matching can miss, and any fill on this thread is a panel
        // background hiding the glass.
        if prop == TMT_FILLCOLOR || prop == TMT_FILLCOLORHINT {
            // Only CONTAINER backgrounds go transparent — `part >= 1` on
            // ItemsView/ListView/TreeView is a row's hover/selection fill,
            // which must keep its real color or highlights vanish.
            if part == 0 || (part == 1 && state == 0) {
                unsafe {
                    *pcolor = 0;
                }
                return 0;
            }
        }
        // Folder/tree/header text on the dark glass sheet — force white
        // so it stays readable (the light-mode default would vanish).
        if prop == TMT_TEXTCOLOR && class_in(htheme, TEXT_CLASSES) {
            unsafe {
                *pcolor = 0x00FFFFFF;
            }
            return 0;
        }
    }
    let orig: GetThemeColorFn = match tramp("GetThemeColor") {
        Some(f) => f,
        None => return -1,
    };
    unsafe { orig(htheme, part, state, prop, pcolor) }
}

/// `StretchDIBits` raster op — plain source overwrite (no alpha blending,
/// so repeated punches never stack).
const SRCCOPY: u32 = 0x00CC0020;

/// `BITMAPINFO` for a 1×1 32-bpp top-down DIB — `biHeight < 0` makes row 0
/// the TOP of the image.
#[repr(C)]
struct BitmapInfo {
    size: u32,
    width: i32,
    height: i32,
    planes: u16,
    bpp: u16,
    compression: u32,
    image_size: u32,
    xppm: i32,
    yppm: i32,
    used: u32,
    important: u32,
    colors: [u32; 1],
}

/// Fill a punched background rect. With no Color Tint swatch
/// (`ACCENT_GRADIENT == 0`) this is transparent `BLACK_BRUSH` through the
/// FillRect TRAMPOLINE — the hooked export would re-enter
/// `fill_rect_hook`, and `GetStockObject(BLACK_BRUSH)` itself resolves to
/// a `BS_SOLID 0x00000000` stock-panel brush → recursion. With a swatch
/// (`0x24BBGGRR`) it stretches one premultiplied BGRA pixel over the rect:
/// `SRCCOPY` overwrites in a single pass, so the same ~14% veil lands on
/// `SysTreeView32`/`DirectUIHWND` that the XAML ruleset puts on the top
/// bar, and `DWMSBT_TRANSIENTWINDOW` keeps blurring through.
fn punch_black(hdc: Hdc, rect: *const Rect) -> bool {
    let grad = ACCENT_GRADIENT.load(Ordering::Acquire);
    if grad != 0 {
        let a = (grad >> 24) & 0xFF;
        let (r, g, b) = (grad & 0xFF, (grad >> 8) & 0xFF, (grad >> 16) & 0xFF);
        let pixel: u32 =
            a << 24 | (((r * a) / 255) << 16) | (((g * a) / 255) << 8) | ((b * a) / 255);
        let bmi = BitmapInfo {
            size: 40,
            width: 1,
            height: -1,
            planes: 1,
            bpp: 32,
            compression: 0, // BI_RGB
            image_size: 0,
            xppm: 0,
            yppm: 0,
            used: 0,
            important: 0,
            colors: [0],
        };
        let (x, y, w, h) = unsafe {
            (
                (*rect).left,
                (*rect).top,
                (*rect).right - (*rect).left,
                (*rect).bottom - (*rect).top,
            )
        };
        if w > 0 && h > 0 {
            return unsafe {
                StretchDIBits(
                    hdc,
                    x,
                    y,
                    w,
                    h,
                    0,
                    0,
                    1,
                    1,
                    &pixel as *const u32 as *const core::ffi::c_void,
                    &bmi as *const BitmapInfo as *const core::ffi::c_void,
                    0, // DIB_RGB_COLORS
                    SRCCOPY,
                )
            } != 0;
        }
    }
    let black = unsafe { GetStockObject(BLACK_BRUSH) as HBrush };
    match tramp::<FillRectFn>("FillRect") {
        Some(f) => unsafe { f(hdc, rect, black) != 0 },
        None => false,
    }
}

/// Background parts the mod punches out: `iPartId == 0` (ItemsView/
/// TreeView/Header/Rebar/Status backgrounds) and `iPartId == 1,
/// iStateId == 0` (CommandModule/PreviewPane background).
fn is_bg_part(part: i32, state: i32) -> bool {
    part == 0 || (part == 1 && state == 0)
}

/// Part/state punch rule — `TreeView` is special: `TVP_TREEITEM` rows
/// only punch `TREIS_NORMAL` (1); `TREIS_HOT`/`SELECTED`/
/// `SELECTEDNOTFOCUS`/`HOTSELECTED` (2/3/5/6) must still draw their
/// hover/selection highlight, so they fall through to the original.
fn should_punch_part(htheme: HTheme, part: i32, state: i32) -> bool {
    if class_in(htheme, &["TreeView"]) {
        return part == 0 || (part == 1 && state == 1);
    }
    // ItemsView/ListView: `part == 0` is the container background — punch
    // it; `part >= 1` is LVP_LISTITEM hover/selection/rect highlights,
    // which must draw or rows lose all interactivity cues.
    if class_in(htheme, &["ItemsView", "ListView"]) {
        return part == 0;
    }
    bg_class(htheme) && is_bg_part(part, state)
}

type DrawThemeBgFn =
    unsafe extern "system" fn(HTheme, Hdc, i32, i32, *const Rect, *const Rect) -> HResult;
unsafe extern "system" fn draw_theme_bg_hook(
    htheme: HTheme,
    hdc: Hdc,
    part: i32,
    state: i32,
    rect: *const Rect,
    clip: *const Rect,
) -> HResult {
    if is_armed() && !rect.is_null() && cabinet_dc(hdc) && should_punch_part(htheme, part, state) {
        if punch_black(hdc, rect) {
            return 0;
        }
    }
    let orig: DrawThemeBgFn = match tramp("DrawThemeBackground") {
        Some(f) => f,
        None => return -1,
    };
    unsafe { orig(htheme, hdc, part, state, rect, clip) }
}

type DrawThemeBgExFn =
    unsafe extern "system" fn(HTheme, Hdc, i32, i32, *const Rect, *const DtbgOpts) -> HResult;
unsafe extern "system" fn draw_theme_bg_ex_hook(
    htheme: HTheme,
    hdc: Hdc,
    part: i32,
    state: i32,
    rect: *const Rect,
    opts: *const DtbgOpts,
) -> HResult {
    if is_armed() && !rect.is_null() && cabinet_dc(hdc) && should_punch_part(htheme, part, state) {
        if punch_black(hdc, rect) {
            return 0;
        }
    }
    let orig: DrawThemeBgExFn = match tramp("DrawThemeBackgroundEx") {
        Some(f) => f,
        None => return -1,
    };
    unsafe { orig(htheme, hdc, part, state, rect, opts) }
}

type GetSysColorBrushFn = unsafe extern "system" fn(i32) -> HBrush;
unsafe extern "system" fn get_sys_color_brush_hook(index: i32) -> HBrush {
    if is_armed() && index == COLOR_WINDOW && thread_has_cabinet() {
        return unsafe { GetStockObject(BLACK_BRUSH) as HBrush };
    }
    let orig: GetSysColorBrushFn = match tramp("GetSysColorBrush") {
        Some(f) => f,
        None => return core::ptr::null_mut(),
    };
    unsafe { orig(index) }
}

#[repr(C)]
struct LogBrush {
    style: u32,
    color: u32,
    hatch: usize,
}

/// Explorer's stock opaque panel colors (BGR, alpha byte ignored) —
/// `SysTreeView32`/DirectUI cache `CreateSolidBrush` handles for these
/// rather than asking the system brush table. `0x00000000` covers raw
/// black brushes too — safe because our own punches go through the
/// trampoline, never re-entering this check.
const STOCK_PANEL_COLORS: &[u32] = &[
    0x00191919, 0x00181818, 0x00141414, 0x00202020, 0x001F1F1F, 0x002C2C2C, 0x00FFFFFF, 0x00F3F3F3,
    0x00000000,
];

/// `true` when `brush` is a `BS_SOLID` brush in one of explorer's stock
/// opaque panel colors.
fn is_stock_panel_brush(brush: HBrush) -> bool {
    if brush.is_null() || (brush as usize) < 0x20 {
        // Pseudo/system brushes aren't GDI objects — GetObjectW would fail.
        return false;
    }
    let mut lb = LogBrush {
        style: 0,
        color: 0,
        hatch: 0,
    };
    let n = unsafe {
        GetObjectW(
            brush,
            core::mem::size_of::<LogBrush>() as i32,
            &mut lb as *mut LogBrush as *mut core::ffi::c_void,
        )
    };
    n == core::mem::size_of::<LogBrush>() as i32
        && lb.style == 0 /* BS_SOLID */
        && STOCK_PANEL_COLORS.contains(&(lb.color & 0x00FFFFFF))
}

type FillRectFn = unsafe extern "system" fn(Hdc, *const Rect, HBrush) -> i32;
unsafe extern "system" fn fill_rect_hook(hdc: Hdc, rect: *const Rect, brush: HBrush) -> i32 {
    if is_armed() && !rect.is_null() && cabinet_dc(hdc) {
        // COLOR_WINDOW fills inside cabinets paint the opaque panel. Two
        // brush spellings hit this path: the real `GetSysColorBrush`
        // handle AND the pseudo-handle `COLOR_WINDOW + 1` (GDI resolves
        // `(HBRUSH)(index+1)` internally — matching the system-brush
        // table entry directly).
        let pseudo = (brush as usize).wrapping_sub(1) == COLOR_WINDOW as usize;
        let real = tramp::<GetSysColorBrushFn>("GetSysColorBrush")
            .map(|orig| unsafe { orig(COLOR_WINDOW) } == brush)
            .unwrap_or(false);
        // Dark-mode panels often skip the system-brush table entirely and
        // cache a `CreateSolidBrush(#191919 / #202020 / #F3F3F3 / …)`.
        // `GetObjectW` exposes the brush's real color. `SysTreeView32`
        // fills its rows (~28px) one by one, so only the WIDTH gates the
        // punch: icons/checkmarks stay under 32px, and hover/selection
        // pills in TreeView/ItemsView draw via DrawThemeBackground or
        // Direct2D, never `FillRect` with a panel color.
        let solid_panel =
            is_stock_panel_brush(brush) && unsafe { (*rect).right - (*rect).left > 40 };
        if pseudo || real || solid_panel {
            // `punch_black` picks transparent black or the FEBD tint —
            // and always goes through the FillRect trampoline, so a
            // BLACK_BRUSH (itself a stock panel color) can't re-enter.
            return if punch_black(hdc, rect) { 1 } else { -1 };
        }
    }
    let orig: FillRectFn = match tramp("FillRect") {
        Some(f) => f,
        None => return -1,
    };
    unsafe { orig(hdc, rect, brush) }
}

// ------------------------------------------------------------------ text
/// `ETO_*`/`TA_*`/`DT_*`/`BPPF_*`/`BPBF_*`/`OBJ_*`/`TRANSPARENT` constants
/// used by the buffered-paint text path.
const ETO_OPAQUE: u32 = 0x0002;
const ETO_CLIPPED: u32 = 0x0004;
const ETO_GLYPH_INDEX: u32 = 0x0010;
const ETO_PDY: u32 = 0x2000;
const TA_UPDATECP: u32 = 0x0001;
const TA_RIGHT: u32 = 0x0002;
const TA_CENTER: u32 = 0x0006;
const TA_BOTTOM: u32 = 0x0008;
const TA_BASELINE: u32 = 0x0018;
const DT_CALCRECT: u32 = 0x0400;
const DT_MODIFYSTRING: u32 = 0x00010000;
const DTT_TEXTCOLOR: u32 = 0x00000001;
const DTT_CALCRECT: u32 = 0x00000200;
const DTT_COMPOSITED: u32 = 0x00002000;
const BPBF_TOPDOWNDIB: i32 = 2;
const BPPF_ERASE: u32 = 0x0001;
const BPPF_NOCLIP: u32 = 0x0004;
const OBJ_FONT: u32 = 6;
const BK_TRANSPARENT: i32 = 1;

/// Coverage→alpha gamma curve from Translucent Windows: GDI glyph masks
/// are 8-bit coverage values; mapping them straight to alpha renders thin
/// strokes too faint. `a^(1/gamma)` with `gamma = 1.2 + 0.3a` lifts
/// mid-coverage pixels so anti-aliased edges read sharp on glass.
static TEXT_ALPHA_GAMMA_LUT: LazyLock<[u8; 256]> = LazyLock::new(|| {
    let mut lut = [0u8; 256];
    for (i, slot) in lut.iter_mut().enumerate() {
        let a = i as f32 / 255.0;
        let gamma = 1.2 + 0.3 * a;
        *slot = (a.powf(1.0 / gamma) * 255.0 + 0.5).clamp(0.0, 255.0) as u8;
    }
    lut
});

type ExtTextOutWFn = unsafe extern "system" fn(
    Hdc,
    i32,
    i32,
    u32,
    *const Rect,
    PCWSTR,
    u32,
    *const i32,
) -> windows_core::BOOL;
/// Buffered-paint text composition (Translucent Windows `HookedExtTextOutW`
/// port): a plain `ExtTextOutW` writes glyphs with alpha 0 onto the
/// `0x00000000` surface — premultiplied alpha 0 renders ADDITIVELY under
/// DWM, so bright windows behind Explorer bleed straight through the
/// letters. Instead we draw the string in white into a buffered paint
/// DIB, turn its coverage map into premultiplied BGRA at the caller's
/// text color through the gamma LUT, and let `EndBufferedPaint` alpha-
/// blend the result onto the cabinet DC — razor-sharp opaque text.
unsafe extern "system" fn ext_text_out_hook(
    hdc: Hdc,
    x: i32,
    y: i32,
    options: u32,
    lprect: *const Rect,
    lpstring: PCWSTR,
    c: u32,
    lpdx: *const i32,
) -> windows_core::BOOL {
    let orig: ExtTextOutWFn = match tramp("ExtTextOutW") {
        Some(f) => f,
        None => return windows_core::BOOL(0),
    };
    if !is_armed() || !cabinet_dc(hdc) {
        return unsafe { orig(hdc, x, y, options, lprect, lpstring, c, lpdx) };
    }
    // The opaque background box — punch the veil before any text lands.
    if options & ETO_OPAQUE != 0 && !lprect.is_null() {
        let _ = punch_black(hdc, lprect);
    }
    let no_text =
        lpstring.is_null() || c == 0 || c > 4096 || unsafe { GetTextAlign(hdc) } & TA_UPDATECP != 0;
    if no_text {
        return unsafe { orig(hdc, x, y, options & !ETO_OPAQUE, lprect, lpstring, c, lpdx) };
    }
    // Extent → the buffered-paint target rect.
    let mut size = Size { cx: 0, cy: 0 };
    let measured = unsafe {
        if options & ETO_GLYPH_INDEX != 0 {
            GetTextExtentPointI(hdc, lpstring.0, c as i32, &mut size)
        } else {
            GetTextExtentPoint32W(hdc, lpstring, c as i32, &mut size)
        }
    };
    if measured == windows_core::BOOL(0) {
        return unsafe { orig(hdc, x, y, options & !ETO_OPAQUE, lprect, lpstring, c, lpdx) };
    }
    if !lpdx.is_null() {
        // lpDx holds per-char advances (pairs when ETO_PDY) — sum to the
        // real run width, it can exceed the measured extent.
        let stride = if options & ETO_PDY != 0 { 2usize } else { 1 };
        let mut dx = 0i32;
        for i in 0..c as usize {
            dx += unsafe { *lpdx.add(i * stride) };
        }
        size.cx = size.cx.max(dx);
    }
    let align = unsafe { GetTextAlign(hdc) };
    let (mut px, mut py) = (x, y);
    let a_h = align & 6;
    if a_h == TA_CENTER {
        px -= size.cx / 2;
    } else if a_h == TA_RIGHT {
        px -= size.cx;
    }
    let a_v = align & 24;
    if a_v == TA_BOTTOM {
        py -= size.cy;
    } else if a_v == TA_BASELINE {
        let mut tm: TextMetricW = unsafe { core::mem::zeroed() };
        if unsafe { GetTextMetricsW(hdc, &mut tm) } == windows_core::BOOL(0) {
            return unsafe { orig(hdc, x, y, options & !ETO_OPAQUE, lprect, lpstring, c, lpdx) };
        }
        py -= tm.ascent;
    }
    let mut text_rect = Rect {
        left: px,
        top: py,
        right: px + size.cx,
        bottom: py + size.cy,
    };
    if options & ETO_CLIPPED != 0 && !lprect.is_null() {
        let r = unsafe { &*lprect };
        text_rect.left = text_rect.left.max(r.left);
        text_rect.top = text_rect.top.max(r.top);
        text_rect.right = text_rect.right.min(r.right);
        text_rect.bottom = text_rect.bottom.min(r.bottom);
    }
    let (tw, th) = (
        text_rect.right - text_rect.left,
        text_rect.bottom - text_rect.top,
    );
    if tw <= 0 || th <= 0 {
        return unsafe { orig(hdc, x, y, options & !ETO_OPAQUE, lprect, lpstring, c, lpdx) };
    }
    let blend = BlendFunction {
        op: 0, // AC_SRC_OVER
        flags: 0,
        alpha: 255,
        format: 1, // AC_SRC_ALPHA
    };
    let params = BpPaintParams {
        cb_size: core::mem::size_of::<BpPaintParams>() as u32,
        dw_flags: BPPF_ERASE | BPPF_NOCLIP,
        prc_exclude: core::ptr::null(),
        p_blend_function: &blend,
    };
    let mut mem_dc: Hdc = core::ptr::null_mut();
    let hpb = unsafe { BeginBufferedPaint(hdc, &text_rect, BPBF_TOPDOWNDIB, &params, &mut mem_dc) };
    if hpb == 0 || mem_dc.is_null() {
        return unsafe { orig(hdc, x, y, options & !ETO_OPAQUE, lprect, lpstring, c, lpdx) };
    }
    unsafe {
        let font = GetCurrentObject(hdc, OBJ_FONT);
        if !font.is_null() {
            SelectObject(mem_dc, font);
        }
        SetTextAlign(mem_dc, align);
        SetLayout(mem_dc, GetLayout(hdc));
        SetBkMode(mem_dc, BK_TRANSPARENT);
        // White glyph mask — coverage = luminance of this draw.
        SetTextColor(mem_dc, 0x00FFFFFF);
    }
    let res = unsafe {
        orig(
            mem_dc,
            x,
            y,
            options & !ETO_OPAQUE,
            lprect,
            lpstring,
            c,
            lpdx,
        )
    };
    let mut pixels: *mut u32 = core::ptr::null_mut();
    let mut row_stride = 0i32;
    if unsafe { GetBufferedPaintBits(hpb, &mut pixels, &mut row_stride) } < 0
        || pixels.is_null()
        || row_stride <= 0
    {
        unsafe {
            let _ = EndBufferedPaint(hpb, windows_core::BOOL(0));
        }
        return res;
    }
    // Caller color → premultiplied output. Dark/muddy grays (secondary
    // columns #6D6D6D, disabled state) can't survive over a bright
    // backdrop — lift them: near-black → pure white, mid-gray →
    // high-contrast silver-white.
    let cr = unsafe { GetTextColor(hdc) } & 0x00FFFFFF;
    let (mut r, mut g, mut b) = (cr & 0xFF, (cr >> 8) & 0xFF, (cr >> 16) & 0xFF);
    if r < 180 && g < 180 && b < 180 {
        if r > 90 {
            (r, g, b) = (230, 230, 235);
        } else {
            (r, g, b) = (255, 255, 255);
        }
    }
    let lut = &*TEXT_ALPHA_GAMMA_LUT;
    for cy in 0..th {
        for cx in 0..tw {
            let p = unsafe { pixels.add((cy * row_stride + cx) as usize) };
            let px = unsafe { *p };
            if px & 0x00FFFFFF == 0 {
                continue;
            }
            // White-mask luminance → coverage → gamma-corrected alpha.
            let luma =
                (((px & 0xFF) + (((px >> 8) & 0xFF) << 1) + ((px >> 16) & 0xFF)) >> 2) as usize;
            let txt_a = lut[luma.min(255)] as u32;
            unsafe {
                *p = (txt_a << 24)
                    | (((r * txt_a) >> 8) << 16)
                    | (((g * txt_a) >> 8) << 8)
                    | ((b * txt_a) >> 8);
            }
        }
    }
    unsafe {
        let _ = EndBufferedPaint(hpb, windows_core::BOOL(1));
    }
    res
}

type DrawThemeTextExFn = unsafe extern "system" fn(
    HTheme,
    Hdc,
    i32,
    i32,
    PCWSTR,
    i32,
    u32,
    *mut Rect,
    *const DttOpts,
) -> HResult;
/// `ItemsView` paints secondary columns (`Date modified`, `Type`, `Size`)
/// and group headers via `DrawThemeTextEx`. Force white text and clear
/// `DTT_COMPOSITED` so uxtheme stops doing its own (broken-alpha) glow
/// composite and lands the draw in our gamma-corrected `ext_text_out_hook`.
unsafe extern "system" fn draw_theme_text_ex_hook(
    htheme: HTheme,
    hdc: Hdc,
    part: i32,
    state: i32,
    psz: PCWSTR,
    cch: i32,
    flags: u32,
    rect: *mut Rect,
    opts: *const DttOpts,
) -> HResult {
    let orig: DrawThemeTextExFn = match tramp("DrawThemeTextEx") {
        Some(f) => f,
        None => return -1,
    };
    if is_armed()
        && cabinet_dc(hdc)
        && (opts.is_null() || unsafe { (*opts).flags } & DTT_CALCRECT == 0)
    {
        let mut o = if opts.is_null() {
            DttOpts {
                size: core::mem::size_of::<DttOpts>() as u32,
                flags: 0,
                cr_text: 0,
                cr_border: 0,
                cr_shadow: 0,
                shadow_type: 0,
                shadow_offset: [0; 2],
                border_size: 0,
                font_prop: 0,
                color_prop: 0,
                state: 0,
                apply_overlay: 0,
                glow_size: 0,
                pfn_callback: 0,
                lparam: 0,
            }
        } else {
            unsafe { *opts }
        };
        o.cr_text = 0x00FFFFFF;
        o.flags |= DTT_TEXTCOLOR;
        o.flags &= !DTT_COMPOSITED;
        return unsafe { orig(htheme, hdc, part, state, psz, cch, flags, rect, &o) };
    }
    unsafe { orig(htheme, hdc, part, state, psz, cch, flags, rect, opts) }
}

/// uxtheme **Ordinal 126** — `DrawTextWithGlow`, the ItemsView label
/// painter. With `nGlowRadius == 0` it renders plain text; skip the glow
/// pipeline entirely and draw through `DrawTextW`/the caller's callback
/// so every glyph goes through `ext_text_out_hook`'s alpha composition.
type DrawTextWithGlowFn = unsafe extern "system" fn(
    Hdc,
    PCWSTR,
    i32,
    *mut Rect,
    u32,
    u32,
    u32,
    i32,
    i32,
    i32,
    usize,
    isize,
) -> i32;
type DrawTextCallback = unsafe extern "system" fn(Hdc, PCWSTR, i32, *mut Rect, u32, isize) -> i32;
unsafe extern "system" fn draw_text_with_glow_hook(
    hdc: Hdc,
    psz: PCWSTR,
    cch: i32,
    rect: *mut Rect,
    flags: u32,
    cr_text: u32,
    cr_glow: u32,
    glow_radius: i32,
    glow_intensity: i32,
    premult: i32,
    callback: usize,
    lparam: isize,
) -> i32 {
    let orig: DrawTextWithGlowFn = match tramp("DrawTextWithGlow") {
        Some(f) => f,
        None => return 0,
    };
    if is_armed()
        && glow_radius == 0
        && cabinet_dc(hdc)
        && flags & DT_CALCRECT == 0
        && !rect.is_null()
    {
        unsafe {
            SetTextColor(hdc, 0x00FFFFFF);
            SetBkColor(hdc, 0);
        }
        let f = flags & !DT_MODIFYSTRING;
        if callback != 0 {
            let cb: DrawTextCallback = unsafe { core::mem::transmute(callback) };
            return unsafe { cb(hdc, psz, cch, rect, f, lparam) };
        }
        return unsafe { DrawTextW(hdc, psz, cch, rect, f) };
    }
    unsafe {
        orig(
            hdc,
            psz,
            cch,
            rect,
            flags,
            cr_text,
            cr_glow,
            glow_radius,
            glow_intensity,
            premult,
            callback,
            lparam,
        )
    }
}

// ------------------------------------------------------------ install ----
fn install() {
    let mut guard = DETOURS.lock().unwrap_or_else(|p| p.into_inner());
    if !guard.is_empty() {
        for d in guard.iter() {
            let _ = unsafe { d.enable() };
        }
        return;
    }
    // uxtheme Ordinal 74 — `GetThemeClass`: resolves theme classes for
    // HTHEME handles opened before we armed (BUG 4's cache-miss).
    let uxt_w: Vec<u16> = "uxtheme.dll"
        .encode_utf16()
        .chain(core::iter::once(0))
        .collect();
    let uxt = unsafe { win32::GetModuleHandleW(PCWSTR(uxt_w.as_ptr())) };
    if !uxt.is_null() {
        // MAKEINTRESOURCEA(74) — pass the ordinal as the name pointer.
        let p = unsafe { win32::GetProcAddress(uxt, PCSTR(74usize as *const u8)) };
        if !p.is_null() {
            GET_THEME_CLASS.store(p as usize, Ordering::Release);
        }
    }
    // (dll, proc lookup, TRAMPS key, hook) — lookups starting with `#`
    // are ORDINALS (`DrawTextWithGlow` is ordinal-only in uxtheme).
    let specs: [(&'static str, &'static str, &'static str, *const ()); 11] = [
        (
            "dwmapi.dll",
            "DwmExtendFrameIntoClientArea",
            "DwmExtendFrameIntoClientArea",
            dwm_extend_hook as *const (),
        ),
        (
            "dwmapi.dll",
            "DwmSetWindowAttribute",
            "DwmSetWindowAttribute",
            dwm_setattr_hook as *const (),
        ),
        (
            "uxtheme.dll",
            "OpenThemeData",
            "OpenThemeData",
            open_theme_hook as *const (),
        ),
        (
            "uxtheme.dll",
            "GetThemeColor",
            "GetThemeColor",
            get_theme_color_hook as *const (),
        ),
        (
            "uxtheme.dll",
            "DrawThemeBackground",
            "DrawThemeBackground",
            draw_theme_bg_hook as *const (),
        ),
        (
            "uxtheme.dll",
            "DrawThemeBackgroundEx",
            "DrawThemeBackgroundEx",
            draw_theme_bg_ex_hook as *const (),
        ),
        (
            "uxtheme.dll",
            "DrawThemeTextEx",
            "DrawThemeTextEx",
            draw_theme_text_ex_hook as *const (),
        ),
        (
            "uxtheme.dll",
            "#126",
            "DrawTextWithGlow",
            draw_text_with_glow_hook as *const (),
        ),
        (
            "user32.dll",
            "GetSysColorBrush",
            "GetSysColorBrush",
            get_sys_color_brush_hook as *const (),
        ),
        (
            "user32.dll",
            "FillRect",
            "FillRect",
            fill_rect_hook as *const (),
        ),
        (
            "gdi32.dll",
            "ExtTextOutW",
            "ExtTextOutW",
            ext_text_out_hook as *const (),
        ),
    ];
    let mut installed = Vec::new();
    for (dll, lookup, key, hook) in specs {
        let dll_w: Vec<u16> = dll.encode_utf16().chain(core::iter::once(0)).collect();
        let module = unsafe { win32::GetModuleHandleW(PCWSTR(dll_w.as_ptr())) };
        if module.is_null() {
            continue;
        }
        let target = if let Some(ord) = lookup
            .strip_prefix('#')
            .and_then(|o| o.parse::<usize>().ok())
        {
            // MAKEINTRESOURCEA(ordinal)
            unsafe { win32::GetProcAddress(module, PCSTR(ord as *const u8)) }
        } else {
            let name_z: Vec<u8> = lookup.bytes().chain(core::iter::once(0)).collect();
            unsafe { win32::GetProcAddress(module, PCSTR(name_z.as_ptr())) }
        };
        if target.is_null() {
            continue;
        }
        let Ok(detour) = (unsafe { retour::RawDetour::new(target as *const (), hook) }) else {
            continue;
        };
        TRAMPS
            .lock()
            .unwrap_or_else(|p| p.into_inner())
            .insert(key, detour.trampoline() as *const () as usize);
        if unsafe { detour.enable() }.is_err() {
            continue;
        }
        installed.push(detour);
    }
    crate::tap_log!("fe_glass: {} detours armed", installed.len());
    *guard = installed;
}
