//! Desktop Window Manager attributes.

use crate::bindings::*;
use windows_core::Result;

windows_core::link!("user32.dll" "system" fn SetWindowCompositionAttribute(hwnd: HWND, data: *const core::ffi::c_void) -> i32);

/// `ACCENT_POLICY` for `SetWindowCompositionAttribute` — the legacy DWM
/// accent path (undocumented but stable since Windows 8). `gradient_color`
/// is `AABBGGRR`: the high byte is alpha and red/blue are swapped vs the
/// `AARRGGBB` strings the settings model stores — see `argb_to_abgr`.
#[repr(C)]
struct AccentPolicy {
    accent_state: u32,
    accent_flags: u32,
    gradient_color: u32,
    animation_id: u32,
}

/// `WINCOMPATTRDATA` — `attrib = WCA_ACCENT_POLICY (19)` carries an
/// `AccentPolicy` by pointer.
#[repr(C)]
struct WindowCompositionAttributeData {
    attrib: u32,
    data: *const core::ffi::c_void,
    size: usize,
}

const WCA_ACCENT_POLICY: u32 = 19;
const ACCENT_DISABLED: u32 = 0;
/// Kept for reference — unusable: its gradient alpha is ignored on
/// Windows 11, so it floods the window with a solid colour.
#[allow(dead_code)]
const ACCENT_ENABLE_TRANSPARENTGRADIENT: u32 = 2;
const ACCENT_ENABLE_ACRYLICBLURBEHIND: u32 = 4;
/// `accent_flags` bit for the standard acrylic tint path.
const ACCENT_FLAG_ACRYLIC: u32 = 0x2;

/// `#AARRGGBB` → `AABBGGRR` (`AccentPolicy::gradient_color` order).
fn argb_to_abgr(argb: u32) -> u32 {
    (argb & 0xff00_0000)
        | ((argb & 0x00ff_0000) >> 16)
        | (argb & 0x0000_ff00)
        | ((argb & 0x0000_00ff) << 16)
}

/// Applies the legacy accent state to `hwnd`. `accent_state` is one of the
/// `ACCENT_*` values; `gradient` is already in `AABBGGRR` order
/// (`argb_to_abgr` converts). Returns `false` when the call rejects —
/// non-fatal: the window just keeps its previous backdrop.
fn set_accent_policy(hwnd: HWND, accent_state: u32, accent_flags: u32, gradient: u32) -> bool {
    let policy = AccentPolicy {
        accent_state,
        accent_flags,
        gradient_color: gradient,
        animation_id: 0,
    };
    let data = WindowCompositionAttributeData {
        attrib: WCA_ACCENT_POLICY,
        data: (&policy as *const AccentPolicy).cast(),
        size: size_of::<AccentPolicy>(),
    };
    // SAFETY: `policy`/`data` outlive the call.
    unsafe {
        SetWindowCompositionAttribute(
            hwnd,
            &data as *const WindowCompositionAttributeData as *const core::ffi::c_void,
        ) != 0
    }
}

/// Clears BOTH backdrop mechanisms — `DWMWA_SYSTEMBACKDROP_TYPE` and the
/// accent policy do not safely coexist, and neither reset clears the other,
/// so every effect transition starts from a clean slate.
fn clear_backdrops(hwnd: HWND) {
    let _ = set_accent_policy(hwnd, ACCENT_DISABLED, 0, 0);
    let _ = set_system_backdrop(hwnd, SystemBackdrop::None);
}

/// The app-UI mirror of an ExplorerBlurMica `effect` id — the host HWND
/// frame (caption + borders) renders the real material; the client is the
/// WebView's responsibility (`page_background` blends the tint in).
///
/// Mapping notes (per design review):
/// - `blur` maps to acrylic: `ACCENT_ENABLE_BLURBEHIND` is dead on
///   Windows 11 22H2+ (flat tint, no blur), while acrylic honours the
///   user's RGBA tint — the closest usable blur.
/// - `acrylic` uses the accent path because `DWMSBT_TRANSIENTWINDOW`
///   (documented acrylic) cannot take a per-window tint colour.
/// - `mica`/`micaalt` have no OS tint API: `DWMWA_SYSTEMBACKDROP_TYPE`
///   samples the desktop wallpaper; the tint shows up via the page sheet.
/// - `blurclear` is the fully-transparent gradient (no tint).
pub fn sync_app_glass(hwnd: HWND, effect: &str, argb: Option<u32>) {
    clear_backdrops(hwnd);
    if !is_composition_enabled() {
        return;
    }
    // NOTE: the client surface must stay untouched — the composition
    // WebView composites it; glass extension makes it an opaque fill.
    // Real material lives on the frame only (DWMSBT/accent below).
    // Gradient alpha bounds: 0x00 is the DWM "default accent colour"
    // sentinel (paints solid blue, not transparent), so floor at 0x01.
    // TRANSPARENTGRADIENT ignores its alpha byte entirely on Win11 and
    // floods solid at any value — every accent effect therefore uses the
    // acrylic path. Acrylic honours alpha as tint strength; the cap at
    // 0x80 keeps the slider from reaching a near-opaque fill.
    let tint = argb
        .map(|a| {
            // Lighter than Explorer's own tint — slider alpha minus 70,
            // bounded [0x01, 0x80]: 0x00 is the DWM "default accent colour"
            // sentinel and 0xFF+ would near-opaque the whole window.
            (a & 0x00ff_ffff) | ((a >> 24).saturating_sub(70).clamp(1, 0x80) << 24)
        })
        .map(argb_to_abgr)
        .unwrap_or(0x0100_0000);
    match effect {
        "mica" => {
            let _ = set_system_backdrop(hwnd, SystemBackdrop::MainWindow);
        }
        "micaalt" => {
            let _ = set_system_backdrop(hwnd, SystemBackdrop::TabbedWindow);
        }
        "acrylic" | "blur" | "blurclear" => {
            let _ = set_accent_policy(
                hwnd,
                ACCENT_ENABLE_ACRYLICBLURBEHIND,
                ACCENT_FLAG_ACRYLIC,
                tint,
            );
        }
        _ => {}
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CornerPreference {
    Default,
    DoNotRound,
    Round,
    RoundSmall,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SystemBackdrop {
    Auto,
    None,
    MainWindow,
    TransientWindow,
    TabbedWindow,
}

fn set_attribute<T>(hwnd: HWND, attribute: i32, value: &T) -> Result<()> {
    // SAFETY: `value` outlives the call and its size is passed explicitly.
    unsafe {
        DwmSetWindowAttribute(
            hwnd,
            attribute as u32,
            (value as *const T).cast(),
            size_of::<T>() as u32,
        )
        .ok()
    }
}

pub fn set_corner_preference(hwnd: HWND, preference: CornerPreference) -> Result<()> {
    let value: i32 = match preference {
        CornerPreference::Default => DWMWCP_DEFAULT,
        CornerPreference::DoNotRound => DWMWCP_DONOTROUND,
        CornerPreference::Round => DWMWCP_ROUND,
        CornerPreference::RoundSmall => DWMWCP_ROUNDSMALL,
    };
    set_attribute(hwnd, DWMWA_WINDOW_CORNER_PREFERENCE, &value)
}

pub fn set_system_backdrop(hwnd: HWND, backdrop: SystemBackdrop) -> Result<()> {
    let value: i32 = match backdrop {
        SystemBackdrop::Auto => DWMSBT_AUTO,
        SystemBackdrop::None => DWMSBT_NONE,
        SystemBackdrop::MainWindow => DWMSBT_MAINWINDOW,
        SystemBackdrop::TransientWindow => DWMSBT_TRANSIENTWINDOW,
        SystemBackdrop::TabbedWindow => DWMSBT_TABBEDWINDOW,
    };
    set_attribute(hwnd, DWMWA_SYSTEMBACKDROP_TYPE, &value)
}

/// Removes the 1px accent/gray border DWM draws around top-level windows.
pub fn set_border_color_none(hwnd: HWND) -> Result<()> {
    let value: u32 = DWMWA_COLOR_NONE;
    set_attribute(hwnd, DWMWA_BORDER_COLOR, &value)
}

/// Custom composition windows draw their own frame and shadow. Leaving the DWM frame
/// enabled exposes its smaller corner arcs outside a larger composition clip.
pub fn set_nonclient_rendering(hwnd: HWND, enabled: bool) -> Result<()> {
    let value = if enabled {
        DWMNCRP_ENABLED
    } else {
        DWMNCRP_DISABLED
    };
    set_attribute(hwnd, DWMWA_NCRENDERING_POLICY, &value)
}

/// Paints the caption in a solid colour (COLORREF, 0x00BBGGRR) so it matches an opaque page.
pub fn set_caption_color(hwnd: HWND, colorref: u32) -> Result<()> {
    set_attribute(hwnd, DWMWA_CAPTION_COLOR, &colorref)
}

pub fn set_immersive_dark_mode(hwnd: HWND, dark: bool) -> Result<()> {
    let value: i32 = dark as i32;
    set_attribute(hwnd, DWMWA_USE_IMMERSIVE_DARK_MODE, &value)
}

/// Disables DWM's minimize/restore transition animations for this window.
pub fn set_transitions_disabled(hwnd: HWND, disabled: bool) -> Result<()> {
    let value: i32 = disabled as i32;
    set_attribute(hwnd, DWMWA_TRANSITIONS_FORCEDISABLED, &value)
}

/// Excludes the window from Aero Peek / "Show desktop" preview.
pub fn set_excluded_from_peek(hwnd: HWND, excluded: bool) -> Result<()> {
    let value: i32 = excluded as i32;
    set_attribute(hwnd, DWMWA_EXCLUDED_FROM_PEEK, &value)
}

/// Extends the DWM frame into the whole client area (sheet-of-glass).
pub fn extend_frame_into_entire_client_area(hwnd: HWND) -> Result<()> {
    let margins = MARGINS {
        cxLeftWidth: -1,
        cxRightWidth: -1,
        cyTopHeight: -1,
        cyBottomHeight: -1,
    };
    // SAFETY: margins outlives the call.
    unsafe { DwmExtendFrameIntoClientArea(hwnd, &margins).ok() }
}

pub fn is_composition_enabled() -> bool {
    // SAFETY: plain FFI call.
    unsafe {
        DwmIsCompositionEnabled()
            .map(|b| b.as_bool())
            .unwrap_or(false)
    }
}
