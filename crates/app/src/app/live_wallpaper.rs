//! Runtime live-wallpaper overlay, separate from the static virtual-desktop cache.
use super::*;
use pecofence_platform::live_wallpaper::{CONTRAST_SAMPLE_FACTOR, LiveWallpaper};

const FILL_DURATION: Duration = Duration::from_millis(1800);
const DRAIN_DURATION: Duration = Duration::from_millis(1600);

struct OpticalTransition {
    started: Instant,
    from: f32,
    velocity: f32,
    target: f32,
    omega: f32,
}

impl OpticalTransition {
    fn sample(&self, now: Instant) -> (f32, f32) {
        let t = now.saturating_duration_since(self.started).as_secs_f32();
        let offset = self.from - self.target;
        let b = self.velocity + self.omega * offset;
        let decay = (-self.omega * t).exp();
        let value = self.target + (offset + b * t) * decay;
        let velocity = (self.velocity - self.omega * b * t) * decay;
        if (value - self.target).abs() < 0.002 && velocity.abs() < 0.02 {
            (self.target, 0.0)
        } else {
            (value.clamp(0.0, 1.0), velocity)
        }
    }
}

#[derive(Default)]
struct GlassActivity {
    active: bool,
    fresh_after: Option<Instant>,
    transition: Option<OpticalTransition>,
}

impl GlassActivity {
    fn set_active(&mut self, active: bool, now: Instant) -> bool {
        if self.active == active {
            return false;
        }
        self.active = active;
        self.fresh_after = active.then_some(now);
        if !active {
            self.retarget(0.0, now, DRAIN_DURATION);
        }
        true
    }

    fn sample(&self, now: Instant) -> (f32, f32) {
        self.transition
            .as_ref()
            .map_or((0.0, 0.0), |t| t.sample(now))
    }

    fn retarget(&mut self, target: f32, now: Instant, duration: Duration) {
        let (from, velocity) = self.sample(now);
        self.transition = Some(OpticalTransition {
            started: now,
            from,
            velocity,
            target,
            omega: 6.0 / duration.as_secs_f32(),
        });
    }

    fn frame_received(&mut self, captured: Instant, now: Instant) {
        if self.active && self.fresh_after.is_some_and(|after| captured >= after) {
            self.fresh_after = None;
            self.retarget(1.0, now, FILL_DURATION);
        }
    }

    fn strength(&self, now: Instant) -> f32 {
        self.sample(now).0
    }

    fn animating(&self, now: Instant) -> bool {
        self.transition.as_ref().is_some_and(|t| {
            let (value, velocity) = t.sample(now);
            value != t.target || velocity != 0.0
        })
    }
}

fn desktop_has_focus() -> bool {
    let root = desktop::root_ancestor(desktop::foreground_window());
    if root.0.is_null() {
        return false;
    }
    let pid = desktop::window_pid(root);
    let class = desktop::class_name(root);
    let shell_pid = desktop::window_pid(desktop::shell_window());
    let our_pid = pecofence_platform::process::current_pid();
    if pid != our_pid {
        return foreground_allows_glass(&class, pid, shell_pid, our_pid, false);
    }
    let r = window::window_rect(root);
    // Our full-screen settings/test surface can cover the desktop too.
    let fullscreen = monitors::query(monitors::monitor_from_window(root)).is_some_and(|m| {
        r.left <= m.bounds.left
            && r.top <= m.bounds.top
            && r.right >= m.bounds.right
            && r.bottom >= m.bounds.bottom
    });
    foreground_allows_glass(&class, pid, shell_pid, our_pid, fullscreen)
}

fn foreground_allows_glass(
    class: &str,
    pid: u32,
    shell_pid: u32,
    our_pid: u32,
    fullscreen: bool,
) -> bool {
    ((class == "Progman" || class == "WorkerW") && pid == shell_pid)
        || (pid == our_pid && !fullscreen)
}

pub(super) struct LiveBackdrop {
    pub worker: LiveWallpaper,
    static_set: Option<Rc<BackdropSets>>,
    last_frame: Option<Instant>,
    applied: Option<(u64, Rc<BackdropSets>)>,
    waiting: bool,
    disabled: bool,
    activity: GlassActivity,
    bootstrap_until: Instant,
    strength: f32,
    #[cfg(debug_assertions)]
    activity_override: Option<bool>,
}

impl Default for LiveBackdrop {
    fn default() -> Self {
        Self {
            worker: LiveWallpaper::new(),
            static_set: None,
            last_frame: None,
            applied: None,
            waiting: false,
            disabled: pecofence_core::brand::var("PECOFENCE_LIVE_WALLPAPER")
                .is_ok_and(|s| s == "0"),
            activity: GlassActivity::default(),
            bootstrap_until: Instant::now() + Duration::from_secs(2),
            strength: 1.0,
            #[cfg(debug_assertions)]
            activity_override: None,
        }
    }
}

impl App {
    pub(super) fn tick_live_wallpaper(&mut self) -> bool {
        let enabled = !self.live_wallpaper.disabled
            && self.state.config.settings.live_wallpaper_capture
            && self.wallpaper_override.is_none()
            && self.ctx.theme.borrow().liquid_glass
            && self.fences.values().any(|f| desktop::is_visible(f.hwnd()));
        let now = Instant::now();
        let focused = self.peek.is_some() || desktop_has_focus();
        #[cfg(debug_assertions)]
        let focused = self.live_wallpaper.activity_override.unwrap_or(focused);
        let active = enabled && focused;
        if self.live_wallpaper.activity.set_active(active, now) {
            tracing::info!(active, "live glass desktop activity changed");
        }
        // Prime a real WPE image once even when launched behind another application.
        // Losing focus retains that image; it must never trigger native-wallpaper fallback.
        let capture = enabled
            && (active
                || (self.live_wallpaper.applied.is_none()
                    && now < self.live_wallpaper.bootstrap_until));
        self.live_wallpaper.worker.set_enabled(capture);
        let material_work = capture || (enabled && self.live_wallpaper.activity.animating(now));
        if material_work {
            self.ctx.frames.request();
        }
        let strength = if enabled {
            self.live_wallpaper.activity.strength(now)
        } else {
            1.0
        };
        let strength_changed = (self.live_wallpaper.strength - strength).abs() > 0.0001;
        self.live_wallpaper.strength = strength;
        for fence in self.fences.values() {
            fence.set_live_glass_strength(strength, false);
        }
        let frame = capture
            .then(|| self.live_wallpaper.worker.take_latest())
            .flatten();
        if let Some(frame) = frame.filter(|f| f.captured_at.elapsed() < Duration::from_millis(500))
        {
            let started = Instant::now();
            self.live_wallpaper
                .activity
                .frame_received(frame.captured_at, now);
            let fingerprint = frame.fingerprint;
            if self
                .live_wallpaper
                .applied
                .as_ref()
                .is_some_and(|(last, set)| {
                    *last == fingerprint && Rc::ptr_eq(set, &self.ctx.backdrops.borrow())
                })
            {
                // A paused WPE frame is a valid heartbeat: no uploads or redraws needed.
                self.live_wallpaper.last_frame = Some(frame.captured_at);
                self.live_wallpaper.waiting = false;
                if strength_changed {
                    self.redraw_live_material();
                }
                return material_work;
            }
            // A theme/topology refresh may have installed a new static set since the last
            // frame. Track it without ever caching a video frame as a static wallpaper.
            if let Some(sig) = &self.wallpaper_sig
                && let Some(static_set) = self.wallpaper_cache.get(sig)
            {
                self.live_wallpaper.static_set = Some(static_set);
            }
            let base = self
                .live_wallpaper
                .static_set
                .clone()
                .unwrap_or_else(|| self.ctx.backdrops.borrow().clone());
            let mut sources = base.acrylic.as_ref().clone();
            let mut replaced = 0;
            for (captured, sample) in frame.images.into_iter().zip(frame.samples) {
                let image = Image {
                    width: captured.width,
                    height: captured.height,
                    bgra: captured.bgra,
                };
                // Common path: one captured desktop root and one monitor. Move its
                // buffer straight into the upload slot, with no full-screen copy or
                // box averaging on the animation/input thread.
                if let Some(source) = sources.iter_mut().find(|s| {
                    s.left == captured.left
                        && s.top == captured.top
                        && s.width == captured.width as i32
                        && s.height == captured.height as i32
                }) {
                    source.image = Image {
                        width: sample.width,
                        height: sample.height,
                        bgra: sample.bgra,
                    };
                    source.gpu = Some(pecofence_render::backdrop::GpuPixels::new(1, image, || {
                        None
                    }));
                    source.downscale = CONTRAST_SAMPLE_FACTOR;
                    replaced += 1;
                    continue;
                }
                for source in &mut sources {
                    // Window coordinates are already screen pixels; do not apply wallpaper
                    // fill/fit or DPI scaling again. This also splits a spanning WPE surface.
                    if source.left >= captured.left
                        && source.top >= captured.top
                        && i64::from(source.left) + i64::from(source.width)
                            <= i64::from(captured.left) + i64::from(captured.width)
                        && i64::from(source.top) + i64::from(source.height)
                            <= i64::from(captured.top) + i64::from(captured.height)
                    {
                        let pixels = image.crop(
                            source.left - captured.left,
                            source.top - captured.top,
                            source.width,
                            source.height,
                        );
                        *source = MonitorBackdrop {
                            left: source.left,
                            top: source.top,
                            width: source.width,
                            height: source.height,
                            downscale: 1,
                            image: pixels,
                            gpu: None,
                        }
                        .into_gpu_backed(8, || None);
                        replaced += 1;
                    }
                }
            }
            if replaced > 0 {
                let set = Rc::new(BackdropSets {
                    acrylic: Rc::new(sources),
                });
                if self.live_wallpaper.last_frame.is_none() {
                    tracing::info!(
                        monitors = replaced,
                        fps = self.live_wallpaper.worker.fps,
                        "Wallpaper Engine live glass active"
                    );
                }
                self.live_wallpaper.last_frame = Some(frame.captured_at);
                self.live_wallpaper.waiting = false;
                self.live_wallpaper.applied = Some((fingerprint, set.clone()));
                let prepared = Instant::now();
                self.apply_live_backdrops(set);
                tracing::trace!(target: "pecofence::live_timing", prep_us = prepared.duration_since(started).as_micros(), draw_us = prepared.elapsed().as_micros(), "live wallpaper apply");
                return material_work;
            }
        }
        if self
            .live_wallpaper
            .last_frame
            .is_some_and(|_| !enabled || !self.live_wallpaper.worker.engine_running())
        {
            self.live_wallpaper.last_frame = None;
            self.live_wallpaper.applied = None;
            self.live_wallpaper.waiting = false;
            if let Some(base) = self.live_wallpaper.static_set.take() {
                // Prefer a newly refreshed static set after a theme/topology change.
                let base = self
                    .wallpaper_sig
                    .as_ref()
                    .and_then(|sig| self.wallpaper_cache.get(sig))
                    .unwrap_or(base);
                self.apply_live_backdrops(base);
            }
            tracing::info!(
                "Wallpaper Engine stopped or live capture disabled; static glass restored"
            );
        } else if enabled && self.live_wallpaper.worker.engine_running() {
            // A transition or temporary capture failure must never silently change the
            // material to the Windows wallpaper. Also protect against an OS wallpaper
            // notification temporarily replacing the context with a newly decoded static set.
            if let Some((_, set)) = &self.live_wallpaper.applied
                && !Rc::ptr_eq(set, &self.ctx.backdrops.borrow())
            {
                self.apply_live_backdrops(set.clone());
            }
            if capture
                && !self.live_wallpaper.waiting
                && self
                    .live_wallpaper
                    .last_frame
                    .is_some_and(|at| at.elapsed() > Duration::from_secs(2))
            {
                self.live_wallpaper.waiting = true;
                tracing::warn!("Wallpaper Engine capture reconnecting; retaining last live frame");
            }
        }
        if strength_changed {
            self.redraw_live_material();
        }
        material_work
    }

    fn redraw_live_material(&self) {
        for fence in self.fences.values() {
            fence.set_live_glass_strength(self.live_wallpaper.strength, true);
        }
    }

    #[cfg(debug_assertions)]
    pub(super) fn test_live_activity(&mut self, active: Option<bool>) {
        if pecofence_core::brand::var_os("PECOFENCE_UI_TEST_WINDOWS").is_some() {
            self.live_wallpaper.activity_override = active;
            self.tick_live_wallpaper();
        }
    }

    pub(super) fn live_render_state(&self) -> String {
        format!(
            "active={} strength={:.3} fps={} has_live={} animating={}",
            self.live_wallpaper.activity.active,
            self.live_wallpaper.strength,
            self.live_wallpaper.worker.fps,
            self.live_wallpaper.applied.is_some(),
            self.live_wallpaper.activity.animating(Instant::now())
        )
    }

    fn apply_live_backdrops(&mut self, set: Rc<BackdropSets>) {
        *self.ctx.backdrops.borrow_mut() = set.clone();
        for fence in self.fences.values() {
            fence.set_live_backdrops(set.clone());
        }
        self.ctx.bitmaps.borrow_mut().prune_wallpapers();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn foreign_apps_and_fullscreen_pause_but_the_desktop_and_our_popups_resume() {
        assert!(foreground_allows_glass("Progman", 1, 1, 2, false));
        assert!(foreground_allows_glass("WorkerW", 1, 1, 2, true));
        assert!(!foreground_allows_glass("WorkerW", 3, 1, 2, false));
        assert!(!foreground_allows_glass(
            "Chrome_WidgetWin_1",
            3,
            1,
            2,
            false
        ));
        assert!(!foreground_allows_glass("GameWindow", 3, 1, 2, true));
        assert!(foreground_allows_glass(
            "PecoFence.Settings",
            2,
            1,
            2,
            false
        ));
        assert!(!foreground_allows_glass(
            "PecoFence.Settings",
            2,
            1,
            2,
            true
        ));
    }

    #[test]
    fn returning_to_desktop_waits_for_fresh_capture_then_fills_smoothly() {
        let now = Instant::now();
        let mut activity = GlassActivity::default();
        assert_eq!(activity.strength(now), 0.0);
        assert!(activity.set_active(true, now));
        activity.frame_received(now - Duration::from_millis(1), now);
        assert_eq!(activity.strength(now + FILL_DURATION), 0.0);
        activity.frame_received(now, now);
        assert_eq!(activity.strength(now), 0.0);
        assert!(activity.strength(now + Duration::from_millis(650)) < 0.7);
        assert!(activity.strength(now + FILL_DURATION) > 0.95);
        let settled = now + FILL_DURATION * 2;
        assert_eq!(activity.strength(settled), 1.0);
        assert!(!activity.animating(settled));
        assert!(activity.set_active(false, settled));
        assert_eq!(activity.strength(settled), 1.0);
        assert!(activity.strength(settled + DRAIN_DURATION / 2) < 0.3);
        assert_eq!(activity.strength(settled + DRAIN_DURATION * 2), 0.0);
        assert!(!activity.animating(settled + DRAIN_DURATION * 2));
    }

    #[test]
    fn reversal_keeps_the_current_strength_and_velocity_without_restarting() {
        let now = Instant::now();
        let mut activity = GlassActivity::default();
        activity.set_active(true, now);
        activity.frame_received(now, now);
        let switched = now + Duration::from_millis(650);
        let before = activity.sample(switched);
        activity.set_active(false, switched);
        let after = activity.sample(switched);
        assert!((before.0 - after.0).abs() < 0.00001);
        assert!((before.1 - after.1).abs() < 0.00001);
        assert!(activity.animating(switched));
        let back = switched + Duration::from_millis(100);
        activity.set_active(true, back);
        let waiting = activity.sample(back);
        activity.frame_received(back - Duration::from_millis(1), back);
        assert_eq!(activity.sample(back), waiting);
        activity.frame_received(back, back);
        let resumed = activity.sample(back);
        assert!((resumed.0 - waiting.0).abs() < 0.00001);
        assert!((resumed.1 - waiting.1).abs() < 0.00001);
        assert_eq!(activity.strength(back + FILL_DURATION * 2), 1.0);
    }
}
