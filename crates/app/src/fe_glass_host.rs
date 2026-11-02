//! Dedicated File Explorer glass (ExplorerBlurMica) customization window:
//! a second, single-instance WebView2 host — maximized, standard caption —
//! split into a settings pane and a live preview pane (ui/fe_glass.html).
//!
//! IPC: the page posts JSON (`{"type": "feReady" | "feApply" | "feCancel"}`)
//! forwarded as `Command::SettingsMessage`; the host pushes `feState` /
//! `feToast` with `post_json`.

use crate::commands::{Command, CommandQueue};
use crate::settings_host::WebEnvironment;
use pecofence_platform::window::{
    self, ClassOptions, MessageHandler, Window, WindowBuilder, WindowClass, style,
};
use pecofence_platform::{HWND, RECT, dwm, monitors, msg};
use pecofence_render::ThemeMode;
use std::cell::RefCell;
use std::rc::Rc;
use windows_core::Result;
use windows_webview::{Controller, WebView};

pub const FE_GLASS_CLASS: &str = "PecoFence.FeGlass";
const FE_GLASS_HTML: &str = include_str!("../../../ui/fe_glass.html");
const I18N_JS: &str = include_str!("../../../ui/i18n.js");

struct HostState {
    controller: Option<Controller>,
    webview: Option<WebView>,
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
    _registrations: Vec<windows_webview::EventRegistration>,
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
            controller: None,
            webview: None,
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
                move |hwnd: HWND, message: u32, _wparam: usize, lparam: isize| -> Option<isize> {
                    match message {
                        msg::WM_SIZE => {
                            let w = msg::lo_i16(lparam);
                            let h = msg::hi_i16(lparam);
                            if let Some(c) = state.borrow().controller.as_ref() {
                                let _ = c.set_bounds(0, 0, w, h);
                                let _ = c.set_visible(w > 0 && h > 0);
                            }
                            Some(0)
                        }
                        msg::WM_MOVE => {
                            if let Some(c) = state.borrow().controller.as_ref() {
                                let _ = c.notify_parent_window_position_changed();
                            }
                            Some(0)
                        }
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
                            Some(0)
                        }
                        msg::WM_SETFOCUS => {
                            if let Some(c) = state.borrow().controller.as_ref() {
                                let _ =
                                    c.move_focus(windows_webview::MoveFocusReason::Programmatic);
                            }
                            Some(0)
                        }
                        msg::WM_DESTROY => {
                            let mut s = state.borrow_mut();
                            s.webview = None;
                            if let Some(c) = s.controller.take() {
                                let _ = c.close();
                            }
                            queue.push(Command::WindowGone(hwnd));
                            Some(0)
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
        let hwnd = window.hwnd();
        let _ = dwm::set_immersive_dark_mode(hwnd, matches!(mode, ThemeMode::Dark));
        apply_caption(hwnd, mode, false);

        // SAFETY: `hwnd` is a live window owned by this thread and outlives the
        // controller (closed in WM_DESTROY).
        let controller = unsafe {
            env.inner()
                .create_controller_for_hwnd(pecofence_platform::hwnd_ptr(hwnd))?
        };
        let webview = controller.webview()?;
        let (cw, ch) = window.client_size();
        controller.set_bounds(0, 0, cw, ch)?;
        controller.set_default_background_color(page_background(mode, None))?;
        if let Ok(settings) = webview.settings() {
            let _ = settings.set_default_context_menus_enabled(false);
            let _ = settings.set_status_bar_enabled(false);
            let _ = settings.set_zoom_control_enabled(false);
        }

        let mut registrations = Vec::new();
        {
            let queue = queue.clone();
            registrations.push(webview.on_web_message_received(move |args| {
                queue.push(Command::SettingsMessage(args.web_message_as_json()));
            })?);
        }

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
        webview.navigate_to_string(&document)?;
        tracing::debug!("fe_glass: page navigation submitted");

        {
            let mut s = state.borrow_mut();
            s.controller = Some(controller);
            s.webview = Some(webview);
        }
        if let Some(c) = state.borrow().controller.as_ref() {
            let _ = c.set_visible(true);
        }
        Ok(Self {
            window,
            state,
            icons: Vec::new(),
            _registrations: registrations,
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

    /// Re-themes the caption and the page background after a light/dark
    /// switch — re-blending the sheet against the still-armed glass tint.
    pub fn set_theme(&self, mode: ThemeMode) {
        let (armed, tint) = {
            let mut s = self.state.borrow_mut();
            s.mode = mode;
            (!s.glass_effect.is_empty(), s.glass_tint)
        };
        let _ = dwm::set_immersive_dark_mode(self.window.hwnd(), matches!(mode, ThemeMode::Dark));
        apply_caption(self.window.hwnd(), mode, armed);
        if let Some(c) = self.state.borrow().controller.as_ref() {
            let _ = c.set_default_background_color(page_background(mode, tint.filter(|_| armed)));
        }
    }

    /// Mirrors an EBM `effect` + resolved tint onto this window — the frame
    /// gets the real DWM material, the sheet a blended tint, and the page a
    /// `glass` message so its `--sheet` token matches. `effect=""` resets.
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
        let (mode, armed, tint) = {
            let s = self.state.borrow();
            (s.mode, !s.glass_effect.is_empty(), s.glass_tint)
        };
        apply_caption(self.window.hwnd(), mode, armed);
        let tint = tint.filter(|_| armed);
        if let Some(c) = self.state.borrow().controller.as_ref() {
            let _ = c.set_default_background_color(page_background(mode, tint));
        }
        let (r, g, b) = sheet_rgb(mode, tint);
        self.post_json(&format!(
            "{{\"type\":\"glass\",\"armed\":{},\"sheet\":\"#{r:02x}{g:02x}{b:02x}\"}}",
            armed
        ));
    }

    pub fn post_json(&self, json: &str) {
        if let Some(w) = self.state.borrow().webview.as_ref() {
            let _ = w.post_web_message_as_json(json);
        }
    }

    /// Closes the browser and destroys the window.
    pub fn close(self) {
        {
            let mut s = self.state.borrow_mut();
            s.webview = None;
            if let Some(c) = s.controller.take() {
                let _ = c.close();
            }
        }
        drop(self.window);
    }
}

/// The sheet colour: theme base blended toward the user's tint (the HWND
/// controller can't composite alpha to the backdrop, so the material's hue
/// is carried into the client — ~45 % of the tint's alpha keeps contrast).
fn sheet_rgb(mode: ThemeMode, tint: Option<(u8, u8, u8, u8)>) -> (u8, u8, u8) {
    let (br, bg, bb) = match mode {
        ThemeMode::Dark => (0x20, 0x20, 0x20),
        ThemeMode::Light => (0xF3, 0xF3, 0xF3),
    };
    let Some((a, tr, tg, tb)) = tint else {
        return (br, bg, bb);
    };
    let k = f32::from(a) / 255.0 * 0.45;
    let blend =
        |base: u8, t: u8| -> u8 { (f32::from(base) * (1.0 - k) + f32::from(t) * k).round() as u8 };
    (blend(br, tr), blend(bg, tg), blend(bb, tb))
}

/// Page background: the theme base, or the tint-blended sheet while glass
/// is armed. Always opaque — the HWND controller renders alpha as opaque.
fn page_background(mode: ThemeMode, tint: Option<(u8, u8, u8, u8)>) -> windows_webview::Color {
    let (r, g, b) = sheet_rgb(mode, tint);
    windows_webview::Color { a: 255, r, g, b }
}

/// Match the page sheet so the native caption belongs to the same surface —
/// skipped while a real material covers the frame (it owns the caption).
fn apply_caption(hwnd: HWND, mode: ThemeMode, glass_armed: bool) {
    if glass_armed {
        return;
    }
    let bg = page_background(mode, None);
    // DWM takes COLORREF (0x00BBGGRR).
    let c = u32::from(bg.r) | (u32::from(bg.g) << 8) | (u32::from(bg.b) << 16);
    let _ = dwm::set_caption_color(hwnd, c);
}
