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
//! `ExtTextOutW` alpha correction is intentionally NOT hooked yet — the
//! buffered-paint composition it needs is a larger port, and a wrong
//! implementation renders text invisible. Stock text color is left alone.

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
windows_core::link!("user32.dll" "system" fn FindWindowExW(parent: Hwnd, childafter: Hwnd, class: PCWSTR, title: PCWSTR) -> Hwnd);
windows_core::link!("user32.dll" "system" fn FillRect(hdc: Hdc, lprc: *const Rect, hbr: HBrush) -> i32);
windows_core::link!("user32.dll" "system" fn SendMessageTimeoutW(hwnd: Hwnd, msg: u32, wparam: usize, lparam: isize, flags: u32, timeout: u32, result: *mut usize) -> usize);
windows_core::link!("user32.dll" "system" fn RedrawWindow(hwnd: Hwnd, lprcupdate: *const core::ffi::c_void, hrgnupdate: *mut core::ffi::c_void, flags: u32) -> windows_core::BOOL);
windows_core::link!("user32.dll" "system" fn SetWindowCompositionAttribute(hwnd: Hwnd, data: *const core::ffi::c_void) -> windows_core::BOOL);
windows_core::link!("gdi32.dll" "system" fn GetStockObject(i: i32) -> *mut core::ffi::c_void);
windows_core::link!("gdi32.dll" "system" fn CreateRectRgn(left: i32, top: i32, right: i32, bottom: i32) -> *mut core::ffi::c_void);
windows_core::link!("gdi32.dll" "system" fn DeleteObject(obj: *mut core::ffi::c_void) -> windows_core::BOOL);
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
/// `FEBD` COPYDATA (BUG 2). `-1` = unset (swallow explorer's writes,
/// don't substitute).
static DESIRED_BACKDROP: core::sync::atomic::AtomicI32 = core::sync::atomic::AtomicI32::new(-1);
/// Accent policy the platform wants — `ACCENT_STATE_*` (0 = disabled,
/// 4 = acrylic blur behind), flags, and the ABGR `GradientColor`.
static ACCENT_STATE: core::sync::atomic::AtomicU32 = core::sync::atomic::AtomicU32::new(0);
static ACCENT_FLAGS: core::sync::atomic::AtomicU32 = core::sync::atomic::AtomicU32::new(0);
static ACCENT_GRADIENT: core::sync::atomic::AtomicU32 =
    core::sync::atomic::AtomicU32::new(0x40101010);

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
    if let Some(c) = THEME_CLASSES
        .lock()
        .unwrap_or_else(|p| p.into_inner())
        .get(&(htheme as usize))
        .cloned()
    {
        return Some(c);
    }
    // Handles opened before the detour armed aren't in the map — resolve
    // uxtheme Ordinal 74 (`GetThemeClass`) instead, then cache.
    let f = GET_THEME_CLASS.load(Ordering::Acquire);
    if f == 0 {
        return None;
    }
    let f: GetThemeClassFn = unsafe { core::mem::transmute(f) };
    let mut buf = [0u16; 128];
    if unsafe { f(htheme, buf.as_mut_ptr(), buf.len() as i32) } != 0 {
        return None;
    }
    let len = buf.iter().position(|&c| c == 0).unwrap_or(buf.len());
    let cls = String::from_utf16_lossy(&buf[..len]);
    if cls.is_empty() {
        return None;
    }
    THEME_CLASSES
        .lock()
        .unwrap_or_else(|p| p.into_inner())
        .insert(htheme as usize, cls.clone());
    Some(cls)
}

fn class_in(htheme: HTheme, list: &[&str]) -> bool {
    class_of(htheme)
        .map(|c| {
            list.iter()
                .any(|t| c.split(';').any(|p| p.trim().eq_ignore_ascii_case(t)))
        })
        .unwrap_or(false)
}

fn bg_class(htheme: HTheme) -> bool {
    class_in(htheme, BG_CLASSES)
}

// ------------------------------------------------- in-process cabinet push
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

/// `WM_THEMECHANGED` + a full redraw — forces DirectUI to repaint through
/// our (now armed/disarmed) hooks immediately, instead of leaving the
/// stock/opaque pixels until the next navigation.
fn refresh_cabinets() {
    for hwnd in cabinet_windows() {
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
        if (prop == TMT_FILLCOLOR || prop == TMT_FILLCOLORHINT) && bg_class(htheme) {
            unsafe {
                *pcolor = 0; // transparent black under DWM composition
            }
            return 0;
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

fn punch_black(hdc: Hdc, rect: *const Rect) -> bool {
    unsafe { FillRect(hdc, rect, GetStockObject(BLACK_BRUSH) as HBrush) != 0 }
}

/// Background parts the mod punches out: `iPartId == 0` (ItemsView/
/// TreeView/Header/Rebar/Status backgrounds) and `iPartId == 1,
/// iStateId == 0` (CommandModule/PreviewPane background).
fn is_bg_part(part: i32, state: i32) -> bool {
    part == 0 || (part == 1 && state == 0)
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
    if is_armed()
        && !rect.is_null()
        && is_bg_part(part, state)
        && cabinet_dc(hdc)
        && bg_class(htheme)
    {
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
    if is_armed()
        && !rect.is_null()
        && is_bg_part(part, state)
        && cabinet_dc(hdc)
        && bg_class(htheme)
    {
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
        if pseudo || real {
            return unsafe { FillRect(hdc, rect, GetStockObject(BLACK_BRUSH) as HBrush) };
        }
    }
    let orig: FillRectFn = match tramp("FillRect") {
        Some(f) => f,
        None => return -1,
    };
    unsafe { orig(hdc, rect, brush) }
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
    let specs: [(&'static str, &'static str, *const ()); 8] = [
        (
            "dwmapi.dll",
            "DwmExtendFrameIntoClientArea",
            dwm_extend_hook as *const (),
        ),
        (
            "dwmapi.dll",
            "DwmSetWindowAttribute",
            dwm_setattr_hook as *const (),
        ),
        ("uxtheme.dll", "OpenThemeData", open_theme_hook as *const ()),
        (
            "uxtheme.dll",
            "GetThemeColor",
            get_theme_color_hook as *const (),
        ),
        (
            "uxtheme.dll",
            "DrawThemeBackground",
            draw_theme_bg_hook as *const (),
        ),
        (
            "uxtheme.dll",
            "DrawThemeBackgroundEx",
            draw_theme_bg_ex_hook as *const (),
        ),
        (
            "user32.dll",
            "GetSysColorBrush",
            get_sys_color_brush_hook as *const (),
        ),
        ("user32.dll", "FillRect", fill_rect_hook as *const ()),
    ];
    let mut installed = Vec::new();
    for (dll, name, hook) in specs {
        let dll_w: Vec<u16> = dll.encode_utf16().chain(core::iter::once(0)).collect();
        let module = unsafe { win32::GetModuleHandleW(PCWSTR(dll_w.as_ptr())) };
        if module.is_null() {
            continue;
        }
        let name_z: Vec<u8> = name.bytes().chain(core::iter::once(0)).collect();
        let target = unsafe { win32::GetProcAddress(module, PCSTR(name_z.as_ptr())) };
        if target.is_null() {
            continue;
        }
        let Ok(detour) = (unsafe { retour::RawDetour::new(target as *const (), hook) }) else {
            continue;
        };
        TRAMPS
            .lock()
            .unwrap_or_else(|p| p.into_inner())
            .insert(name, detour.trampoline() as *const () as usize);
        if unsafe { detour.enable() }.is_err() {
            continue;
        }
        installed.push(detour);
    }
    crate::tap_log!("fe_glass: {} detours armed", installed.len());
    *guard = installed;
}
