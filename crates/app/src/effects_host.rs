//! Host for the native desktop effects overlay (`تأثيرات سطح المكتب الحية`).
//!
//! One click-through `WS_POPUP` window spanning the virtual screen, rendered
//! by a [`Panel`] (Windows.UI.Composition → Direct2D, premultiplied alpha) and
//! anchored into the desktop band by `DesktopAnchor::register_desktop_surface`
//! — directly above the icon host (icons, wallpaper, video wallpaper) and below
//! every fence and the clock widget.
//!
//! Timers on the overlay HWND drive everything:
//! - `TIMER_FRAME` (~30 FPS): obstacle poll → `Engine::tick` → `Panel::draw`.
//! - `TIMER_OCCLUSION` (500 ms): fullscreen/maximized-foreground detection →
//!   kill the frame timer and hide the visual (0 % CPU/GPU while occluded).
//! - `TIMER_WAKE` (100 ms, only while `Engine::idle`): cursor poll that wakes
//!   ripples without ever running the render loop.

use std::cell::RefCell;
use std::rc::Rc;
use std::time::Instant;

use pecofence_platform::window::{
    self, ClassOptions, MessageHandler, Window, WindowBuilder, WindowClass, style,
};
use pecofence_platform::{HWND, desktop, dwm, monitors, msg, wmf_wallpaper};
use pecofence_render::{Panel, RenderStack};
use windows_composition::{ContainerVisual, DesktopWindowTarget};
use windows_core::Result;

use crate::commands::{Command, CommandQueue};
use crate::effects_engine::{Effect, Engine, FxCache};

const CLASS_NAME: &str = "PecoFence.DesktopEffects";

const TIMER_FRAME: usize = 1;
const TIMER_OCCLUSION: usize = 2;
const TIMER_WAKE: usize = 3;

const FRAME_MS: u32 = 33; // ~30 FPS default pacing
const OCCLUSION_MS: u32 = 500;
const WAKE_MS: u32 = 100;

/// The app-facing handle. Cheap to clone; the real state lives in the window's
/// `Rc<RefCell<HostCore>>` so the message handler owns the same data.
pub struct EffectsHost {
    class: WindowClass,
    stack: Rc<RenderStack>,
    core: Rc<RefCell<HostCore>>,
}

struct HostCore {
    // Live overlay objects; all `None` while the effect is "none" — that is
    // the zero-resource state (window destroyed, surfaces released).
    window: Option<Window>,
    _target: Option<DesktopWindowTarget>,
    root: Option<ContainerVisual>,
    panel: Option<Panel>,
    engine: Option<Engine>,
    cache: FxCache,
    // Fence + clock HWNDs polled for obstacle rects each frame; `rects` is the
    // reused scratch (no per-tick allocation).
    obstacle_hwnds: Vec<HWND>,
    rects: Vec<(f32, f32, f32, f32)>,
    // The overlay's own HWND + other windows the occlusion check must skip.
    paused: bool,
    sleeping: bool,
    wants_cursor: bool,
    /// `Ok(false)` from `Panel::draw` meant the GPU device is gone; the app
    /// rebuilds the stack and calls `recreate_surfaces` (same recovery path
    /// as the fence windows).
    device_lost: bool,
    queue: CommandQueue,
    last_tick: Instant,
    dpi: u32,
}

impl EffectsHost {
    pub fn register_class() -> Result<WindowClass> {
        WindowClass::register(CLASS_NAME, ClassOptions::default())
    }

    /// The class must already be registered (see [`Self::register_class`]);
    /// kept alive by `self`. `queue` carries `Command::DeviceLost` to the app
    /// the way fence windows report it.
    pub fn new(class: WindowClass, stack: Rc<RenderStack>, queue: CommandQueue) -> Self {
        Self {
            class,
            stack,
            core: Rc::new(RefCell::new(HostCore {
                window: None,
                _target: None,
                root: None,
                panel: None,
                engine: None,
                cache: FxCache::default(),
                obstacle_hwnds: Vec::new(),
                rects: Vec::with_capacity(24),
                paused: false,
                sleeping: false,
                wants_cursor: false,
                device_lost: false,
                queue,
                last_tick: Instant::now(),
                dpi: 96,
            })),
        }
    }

    /// The overlay HWND, for `DesktopAnchor::register_desktop_surface`.
    pub fn hwnd(&self) -> Option<HWND> {
        self.core.borrow().window.as_ref().map(|w| w.hwnd())
    }

    /// Apply the persisted trio. `"none"`/unknown ids tear the overlay down
    /// completely (zero memory, zero GPU). Live changes reconfigure in place.
    pub fn apply(&self, effect_id: &str, intensity: f32, collision: bool) {
        let Some(effect) = Effect::from_id(effect_id) else {
            self.close();
            return;
        };
        {
            let mut c = self.core.borrow_mut();
            if let Some(e) = c.engine.as_mut()
                && e.effect() == effect
            {
                e.set_intensity(intensity);
                e.set_collision(collision);
                c.wants_cursor = effect.wants_cursor();
                return;
            }
        }
        if let Err(e) = self.open(effect, intensity, collision) {
            tracing::error!(error = %e, ?effect, "effects: failed to open overlay");
            self.close();
        }
    }

    /// Fence/clock window set changed — the host polls these HWNDs per frame.
    pub fn set_obstacle_windows(&self, hwnds: Vec<HWND>) {
        self.core.borrow_mut().obstacle_hwnds = hwnds;
    }

    /// The render stack was rebuilt after device loss: `Panel::recreate` makes
    /// a fresh surface from the new graphics device (a plain `resize` would
    /// draw on the dead one), and the brush cache is dropped because every
    /// cached brush was created on the lost device.
    pub fn recreate_surfaces(&self) {
        let mut c = self.core.borrow_mut();
        if let Some(p) = c.panel.as_mut() {
            if p.recreate(&self.stack).is_ok() {
                c.cache = FxCache::default();
                c.device_lost = false;
            }
        }
    }

    /// See `FenceWindow::device_lost` — the app's recovery asks everyone.
    pub fn device_lost(&self) -> bool {
        self.core.borrow().device_lost
    }

    /// Create the overlay window + composition tree + engine, start timers.
    fn open(&self, effect: Effect, intensity: f32, collision: bool) -> Result<()> {
        // Tear down any previous instance first (effect switches rebuild).
        self.close();

        // Virtual screen = union of every monitor's bounds (physical px).
        let (mut vx, mut vy, mut vr, mut vb) = (i32::MAX, i32::MAX, i32::MIN, i32::MIN);
        for m in monitors::enumerate() {
            vx = vx.min(m.bounds.left);
            vy = vy.min(m.bounds.top);
            vr = vr.max(m.bounds.right);
            vb = vb.max(m.bounds.bottom);
        }
        if vx > vr {
            vx = 0;
            vy = 0;
            vr = 1;
            vb = 1;
        }
        let (w_px, h_px) = ((vr - vx).max(1), (vb - vy).max(1));

        let core = self.core.clone();
        let handler: MessageHandler = Box::new(
            move |hwnd: HWND, message: u32, wparam: usize, lparam: isize| -> Option<isize> {
                let mut c = match core.try_borrow_mut() {
                    Ok(c) => c,
                    Err(_) => return None,
                };
                match message {
                    msg::WM_NCHITTEST => Some(msg::HTTRANSPARENT),
                    msg::WM_TIMER => {
                        match wparam {
                            TIMER_FRAME => c.frame(),
                            TIMER_OCCLUSION => c.occlusion_check(),
                            TIMER_WAKE => c.wake_check(),
                            _ => {}
                        }
                        Some(0)
                    }
                    msg::WM_SIZE => {
                        c.resize(lparam as i32 & 0xFFFF, (lparam >> 16) as i32 & 0xFFFF);
                        Some(0)
                    }
                    msg::WM_DPICHANGED => {
                        c.dpi = window::dpi_of(hwnd);
                        c.resize_to_window();
                        Some(0)
                    }
                    _ => None,
                }
            },
        );

        // WS_POPUP + no-activate/toolwindow + no redirection bitmap, exactly
        // the extended style the fence windows use: `DesktopWindowTarget`
        // windows must NOT be `WS_EX_LAYERED` — layered redirection conflicts
        // with `WS_EX_NOREDIRECTIONBITMAP`; the composition tree provides the
        // transparency itself.
        let window = WindowBuilder::new(&self.class)
            .title("PecoFence Desktop Effects")
            .style(style::POPUP | style::CLIPCHILDREN | style::CLIPSIBLINGS)
            .ex_style(style::EX_NOACTIVATE | style::EX_TOOLWINDOW | style::EX_NOREDIRECTIONBITMAP)
            .bounds(vx, vy, w_px, h_px)
            .create(handler)?;

        let hwnd = window.hwnd();
        // Click-through comes from WM_NCHITTEST → HTTRANSPARENT only. An empty
        // SetWindowRgn would also make the window click-through, but it clips
        // the window's update region to nothing — the composition visual tree
        // would be cropped to a zero-area shape and never composited.
        let _ = dwm::set_transitions_disabled(hwnd, true);

        // Composition tree: target on the HWND → container → Panel sprite.
        let target = self.stack.create_target(hwnd)?;
        let root = self.stack.compositor.create_container_visual();
        let mut panel = Panel::new(&self.stack)?;
        root.children().insert_at_top(&panel.visual);
        target.set_root(&root);

        let dpi = window::dpi_of(hwnd);
        panel_resize(&mut panel, w_px, h_px)?;
        let scale = dpi.max(96) as f32 / 96.0;
        let engine = Engine::new(
            effect,
            intensity,
            collision,
            w_px as f32 / scale,
            h_px as f32 / scale,
        );

        {
            let mut c = self.core.borrow_mut();
            c.dpi = dpi;
            c.wants_cursor = effect.wants_cursor();
            c.engine = Some(engine);
            c.cache = FxCache::default();
            c.panel = Some(panel);
            c.root = Some(root);
            c._target = Some(target);
            c.window = Some(window);
            c.paused = false;
            c.sleeping = false;
            c.last_tick = Instant::now();
        }
        let c = self.core.borrow();
        let w = c.window.as_ref().unwrap();
        w.set_timer(TIMER_FRAME, FRAME_MS);
        w.set_timer(TIMER_OCCLUSION, OCCLUSION_MS);
        w.show_no_activate();
        tracing::info!(?effect, vx, vy, w_px, h_px, hwnd = ?hwnd, "effects: overlay opened");
        Ok(())
    }

    /// Destroy the overlay and release every GPU resource — the "none" state.
    pub fn close(&self) {
        let mut c = self.core.borrow_mut();
        if let Some(w) = c.window.take() {
            w.kill_timer(TIMER_FRAME);
            w.kill_timer(TIMER_OCCLUSION);
            w.kill_timer(TIMER_WAKE);
        }
        // Order matters: drop the visual tree before the target, the target
        // before the window.
        c.engine = None;
        c.cache = FxCache::default();
        if let Some(root) = c.root.take() {
            if let Some(p) = c.panel.take() {
                root.children().remove(&p.visual);
            }
        }
        c._target = None;
        c.paused = false;
        c.sleeping = false;
        // `window` drops here → DestroyWindow.
    }
}

fn panel_resize(panel: &mut Panel, w_px: i32, h_px: i32) -> Result<()> {
    panel.resize(w_px.max(1), h_px.max(1))?;
    Ok(())
}

impl HostCore {
    /// ~30 FPS: poll obstacles → tick → draw → present.
    fn frame(&mut self) {
        let (Some(window), Some(panel), Some(engine)) = (
            self.window.as_ref(),
            self.panel.as_mut(),
            self.engine.as_mut(),
        ) else {
            return;
        };
        let now = Instant::now();
        let dt = now.duration_since(self.last_tick).as_secs_f32();
        self.last_tick = now;

        // Obstacle rects: fence/clock HWNDs → surface coords (DIP).
        let origin = window.window_rect();
        let scale = self.dpi.max(96) as f32 / 96.0;
        self.rects.clear();
        for &h in &self.obstacle_hwnds {
            if h.0.is_null() || !window::is_window(h) || !desktop::is_visible(h) {
                continue;
            }
            let r = window::window_rect(h);
            if r.right <= r.left || r.bottom <= r.top {
                continue;
            }
            self.rects.push((
                (r.left - origin.left) as f32 / scale,
                (r.top - origin.top) as f32 / scale,
                (r.right - origin.left) as f32 / scale,
                (r.bottom - origin.top) as f32 / scale,
            ));
        }
        engine.set_obstacles(&self.rects);

        if self.wants_cursor {
            let pt = window::cursor_pos();
            engine.set_cursor(
                (pt.x - origin.left) as f32 / scale,
                (pt.y - origin.top) as f32 / scale,
            );
        }
        engine.tick(dt.max(0.0));

        let dpi = self.dpi;
        let cache = &mut self.cache;
        let ok = panel.draw(dpi, |s, _w, _h| engine.draw(s, cache));
        match ok {
            Ok(true) => {}
            Ok(false) => {
                // Device lost — the surface draw retired itself; rebuilding the
                // surface on the dead device would keep failing, so hand the
                // recovery to the app exactly like a fence window does. It is
                // reported once: nothing else would ask again.
                if !self.device_lost {
                    self.device_lost = true;
                    tracing::warn!("effects: composition surface lost its device");
                    self.queue.push(Command::DeviceLost);
                }
            }
            Err(e) => tracing::warn!(error = %e, "effects: draw failed"),
        }

        // Cursor-driven effects idle → swap the frame timer for the cheap
        // wake poll (0 % GPU while the pointer rests).
        if engine.idle() && !self.sleeping {
            self.sleeping = true;
            window.kill_timer(TIMER_FRAME);
            window.set_timer(TIMER_WAKE, WAKE_MS);
        }
    }

    /// 500 ms: is a fullscreen app / game / maximized window hiding the
    /// desktop? PecoFence-owned windows are exempt inside
    /// `should_pause_playback` (own-PID + NOACTIVATE filters cover the video
    /// wallpaper HWND, the overlay itself, fences, settings and the clock).
    fn occlusion_check(&mut self) {
        let occluded = wmf_wallpaper::should_pause_playback() || user_notice_blocked();
        if occluded == self.paused {
            return;
        }
        self.paused = occluded;
        let Some(window) = self.window.as_ref() else {
            return;
        };
        if occluded {
            // Hide the visual BEFORE killing the timer so no stale frame
            // lingers above the app that just went fullscreen, then remove the
            // HWND from the desktop composition pass entirely — the visual
            // alone still costs DWM an empty composited layer each frame.
            if let Some(p) = self.panel.as_ref() {
                p.visual.set_visible(false);
            }
            window.hide();
            window.kill_timer(TIMER_FRAME);
            window.kill_timer(TIMER_WAKE);
            tracing::debug!("effects: auto-paused (occluded)");
        } else {
            if let Some(p) = self.panel.as_ref() {
                p.visual.set_visible(true);
            }
            window.show_no_activate();
            self.last_tick = Instant::now();
            window.set_timer(TIMER_FRAME, FRAME_MS);
            tracing::debug!("effects: auto-resumed");
        }
    }

    /// 100 ms while the effect idles: cursor motion wakes the frame timer.
    fn wake_check(&mut self) {
        let Some(window) = self.window.as_ref() else {
            return;
        };
        if !self.sleeping || self.paused {
            window.kill_timer(TIMER_WAKE);
            return;
        }
        let pt = window::cursor_pos();
        let origin = window.window_rect();
        let scale = self.dpi.max(96) as f32 / 96.0;
        if let Some(engine) = self.engine.as_mut() {
            engine.set_cursor(
                (pt.x - origin.left) as f32 / scale,
                (pt.y - origin.top) as f32 / scale,
            );
            if !engine.idle() {
                self.sleeping = false;
                window.kill_timer(TIMER_WAKE);
                self.last_tick = Instant::now();
                window.set_timer(TIMER_FRAME, FRAME_MS);
            }
        }
    }

    fn resize(&mut self, w_px: i32, h_px: i32) {
        if let Some(panel) = self.panel.as_mut() {
            let _ = panel.resize(w_px.max(1), h_px.max(1));
            let scale = self.dpi.max(96) as f32 / 96.0;
            if let Some(e) = self.engine.as_mut() {
                e.set_size(w_px as f32 / scale, h_px as f32 / scale);
            }
        }
    }

    fn resize_to_window(&mut self) {
        if let Some(w) = self.window.as_ref() {
            let r = w.window_rect();
            self.resize(r.right - r.left, r.bottom - r.top);
        }
    }
}

/// `SHQueryUserNotificationState` extras beyond the foreground-window check.
/// Only `QUNS_RUNNING_D3D_FULL_SCREEN` (3) counts: it flags an exclusive D3D
/// fullscreen device whose window the coverage check can miss, and — unlike
/// `QUNS_BUSY`/`QUNS_PRESENTATION_MODE` — it can never be triggered by this
/// overlay itself. BUSY/PRESENTATION would self-trigger: a fullscreen top-level
/// window looks like a fullscreen app to the shell, so showing the overlay
/// reported BUSY, hiding it cleared BUSY, and the check flapped every tick.
/// On multi-monitor setups QUNS is global while the game covers one display —
/// the foreground coverage check remains the per-monitor signal.
fn user_notice_blocked() -> bool {
    monitors::enumerate().len() <= 1
        && matches!(
            pecofence_platform::store::user_notification_state(),
            Some(3) // QUNS_RUNNING_D3D_FULL_SCREEN
        )
}
