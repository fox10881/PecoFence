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
windows_core::link!("user32.dll" "system" fn FillRect(hdc: Hdc, lprc: *const Rect, hbr: HBrush) -> i32);
windows_core::link!("gdi32.dll" "system" fn GetStockObject(i: i32) -> *mut core::ffi::c_void);
windows_core::link!("kernel32.dll" "system" fn GetCurrentThreadId() -> u32);

const GA_ROOT: u32 = 2;
const BLACK_BRUSH: i32 = 4;
const COLOR_WINDOW: i32 = 5;
const TMT_FILLCOLOR: i32 = 3802;
const TMT_FILLCOLORHINT: i32 = 3821;
const DWMWA_USE_HOSTBACKDROPBRUSH: u32 = 17;
const DWMWA_SYSTEMBACKDROP_TYPE: u32 = 38;

/// Theme classes whose background parts get punched to transparent black —
/// the mod's exact target list for the DirectUI/Win32 surfaces.
const BG_CLASSES: &[&str] = &[
    "ItemsView", "ListView", "TreeView", "Header", "CommandModule",
    "PreviewPane", "Toolbar", "Rebar", "Status", "AddressBand", "SearchBox",
];

static ARMED: AtomicBool = AtomicBool::new(false);
static DETOURS: Mutex<Vec<retour::RawDetour>> = Mutex::new(Vec::new());
static TRAMPS: LazyLock<Mutex<HashMap<&'static str, usize>>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));
/// `HTHEME` → class list (semicolon-separated, as passed to
/// `OpenThemeData`). Populated by the open-theme detour.
static THEME_CLASSES: LazyLock<Mutex<HashMap<usize, String>>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));

pub fn set_armed(on: bool) {
    ARMED.store(on, Ordering::Release);
    if on {
        install();
    }
}

pub fn is_armed() -> bool {
    ARMED.load(Ordering::Acquire)
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

fn cabinet_dc(hdc: Hdc) -> bool {
    if hdc.is_null() {
        return false;
    }
    let w = unsafe { WindowFromDC(hdc) };
    if w.is_null() {
        return false;
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
    THEME_CLASSES
        .lock()
        .unwrap_or_else(|p| p.into_inner())
        .get(&(htheme as usize))
        .cloned()
}

fn bg_class(htheme: HTheme) -> bool {
    class_of(htheme)
        .map(|c| BG_CLASSES.iter().any(|t| c.split(';').any(|p| p.trim().eq_ignore_ascii_case(t))))
        .unwrap_or(false)
}

// -------------------------------------------------------------- hooks ----
type DwmExtendFn = unsafe extern "system" fn(Hwnd, *const Margins) -> HResult;
unsafe extern "system" fn dwm_extend_hook(hwnd: Hwnd, margins: *const Margins) -> HResult {
    let orig: DwmExtendFn = match tramp("DwmExtendFrameIntoClientArea") {
        Some(f) => f,
        None => return -1,
    };
    if is_armed() && is_cabinet(hwnd) {
        let m = Margins { left: -1, right: -1, top: -1, bottom: -1 };
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
    // swallow those writes on cabinets while our glass is armed. Ours
    // arrive through pecofence.exe's own dwmapi, not this process's.
    if is_armed() && is_cabinet(hwnd) && (attr == DWMWA_SYSTEMBACKDROP_TYPE || attr == DWMWA_USE_HOSTBACKDROPBRUSH) {
        return 0;
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
    if is_armed()
        && !pcolor.is_null()
        && (prop == TMT_FILLCOLOR || prop == TMT_FILLCOLORHINT)
        && bg_class(htheme)
        && thread_has_cabinet()
    {
        unsafe {
            *pcolor = 0; // transparent black under DWM composition
        }
        return 0;
    }
    let orig: GetThemeColorFn = match tramp("GetThemeColor") {
        Some(f) => f,
        None => return -1,
    };
    unsafe { orig(htheme, part, state, prop, pcolor) }
}

fn punch_black(hdc: Hdc, rect: *const Rect) -> bool {
    unsafe {
        FillRect(hdc, rect, GetStockObject(BLACK_BRUSH) as HBrush) != 0
    }
}

/// Background parts the mod punches out: `iPartId == 0` (ItemsView/
/// TreeView/Header/Rebar/Status backgrounds) and `iPartId == 1,
/// iStateId == 0` (CommandModule/PreviewPane background).
fn is_bg_part(part: i32, state: i32) -> bool {
    part == 0 || (part == 1 && state == 0)
}

type DrawThemeBgFn = unsafe extern "system" fn(HTheme, Hdc, i32, i32, *const Rect, *const Rect) -> HResult;
unsafe extern "system" fn draw_theme_bg_hook(
    htheme: HTheme,
    hdc: Hdc,
    part: i32,
    state: i32,
    rect: *const Rect,
    clip: *const Rect,
) -> HResult {
    if is_armed() && !rect.is_null() && is_bg_part(part, state) && cabinet_dc(hdc) && bg_class(htheme) {
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

type DrawThemeBgExFn = unsafe extern "system" fn(HTheme, Hdc, i32, i32, *const Rect, *const DtbgOpts) -> HResult;
unsafe extern "system" fn draw_theme_bg_ex_hook(
    htheme: HTheme,
    hdc: Hdc,
    part: i32,
    state: i32,
    rect: *const Rect,
    opts: *const DtbgOpts,
) -> HResult {
    if is_armed() && !rect.is_null() && is_bg_part(part, state) && cabinet_dc(hdc) && bg_class(htheme) {
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
        // COLOR_WINDOW fills inside cabinets paint the opaque panel.
        if let Some(orig) = tramp::<GetSysColorBrushFn>("GetSysColorBrush")
            && unsafe { orig(COLOR_WINDOW) } == brush
        {
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
    let specs: [(&'static str, &'static str, *const ()); 8] = [
        ("dwmapi.dll", "DwmExtendFrameIntoClientArea", dwm_extend_hook as *const ()),
        ("dwmapi.dll", "DwmSetWindowAttribute", dwm_setattr_hook as *const ()),
        ("uxtheme.dll", "OpenThemeData", open_theme_hook as *const ()),
        ("uxtheme.dll", "GetThemeColor", get_theme_color_hook as *const ()),
        ("uxtheme.dll", "DrawThemeBackground", draw_theme_bg_hook as *const ()),
        ("uxtheme.dll", "DrawThemeBackgroundEx", draw_theme_bg_ex_hook as *const ()),
        ("user32.dll", "GetSysColorBrush", get_sys_color_brush_hook as *const ()),
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
        let target = unsafe {
            win32::GetProcAddress(module, PCSTR(name_z.as_ptr()))
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
            .insert(name, detour.trampoline() as *const () as usize);
        if unsafe { detour.enable() }.is_err() {
            continue;
        }
        installed.push(detour);
    }
    crate::tap_log!("fe_glass: {} detours armed", installed.len());
    *guard = installed;
}
