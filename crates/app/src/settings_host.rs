//! On-demand WebView2 settings window (plan §9): created when opened, destroyed when closed so
//! the browser processes go away and nothing stays resident.
//!
//! IPC: the page posts JSON (`{"type": "ready" | "patchSettings" | "setRules" | "action"}`);
//! the host pushes `{"type": "state", ...}` / `{"type": "toast", ...}` with `post_json`.

use crate::commands::{Command, CommandQueue};
use crate::webview_comp::{CompEnvironment, CompWebview};
use coredesk_platform::window::{
    self, ClassOptions, MessageHandler, Window, WindowBuilder, WindowClass, style,
};
use coredesk_platform::{HWND, RECT, dwm, frameless, monitors, msg};
use coredesk_render::ThemeMode;
use std::cell::{Cell, RefCell};
use std::path::PathBuf;
use std::rc::Rc;
use std::time::Instant;
use windows_core::Result;

pub const SETTINGS_CLASS: &str = "CoreDeskEngine.Settings";
const SETTINGS_HTML: &str = include_str!("../../../ui/settings.html");
const I18N_JS: &str = include_str!("../../../ui/i18n.js");

fn profile_name(instance: Option<&str>) -> String {
    use std::hash::{Hash, Hasher};
    let Some(instance) = instance.map(str::trim).filter(|name| !name.is_empty()) else {
        return "default".into();
    };
    // Instance names are not filesystem paths. Hash them to avoid separators,
    // invalid Windows filename characters and collisions from lossy sanitizing.
    let mut hash = std::collections::hash_map::DefaultHasher::new();
    instance.hash(&mut hash);
    format!("instance-{:016x}", hash.finish())
}

/// The WebView2 environment. Kept alive across opens (warm start ≈ 200 ms); the browser
/// processes exit by themselves when no controller is alive.
pub struct WebEnvironment {
    env: CompEnvironment,
}

impl WebEnvironment {
    pub fn create() -> Result<Self> {
        let user_data = std::env::var_os("LOCALAPPDATA")
            .map(PathBuf::from)
            .unwrap_or_else(std::env::temp_dir)
            .join(coredesk_core::brand::ID)
            // Keep independent instances from sharing incompatible controller options.
            .join("WebView2Profiles")
            .join(profile_name(
                coredesk_core::brand::var("COREDESK_INSTANCE")
                    .ok()
                    .as_deref(),
            ));
        // Debug builds: `COREDESK_WEBVIEW_ARGS=--remote-debugging-port=9352` lets a test drive
        // the page (the options always pass browser arguments, so WebView2's own
        // WEBVIEW2_ADDITIONAL_BROWSER_ARGUMENTS is not honoured).
        let browser_args = if cfg!(debug_assertions) {
            coredesk_core::brand::var("COREDESK_WEBVIEW_ARGS").unwrap_or_default()
        } else {
            String::new()
        };
        tracing::debug!("settings: creating WebView2 environment");
        let env = CompEnvironment::create(&user_data, &browser_args)?;
        tracing::debug!("settings: WebView2 environment ready");
        Ok(Self { env })
    }

    /// Shared with secondary hosts (the FE-glass editor) so one environment
    /// serves every WebView2 window the app opens.
    pub(crate) fn inner(&self) -> &CompEnvironment {
        &self.env
    }
}

struct HostState {
    comp: Option<CompWebview>,
    /// `WM_MOUSELEAVE` tracking armed — reset when the leave message fires.
    mouse_tracked: Cell<bool>,
    /// Last caption press (time + screen point) for manual dbl-click detect.
    last_cap: Cell<Option<(Instant, i32, i32)>>,
    /// The app's theme mode — needed to re-blend the sheet on glass/theme changes.
    mode: ThemeMode,
    /// The mirrored EBM effect (`""` = none) + resolved tint for this window.
    glass_effect: String,
    glass_tint: Option<(u8, u8, u8, u8)>,
}

pub struct SettingsHost {
    window: Window,
    state: Rc<RefCell<HostState>>,
    /// The page reported `ready`: messages posted from now on reach its script.
    pub ready: bool,
    /// Caption icons (small/big); destroyed with the host.
    icons: Vec<coredesk_platform::tray::OwnedIcon>,
}

impl SettingsHost {
    pub fn register_class() -> Result<WindowClass> {
        WindowClass::register(SETTINGS_CLASS, ClassOptions::default())
    }

    /// Creates the window and the WebView2 controller and loads the settings page. Page
    /// messages are forwarded to the command queue as `Command::SettingsMessage`.
    pub fn open(
        class: &WindowClass,
        env: &WebEnvironment,
        mode: ThemeMode,
        _liquid_glass: bool,
        queue: CommandQueue,
    ) -> Result<Self> {
        let state = Rc::new(RefCell::new(HostState {
            comp: None,
            mouse_tracked: Cell::new(false),
            last_cap: Cell::new(None),
            mode,
            glass_effect: String::new(),
            glass_tint: None,
        }));

        let mons = monitors::enumerate();
        let primary = mons.iter().find(|m| m.primary).or(mons.first());
        let scale = primary.map(|m| m.scale()).unwrap_or(1.0);
        let place = primary.map(|m| placement(m.work_area, scale));
        let (min_w, min_h) = place
            .map(|p| (p.min_w, p.min_h))
            .unwrap_or(((720.0 * scale) as i32, (480.0 * scale) as i32));

        let handler: MessageHandler = {
            let state = state.clone();
            let queue = queue.clone();
            Box::new(
                move |hwnd: HWND, message: u32, wparam: usize, lparam: isize| -> Option<isize> {
                    match message {
                        // Frameless: the client covers the whole window
                        // (native caption hidden); hit zones supply drag,
                        // resize, and the page-drawn caption buttons.
                        msg::WM_NCCALCSIZE => Some(frameless::calc_size(hwnd, wparam, lparam)),
                        msg::WM_NCHITTEST => Some(frameless::hit_test(hwnd, lparam)),
                        // DefWindowProc's NC button tracker needs a real
                        // frame to compute the hit rects — with none it
                        // swallows Min/Max/Close. Fire them on the DOWN;
                        // caption dbl-click toggles zoom ourselves too.
                        msg::WM_NCLBUTTONDOWN | msg::WM_NCLBUTTONDBLCLK => frameless::nc_button(
                            hwnd,
                            message,
                            wparam,
                            lparam,
                            &state.borrow().last_cap,
                        ),
                        msg::WM_SIZE => {
                            let w = msg::lo_i16(lparam);
                            let h = msg::hi_i16(lparam);
                            if let Some(c) = state.borrow().comp.as_ref() {
                                c.set_bounds(w, h);
                                // Same as WM_MOVE: a resize-only change still
                                // re-anchors the WebView2 input twin — skips
                                // it and the twin lingers as a black rect.
                                c.notify_parent_window_position_changed();
                                c.post_json(&format!(
                                    "{{\"t\":\"wstate\",\"max\":{}}}",
                                    frameless::is_zoomed(hwnd)
                                ));
                            }
                            Some(0)
                        }
                        msg::WM_MOVE => {
                            if let Some(c) = state.borrow().comp.as_ref() {
                                c.notify_parent_window_position_changed();
                            }
                            Some(0)
                        }
                        // Black on a glass-extended surface is the
                        // transparency convention: the client shows real
                        // windows behind it. (Measured live — painting
                        // anything else leaves an opaque fill visible
                        // through the transparent WebView.)
                        // Never paint the client surface: under the
                        // composition-hosted transparent WebView an
                        // unpainted redirection surface composites as
                        // transparent — any fill makes it opaque.
                        msg::WM_ERASEBKGND => Some(1),
                        msg::WM_GETMINMAXINFO => {
                            // For the monitor the window is on now, which may have another
                            // scale than the one it opened on.
                            let (min_w, min_h) =
                                monitors::query(monitors::monitor_from_window(hwnd))
                                    .map(|m| placement(m.work_area, m.scale()))
                                    .map_or((min_w, min_h), |p| (p.min_w, p.min_h));
                            // SAFETY: lParam is the MINMAXINFO for this message.
                            unsafe { window::minmaxinfo_set_min_track(lparam, min_w, min_h) };
                            Some(0)
                        }
                        msg::WM_DPICHANGED => {
                            // A per-monitor aware window resizes itself: take the size Windows
                            // suggests for the new scale.
                            // SAFETY: lParam points to the suggested RECT for the duration of
                            // the message.
                            let r = unsafe { *(lparam as *const RECT) };
                            let _ = window::set_window_bounds(
                                hwnd,
                                r.left,
                                r.top,
                                r.right - r.left,
                                r.bottom - r.top,
                            );
                            if let Some(c) = state.borrow().comp.as_ref() {
                                c.set_rasterization_scale(window::dpi_of(hwnd));
                            }
                            Some(0)
                        }
                        msg::WM_SETFOCUS => {
                            // Keyboard focus belongs to the page, not the host
                            // window — a composition controller has no child
                            // hwnd, so without MoveFocus it never sees a key.
                            if let Some(c) = state.borrow().comp.as_ref() {
                                c.move_focus();
                            }
                            Some(0)
                        }
                        msg::WM_SETCURSOR => {
                            if (lparam & 0xffff) as i64 != msg::HTCLIENT as i64 {
                                return None;
                            }
                            let s = state.borrow();
                            if let Some(c) = s.comp.as_ref() {
                                let cursor = c.cursor();
                                window::set_cursor(if cursor.is_null() {
                                    let id = c.system_cursor_id();
                                    window::load_system_cursor(if id == 0 { 32512 } else { id })
                                } else {
                                    cursor
                                });
                            }
                            Some(1)
                        }
                        msg::WM_ACTIVATE if wparam & 0xffff != 0 => {
                            queue.push(Command::SettingsActivated);
                            None
                        }
                        msg::WM_DESTROY => {
                            let mut s = state.borrow_mut();
                            if let Some(c) = s.comp.take() {
                                c.close();
                            }
                            queue.push(Command::WindowGone(hwnd));
                            Some(0)
                        }
                        // Composition hosting has no child HWND to receive the
                        // mouse — forward the messages the WebView cares about.
                        m if m == msg::WM_MOUSEMOVE
                            || (0x0201..=0x020E).contains(&m)
                            || m == msg::WM_MOUSELEAVE =>
                        {
                            let s = state.borrow();
                            match s.comp.as_ref() {
                                Some(c) => {
                                    c.forward_wm(hwnd, m, wparam, lparam, &s.mouse_tracked);
                                    Some(0)
                                }
                                None => None,
                            }
                        }
                        _ => None,
                    }
                },
            )
        };

        let (x, y, w, h) = place.map(|p| (p.x, p.y, p.w, p.h)).unwrap_or((
            100,
            100,
            (960.0 * scale) as i32,
            (660.0 * scale) as i32,
        ));

        let window = WindowBuilder::new(class)
            .title(coredesk_core::i18n::text("CoreDesk Engine | 设置"))
            .style(style::OVERLAPPEDWINDOW | style::CLIPCHILDREN)
            .bounds(x, y, w, h)
            .create(handler)?;
        // The create-time NCCALCSIZE ran before our handler installed —
        // re-run it so the frameless client actually covers the window.
        let _ = window.recalculate_frame();
        let hwnd = window.hwnd();
        tracing::debug!(?hwnd, "settings: host window created");
        let _ = dwm::set_immersive_dark_mode(hwnd, matches!(mode, ThemeMode::Dark));
        apply_caption(hwnd, mode, false);

        // `no-mica` keeps the body painting `var(--sheet)` — with the
        // composition host that sheet can be genuinely translucent, and the
        // DWM material on this HWND shows through it.
        let payload = coredesk_core::i18n::ui_payload()
            .to_string()
            .replace('<', "\\u003c");
        let document = SETTINGS_HTML
            .replace(
                "<!-- COREDESK_LOCALE -->",
                &format!("<script>window.COREDESK_LOCALE={payload};</script>"),
            )
            .replace(
                "<script src=\"i18n.js\"></script>",
                &format!("<script>{I18N_JS}</script>"),
            )
            .replacen(
                "<html lang=\"zh-CN\">",
                "<html lang=\"zh-CN\" class=\"no-mica\">",
                1,
            );

        let (cw, ch) = window.client_size();
        let comp = {
            let queue = queue.clone();
            CompWebview::create(
                env.inner(),
                hwnd,
                window::dpi_of(hwnd),
                (cw, ch),
                &document,
                move |json| queue.push(Command::SettingsMessage(json)),
            )?
        };
        tracing::debug!("settings: composition controller ready");

        state.borrow_mut().comp = Some(comp);
        // The host itself is shown once the page reports `ready`
        // (`App::on_settings_message`): shown at once, it was white until
        // WebView2 painted, even in dark mode.
        Ok(Self {
            window,
            state,
            ready: false,
            icons: Vec::new(),
        })
    }

    pub fn hwnd(&self) -> HWND {
        self.window.hwnd()
    }

    pub fn update_language(&self) {
        window::set_title(
            self.window.hwnd(),
            coredesk_core::i18n::text("CoreDesk Engine | 设置"),
        );
    }

    /// Sets the window's caption icons (`WM_SETICON` small + big) from premultiplied BGRA.
    pub fn set_icons(&mut self, small: (i32, Vec<u8>), big: (i32, Vec<u8>)) {
        const WM_SETICON: u32 = 0x0080;
        let mut icons = Vec::new();
        for (which, (px, bgra)) in [(0usize, small), (1usize, big)] {
            if let Ok(icon) = coredesk_platform::tray::OwnedIcon::from_bgra(px, &bgra) {
                window::send_message(self.window.hwnd(), WM_SETICON, which, icon.raw());
                icons.push(icon);
            }
        }
        self.icons = icons;
    }

    /// Re-themes the caption and the page sheet after a light/dark switch.
    pub fn set_theme(&self, mode: ThemeMode, _liquid_glass: bool) {
        let mut s = self.state.borrow_mut();
        s.mode = mode;
        let armed = !s.glass_effect.is_empty();
        let effect = s.glass_effect.clone();
        let tint = s.glass_tint;
        drop(s);
        let _ = dwm::set_immersive_dark_mode(self.window.hwnd(), matches!(mode, ThemeMode::Dark));
        apply_caption(self.window.hwnd(), mode, armed);
        window::invalidate(self.window.hwnd());
        self.post_json(&glass_message(mode, &effect, tint));
    }

    /// Mirrors an EBM `effect` + resolved tint onto this window: the frame
    /// gets the real DWM material (DWMSBT/accent) and the page's `--sheet`
    /// becomes translucent so the material shows through the client area.
    /// `effect=""` restores the stock opaque look.
    pub fn set_glass(&self, effect: &str, tint: Option<(u8, u8, u8, u8)>) {
        {
            let mut s = self.state.borrow_mut();
            s.glass_effect = effect.to_string();
            s.glass_tint = tint;
        }
        let argb = tint.map(|(a, r, g, b)| {
            (u32::from(a) << 24) | (u32::from(r) << 16) | (u32::from(g) << 8) | u32::from(b)
        });
        dwm::sync_app_glass(self.window.hwnd(), effect, argb);
        window::invalidate(self.window.hwnd());
        let mode = self.state.borrow().mode;
        let armed = !effect.is_empty();
        apply_caption(self.window.hwnd(), mode, armed);
        self.post_json(&glass_message(mode, effect, tint));
    }

    pub fn post_json(&self, json: &str) {
        if let Some(c) = self.state.borrow().comp.as_ref() {
            c.post_json(json);
        }
    }

    /// Closes the browser and destroys the window.
    #[allow(dead_code)]
    pub fn close(self) {
        {
            let mut s = self.state.borrow_mut();
            if let Some(c) = s.comp.take() {
                c.close();
            }
        }
        drop(self.window);
    }
}

/// The `--sheet` value for a state — now an alpha-carrying colour so the
/// window's real DWM material shows through the page:
/// - `effect=""` → the opaque theme base (no material behind the client).
/// - accent effects (acrylic/blur/blurclear) → a neutral translucent veil;
///   the accent policy already carries the user's tint on the frame, so
///   painting it again on the client would double it.
/// - mica/micaalt → the user's tint at its own alpha, since the DWM system
///   backdrop can't take a colour and the frame stays untinted mica.
fn sheet_css(mode: ThemeMode, effect: &str, tint: Option<(u8, u8, u8, u8)>) -> String {
    // Armed = the user's Explorer RGBA as a translucent veil over the
    // truly transparent client — alpha passes through 1:1 from their
    // opacity slider, no clamps. Unarmed = stock opaque base.
    match effect {
        "" => match mode {
            ThemeMode::Dark => "#0d1524".to_string(),
            ThemeMode::Light => "#f3f3f3".to_string(),
        },
        _ => tint.map_or("transparent".to_string(), |(a, r, g, b)| {
            // Our windows run lighter than Explorer's own tint: the user's
            // alpha minus 70, floored at 1 — 0 is never emitted.
            let a = a.saturating_sub(70).max(1);
            format!("rgba({r},{g},{b},{:.3})", f32::from(a) / 255.0)
        }),
    }
}

pub(crate) fn glass_message(
    mode: ThemeMode,
    effect: &str,
    tint: Option<(u8, u8, u8, u8)>,
) -> String {
    let sheet = sheet_css(mode, effect, tint);
    format!(
        "{{\"type\":\"glass\",\"armed\":{},\"sheet\":\"{sheet}\"}}",
        !effect.is_empty()
    )
}

/// Match the page sheet so the native caption belongs to the same surface —
/// skipped while a real material covers the frame (it owns the caption).
fn apply_caption(hwnd: HWND, mode: ThemeMode, glass_armed: bool) {
    if glass_armed {
        return;
    }
    let (r, g, b) = match mode {
        ThemeMode::Dark => (0x20u32, 0x20u32, 0x20u32),
        ThemeMode::Light => (0xF3u32, 0xF3u32, 0xF3u32),
    };
    // DWM takes COLORREF (0x00BBGGRR).
    let _ = dwm::set_caption_color(hwnd, r | (g << 8) | (b << 16));
}

/// Where the settings window opens on a work area (physical px) at `scale`, and its minimum
/// tracking size.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Placement {
    x: i32,
    y: i32,
    w: i32,
    h: i32,
    min_w: i32,
    min_h: i32,
}

/// The designed 960 x 660 DIP window (minimum 720 x 480), centred on the work area. Both
/// shrink to fit it: 1366 x 768 at 125 % would otherwise open 825 px tall on a 708 px work
/// area, caption above the screen where it cannot be dragged back.
fn placement(work: RECT, scale: f32) -> Placement {
    let (area_w, area_h) = (work.right - work.left, work.bottom - work.top);
    let w = ((960.0 * scale) as i32).min(area_w);
    let h = ((660.0 * scale) as i32).min(area_h);
    Placement {
        x: work.left + (area_w - w) / 2,
        y: work.top + (area_h - h) / 2,
        w,
        h,
        min_w: ((720.0 * scale) as i32).min(w),
        min_h: ((480.0 * scale) as i32).min(h),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn work(w: i32, h: i32) -> RECT {
        RECT {
            left: 0,
            top: 0,
            right: w,
            bottom: h,
        }
    }

    /// 1366 x 768 at 125 % (a 48 DIP taskbar = 60 px): the window must fit, caption on screen.
    #[test]
    fn settings_window_fits_small_high_dpi_screens() {
        for (area, scale) in [
            (work(1366, 708), 1.25),
            (work(1920, 996), 1.75),
            (work(1280, 672), 1.5),
            (work(2560, 1392), 1.0),
        ] {
            let p = placement(area, scale);
            assert!(
                p.y >= area.top && p.x >= area.left,
                "{p:?} off-screen on {area:?}"
            );
            assert!(
                p.y + p.h <= area.bottom && p.x + p.w <= area.right,
                "{p:?} too big"
            );
            assert!(
                p.min_w <= p.w && p.min_h <= p.h,
                "{p:?} minimum above the window"
            );
        }
        // Room to spare: the designed size, centred.
        let p = placement(work(2560, 1392), 1.0);
        assert_eq!((p.w, p.h), (960, 660));
        assert_eq!((p.x, p.y), (800, 366));
    }
}
