//! What a screen rect shows behind a given window: the composed backdrop
//! the clock widget sits on, sampled without ever reading the widget's own
//! pixels. Walks the z-order below the widget for the first opaque window
//! covering the rect and `PrintWindow`s it; the desktop layer falls back
//! to the live-wallpaper player, then the static wallpaper picture, then
//! the desktop background colour.
//!
//! `sample` does real blocking work (`PrintWindow` waits on the foreign
//! window's thread, WIC decodes the wallpaper file) — call it off the UI
//! thread, with COM initialized (`com::MtaGuard`) for the wallpaper query.

use crate::HWND;
use crate::bindings::*;
use crate::desktop::{class_name, is_cloaked, is_visible};
use crate::live_wallpaper_still as still;
use crate::monitors::MonitorInfo;
use crate::wallpaper::{Position, decode_stretched, query};
use crate::window::window_rect;

/// The backdrop analysis for one sampled region.
#[derive(Clone, Copy, Debug)]
pub struct Sample {
    /// Rec.709 relative luminance of the averaged region (0..=255).
    pub luminance: u8,
    /// Dominant saturated colour of the region; `#38bdf8` when the patch
    /// is essentially grayscale.
    pub accent: [u8; 3],
}

const DEFAULT_ACCENT: [u8; 3] = [0x38, 0xbd, 0xf8];

fn area(r: RECT) -> i64 {
    (r.right - r.left).max(0) as i64 * (r.bottom - r.top).max(0) as i64
}

fn intersect(a: RECT, b: RECT) -> RECT {
    RECT {
        left: a.left.max(b.left),
        top: a.top.max(b.top),
        right: a.right.min(b.right),
        bottom: a.bottom.min(b.bottom),
    }
}

/// The first opaque window below `exclude` in z-order covering at least
/// ~60% of `rect`, `PrintWindow`ed whole; the crop is computed by the
/// caller from the returned frame rect. `None` when the desktop layer
/// itself is what shows (Progman/WorkerW reached, or nothing covers).
fn probe_under(exclude: HWND, rect: RECT) -> Option<(Vec<u8>, u32, u32, RECT)> {
    let mut w = unsafe { GetWindow(exclude, GW_HWNDNEXT as u32) };
    let need = area(rect) * 3 / 5;
    for _ in 0..64 {
        if w.0.is_null() {
            return None;
        }
        if is_visible(w) && !is_cloaked(w) {
            let class = class_name(w);
            if class == "Progman" || class == "WorkerW" {
                // The shell's own surface — wallpaper/icons, handled by
                // the wallpaper probe instead.
                return None;
            }
            if !class.starts_with("PecoFence.") && area(intersect(rect, window_rect(w))) >= need {
                let hit = still::capture(w, |p, ww, hh| (p.to_vec(), ww, hh));
                if let Some((frame, (px, ww, hh))) = hit {
                    if px.iter().any(|&b| b != 0) {
                        return Some((px, ww, hh, frame));
                    }
                    // PrintWindow gave back an empty frame (a player that
                    // renders nothing GDI-visible): keep walking z-order.
                }
            }
        }
        w = unsafe { GetWindow(w, GW_HWNDNEXT as u32) };
    }
    None
}

/// The wallpaper layer itself: a live player's window when one covers the
/// monitor, else the static picture decoded and laid out by the monitor's
/// position, else `None` (solid background colour only).
fn probe_wallpaper(rect: RECT, monitors: &[MonitorInfo]) -> Option<(Vec<u8>, u32, u32, RECT)> {
    let cx = (rect.left + rect.right) / 2;
    let cy = (rect.top + rect.bottom) / 2;
    let mon = monitors
        .iter()
        .find(|m| {
            let b = m.bounds;
            cx >= b.left && cx < b.right && cy >= b.top && cy < b.bottom
        })
        .or_else(|| monitors.iter().find(|m| m.primary))
        .or(monitors.first())?;

    if let Some((hwnd, _)) = still::covering(&still::windows(), mon.bounds)
        && let Some((frame, (px, ww, hh))) = still::capture(hwnd, |p, w, h| (p.to_vec(), w, h))
        && px.iter().any(|&b| b != 0)
    {
        return Some((px, ww, hh, frame));
    }

    let snap = query().ok()?;
    let m = snap
        .monitors
        .iter()
        .find(|m| {
            let r = m.rect;
            cx >= r.left && cx < r.right && cy >= r.top && cy < r.bottom
        })
        .or(snap.monitors.first())?;
    let path = m.path.as_deref()?;
    let (mw, mh) = (
        (m.rect.right - m.rect.left).max(1) as u32,
        (m.rect.bottom - m.rect.top).max(1) as u32,
    );
    // Tile/Center keep most of the frame on the desktop background colour;
    // the sampled patch is dominated by it, so decode at native scale and
    // let the crop land where it lands.
    let img = match snap.position {
        Position::Center | Position::Tile => {
            decode_stretched(path, mw.min(1920), mh.min(1920)).ok()
        }
        _ => decode_stretched(path, mw, mh).ok(),
    }?;
    let fw = img.width.max(1);
    let fh = img.height.max(1);
    let frame = RECT {
        left: m.rect.left,
        top: m.rect.top,
        right: m.rect.left + fw as i32,
        bottom: m.rect.top + fh as i32,
    };
    Some((img.bgra, fw, fh, frame))
}

/// The monitor's background colour as a one-pixel sample — the correct
/// answer when nothing but the desktop background shows.
fn probe_background(rect: RECT, monitors: &[MonitorInfo]) -> Sample {
    let snap = query().ok();
    let rgb = snap.as_ref().map(|s| s.background).unwrap_or([0, 0, 0]);
    let _ = (rect, monitors);
    Sample {
        luminance: luminance(rgb[0], rgb[1], rgb[2]),
        accent: DEFAULT_ACCENT,
    }
}

fn luminance(r: u8, g: u8, b: u8) -> u8 {
    (0.2126 * r as f32 + 0.7152 * g as f32 + 0.0722 * b as f32).round() as u8
}

/// Average the `rect`∩`frame` region of a BGRA buffer whose screen
/// footprint is `frame`; derive mean luminance and a saturated accent.
fn analyze(px: &[u8], w: u32, h: u32, frame: RECT, rect: RECT) -> Sample {
    let crop = intersect(rect, frame);
    if area(crop) == 0 {
        return Sample {
            luminance: 0,
            accent: DEFAULT_ACCENT,
        };
    }
    // The buffer is the window/decoded image rendered at `frame`'s size.
    let sx = w as f32 / (frame.right - frame.left).max(1) as f32;
    let sy = h as f32 / (frame.bottom - frame.top).max(1) as f32;
    let (x0, y0) = (
        ((crop.left - frame.left) as f32 * sx) as usize,
        ((crop.top - frame.top) as f32 * sy) as usize,
    );
    let (x1, y1) = (
        (((crop.right - frame.left) as f32 * sx) as usize).min(w as usize),
        (((crop.bottom - frame.top) as f32 * sy) as usize).min(h as usize),
    );
    if x1 <= x0 || y1 <= y0 {
        return Sample {
            luminance: 0,
            accent: DEFAULT_ACCENT,
        };
    }
    // Stride the probe for big regions: ~64 sample points per axis is
    // plenty for an average and keeps a 4K capture cheap to read.
    let (nx, ny) = ((x1 - x0).min(64), (y1 - y0).min(64));
    let (dx, dy) = (
        ((x1 - x0) / nx.max(1)).max(1),
        ((y1 - y0) / ny.max(1)).max(1),
    );
    let (mut lr, mut lg, mut lb, mut n) = (0f64, 0f64, 0f64, 0f64);
    let (mut ar, mut ag, mut ab, mut an) = (0f64, 0f64, 0f64, 0f64);
    for yy in (y0..y1).step_by(dy) {
        for xx in (x0..x1).step_by(dx) {
            let i = (yy * w as usize + xx) * 4;
            if i + 2 >= px.len() {
                continue;
            }
            let (b, g, r) = (px[i], px[i + 1], px[i + 2]);
            lr += r as f64;
            lg += g as f64;
            lb += b as f64;
            n += 1.0;
            let (mx, mn) = (r.max(g).max(b) as f64, r.min(g).min(b) as f64);
            let sat = if mx > 0.0 { (mx - mn) / mx } else { 0.0 };
            if sat > 0.35 && mx > 76.0 {
                ar += r as f64;
                ag += g as f64;
                ab += b as f64;
                an += 1.0;
            }
        }
    }
    if n == 0.0 {
        return Sample {
            luminance: 0,
            accent: DEFAULT_ACCENT,
        };
    }
    let accent = if an > 0.0 {
        [(ar / an) as u8, (ag / an) as u8, (ab / an) as u8]
    } else {
        DEFAULT_ACCENT
    };
    Sample {
        luminance: luminance((lr / n) as u8, (lg / n) as u8, (lb / n) as u8),
        accent,
    }
}

/// The backdrop under `rect` as `exclude` sees it (its own pixels are never
/// read). Off-UI-thread only — see the module docs.
pub fn sample(exclude: HWND, rect: RECT, monitors: &[MonitorInfo]) -> Sample {
    if let Some((px, w, h, frame)) = probe_under(exclude, rect) {
        return analyze(&px, w, h, frame, rect);
    }
    if let Some((px, w, h, frame)) = probe_wallpaper(rect, monitors) {
        return analyze(&px, w, h, frame, rect);
    }
    probe_background(rect, monitors)
}
