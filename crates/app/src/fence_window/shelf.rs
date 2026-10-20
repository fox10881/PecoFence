//! 3D shelf: Y-shear geometry for a fence shown as a parallelogram ("angled shelf").
//!
//! The HWND stays the axis-aligned bounding box. Inside it the content slab is a
//! `w x slab_h` rectangle whose left / right edge tops sit `skew_left` / `skew_right`
//! DIPs below the window top (at least one is 0 — the higher edge). A slab point
//! `(x, y)` maps to window `(x, y + lerp(skew_left, skew_right, x / w))`; hit tests
//! run the inverse (`unskew`). The two resize handles sit on the top of each edge,
//! so "the handle with the highest Y value (lowest on screen) is the closer edge"
//! is literally `skew_left.cmp(&skew_right)`; icons shrink toward the far edge by
//! `depth_scale`.

use super::*;

/// Which vertical edge a shelf handle drives.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum SkewEdge {
    Left,
    Right,
}

/// A shelf-edge drag in progress. The window's top moves mid-drag (whichever edge
/// sits higher owns it), so everything here is in screen px: the pointer's Y at
/// press, both edge tops at press, and the slab height the bounding box re-fits to.
pub(super) struct SkewDrag {
    pub(super) edge: SkewEdge,
    pub(super) grab_y: i32,
    pub(super) left_top: i32,
    pub(super) right_top: i32,
    /// Slab height (DIPs) when the drag began; the window grows around it.
    pub(super) slab_h: f32,
    /// Whether the pointer actually moved (a plain press releases without saving).
    pub(super) moved: bool,
}

/// Width (DIPs) of the edge-handle grab zone, and its height around the edge top.
pub(super) const SHEAR_HANDLE_W: f32 = 18.0;
/// Grab-zone height (DIPs) below the edge top the handle sits on.
pub(super) const SHEAR_HANDLE_H: f32 = 18.0;
/// Deepest an edge may be dragged (DIPs) — keeps the tilt readable and the window bounded.
pub(super) const MAX_SKEW_DIP: f32 = 960.0;

impl FenceViewState {
    /// Whether the window is drawn and hit-tested as a sheared parallelogram.
    /// A rolled-up fence is a plain title bar: the shear (and its extra window
    /// height) only applies to the expanded slab.
    pub(super) fn skew_active(&self) -> bool {
        self.shelf_3d && !self.rolled_up
    }

    /// Left / right edge-top offsets in DIPs (0,0 when shelf mode is off).
    pub(super) fn edge_offsets(&self) -> (f32, f32) {
        if self.skew_active() {
            (self.skew_left, self.skew_right)
        } else {
            (0.0, 0.0)
        }
    }

    /// The window size in DIPs.
    pub(super) fn window_dips(&self) -> (f32, f32) {
        let (w, h) = window::window_rect_size(self.hwnd);
        let s = self.scale();
        (w as f32 / s, h as f32 / s)
    }

    /// Extra window height the shear needs beyond the slab (DIPs).
    pub(super) fn skew_spread(&self) -> f32 {
        let (sl, sr) = self.edge_offsets();
        sl.max(sr) - sl.min(sr)
    }

    /// The same spread in device px (window heights include it while sheared).
    pub(super) fn spread_px(&self) -> i32 {
        (self.skew_spread() * self.scale()).round() as i32
    }

    /// Slab height in DIPs: the window's full height minus the spread the shear adds.
    pub(super) fn slab_h(&self) -> f32 {
        let (_, h) = self.window_dips();
        (h - self.skew_spread()).max(1.0)
    }

    /// The edge-top offset at slab x fraction `xf` (0 = left edge, 1 = right).
    pub(super) fn shear_at(&self, xf: f32) -> f32 {
        let (sl, sr) = self.edge_offsets();
        shear_offset_at(sl, sr, xf)
    }

    /// Y-shear as (m12, m32) for a `Matrix3x2`: window_y = x·m12 + y + m32 over the
    /// slab width `w` — i.e. `y + lerp(skew_left, skew_right, x / w)`.
    pub(super) fn shear_m(&self, w: f32) -> (f32, f32) {
        let (sl, sr) = self.edge_offsets();
        ((sr - sl) / w.max(1.0), sl)
    }

    /// Forward map: slab DIPs -> window DIPs.
    pub(super) fn shear_point(&self, sx: f32, sy: f32) -> (f32, f32) {
        let (w, _) = self.window_dips();
        (sx, sy + self.shear_at(sx / w.max(1.0)))
    }

    /// Inverse map: window DIPs -> slab DIPs.
    pub(super) fn unskew(&self, wx: f32, wy: f32) -> (f32, f32) {
        let (w, _) = self.window_dips();
        (wx, wy - self.shear_at(wx / w.max(1.0)))
    }

    /// Whether a window-space point (DIPs) lies inside the parallelogram.
    pub(super) fn inside(&self, wx: f32, wy: f32) -> bool {
        let (w, _) = self.window_dips();
        let (dx, dy) = self.unskew(wx, wy);
        dx >= 0.0 && dx < w && dy >= 0.0 && dy < self.slab_h()
    }

    /// Client pixel point -> slab DIPs, or None outside the parallelogram. When the
    /// shelf is flat every client point maps through, like before.
    pub(super) fn slab_point(&self, x_px: i32, y_px: i32) -> Option<(f32, f32)> {
        let s = self.scale();
        let (wx, wy) = (x_px as f32 / s, y_px as f32 / s);
        if !self.skew_active() {
            return Some((wx, wy));
        }
        self.inside(wx, wy).then(|| self.unskew(wx, wy))
    }

    /// Client px -> slab-space px: the unskewed equivalent of the client point (identity
    /// when flat). Hit functions prepend this so their existing px math stays put.
    pub(super) fn unskew_px(&self, x: i32, y: i32) -> Option<(i32, i32)> {
        let (sx, sy) = self.slab_point(x, y)?;
        let s = self.scale();
        Some(((sx * s).round() as i32, (sy * s).round() as i32))
    }

    /// Client px -> slab-space px without the inside test: held drags (marquee, scroll thumb,
    /// tab reorder) keep moving while the pointer is past the parallelogram's edge, and their
    /// deltas must stay in the same space as the stored start points.
    pub(super) fn unskew_unbounded(&self, x_px: i32, y_px: i32) -> (i32, i32) {
        let s = self.scale();
        if !self.skew_active() {
            return (x_px, y_px);
        }
        let (sx, sy) = self.unskew(x_px as f32 / s, y_px as f32 / s);
        ((sx * s).round() as i32, (sy * s).round() as i32)
    }

    /// Y-shear inside the content surface: (m12, m32) of the surface-local transform
    /// `wy = cy + k·cx + pad`, where `pad` lifts the content under the higher end of
    /// the surface's x range (the surface's top sits there — see `layout_panels`).
    pub(super) fn content_shear(&self) -> (f32, f32) {
        if !self.skew_active() {
            return (0.0, 0.0);
        }
        let (w, _) = self.window_dips();
        let a = self.shear_at(self.rail_w() / w.max(1.0));
        let b = self.shear_at(1.0);
        let k = (self.skew_right - self.skew_left) / w.max(1.0);
        (k, (a - b).max(0.0))
    }

    /// Height the shear adds inside the content surface (DIPs): the difference of the
    /// edge offsets across the surface's x range — `layout_panels` reserves it.
    pub(super) fn content_shear_amp(&self) -> f32 {
        if !self.skew_active() {
            return 0.0;
        }
        let (w, _) = self.window_dips();
        (self.shear_at(self.rail_w() / w.max(1.0)) - self.shear_at(1.0)).abs()
    }

    /// The nearer edge is the deeper one — its top handle sits lower on the screen.
    pub(super) fn depth_near_left(&self) -> bool {
        self.skew_left > self.skew_right
    }

    /// Icon scale factor for a cell centred at slab x: the near edge stays at 1.0,
    /// the far edge shrinks to `1 - depth_scale`. Flat at 0 depth or no tilt.
    pub(super) fn depth_at(&self, slab_x: f32) -> f32 {
        if !self.skew_active() {
            return 1.0;
        }
        let (w, _) = self.window_dips();
        depth_factor(
            self.depth_near_left(),
            self.depth_scale,
            self.skew_spread(),
            slab_x / w.max(1.0),
        )
    }

    /// Which handle (if any) covers a window-space point in DIPs: a square zone on
    /// the top of each edge.
    pub(super) fn handle_at(&self, wx: f32, wy: f32) -> Option<SkewEdge> {
        if !self.skew_active() || self.locked || self.rolled_up {
            return None;
        }
        let (w, _) = self.window_dips();
        let in_y =
            |top: f32| wy >= (top - SHEAR_HANDLE_H / 2.0).max(0.0) && wy <= top + SHEAR_HANDLE_H;
        if wx < SHEAR_HANDLE_W && in_y(self.skew_left) {
            return Some(SkewEdge::Left);
        }
        if wx > w - SHEAR_HANDLE_W && in_y(self.skew_right) {
            return Some(SkewEdge::Right);
        }
        None
    }

    /// Live skew values for the config record (`set_fence_bounds` writes them).
    pub(super) fn skew_dips(&self) -> (f32, f32) {
        if self.shelf_3d {
            (self.skew_left, self.skew_right)
        } else {
            (0.0, 0.0)
        }
    }

    /// New edge offsets from the app (apply_fence_view); normalised so at least one
    /// edge sits at the window top and clamped to the deepest allowed tilt.
    pub(super) fn set_skew(&mut self, left: f32, right: f32) {
        let (left, right) = normalize_offsets(left, right);
        if (self.skew_left - left).abs() > f32::EPSILON
            || (self.skew_right - right).abs() > f32::EPSILON
        {
            self.skew_left = left;
            self.skew_right = right;
        }
    }

    /// One frame of a handle drag, given the pointer's screen Y: the dragged edge's
    /// top follows it, the other edge's screen position does not move, and the
    /// bounding box re-fits (top = higher edge, height = slab + spread). Updates the
    /// stored skew and returns the window's new top + height when they change.
    pub(super) fn move_skew_drag(&mut self, screen_y: i32) -> Option<(i32, i32)> {
        let Some(d) = &self.skew_drag else {
            return None;
        };
        let dy = screen_y - d.grab_y;
        if dy == 0 && !d.moved {
            return None;
        }
        let (edge, mut lt, mut rt, slab_h) = (d.edge, d.left_top, d.right_top, d.slab_h);
        let max_spread = (MAX_SKEW_DIP * self.scale()).round() as i32;
        match edge {
            // The dragged edge's top follows the pointer's cumulative delta; it may
            // rise above the other (the tilt flips) or sink below it, but the
            // spread — and so the window height — stays bounded.
            SkewEdge::Left => lt = (lt + dy).clamp(rt - max_spread, rt + max_spread),
            SkewEdge::Right => rt = (rt + dy).clamp(lt - max_spread, lt + max_spread),
        }
        if let Some(d) = &mut self.skew_drag {
            d.moved = true;
        }
        let top = lt.min(rt);
        let spread_px = (lt - rt).abs();
        let s = self.scale();
        self.skew_left = (lt - top) as f32 / s;
        self.skew_right = (rt - top) as f32 / s;
        let height = (slab_h * s).round() as i32 + spread_px;
        let r = window::window_rect(self.hwnd);
        (r.top != top || r.bottom - r.top != height).then_some((top, height))
    }

    /// Begins a handle drag (client px of the press): the edge tops in screen px
    /// and the frozen slab height the re-bounding works from.
    pub(super) fn begin_skew_drag(&mut self, edge: SkewEdge, screen_y: i32) {
        let r = window::window_rect(self.hwnd);
        let s = self.scale();
        self.skew_drag = Some(SkewDrag {
            edge,
            grab_y: screen_y,
            left_top: r.top + (self.skew_left * s).round() as i32,
            right_top: r.top + (self.skew_right * s).round() as i32,
            slab_h: self.slab_h(),
            moved: false,
        });
    }
}

/// Linear interpolation of the edge offsets across the slab width (the shear's
/// `y += offset` term at x fraction `xf`). Pure math — the unit tests below and
/// every caller share this one.
fn shear_offset_at(sl: f32, sr: f32, xf: f32) -> f32 {
    sl + (sr - sl) * xf.clamp(0.0, 1.0)
}

/// Progressive icon scale at a slab x fraction: `t` runs 0 at the nearer edge
/// (`near_left` picks which side that is) to 1 at the far one; the far edge ends
/// up at `1 - depth`. Identity (1.0) at zero depth or zero tilt — a flat fence
/// must look exactly like it did before.
fn depth_factor(near_left: bool, depth: f32, spread: f32, xf: f32) -> f32 {
    if depth <= 0.0 || spread <= 0.0 {
        return 1.0;
    }
    let xf = xf.clamp(0.0, 1.0);
    let t = if near_left { xf } else { 1.0 - xf };
    1.0 - depth * t
}

/// Normalizes edge offsets so the higher edge sits at 0 and clamps both to the
/// deepest allowed tilt.
fn normalize_offsets(left: f32, right: f32) -> (f32, f32) {
    let m = left.min(right);
    (
        (left - m).clamp(0.0, MAX_SKEW_DIP),
        (right - m).clamp(0.0, MAX_SKEW_DIP),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shear_roundtrip() {
        // Forward then inverse map returns the original slab y for any point.
        for (sl, sr) in [(0.0_f32, 0.0), (0.0, 40.0), (25.0, 0.0), (12.0, 87.0)] {
            for (w, x, y) in [
                (200.0_f32, 0.0, 30.0),
                (200.0, 100.0, 55.0),
                (317.0, 316.9, 0.0),
            ] {
                let wy = y + shear_offset_at(sl, sr, x / w);
                let back = wy - shear_offset_at(sl, sr, x / w);
                assert!((back - y).abs() < 1e-5, "roundtrip {sl}/{sr} x={x} y={y}");
            }
        }
    }

    #[test]
    fn depth_direction() {
        // Near edge full size, far edge shrunk by depth — whichever side is near.
        assert_eq!(depth_factor(true, 0.5, 40.0, 0.0), 1.0);
        assert_eq!(depth_factor(true, 0.5, 40.0, 1.0), 0.5);
        assert_eq!(depth_factor(false, 0.5, 40.0, 0.0), 0.5);
        assert_eq!(depth_factor(false, 0.5, 40.0, 1.0), 1.0);
        // Monotonic shrink toward the far edge.
        let near = depth_factor(true, 0.8, 40.0, 0.25);
        let far = depth_factor(true, 0.8, 40.0, 0.75);
        assert!(near > far);
    }

    #[test]
    fn depth_zero_is_identity() {
        // 0.0 must draw every icon the same size despite the skew (spec).
        for xf in [0.0_f32, 0.25, 0.5, 1.0] {
            assert_eq!(depth_factor(true, 0.0, 40.0, xf), 1.0);
            assert_eq!(depth_factor(false, 0.0, 40.0, xf), 1.0);
        }
    }

    #[test]
    fn flat_tilt_is_identity() {
        // No spread (edges level): no depth scaling, no matter the factor.
        for xf in [0.0_f32, 0.5, 1.0] {
            assert_eq!(depth_factor(true, 0.9, 0.0, xf), 1.0);
        }
    }

    #[test]
    fn normalize_offsets_lifts_the_higher_edge_to_zero() {
        assert_eq!(normalize_offsets(30.0, 40.0), (0.0, 10.0));
        assert_eq!(normalize_offsets(40.0, 30.0), (10.0, 0.0));
        assert_eq!(normalize_offsets(0.0, 0.0), (0.0, 0.0));
        // Negative input (an edge above the window top) re-normalizes too.
        assert_eq!(normalize_offsets(-20.0, 30.0), (0.0, 50.0));
        // And the deepest tilt is bounded.
        assert_eq!(normalize_offsets(0.0, MAX_SKEW_DIP + 50.0).1, MAX_SKEW_DIP);
    }
}
