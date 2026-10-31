//! Adaptive Theme Engine — the pure-math core of cross-monitor `.peco` themes.
//!
//! A theme remembers the monitor it was saved on (`SourceCanvas`) and where the bundled
//! background asset was drawn on it. Fences additionally carry *canvas coordinates*:
//! their rectangle normalized to the drawn asset, so a fence parked on a visual feature
//! (a shelf painted into the wallpaper) lands on the same feature wherever the theme is
//! applied — a 21:9 ultrawide, a 3:2 tablet, a different DPI — provided the background
//! is laid out by [`adaptive_fit`], the engine's own placement rule:
//!
//! * `contain == false` → strict Cover: a uniform `max` scale fills the screen and
//!   the overflowing axis center-crops (the platform renders it — Windows "Fill"
//!   for images, the video blit for video). No distortion, no margins.
//! * `contain == true` → strict Contain: a uniform `min` scale keeps the whole
//!   frame centered and uncropped; the leftover margins are meant to be filled
//!   by a blurred copy of the asset (the "pillarbox").

use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

/// A 0..1 rectangle: `x`,`y` the top-left corner, `w`,`h` the size — relative to
/// whatever frame it was measured in (the drawn asset for canvas coordinates).
#[derive(Clone, Copy, Debug, Default, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase")]
pub struct NormRect {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
}

impl NormRect {
    /// `self` clamped into 0..1 — fences poking past the asset edge snap onto it.
    pub fn clamped(self) -> NormRect {
        let x = self.x.clamp(0.0, 1.0);
        let y = self.y.clamp(0.0, 1.0);
        NormRect {
            x,
            y,
            w: (self.x + self.w).clamp(0.0, 1.0) - x,
            h: (self.y + self.h).clamp(0.0, 1.0) - y,
        }
    }
}

/// Where the theme was saved: the source monitor's pixel size and scale factor, and the
/// rectangle the uncropped background asset occupied on it (monitor-relative px,
/// `adaptive_fit` basis — the engine's own coordinate space on both ends).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase")]
pub struct SourceCanvas {
    /// Monitor pixel size at capture (`{width, height}`, e.g. `[3840, 2160]`).
    pub resolution: [u32; 2],
    /// Windows text-scaling DPI at capture (96 = 100 %, 288 = 300 %).
    pub dpi: u32,
    /// `{left, top, right, bottom}` in monitor px: where `adaptive_fit` placed the
    /// uncropped asset on the source monitor — the frame canvas coordinates live in.
    pub bg_rect: [i32; 4],
    /// The bundled asset's own pixel size at capture (telemetry/fallback; the
    /// authoritative dimensions are re-probed from the bundled file on apply).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub asset: Option<[u32; 2]>,
}

/// Which frame the `canvas` coordinates are normalized against.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase")]
pub enum CanvasSpace {
    /// Legacy: normalized against the drawn screen rect (`bg_rect` for images, the
    /// full monitor for cover-cropped video). The fence keeps its *screen* position,
    /// not its media pixel — crops shift it off the wallpaper's visual features.
    #[default]
    Screen,
    /// Media-anchored: normalized against the raw asset's pixel space (0..1 over the
    /// media frame). Fences glue to media pixels, so shelves keep their fences across
    /// any aspect-ratio crop. Requires `source.asset` dims at apply time.
    Media,
}

impl CanvasSpace {
    fn is_default(&self) -> bool {
        *self == CanvasSpace::Screen
    }
}

/// `theme.json` inside a `.peco` archive: everything the adaptive engine needs on top
/// of `config.json`. Unknown to older readers — they extract it and move on.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ThemeMeta {
    /// Source monitor + drawn-asset rect at save time; `None` for themes saved without
    /// a usable background or monitor snapshot.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source: Option<SourceCanvas>,
    /// Fence id (UUID string) → that fence's rect normalized to the drawn asset.
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub canvas: BTreeMap<String, NormRect>,
    /// What the canvas coords are normalized against; `screen` (absent) for archives
    /// written before media-anchored mapping.
    #[serde(default, skip_serializing_if = "CanvasSpace::is_default")]
    pub canvas_space: CanvasSpace,
}

/// How the asset should be rendered on a monitor of `dst` px.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AspectFit {
    /// Strict Cover: a uniform `max`-axis scale fills the monitor, the overflowing
    /// axis center-crops. The platform renders the crop (Windows "Fill" for
    /// images, the video blit for video), so `bg` is the whole monitor.
    Cover,
    /// Strict Contain: a uniform `min`-axis scale keeps the whole asset centered
    /// and uncropped at `bg` (monitor-relative px, `{left, top, right, bottom}`);
    /// the leftover margins on one axis are to be filled by a blurred copy.
    Capped {
        bg: [i32; 4],
        /// The uniform scale applied to the asset (`{x, y}` — always equal).
        stretch: [f32; 2],
    },
}

/// The cover/contain decision for drawing a `src` px asset on a `dst` px monitor:
/// `contain` picks strict Contain, anything else is strict Cover — never any
/// anisotropic distortion.
pub fn adaptive_fit(src_w: u32, src_h: u32, dst_w: u32, dst_h: u32, contain: bool) -> AspectFit {
    if !contain || src_w == 0 || src_h == 0 || dst_w == 0 || dst_h == 0 {
        return AspectFit::Cover;
    }
    // Strict Contain: no distortion at all — the whole asset at the uniform
    // min-scale, centered; the leftover margins take the blurred backdrop.
    let s = (dst_w as f32 / src_w as f32).min(dst_h as f32 / src_h as f32);
    let bw = (src_w as f32 * s).round().min(dst_w as f32).max(1.0) as i32;
    let bh = (src_h as f32 * s).round().min(dst_h as f32).max(1.0) as i32;
    let left = ((dst_w as i32 - bw) / 2).max(0);
    let top = ((dst_h as i32 - bh) / 2).max(0);
    AspectFit::Capped {
        bg: [left, top, left + bw, top + bh],
        stretch: [s, s],
    }
}

/// The drawn-asset px rect of a fit (`Cover` covers the whole monitor).
pub fn bg_rect(fit: &AspectFit, dst_w: u32, dst_h: u32) -> [i32; 4] {
    match fit {
        AspectFit::Cover => [0, 0, dst_w as i32, dst_h as i32],
        AspectFit::Capped { bg, .. } => *bg,
    }
}

/// Fence px rect (`{l, t, r, b}`, monitor-relative) → normalized coordinates on the
/// asset drawn at `bg` (same space). Clamped: a fence hanging off the asset edge snaps
/// onto the edge rather than being dropped.
pub fn to_canvas(fence_px: [i32; 4], bg: [i32; 4]) -> NormRect {
    let bw = (bg[2] - bg[0]).max(1) as f32;
    let bh = (bg[3] - bg[1]).max(1) as f32;
    NormRect {
        x: (fence_px[0] - bg[0]) as f32 / bw,
        y: (fence_px[1] - bg[1]) as f32 / bh,
        w: (fence_px[2] - fence_px[0]) as f32 / bw,
        h: (fence_px[3] - fence_px[1]) as f32 / bh,
    }
    .clamped()
}

/// Inverse of [`to_canvas`]: normalized asset coordinates → monitor-relative px rect.
pub fn from_canvas(n: NormRect, bg: [i32; 4]) -> [i32; 4] {
    let bw = (bg[2] - bg[0]) as f32;
    let bh = (bg[3] - bg[1]) as f32;
    let n = n.clamped();
    let l = bg[0] + (n.x * bw).round() as i32;
    let t = bg[1] + (n.y * bh).round() as i32;
    [
        l,
        t,
        l + (n.w * bw).round() as i32,
        t + (n.h * bh).round() as i32,
    ]
}

/// Media→monitor affine map, one per axis: `screen_px = media_px * k + off`
/// (monitor-relative px). The media's own pixel space is the fence anchor — a shelf
/// painted into the wallpaper keeps its fences regardless of how the panel crops or
/// letterboxes the frame.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MediaMap {
    pub kx: f32,
    pub ky: f32,
    pub ox: f32,
    pub oy: f32,
}

/// The map for a contained asset (image, or letterboxed video): the whole `aw x ah`
/// frame is drawn at the contain `bg` rect, so media `0..aw` maps onto
/// `bg.left..bg.right`. Cover-mode callers use [`video_map`] instead — the center
/// crop is media-agnostic.
pub fn image_map(aw: u32, ah: u32, mw: u32, mh: u32) -> MediaMap {
    let [l, t, r, b] = bg_rect(&adaptive_fit(aw, ah, mw, mh, true), mw, mh);
    MediaMap {
        kx: (r - l) as f32 / aw.max(1) as f32,
        ky: (b - t) as f32 / ah.max(1) as f32,
        ox: l as f32,
        oy: t as f32,
    }
}

/// The map for a cover-cropped asset on a `mw x mh` monitor — the same center-crop
/// the video player applies and Windows "Fill" applies to images: uniform
/// `scale = max(mw/vw, mh/vh)`, then the overflowing axis crops symmetrically.
/// The visible source rect starts at `(sx, sy)` in media pixels.
pub fn video_map(vw: u32, vh: u32, mw: u32, mh: u32) -> MediaMap {
    let scale = (mw as f32 / vw.max(1) as f32).max(mh as f32 / vh.max(1) as f32);
    let sx = (vw as f32 - mw as f32 / scale) / 2.0;
    let sy = (vh as f32 - mh as f32 / scale) / 2.0;
    MediaMap {
        kx: scale,
        ky: scale,
        ox: -sx * scale,
        oy: -sy * scale,
    }
}

/// Fence px rect (`{l, t, r, b}`, monitor-relative) → normalized media coordinates
/// (0..1 over the asset's own `aw x ah` frame, independent of the screen it was drawn
/// on). Clamped like [`to_canvas`].
pub fn to_media_canvas(fence_px: [i32; 4], map: &MediaMap, aw: u32, ah: u32) -> NormRect {
    let (aw, ah) = (aw.max(1) as f32, ah.max(1) as f32);
    NormRect {
        x: (fence_px[0] as f32 - map.ox) / map.kx / aw,
        y: (fence_px[1] as f32 - map.oy) / map.ky / ah,
        w: (fence_px[2] - fence_px[0]) as f32 / map.kx / aw,
        h: (fence_px[3] - fence_px[1]) as f32 / map.ky / ah,
    }
    .clamped()
}

/// Inverse of [`to_media_canvas`]: normalized media coords → monitor-relative px rect
/// on the monitor this `map` was built for. May exceed the monitor when the fence's
/// media region is cropped out — callers clamp it onto the panel.
pub fn from_media_canvas(n: NormRect, map: &MediaMap, aw: u32, ah: u32) -> [i32; 4] {
    let (aw, ah) = (aw.max(1) as f32, ah.max(1) as f32);
    let n = n.clamped();
    let l = (n.x * aw * map.kx + map.ox).round() as i32;
    let t = (n.y * ah * map.ky + map.oy).round() as i32;
    [
        l,
        t,
        l + (n.w * aw * map.kx).round() as i32,
        t + (n.h * ah * map.ky).round() as i32,
    ]
}

/// `value` scaled by the DPI ratio `target_dpi / source_dpi` (grid layout parameters:
/// icon size, column/row spacing — so a theme authored at 100 % keeps its visual
/// proportions on a 300 % panel instead of reading microscopic).
pub fn dpi_scaled(value: f32, target_dpi: u32, source_dpi: u32) -> f32 {
    if source_dpi == 0 {
        return value;
    }
    value * target_dpi as f32 / source_dpi as f32
}

/// The `{Width}x{Height}_{Scale}%` suffix appended to a saved theme's name
/// (`MySetup_3840x2160_300%.peco`) so the library card shows its capture context.
pub fn source_suffix(source: &SourceCanvas) -> String {
    format!(
        "{}x{}_{}%",
        source.resolution[0],
        source.resolution[1],
        ((source.dpi as f32 / 96.0) * 100.0).round() as u32
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cover_fills_whole_monitor() {
        // Strict Cover: identical aspect, mild mismatch and extreme mismatch all
        // hand the platform the raw asset — `bg` is always the whole monitor.
        assert_eq!(
            adaptive_fit(1920, 1080, 2560, 1440, false),
            AspectFit::Cover
        );
        assert_eq!(adaptive_fit(1024, 768, 1280, 1024, false), AspectFit::Cover);
        assert_eq!(
            adaptive_fit(1920, 1080, 3440, 1440, false),
            AspectFit::Cover
        );
        assert_eq!(bg_rect(&AspectFit::Cover, 3440, 1440), [0, 0, 3440, 1440]);
    }

    #[test]
    fn contain_is_uniform_and_centered() {
        // 16:9 asset on a 21:9 target → strict Contain: uniform scale, full
        // height, symmetric side margins.
        let fit = adaptive_fit(1920, 1080, 3440, 1440, true);
        let AspectFit::Capped { bg, stretch } = fit else {
            panic!("expected contain fit, got {fit:?}");
        };
        // scale = min(3440/1920, 1440/1080) = min(1.792, 1.333) = 1.333 → 2560x1440.
        assert_eq!(bg[1], 0);
        assert_eq!(bg[3], 1440);
        assert_eq!(bg[2] - bg[0], 2560);
        assert_eq!(bg[0], (3440 - 2560) / 2); // margins split evenly on the wide axis
        assert_eq!(stretch[0], stretch[1]); // strictly uniform — zero distortion
    }

    #[test]
    fn tall_target_letterboxes() {
        // 21:9 asset on 16:9 → margins top/bottom.
        let fit = adaptive_fit(3440, 1440, 1920, 1080, true);
        let AspectFit::Capped { bg, .. } = fit else {
            panic!("expected contain fit, got {fit:?}");
        };
        assert_eq!(bg[0], 0);
        assert_eq!(bg[2], 1920);
        assert!(bg[1] > 0 && bg[3] < 1080);
    }

    #[test]
    fn contain_on_taller_monitor_centers_vertically() {
        // 16:9 media on a 3:2 monitor → uniform min scale, centered,
        // whole frame visible (no crop, no stretch).
        let fit = adaptive_fit(1280, 720, 2880, 1920, true);
        let AspectFit::Capped { bg, stretch } = fit else {
            panic!("expected contain fit, got {fit:?}");
        };
        // scale = min(2880/1280, 1920/720) = min(2.25, 2.66…) = 2.25 → 2880x1620.
        assert_eq!(bg, [0, 150, 2880, 1770]);
        assert_eq!(stretch[0], stretch[1]); // strictly uniform — zero distortion
        // Margins split evenly top/bottom.
        assert_eq!(bg[1], (1920 - (bg[3] - bg[1])) / 2);
    }

    #[test]
    fn contain_pillarboxes_on_wide_targets() {
        // 3:2 media on a 16:9 monitor → contain width limited, side bars.
        let fit = adaptive_fit(2880, 1920, 3840, 2160, true);
        let AspectFit::Capped { bg, stretch } = fit else {
            panic!("expected contain fit, got {fit:?}");
        };
        // scale = min(3840/2880, 2160/1920) = min(1.333, 1.125) = 1.125 → 3240x2160.
        assert_eq!(bg[1], 0);
        assert_eq!(bg[3], 2160);
        assert_eq!(bg[2] - bg[0], 3240);
        assert_eq!(bg[0], (3840 - 3240) / 2);
        assert_eq!(stretch[0], stretch[1]);
    }

    #[test]
    fn media_canvas_roundtrips_through_contain_rect() {
        // Media-anchored fences survive a contain placement: the contain bg_rect IS
        // the media→monitor map, so a normalized point lands back pixel-exact.
        let fit = adaptive_fit(1280, 720, 2880, 1920, true);
        let bg = bg_rect(&fit, 2880, 1920);
        let map = image_map(1280, 720, 2880, 1920);
        let n = to_media_canvas([600, 400, 1200, 900], &map, 1280, 720);
        assert_eq!(from_media_canvas(n, &map, 1280, 720), [600, 400, 1200, 900]);
        // And the same point through the plain canvas path (screen-anchored import):
        let n2 = to_canvas([600, 400, 1200, 900], bg);
        assert_eq!(from_canvas(n2, bg), [600, 400, 1200, 900]);
    }

    #[test]
    fn canvas_roundtrip() {
        let bg = [200, 0, 2560, 1440]; // asset drawn at 2360x1440 with side bars
        let n = to_canvas([700, 360, 1180, 720], bg);
        assert_eq!(from_canvas(n, bg), [700, 360, 1180, 720]);
    }

    #[test]
    fn canvas_clamps_outside_rects() {
        let bg = [0, 0, 1000, 1000];
        let n = to_canvas([-500, -100, 500, 500], bg);
        assert_eq!(n.x, 0.0);
        assert_eq!(n.y, 0.0);
        assert!((n.w - 0.5).abs() < 0.001 && (n.h - 0.5).abs() < 0.001);
    }

    /// 1280x720 video cover-cropped on a 3:2 2880x1920 Surface: the player crops to the
    /// center 1080 media px wide (sx=100), scale 8/3. A fence's normalized media coords
    /// must differ from its screen-normalized coords — and must reproduce the same media
    /// pixel on an uncropped 16:9 4K panel.
    #[test]
    fn media_anchor_survives_cover_crop() {
        let surf = video_map(1280, 720, 2880, 1920);
        // scale = 1920/720 = 2.667; visible media x ∈ [100, 1180]
        assert!((surf.kx - 8.0 / 3.0).abs() < 0.001);
        assert!((surf.ox - (-100.0 * 8.0 / 3.0)).abs() < 0.1);
        assert_eq!(surf.oy, 0.0);
        let n = to_media_canvas([500, 500, 1000, 900], &surf, 1280, 720);
        // screen-normalized would be 500/2880 = 0.1736; media-anchored is
        // (500 + 100*8/3) / (8/3) / 1280 = 287.5/1280 ≈ 0.2246
        assert!((n.x - 0.2246).abs() < 0.001, "n.x={}", n.x);
        // On the 16:9 4K TV the full frame shows: px = media * 3
        let tv = video_map(1280, 720, 3840, 2160);
        assert!((tv.kx - 3.0).abs() < 0.001 && (tv.ox - 0.0).abs() < 0.001);
        let px = from_media_canvas(n, &tv, 1280, 720);
        // media x 287.5 * 3 = 862.5 → 863 (NOT the proportional 667) — glued to media.
        assert_eq!(px[0], 863);
        // Same map on both ends → exact roundtrip.
        assert_eq!(
            from_media_canvas(n, &surf, 1280, 720),
            [500, 500, 1000, 900]
        );
    }

    #[test]
    fn image_map_is_bg_rect_anchored() {
        // Whole asset drawn at the contain rect: media-normalized == bg-normalized,
        // so media anchors coincide with the legacy canvas for contained images.
        let map = image_map(1920, 1080, 3440, 1440);
        let [l, t, r, b] = bg_rect(&adaptive_fit(1920, 1080, 3440, 1440, true), 3440, 1440);
        assert!((map.ox - l as f32).abs() < 0.001 && (map.oy - t as f32).abs() < 0.001);
        assert!((map.kx - (r - l) as f32 / 1920.0).abs() < 0.001);
        let n = to_media_canvas([700, 360, 1180, 720], &map, 1920, 1080);
        assert_eq!(
            from_media_canvas(n, &map, 1920, 1080),
            from_canvas(to_canvas([700, 360, 1180, 720], [l, t, r, b]), [l, t, r, b])
        );
    }

    /// Reproduction for the theme-load drift: a media-anchored fence is placed at an
    /// absolute monitor px rect, then `normalize` stores it work-area-relative with a
    /// picked `RightBottom`/`LeftBottom` anchor. When the work area later changes
    /// (taskbar style swap, auto-hide toggle — the wallpaper art never moves), the
    /// edge-gap anchor re-computes the fence off its media pixel.
    ///
    /// `canvas_anchor` must therefore pin each axis to the edge the taskbar does NOT
    /// reserve: the work-area gap against the reserved edge is what resizes.
    #[test]
    fn media_anchor_drifts_when_work_area_changes() {
        use crate::geometry::{PxRect, WorkArea, canvas_anchor, denormalize, normalize};
        use crate::model::Anchor;
        // `mk(l, t, r, b)` reserves px off each monitor edge (taskbar/app-bar).
        let mk = |l: i32, t: i32, r: i32, b: i32| WorkArea {
            device_path: "m".into(),
            gdi_name: String::new(),
            left: l,
            top: t,
            right: 3840 - r,
            bottom: 2160 - b,
            dpi: 288,
            mon_left: 0,
            mon_top: 0,
            mon_right: 3840,
            mon_bottom: 2160,
        };
        // Real saved values (theme 8-50, games fence): canvas -> monitor px.
        let n = NormRect {
            x: 0.7299479,
            y: 0.27314815,
            w: 0.23281252,
            h: 0.58009255,
        };
        let map = video_map(640, 360, 3840, 2160);
        let r = from_media_canvas(n, &map, 640, 360);
        let saved = PxRect {
            left: r[0],
            top: r[1],
            right: r[2],
            bottom: r[3],
        };
        assert_eq!((saved.left, saved.top), (2803, 590));

        // The picked anchor preserves the gap to the bottom edge: a 78px work-area
        // delta (the real 2064->1986 taskbar-style swap) slides the fence off the art.
        let g = normalize(saved, &mk(0, 0, 0, 96));
        assert_eq!(g.anchor, Anchor::RightBottom);
        let drifted = denormalize(&g, &mk(0, 0, 0, 174));
        assert_eq!(saved.top - drifted.top, 78, "edge-gap anchor re-anchored");

        // For every docked edge the dynamic anchor picks the opposite edge(s), and
        // the fence holds its absolute monitor position when that edge resizes.
        // (right-edge growth is capped at 30px so the fence still fits the work area —
        // `denormalize` clamps; the drift mechanism is identical.)
        for (reserved, want, grown) in [
            (mk(0, 0, 0, 96), Anchor::RightTop, mk(0, 0, 0, 174)), // bottom -> top
            (mk(0, 96, 0, 0), Anchor::RightBottom, mk(0, 174, 0, 0)), // top -> bottom
            (mk(96, 0, 0, 0), Anchor::RightBottom, mk(174, 0, 0, 0)), // left -> right
            (mk(0, 0, 96, 0), Anchor::LeftBottom, mk(0, 0, 30, 0)), // right -> left
            (mk(0, 0, 0, 0), Anchor::RightBottom, mk(0, 0, 0, 0)), // none -> picked
        ] {
            let mut g = normalize(saved, &reserved);
            g.anchor = canvas_anchor(&g, &reserved);
            assert_eq!(g.anchor, want);
            let held = denormalize(&g, &grown);
            assert_eq!(
                (held.left, held.top),
                (saved.left, saved.top),
                "anchor {want:?} must hold the media pixel"
            );
        }
    }

    #[test]
    fn canvas_space_defaults_to_screen() {
        // Archives written before media anchoring keep the legacy interpretation.
        let meta: ThemeMeta =
            serde_json::from_str("{\"canvas\":{\"a\":{\"x\":0.1,\"y\":0.2,\"w\":0.3,\"h\":0.4}}}")
                .unwrap();
        assert_eq!(meta.canvas_space, CanvasSpace::Screen);
    }

    #[test]
    fn dpi_ratio_scales() {
        assert_eq!(dpi_scaled(48.0, 192, 96), 96.0);
        assert_eq!(dpi_scaled(48.0, 96, 96), 48.0);
        assert_eq!(dpi_scaled(48.0, 96, 0), 48.0);
    }

    #[test]
    fn suffix_formats() {
        let s = SourceCanvas {
            resolution: [3840, 2160],
            dpi: 288,
            bg_rect: [0, 0, 3840, 2160],
            asset: None,
        };
        assert_eq!(source_suffix(&s), "3840x2160_300%");
    }
}
