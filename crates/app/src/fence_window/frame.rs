//! Per-frame plumbing: the client-tween tick and the window-height animation phases.

use super::*;

impl FenceViewState {
    /// Advances client-side tweens and per-frame auto-scroll for one frame: the scroll glide,
    /// marquee / drag edge auto-scroll, the scrollbar's width and alpha, and the state fades.
    /// Each surface is redrawn at most once, only when something on it moved. Returns true
    /// while anything still moves (the caller then requests another frame). Must not change
    /// window geometry or send messages synchronously (posting is fine) — the caller does
    /// that with no borrow held.
    pub(super) fn tick(&mut self, now: Instant, draw_chrome: bool) -> bool {
        let mut more = self.tick_shadow_alpha(now);
        let mut content = false;
        let mut glide_done = false;
        if let Some(t) = self.scroll_anim {
            // A hard wall at both ends even mid-glide: no eased sample (nor a future
            // overshooting curve) may push items past the content's edges.
            self.scroll_y = t.value_at(now).clamp(0.0, self.scroll_max());
            let done = t.is_done(now);
            if done {
                // Exactly the (pixel-snapped) destination, whatever the eased sample was.
                self.scroll_y = t.target();
                self.scroll_anim = None;
                glide_done = true;
            }
            content = true;
            more |= !done;
        }
        if self.marquee_autoscroll(now) {
            content = true;
            more = true;
        }
        if self.drag_autoscroll(now) {
            content = true;
            more = true;
        }
        if self.tab_strip_autoscroll(now) {
            more = true;
        }
        if self.sb_busy {
            content = true;
            self.sb_arrow_fades.prune(now, |_| true);
            self.sb_busy = !self.sb_width.is_done(now)
                || !self.sb_alpha.is_done(now)
                || !self.sb_parts.is_done(now)
                || self.sb_arrow_fades.busy(now);
            more |= self.sb_busy;
        }
        if self.content_busy {
            content = true;
        }
        if self.rolled_up && self.roll_anim.is_none() {
            // Rolled: nothing is visible, and a glide must not keep moving the hidden offset.
            self.content_busy = false;
            self.scroll_anim = None;
            content = false;
        }
        if content {
            if glide_done {
                self.note_scroll_activity();
                self.request_hover_refresh();
            }
            if self.marquee.is_some() {
                self.update_marquee_selection();
            }
            let before = self.scroll_y;
            let _ = self.redraw_content();
            if self.scroll_y != before {
                // The draw clamped the offset (content shrank mid-glide): drop the glide.
                self.scroll_anim = None;
            }
            more |= self.content_busy;
        }
        let rolling_live_glass = self.height_animating()
            && self.theme.liquid_glass
            && self.gpu_glass.borrow().strength() > 0.0;
        if draw_chrome && (self.chrome_busy || rolling_live_glass) {
            let _ = self.draw_chrome_panel();
            more |= self.chrome_busy;
        }
        more
    }
}

impl FenceWindow {
    /// One compositor frame: advances the roll / auto-height animation and every other client
    /// tween. Returns true while something still moves (the app then asks the frame clock for
    /// the next frame). Re-entrancy: the window is resized with no borrow held.
    pub fn on_frame(&self, now: Instant) -> bool {
        let hwnd = self.hwnd();
        let moved = flush_window_drag(&self.view, hwnd, now, None);
        // Phase 1: compute under a short borrow.
        let (height, more) = {
            let mut guard = self.view.borrow_mut();
            let Some(v) = guard.as_mut() else {
                return false;
            };
            // A collapse to a left tab column (or an expand from it) also moves the width.
            let width = v
                .roll_anim
                .as_ref()
                .and_then(|a| a.width)
                .map(|t| t.value_at(now).round() as i32);
            if let Some(t) = v.roll_anim.as_ref().and_then(|a| a.content_alpha) {
                v.content_panel
                    .visual
                    .set_opacity(t.value_at(now).clamp(0.0, 1.0));
            }
            let height = v
                .roll_anim
                .as_ref()
                .map(|a| (a.height, Some(a.rolled_at_end)))
                .or_else(|| v.height_anim.map(|t| (t, None)))
                .map(|(t, rolled)| {
                    (
                        (t.value_at(now).round() as i32).max(v.title_h_px()),
                        t.is_done(now),
                        rolled,
                    )
                });
            // If the size changes, WM_SIZE will render the new chrome. Drawing hover
            // feedback here would first publish the same frame at the previous size.
            let draw_chrome = !moved
                && height.is_none_or(|(h, _, _)| {
                    let (cur_w, cur_h) = window::window_rect_size(hwnd);
                    h == cur_h && width.is_none_or(|w| w == cur_w)
                });
            let more = v.tick(now, draw_chrome);
            (
                height.map(|(h, done, rolled)| (width, h, done, rolled)),
                more,
            )
        };
        // Phase 2: window geometry with no borrow held (WM_SIZE re-enters the handler; while
        // an animation runs it only redraws the chrome and moves the shadow).
        let Some((w, h, done, rolled_at_end)) = height else {
            return more;
        };
        let r = window::window_rect(hwnd);
        let w = w.unwrap_or(r.right - r.left);
        if h != r.bottom - r.top || w != r.right - r.left {
            // Publish a complete liquid-glass frame before enlarging the native window.
            // Repeated surface Resize followed by a later WM_SIZE draw could expose an
            // old collapsed capsule while the window had already become full height.
            self.with_view(|v| {
                if v.theme.liquid_glass {
                    let next = RECT {
                        right: r.left + w,
                        bottom: r.top + h,
                        ..r
                    };
                    v.update_backdrop_at(next);
                    let drawn = v
                        .update_shape_clip(w, h)
                        .and_then(|_| v.resize_chrome_viewport(w, h))
                        .and_then(|_| v.draw_chrome_panel());
                    match drawn {
                        Ok(()) => v.prepared_height_frame = Some((w, h)),
                        Err(error) => tracing::error!(%error, "prepare height frame failed"),
                    }
                }
            });
            if let Err(error) = window::set_window_bounds(hwnd, r.left, r.top, w, h) {
                tracing::error!(%error, "apply height frame failed");
            }
            self.with_view(|v| v.prepared_height_frame = None);
        }
        tracing::trace!(target: "pecofence::height_motion", height = h, done,
            rolled = ?rolled_at_end, "height frame");
        if !done {
            return true;
        }
        // Phase 3: final geometry — lay the frozen content out once and rasterise everything.
        let (queue, behavior) = {
            let mut guard = self.view.borrow_mut();
            let Some(v) = guard.as_mut() else {
                return more;
            };
            v.roll_anim = None;
            v.height_anim = None;
            if let Some(rolled) = rolled_at_end {
                v.rolled_up = rolled;
                v.roll_t = Tween::at(rolled as u8 as f32, Instant::now());
                let _ = v.motion.stop(&v.content_panel.visual, Prop::Opacity);
                v.content_panel.visual.set_opacity(1.0);
                v.content_panel.visual.set_visible(!rolled);
            }
            // The resting window gets a crop cut for its final rect (one crop + upload per
            // animation, in the pass that redraws everything anyway).
            v.update_backdrop();
            let (w, h) = window::window_rect_size(hwnd);
            if let Err(e) = v.layout_panels(w, h).and_then(|_| {
                if !v.rolled_up
                    && (v.content_dirty
                        || v.content_ink != Some(v.material_foreground().text_primary))
                {
                    v.redraw()
                } else {
                    v.redraw_chrome_only()
                }
            }) {
                tracing::error!(error = %e, "redraw after height animation failed");
            }
            v.update_shadow_now();
            (v.queue.clone(), v.behavior.clone())
        };
        queue_bounds_changed(&queue, &behavior, self.id, hwnd);
        more
    }
}
