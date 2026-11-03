//! Dedicated File Explorer glass (ExplorerBlurMica) customization window:
//! a second, single-instance WebView2 host — maximized, standard caption —
//! split into a settings pane and a live preview pane (ui/fe_glass.html).
//!
//! IPC: the page posts JSON (`{"type": "feReady" | "feApply" | "feCancel"}`)
//! forwarded as `Command::SettingsMessage`; the host pushes `feState` /
//! `feToast` with `post_json`.

use crate::commands::{Command, CommandQueue};
use crate::settings_host::{WebEnvironment, glass_message};
use crate::webview_comp::CompWebview;
use pecofence_platform::window::{
    self, ClassOptions, MessageHandler, Window, WindowBuilder, WindowClass, style,
};
use pecofence_platform::{HWND, RECT, dwm, frameless, monitors, msg};
use pecofence_render::ThemeMode;
use std::cell::{Cell, RefCell};
use std::rc::Rc;
use std::time::Instant;
use windows_core::Result;

pub const FE_GLASS_CLASS: &str = "PecoFence.FeGlass";
const FE_GLASS_HTML: &str = include_str!("../../../ui/fe_glass.html");
const I18N_JS: &str = include_str!("../../../ui/i18n.js");

struct HostState {
    comp: Option<CompWebview>,
    mouse_tracked: Cell<bool>,
    /// Last caption press (time + screen point) for manual dbl-click detect.
    last_cap: Cell<Option<(Instant, i32, i32)>>,
    /// The app's theme mode — needed to re-blend the sheet on glass/theme changes.
    mode: ThemeMode,
    /// The mirrored EBM effect (`""` = none) + resolved tint for this window.
    glass_effect: String,
    glass_tint: Option<(u8, u8, u8, u8)>,
}

pub struct FeGlassHost {
    window: Window,
    state: Rc<RefCell<HostState>>,
    /// Caption icons (small/big); destroyed with the host.
    icons: Vec<pecofence_platform::tray::OwnedIcon>,
}

impl FeGlassHost {
    pub fn register_class() -> Result<WindowClass> {
        WindowClass::register(FE_GLASS_CLASS, ClassOptions::default())
    }

    /// Creates the window (hidden until the page reports `feReady`, when it is
    /// shown maximized) and loads the customization page. Page messages are
    /// forwarded to the command queue as `Command::SettingsMessage`.
    pub fn open(
        class: &WindowClass,
        env: &WebEnvironment,
        mode: ThemeMode,
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
        let place = primary.map(|m| m.work_area);

        let handler: MessageHandler = {
            let state = state.clone();
            let queue = queue.clone();
            Box::new(
                move |hwnd: HWND, message: u32, wparam: usize, lparam: isize| -> Option<isize> {
                    match message {
                        // Frameless: client covers the whole window; hit
                        // zones supply drag, resize, and caption buttons.
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
                        // Black on a glass-extended surface = see-through:
                        // the transparent WebView reveals real windows
                        // behind this one.
                        // Never paint the client surface: under the
                        // composition-hosted transparent WebView an
                        // unpainted redirection surface composites as
                        // transparent — any fill makes it opaque.
                        msg::WM_ERASEBKGND => Some(1),
                        msg::WM_DPICHANGED => {
                            // SAFETY: lParam points to the suggested RECT for
                            // the duration of the message.
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
                        msg::WM_DESTROY => {
                            let mut s = state.borrow_mut();
                            if let Some(c) = s.comp.take() {
                                c.close();
                            }
                            queue.push(Command::WindowGone(hwnd));
                            Some(0)
                        }
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

        // Opens maximized; the work-area bounds are what the window restores to.
        let r = place.unwrap_or(RECT {
            left: 100,
            top: 100,
            right: (960.0 * scale) as i32 + 100,
            bottom: (660.0 * scale) as i32 + 100,
        });
        let window = WindowBuilder::new(class)
            .title(pecofence_core::i18n::text("资源管理器背景效果"))
            .style(style::OVERLAPPEDWINDOW | style::CLIPCHILDREN)
            .bounds(r.left, r.top, r.right - r.left, r.bottom - r.top)
            .create(handler)?;
        // The create-time NCCALCSIZE ran before our handler installed —
        // re-run it so the frameless client actually covers the window.
        let _ = window.recalculate_frame();
        let hwnd = window.hwnd();
        let _ = dwm::set_immersive_dark_mode(hwnd, matches!(mode, ThemeMode::Dark));
        apply_caption(hwnd, mode, false);

        let payload = pecofence_core::i18n::ui_payload()
            .to_string()
            .replace('<', "\\u003c");
        let document = FE_GLASS_HTML
            .replace(
                "<!-- PECOFENCE_LOCALE -->",
                &format!("<script>window.PECOFENCE_LOCALE={payload};</script>"),
            )
            .replace(
                "<script src=\"i18n.js\"></script>",
                &format!("<script>{I18N_JS}</script>"),
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
        tracing::debug!("fe_glass: composition controller ready");

        state.borrow_mut().comp = Some(comp);
        Ok(Self {
            window,
            state,
            icons: Vec::new(),
        })
    }

    pub fn hwnd(&self) -> HWND {
        self.window.hwnd()
    }

    pub fn update_language(&self) {
        window::set_title(
            self.window.hwnd(),
            pecofence_core::i18n::text("资源管理器背景效果"),
        );
    }

    /// Sets the window's caption icons (`WM_SETICON` small + big) from premultiplied BGRA.
    pub fn set_icons(&mut self, small: (i32, Vec<u8>), big: (i32, Vec<u8>)) {
        const WM_SETICON: u32 = 0x0080;
        let mut icons = Vec::new();
        for (which, (px, bgra)) in [(0usize, small), (1usize, big)] {
            if let Ok(icon) = pecofence_platform::tray::OwnedIcon::from_bgra(px, &bgra) {
                window::send_message(self.window.hwnd(), WM_SETICON, which, icon.raw());
                icons.push(icon);
            }
        }
        self.icons = icons;
    }

    /// Re-themes the caption and the page sheet after a light/dark switch.
    pub fn set_theme(&self, mode: ThemeMode) {
        let (effect, tint) = {
            let mut s = self.state.borrow_mut();
            s.mode = mode;
            (s.glass_effect.clone(), s.glass_tint)
        };
        let _ = dwm::set_immersive_dark_mode(self.window.hwnd(), matches!(mode, ThemeMode::Dark));
        apply_caption(self.window.hwnd(), mode, !effect.is_empty());
        window::invalidate(self.window.hwnd());
        self.post_json(&glass_message(mode, &effect, tint));
    }

    /// Mirrors an EBM `effect` + resolved tint onto this window — the frame
    /// gets the real DWM material and the page's `--sheet` goes translucent
    /// so the material shows through. `effect=""` resets to opaque.
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
        apply_caption(self.window.hwnd(), mode, !effect.is_empty());
        self.post_json(&glass_message(mode, effect, tint));
    }

    pub fn post_json(&self, json: &str) {
        if let Some(c) = self.state.borrow().comp.as_ref() {
            c.post_json(json);
        }
    }

    /// Closes the browser and destroys the window.
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
