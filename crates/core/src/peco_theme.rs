//! `.peco` Custom Theme Archive packer/unpacker and layout preview renderer.
//!
//! A `.peco` file is an uncompressed (`STORE`, method = 0) ZIP archive containing:
//! - `config.json`: all current fences, exact coordinates, sizes, and visual settings.
//! - `background.<ext>`: the background media file (image or H.264 `.mp4` video).
//! - `theme.json`: adaptive-engine metadata (source monitor canvas, per-fence asset
//!   coordinates) — absent in archives written before the engine existed.
//! - `preview.jpg`: a generated thumbnail of the desktop layout.

use crate::model::{Config, ThemeStyle};
use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use uuid::Uuid;

const LOCAL_FILE_HEADER_SIG: u32 = 0x0403_4b50;
const CENTRAL_DIR_HEADER_SIG: u32 = 0x0201_4b50;
const END_OF_CENTRAL_DIR_SIG: u32 = 0x0605_4b50;
const UTF8_FLAG: u16 = 0x0800;
const STORE_METHOD: u16 = 0;

/// Maximum `.peco` archive size accepted on import (512 MB, accommodating high-bitrate MP4 loops).
const MAX_PECO_ARCHIVE_BYTES: u64 = 512 * 1024 * 1024;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PecoArchiveEntry {
    pub name: String,
    pub data: Vec<u8>,
}

#[derive(Clone, Debug)]
pub struct ExtractedPecoTheme {
    pub temp_dir: PathBuf,
    pub config_path: PathBuf,
    pub background_path: Option<PathBuf>,
    pub preview_path: Option<PathBuf>,
    /// `theme.json`, when the archive carries adaptive-engine metadata.
    pub meta: Option<crate::adaptive::ThemeMeta>,
}

/// Standard IEEE 802.3 CRC-32 used by ZIP headers.
pub fn crc32(bytes: &[u8]) -> u32 {
    let mut crc = 0xFFFF_FFFFu32;
    for &b in bytes {
        crc ^= b as u32;
        for _ in 0..8 {
            let mask = (crc & 1).wrapping_neg();
            crc = (crc >> 1) ^ (0xEDB8_8320 & mask);
        }
    }
    !crc
}

/// Packs `entries` `(filename, data)` into a standard uncompressed (`STORE`, method 0) ZIP archive.
pub fn pack_uncompressed_zip(entries: &[(&str, &[u8])]) -> Vec<u8> {
    let total_payload: usize = entries.iter().map(|(n, d)| n.len() + d.len() + 96).sum();
    let mut out = Vec::with_capacity(total_payload + 64);
    let mut central_dir = Vec::with_capacity(entries.len() * 64);

    for &(name, data) in entries {
        let name_bytes = name.as_bytes();
        let offset = out.len() as u32;
        let crc = crc32(data);
        let size = data.len() as u32;
        let name_len = name_bytes.len() as u16;

        // Local file header (30 bytes + name)
        out.extend_from_slice(&LOCAL_FILE_HEADER_SIG.to_le_bytes());
        out.extend_from_slice(&20u16.to_le_bytes()); // version needed (2.0)
        out.extend_from_slice(&UTF8_FLAG.to_le_bytes());
        out.extend_from_slice(&STORE_METHOD.to_le_bytes());
        out.extend_from_slice(&0u16.to_le_bytes()); // mod time
        out.extend_from_slice(&0u16.to_le_bytes()); // mod date
        out.extend_from_slice(&crc.to_le_bytes());
        out.extend_from_slice(&size.to_le_bytes()); // compressed size
        out.extend_from_slice(&size.to_le_bytes()); // uncompressed size
        out.extend_from_slice(&name_len.to_le_bytes());
        out.extend_from_slice(&0u16.to_le_bytes()); // extra field length
        out.extend_from_slice(name_bytes);
        out.extend_from_slice(data);

        // Central directory file header (46 bytes + name)
        central_dir.extend_from_slice(&CENTRAL_DIR_HEADER_SIG.to_le_bytes());
        central_dir.extend_from_slice(&20u16.to_le_bytes()); // version made by
        central_dir.extend_from_slice(&20u16.to_le_bytes()); // version needed
        central_dir.extend_from_slice(&UTF8_FLAG.to_le_bytes());
        central_dir.extend_from_slice(&STORE_METHOD.to_le_bytes());
        central_dir.extend_from_slice(&0u16.to_le_bytes()); // mod time
        central_dir.extend_from_slice(&0u16.to_le_bytes()); // mod date
        central_dir.extend_from_slice(&crc.to_le_bytes());
        central_dir.extend_from_slice(&size.to_le_bytes());
        central_dir.extend_from_slice(&size.to_le_bytes());
        central_dir.extend_from_slice(&name_len.to_le_bytes());
        central_dir.extend_from_slice(&0u16.to_le_bytes()); // extra len
        central_dir.extend_from_slice(&0u16.to_le_bytes()); // comment len
        central_dir.extend_from_slice(&0u16.to_le_bytes()); // disk number
        central_dir.extend_from_slice(&0u16.to_le_bytes()); // internal attrs
        central_dir.extend_from_slice(&0u32.to_le_bytes()); // external attrs
        central_dir.extend_from_slice(&offset.to_le_bytes());
        central_dir.extend_from_slice(name_bytes);
    }

    let cd_offset = out.len() as u32;
    let cd_size = central_dir.len() as u32;
    let count = entries.len() as u16;
    out.extend_from_slice(&central_dir);

    // End of central directory record (22 bytes)
    out.extend_from_slice(&END_OF_CENTRAL_DIR_SIG.to_le_bytes());
    out.extend_from_slice(&0u16.to_le_bytes()); // disk number
    out.extend_from_slice(&0u16.to_le_bytes()); // cd start disk
    out.extend_from_slice(&count.to_le_bytes());
    out.extend_from_slice(&count.to_le_bytes());
    out.extend_from_slice(&cd_size.to_le_bytes());
    out.extend_from_slice(&cd_offset.to_le_bytes());
    out.extend_from_slice(&0u16.to_le_bytes()); // comment len

    out
}

fn read_u16(buf: &[u8], off: usize) -> Option<u16> {
    let b = buf.get(off..off + 2)?;
    Some(u16::from_le_bytes([b[0], b[1]]))
}

fn read_u32(buf: &[u8], off: usize) -> Option<u32> {
    let b = buf.get(off..off + 4)?;
    Some(u32::from_le_bytes([b[0], b[1], b[2], b[3]]))
}

fn sanitize_entry_name(raw: &str) -> Option<String> {
    let file_name = raw.rsplit(['/', '\\']).next()?.trim();
    if file_name.is_empty()
        || file_name == "."
        || file_name == ".."
        || file_name.contains(':')
        || file_name.chars().any(|c| c.is_control())
    {
        return None;
    }
    Some(file_name.to_string())
}

/// Unpacks an uncompressed (`STORE`, method 0) ZIP archive into memory entries.
pub fn unpack_uncompressed_zip(archive: &[u8]) -> Result<Vec<PecoArchiveEntry>, String> {
    if archive.len() < 22 {
        return Err("invalid .peco archive (too short)".into());
    }
    // Locate End of Central Directory record near the end of the file.
    let search_from = archive.len().saturating_sub(65_557);
    let mut eocd_pos = None;
    for pos in (search_from..=archive.len() - 22).rev() {
        if read_u32(archive, pos) == Some(END_OF_CENTRAL_DIR_SIG) {
            eocd_pos = Some(pos);
            break;
        }
    }
    let eocd =
        eocd_pos.ok_or_else(|| "invalid .peco archive (missing central directory)".to_string())?;
    let entry_count = read_u16(archive, eocd + 10).ok_or("corrupt EOCD")? as usize;
    let cd_offset = read_u32(archive, eocd + 16).ok_or("corrupt EOCD")? as usize;

    let mut entries = Vec::with_capacity(entry_count.min(32));
    let mut cursor = cd_offset;
    for _ in 0..entry_count {
        if read_u32(archive, cursor) != Some(CENTRAL_DIR_HEADER_SIG) {
            return Err("corrupt central directory header in .peco archive".into());
        }
        let method = read_u16(archive, cursor + 10).ok_or("corrupt CDH")?;
        if method != STORE_METHOD {
            return Err(
                "unsupported compression method in .peco archive (expected uncompressed STORE)"
                    .into(),
            );
        }
        let expected_crc = read_u32(archive, cursor + 16).ok_or("corrupt CDH")?;
        let comp_size = read_u32(archive, cursor + 20).ok_or("corrupt CDH")? as usize;
        let uncomp_size = read_u32(archive, cursor + 24).ok_or("corrupt CDH")? as usize;
        let name_len = read_u16(archive, cursor + 28).ok_or("corrupt CDH")? as usize;
        let extra_len = read_u16(archive, cursor + 30).ok_or("corrupt CDH")? as usize;
        let comment_len = read_u16(archive, cursor + 32).ok_or("corrupt CDH")? as usize;
        let local_offset = read_u32(archive, cursor + 42).ok_or("corrupt CDH")? as usize;

        if comp_size != uncomp_size {
            return Err("mismatched size in uncompressed .peco entry".into());
        }
        let name_slice = archive
            .get(cursor + 46..cursor + 46 + name_len)
            .ok_or("truncated entry name")?;
        let raw_name = std::str::from_utf8(name_slice).map_err(|_| "non-UTF8 entry name")?;
        cursor = cursor
            .checked_add(46 + name_len + extra_len + comment_len)
            .ok_or("overflow in CDH")?;

        let Some(clean_name) = sanitize_entry_name(raw_name) else {
            continue;
        };

        if read_u32(archive, local_offset) != Some(LOCAL_FILE_HEADER_SIG) {
            return Err("corrupt local file header in .peco archive".into());
        }
        let lfh_name_len = read_u16(archive, local_offset + 26).ok_or("corrupt LFH")? as usize;
        let lfh_extra_len = read_u16(archive, local_offset + 28).ok_or("corrupt LFH")? as usize;
        let data_start = local_offset
            .checked_add(30 + lfh_name_len + lfh_extra_len)
            .ok_or("overflow in LFH")?;
        let data_end = data_start
            .checked_add(uncomp_size)
            .ok_or("overflow in entry data")?;
        let data = archive
            .get(data_start..data_end)
            .ok_or("truncated entry payload in .peco archive")?;

        if crc32(data) != expected_crc {
            return Err(format!("CRC mismatch in .peco entry {clean_name}"));
        }

        entries.push(PecoArchiveEntry {
            name: clean_name,
            data: data.to_vec(),
        });
    }

    Ok(entries)
}

/// Writes a `.peco` theme archive containing `config.json`, optional `theme.json`,
/// optional `background.<ext>`, and `preview.jpg`.
pub fn write_peco_file(
    path: &Path,
    config_json: &str,
    meta_json: Option<&str>,
    background: Option<(&str, &[u8])>,
    preview_jpg: &[u8],
) -> io::Result<()> {
    let mut entries: Vec<(&str, &[u8])> = Vec::with_capacity(4);
    entries.push(("config.json", config_json.as_bytes()));
    if let Some(meta) = meta_json {
        entries.push(("theme.json", meta.as_bytes()));
    }
    if let Some((bg_name, bg_bytes)) = background {
        entries.push((bg_name, bg_bytes));
    }
    entries.push(("preview.jpg", preview_jpg));
    let zip_bytes = pack_uncompressed_zip(&entries);
    fs::write(path, zip_bytes)
}

/// Reads `src` into a `background.<ext>` archive entry, and prunes `cfg`'s
/// `video_wallpaper_path` unless the bundled media is a video: a theme exported over a
/// static image must not resurrect a stale video wallpaper on import. `None` when `src`
/// cannot be read.
pub fn background_entry(cfg: &mut Config, src: &Path) -> Option<(String, Vec<u8>)> {
    let ext = src
        .extension()
        .and_then(|e| e.to_str())
        .map(|e| e.to_ascii_lowercase())
        .filter(|e| {
            matches!(
                e.as_str(),
                "mp4" | "jpg" | "jpeg" | "png" | "bmp" | "webp" | "gif" | "tif" | "tiff"
            )
        })
        .unwrap_or_else(|| "jpg".into());
    if ext != "mp4" {
        if let Some(stale) = cfg.settings.video_wallpaper_path.take() {
            // A still-configured video path reaching an image bundle means a caller
            // exported without reconciling the player — the path is dropped here,
            // but the leak is loud so the caller can be traced and fixed.
            tracing::error!(
                stale = %stale,
                image = %src.display(),
                "stale video_wallpaper_path leaked into an image bundle; dropped"
            );
        }
    } else {
        tracing::info!(video = %src.display(), "bundling video wallpaper asset");
    }
    std::fs::read(src)
        .ok()
        .map(|bytes| (format!("background.{ext}"), bytes))
}

fn is_background_media_name(name: &str) -> bool {
    let lower = name.to_ascii_lowercase();
    if lower == "config.json" || lower == "preview.jpg" || lower == "preview.jpeg" {
        return false;
    }
    matches!(
        Path::new(&lower).extension().and_then(|e| e.to_str()),
        Some(
            "mp4"
                | "jpg"
                | "jpeg"
                | "png"
                | "bmp"
                | "webp"
                | "gif"
                | "tif"
                | "tiff"
                | "heic"
                | "avif"
        )
    )
}

/// Largest `preview.jpg` payload accepted from a `.peco` (the generated previews are ~40 KB;
/// this only stops a malformed archive from making the gallery read megabytes).
const MAX_PREVIEW_BYTES: u64 = 16 * 1024 * 1024;

/// Reads just the `preview.jpg`/`preview.jpeg` entry out of a `.peco` archive. The archive's
/// background video can be hundreds of MB, so only the end-of-central-directory tail and the
/// one entry's byte range are read — never the whole file.
pub fn read_peco_preview(path: &Path) -> Option<Vec<u8>> {
    use std::io::{Read, Seek, SeekFrom};
    let mut f = fs::File::open(path).ok()?;
    let len = f.metadata().ok()?.len();
    if len < 22 {
        return None;
    }
    let tail_len = len.min(65_557 + 22);
    f.seek(SeekFrom::End(-(tail_len as i64))).ok()?;
    let mut tail = vec![0u8; tail_len as usize];
    f.read_exact(&mut tail).ok()?;
    let mut eocd = None;
    for pos in (0..=tail.len() - 22).rev() {
        if read_u32(&tail, pos) == Some(END_OF_CENTRAL_DIR_SIG) {
            eocd = Some(pos);
            break;
        }
    }
    let eocd = eocd?;
    let entry_count = read_u16(&tail, eocd + 10)? as usize;
    let mut cursor = read_u32(&tail, eocd + 16)? as u64;

    let mut hdr = [0u8; 46];
    for _ in 0..entry_count.min(4096) {
        f.seek(SeekFrom::Start(cursor)).ok()?;
        f.read_exact(&mut hdr).ok()?;
        if u32::from_le_bytes(hdr[0..4].try_into().ok()?) != CENTRAL_DIR_HEADER_SIG {
            return None;
        }
        let method = u16::from_le_bytes(hdr[10..12].try_into().ok()?);
        if method != STORE_METHOD {
            return None;
        }
        let size = u32::from_le_bytes(hdr[20..24].try_into().ok()?) as u64;
        let name_len = u16::from_le_bytes(hdr[28..30].try_into().ok()?) as u64;
        let extra_len = u16::from_le_bytes(hdr[30..32].try_into().ok()?) as u64;
        let comment_len = u16::from_le_bytes(hdr[32..34].try_into().ok()?) as u64;
        let local_offset = u32::from_le_bytes(hdr[42..46].try_into().ok()?) as u64;
        let mut name = vec![0u8; name_len as usize];
        f.read_exact(&mut name).ok()?;
        cursor = cursor.checked_add(46 + name_len + extra_len + comment_len)?;

        let Ok(raw_name) = std::str::from_utf8(&name) else {
            continue;
        };
        let Some(file_name) = sanitize_entry_name(raw_name) else {
            continue;
        };
        let lower = file_name.to_ascii_lowercase();
        if lower != "preview.jpg" && lower != "preview.jpeg" {
            continue;
        }
        if size > MAX_PREVIEW_BYTES {
            return None;
        }
        // The local header carries its own name/extra lengths before the payload.
        f.seek(SeekFrom::Start(local_offset)).ok()?;
        let mut lfh = [0u8; 30];
        f.read_exact(&mut lfh).ok()?;
        if u32::from_le_bytes(lfh[0..4].try_into().ok()?) != LOCAL_FILE_HEADER_SIG {
            return None;
        }
        let l_name_len = u16::from_le_bytes(lfh[26..28].try_into().ok()?) as i64;
        let l_extra_len = u16::from_le_bytes(lfh[28..30].try_into().ok()?) as i64;
        f.seek(SeekFrom::Current(l_name_len + l_extra_len)).ok()?;
        let mut data = vec![0u8; size as usize];
        f.read_exact(&mut data).ok()?;
        return Some(data);
    }
    None
}

/// Extracts a `.peco` archive into a fresh temporary directory and returns the extracted paths.
pub fn extract_peco_to_temp(peco_path: &Path) -> Result<ExtractedPecoTheme, String> {
    let meta = fs::metadata(peco_path).map_err(|e| e.to_string())?;
    if meta.len() > MAX_PECO_ARCHIVE_BYTES {
        return Err("theme file exceeds maximum supported size".into());
    }
    let bytes = fs::read(peco_path).map_err(|e| e.to_string())?;
    let entries = unpack_uncompressed_zip(&bytes)?;

    let temp_dir = std::env::temp_dir().join(format!("pecofence-theme-{}", Uuid::new_v4()));
    fs::create_dir_all(&temp_dir).map_err(|e| e.to_string())?;

    let mut config_path = None;
    let mut background_path = None;
    let mut preview_path = None;
    let mut meta = None;

    for entry in entries {
        let lower = entry.name.to_ascii_lowercase();
        // theme.json is parsed from memory — nothing on disk needs it.
        if lower == "theme.json" {
            meta = serde_json::from_slice(&entry.data).ok();
            continue;
        }
        let dest = temp_dir.join(&entry.name);
        fs::write(&dest, &entry.data).map_err(|e| e.to_string())?;
        if lower == "config.json" {
            config_path = Some(dest);
        } else if lower == "preview.jpg" || lower == "preview.jpeg" {
            preview_path = Some(dest);
        } else if background_path.is_none() && is_background_media_name(&entry.name) {
            background_path = Some(dest);
        }
    }

    let Some(config_path) = config_path else {
        let _ = fs::remove_dir_all(&temp_dir);
        return Err("missing config.json in .peco theme archive".into());
    };

    Ok(ExtractedPecoTheme {
        temp_dir,
        config_path,
        background_path,
        preview_path,
        meta,
    })
}

/// Renders a `width x height` (e.g. 640x360) BGRA thumbnail of the desktop layout for `preview.jpg`.
pub fn render_layout_preview_bgra(
    cfg: &Config,
    bg: Option<(u32, u32, &[u8])>,
    width: u32,
    height: u32,
) -> Vec<u8> {
    let w = width.max(16) as usize;
    let h = height.max(16) as usize;
    let mut canvas = vec![0u8; w * h * 4];

    // 1. Fill background from wallpaper/video frame (cover-scaled) or a desktop gradient.
    if let Some((bw, bh, bpx)) = bg
        && bw > 0
        && bh > 0
        && bpx.len() >= (bw as usize) * (bh as usize) * 4
    {
        let sw = bw as usize;
        let sh = bh as usize;
        let (cx, cy, cw, ch) = if sw * h > w * sh {
            let cw = (sh * w / h).max(1).min(sw);
            ((sw - cw) / 2, 0, cw, sh)
        } else {
            let ch = (sw * h / w).max(1).min(sh);
            (0, (sh - ch) / 2, sw, ch)
        };
        for y in 0..h {
            let sy = (cy + y * ch / h).min(sh - 1);
            for x in 0..w {
                let sx = (cx + x * cw / w).min(sw - 1);
                let s = (sy * sw + sx) * 4;
                let d = (y * w + x) * 4;
                canvas[d] = bpx[s];
                canvas[d + 1] = bpx[s + 1];
                canvas[d + 2] = bpx[s + 2];
                canvas[d + 3] = 255;
            }
        }
    } else {
        for y in 0..h {
            for x in 0..w {
                let d = (y * w + x) * 4;
                let fx = x as f32 / w as f32;
                let fy = y as f32 / h as f32;
                canvas[d] = (68.0 + 52.0 * (1.0 - fy) + 24.0 * fx) as u8;
                canvas[d + 1] = (46.0 + 38.0 * (1.0 - fy)) as u8;
                canvas[d + 2] = (32.0 + 28.0 * fx) as u8;
                canvas[d + 3] = 255;
            }
        }
    }

    // 2. Overlay current layout's fences at their normalized positions.
    let layout_idx = cfg.last_shown().unwrap_or(0);
    let Some(layout) = cfg.layouts.get(layout_idx).or_else(|| cfg.layouts.first()) else {
        return canvas;
    };

    let style = cfg.settings.theme_style;
    for fence in &layout.fences {
        if fence.tab_host.is_some() {
            continue;
        }
        let g = &fence.geometry;
        let ww = g.work_w.max(640.0);
        let wh = g.work_h.max(360.0);
        let fx = g.x;
        let fy = g.y;
        let fh = if fence.rolled_up { 32.0 } else { g.h };

        let x0 = ((fx / ww) * w as f32).round() as i32;
        let y0 = ((fy / wh) * h as f32).round() as i32;
        let fw_px = ((g.w / ww) * w as f32).round().max(24.0) as i32;
        let fh_px = ((fh / wh) * h as f32).round().max(14.0) as i32;
        let x1 = x0 + fw_px;
        let y1 = y0 + fh_px;

        let (skew_l, skew_r) = if fence.view.shelf_3d() {
            (
                ((g.skew_left.unwrap_or(0.0) / wh) * h as f32).round() as i32,
                ((g.skew_right.unwrap_or(0.0) / wh) * h as f32).round() as i32,
            )
        } else {
            (0, 0)
        };

        let tint = fence
            .appearance
            .as_ref()
            .and_then(|a| a.tint_rgb)
            .unwrap_or(match style {
                ThemeStyle::Bookshelf => [0x5C, 0x40, 0x33],
                ThemeStyle::LiquidGlass => [0x38, 0x4A, 0x68],
                ThemeStyle::Fluent | ThemeStyle::InvisibleCanvas => [0x24, 0x28, 0x32],
            });
        let alpha: u32 = match style {
            ThemeStyle::InvisibleCanvas => 35,
            ThemeStyle::Bookshelf => 225,
            ThemeStyle::LiquidGlass => 135,
            ThemeStyle::Fluent => 175,
        };
        let title_h = 10.min(fh_px);

        for px_x in x0.max(0)..x1.min(w as i32) {
            let t = if fw_px > 1 {
                (px_x - x0) as f32 / (fw_px - 1) as f32
            } else {
                0.0
            };
            let dy = (skew_l as f32 * (1.0 - t) + skew_r as f32 * t).round() as i32;
            let col_top = y0 + dy;
            let col_bot = y1 + dy;
            for px_y in col_top.max(0)..col_bot.min(h as i32) {
                let d = (px_y as usize * w + px_x as usize) * 4;
                let border = px_x == x0 || px_x == x1 - 1 || px_y == col_top || px_y == col_bot - 1;
                let in_title = px_y < col_top + title_h;
                if border {
                    blend_pixel(&mut canvas[d..d + 4], [235, 240, 250], 170);
                } else if in_title {
                    blend_pixel(&mut canvas[d..d + 4], tint, (alpha + 40).min(240));
                } else {
                    blend_pixel(&mut canvas[d..d + 4], tint, alpha);
                }
            }
        }
    }

    canvas
}

fn blend_pixel(dst: &mut [u8], rgb: [u8; 3], alpha: u32) {
    let inv = 255 - alpha;
    dst[0] = ((rgb[2] as u32 * alpha + dst[0] as u32 * inv) / 255) as u8;
    dst[1] = ((rgb[1] as u32 * alpha + dst[1] as u32 * inv) / 255) as u8;
    dst[2] = ((rgb[0] as u32 * alpha + dst[2] as u32 * inv) / 255) as u8;
    dst[3] = 255;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn read_peco_preview_extracts_only_the_jpeg_entry() {
        let cfg_json = r#"{"schemaVersion":1}"#;
        // A multi-megabyte background proves the reader does not depend on archive size.
        let big_media = vec![0xABu8; 3 * 1024 * 1024];
        let jpg = b"\xFF\xD8\xFF\xE0the-preview-bytes\xFF\xD9";
        let packed = pack_uncompressed_zip(&[
            ("config.json", cfg_json.as_bytes()),
            ("background.mp4", &big_media),
            ("preview.jpg", jpg),
        ]);
        let dir = std::env::temp_dir().join(format!("peco-preview-test-{}", Uuid::new_v4()));
        fs::create_dir_all(&dir).unwrap();
        let peco = dir.join("theme.peco");
        fs::write(&peco, &packed).unwrap();
        let preview = read_peco_preview(&peco).expect("preview entry");
        assert_eq!(preview, jpg);
        let _ = fs::remove_dir_all(&dir);
    }

    /// Adaptive metadata: `theme.json` survives write → extract, and archives without it
    /// (everything written before the engine existed) read `meta == None` — the apply path
    /// then falls back to remapped geometry.
    #[test]
    fn theme_meta_roundtrips_and_legacy_reads_none() {
        let cfg_json = r#"{"schemaVersion":1}"#;
        let meta = crate::adaptive::ThemeMeta {
            source: Some(crate::adaptive::SourceCanvas {
                resolution: [3840, 2160],
                dpi: 192,
                bg_rect: [0, 0, 3840, 2160],
                asset: Some([1920, 1080]),
            }),
            canvas: [(
                "fence-1".to_string(),
                crate::adaptive::NormRect {
                    x: 0.25,
                    y: 0.5,
                    w: 0.5,
                    h: 0.25,
                },
            )]
            .into_iter()
            .collect(),
            canvas_space: crate::adaptive::CanvasSpace::Media,
        };
        let meta_json = serde_json::to_string(&meta).unwrap();
        let dir = std::env::temp_dir().join(format!("peco-meta-test-{}", Uuid::new_v4()));
        fs::create_dir_all(&dir).unwrap();

        let with_meta = dir.join("with-meta.peco");
        write_peco_file(
            &with_meta,
            cfg_json,
            Some(&meta_json),
            None,
            b"\xFF\xD8\xFF\xE0p",
        )
        .unwrap();
        let ex = extract_peco_to_temp(&with_meta).unwrap();
        let got = ex.meta.expect("theme.json parsed");
        assert_eq!(got.source.as_ref().unwrap().resolution, [3840, 2160]);
        assert_eq!(got.source.as_ref().unwrap().dpi, 192);
        assert_eq!(got.canvas["fence-1"].w, 0.5);
        let _ = fs::remove_dir_all(&ex.temp_dir);

        let legacy = dir.join("legacy.peco");
        write_peco_file(&legacy, cfg_json, None, None, b"\xFF\xD8\xFF\xE0p").unwrap();
        let ex = extract_peco_to_temp(&legacy).unwrap();
        assert!(ex.meta.is_none());
        let _ = fs::remove_dir_all(&ex.temp_dir);
        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn uncompressed_zip_roundtrips_config_media_and_preview() {
        let cfg_json = r#"{"schemaVersion":1}"#;
        let fake_media = b"fake-mp4-video-payload-bytes";
        let fake_preview = b"\xFF\xD8\xFF\xE0preview-jpg\xFF\xD9";
        let packed = pack_uncompressed_zip(&[
            ("config.json", cfg_json.as_bytes()),
            ("background.mp4", fake_media),
            ("preview.jpg", fake_preview),
        ]);
        let unpacked = unpack_uncompressed_zip(&packed).expect("unpack .peco");
        assert_eq!(unpacked.len(), 3);
        assert_eq!(unpacked[0].name, "config.json");
        assert_eq!(unpacked[0].data, cfg_json.as_bytes());
        assert_eq!(unpacked[1].name, "background.mp4");
        assert_eq!(unpacked[1].data, fake_media);
        assert_eq!(unpacked[2].name, "preview.jpg");
        assert_eq!(unpacked[2].data, fake_preview);
    }

    /// A theme exported while a stale `video_wallpaper_path` lingers must not carry it:
    /// an image bundle keeps the picture and clears the video field.
    #[test]
    fn background_entry_bundles_image_and_prunes_video_path() {
        let dir = std::env::temp_dir().join(format!("peco-bg-test-{}", Uuid::new_v4()));
        fs::create_dir_all(&dir).unwrap();
        let img = dir.join("wall.webp");
        let pixels = b"fake-webp-payload";
        fs::write(&img, pixels).unwrap();

        let mut cfg = Config::default();
        cfg.settings.video_wallpaper_path = Some(r"C:\stale\old-video.mp4".into());
        let (name, data) = background_entry(&mut cfg, &img).expect("entry");
        assert_eq!(name, "background.webp");
        assert_eq!(data, pixels);
        assert_eq!(cfg.settings.video_wallpaper_path, None);
        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn background_entry_keeps_video_path_for_video_bundle() {
        let dir = std::env::temp_dir().join(format!("peco-bg-test-{}", Uuid::new_v4()));
        fs::create_dir_all(&dir).unwrap();
        let vid = dir.join("wall.mp4");
        fs::write(&vid, b"fake-mp4").unwrap();

        let mut cfg = Config::default();
        cfg.settings.video_wallpaper_path = Some(vid.to_string_lossy().into_owned());
        let (name, _) = background_entry(&mut cfg, &vid).expect("entry");
        assert_eq!(name, "background.mp4");
        assert!(cfg.settings.video_wallpaper_path.is_some());
        let _ = fs::remove_dir_all(&dir);
    }

    /// `cargo test -p pecofence-core wall_dir -- --ignored --nocapture` —
    /// bundles a real image from the user's wallpaper folder like the export does.
    #[test]
    #[ignore = "reads real files from the local wallpaper folder"]
    fn wall_dir_image_bundles_as_background_entry() {
        let img =
            Path::new(r"C:\Users\yahya\Downloads\WALL\Gemini_Generated_Image_91tt0891tt0891tt.jpg");
        let mut cfg = Config::default();
        cfg.settings.video_wallpaper_path =
            Some(r"C:\Users\yahya\Downloads\WALL\mp4\upscale_4k_00004-audio.mp4".into());
        let (name, data) = background_entry(&mut cfg, img).expect("WALL image entry");
        eprintln!("entry={name} bytes={}", data.len());
        assert!(name.starts_with("background."));
        assert!(!name.ends_with(".mp4"));
        assert_eq!(cfg.settings.video_wallpaper_path, None);
        assert_eq!(data, std::fs::read(img).unwrap());
    }
}
