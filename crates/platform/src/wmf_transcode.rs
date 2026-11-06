//! One-time video wallpaper transcode for imports.
//!
//! `IMFSourceReader` decodes the source while its video processor scales each
//! frame down to the largest display's physical pixels; `IMFSinkWriter`
//! re-encodes those frames to H.264 (hardware MFTs allowed) at a
//! wallpaper-class bitrate. The product lands in `imported_wallpapers/` as a
//! deterministic `video-<srclen>-<srcmtime>-<W>x<H>.mp4` — the name alone is the
//! reuse key, so re-picking the same file or re-applying an exported theme's
//! already-optimized `background.mp4` never pays a second encode.
//!
//! `.cde` bundles ship the optimized file: a 250 MB 4K loop arrives as a
//! ~10 MB device-matched MP4 the player also fits in the RAM byte-stream cache.

use crate::wide::to_wide;
use crate::wmf_bindings::{
    IMFAttributes, IMFMediaType, IMFSourceReader, MF_MT_AVG_BITRATE, MF_MT_FRAME_RATE,
    MF_MT_FRAME_SIZE, MF_MT_INTERLACE_MODE, MF_MT_MAJOR_TYPE, MF_MT_PIXEL_ASPECT_RATIO,
    MF_MT_SUBTYPE, MF_PD_DURATION, MF_SOURCE_READER_ALL_STREAMS,
    MF_SOURCE_READER_FIRST_VIDEO_STREAM, MF_SOURCE_READER_MEDIASOURCE,
    MF_SOURCE_READERF_ENDOFSTREAM, MF_SOURCE_READERF_ERROR, MFCreateAttributes, MFCreateMediaType,
    MFCreateSinkWriterFromURL, MFCreateSourceReaderFromURL, MFMediaType_Video,
    MFVIDEOINTERLACE_PROGRESSIVE,
};
use crate::wmf_wallpaper::{
    ComThreadGuard, MF_READWRITE_ENABLE_HARDWARE_TRANSFORMS,
    MF_SOURCE_READER_ENABLE_ADVANCED_VIDEO_PROCESSING, MF_SOURCE_READER_ENABLE_VIDEO_PROCESSING,
    MF_VIDEO_FORMAT_H264, MF_VIDEO_FORMAT_NV12, MF_VIDEO_FORMAT_RGB32, MfSessionGuard,
    is_h264_subtype,
};
use std::path::{Path, PathBuf};
use windows_core::{GUID, PCWSTR, Result};

/// What the transcode needs to know about a source without decoding a frame.
struct VideoProbe {
    width: u32,
    height: u32,
    fps_num: u32,
    fps_den: u32,
    /// `MF_PD_DURATION` in 100 ns units; 0 when the container won't say.
    duration_hns: u64,
    is_h264: bool,
    file_len: u64,
    /// The stream's native media type — the source reader only accepts an
    /// output type built by overriding attributes on this type (a from-scratch
    /// type misses the stream descriptors the VP needs for scaling).
    native: IMFMediaType,
}

/// `MFSetAttributeSize` packing — high word = width, low word = height.
fn pack_u32x2(w: u32, h: u32) -> u64 {
    ((w as u64) << 32) | h as u64
}

fn unpack_u32x2(v: windows_core::Result<u64>) -> windows_core::Result<(u32, u32)> {
    v.map(|v| ((v >> 32) as u32, v as u32))
}

/// Reads the source's native video stream description and container duration.
fn probe_video(path: &Path) -> Result<VideoProbe> {
    let wide = to_wide(&path.to_string_lossy());
    let reader = unsafe { MFCreateSourceReaderFromURL(PCWSTR(wide.as_ptr()), None) }?;
    let native =
        unsafe { reader.GetNativeMediaType(MF_SOURCE_READER_FIRST_VIDEO_STREAM as u32, 0)? };
    let packed = unsafe { native.GetUINT64(&MF_MT_FRAME_SIZE)? };
    let (width, height) = ((packed >> 32) as u32, packed as u32);
    if width == 0 || height == 0 {
        return Err(windows_core::HRESULT(0x8004_0005u32 as _).into()); // E_FAIL
    }
    let subtype = unsafe { native.GetGUID(&MF_MT_SUBTYPE).unwrap_or_default() };
    let fps = unsafe { native.GetUINT64(&MF_MT_FRAME_RATE).unwrap_or(30 << 32 | 1) };
    let duration_hns = unsafe {
        reader
            .GetPresentationAttribute(MF_SOURCE_READER_MEDIASOURCE as u32, &MF_PD_DURATION)
            .map(|pv| pv.Anonymous.Anonymous.Anonymous.uhVal)
            .unwrap_or(0)
    };
    let file_len = std::fs::metadata(path).map(|m| m.len()).unwrap_or(0);
    Ok(VideoProbe {
        width,
        height,
        fps_num: (fps >> 32) as u32,
        fps_den: (fps as u32).max(1),
        duration_hns,
        is_h264: is_h264_subtype(&subtype),
        file_len,
        native,
    })
}

/// Cover-downscale: the largest uniform scale with `out <= src` that still
/// covers `target` in both axes (a pure fit would leave one side short).
/// `None` when the source already fits inside the target — no upscale transcodes.
fn cover_size(src_w: u32, src_h: u32, target_w: u32, target_h: u32) -> Option<(u32, u32)> {
    let scale = (target_w as f64 / src_w as f64).max(target_h as f64 / src_h as f64);
    if scale >= 1.0 {
        return None;
    }
    let even = |v: f64| ((v.round() as u32) & !1).max(2);
    Some((even(src_w as f64 * scale), even(src_h as f64 * scale)))
}

/// Wallpaper-class bitrate: ~2.5 bits/pixel — 5.2 Mbps at 1080p, 9.2 Mbps at
/// 1440p, clamped so tiny inputs don't starve and huge crops don't balloon.
fn wallpaper_bitrate(w: u32, h: u32) -> u32 {
    ((w as u64 * h as u64 * 5 / 2).clamp(2_000_000, 10_000_000)) as u32
}

/// Deterministic optimized-asset name — the (source, monitor set) reuse key.
fn optimized_name(src: &Path, src_len: u64, target_w: u32, target_h: u32) -> String {
    let mtime = std::fs::metadata(src)
        .and_then(|m| m.modified())
        .ok()
        .and_then(|t| t.duration_since(std::time::UNIX_EPOCH).ok())
        .map(|d| d.as_secs())
        .unwrap_or(0);
    format!("video-{src_len:x}-{mtime:x}-{target_w}x{target_h}.mp4")
}

/// The reader's negotiated output type: the *native* type cloned (so every
/// stream attribute — apertures, rotation, alignment — survives) with the
/// subtype overridden at the native frame size. `SetCurrentMediaType` accepts
/// this for NV12/RGB32 at source dims; the software VP rejects any frame-size
/// that differs from the source, so scaling lives in the sink writer instead.
fn reader_output_type(
    native: &IMFMediaType,
    subtype: &GUID,
    w: u32,
    h: u32,
) -> std::result::Result<IMFMediaType, String> {
    let mt = unsafe { MFCreateMediaType().map_err(|e| e.to_string())? };
    unsafe {
        native.CopyAllItems(&mt).ok().map_err(|e| e.to_string())?;
        mt.SetGUID(&MF_MT_SUBTYPE, subtype)
            .ok()
            .map_err(|e| e.to_string())?;
        mt.SetUINT64(&MF_MT_FRAME_SIZE, pack_u32x2(w, h))
            .ok()
            .map_err(|e| e.to_string())?;
    }
    Ok(mt)
}

/// Builds a video media type: `subtype`, frame size, fps, progressive, square
/// pixels. `bitrate` applies only to encoder output types.
fn video_media_type(
    subtype: &GUID,
    w: u32,
    h: u32,
    fps_num: u32,
    fps_den: u32,
    bitrate: u32,
) -> std::result::Result<IMFMediaType, String> {
    let mt = unsafe { MFCreateMediaType().map_err(|e| e.to_string())? };
    unsafe {
        mt.SetGUID(&MF_MT_MAJOR_TYPE, &MFMediaType_Video)
            .ok()
            .map_err(|e| e.to_string())?;
        mt.SetGUID(&MF_MT_SUBTYPE, subtype)
            .ok()
            .map_err(|e| e.to_string())?;
        mt.SetUINT64(&MF_MT_FRAME_SIZE, pack_u32x2(w, h))
            .ok()
            .map_err(|e| e.to_string())?;
        mt.SetUINT64(&MF_MT_FRAME_RATE, pack_u32x2(fps_num, fps_den))
            .ok()
            .map_err(|e| e.to_string())?;
        mt.SetUINT32(&MF_MT_INTERLACE_MODE, MFVIDEOINTERLACE_PROGRESSIVE)
            .ok()
            .map_err(|e| e.to_string())?;
        mt.SetUINT64(&MF_MT_PIXEL_ASPECT_RATIO, pack_u32x2(1, 1))
            .ok()
            .map_err(|e| e.to_string())?;
        if bitrate != 0 {
            mt.SetUINT32(&MF_MT_AVG_BITRATE, bitrate)
                .ok()
                .map_err(|e| e.to_string())?;
        }
    }
    Ok(mt)
}

/// The transcode worker's full result for one `(src, display, dir)` triple.
///
/// `Ok(None)` — the source is already at/below target and already lean H.264:
/// the caller keeps using it directly. `Ok(Some(path))` — the optimized file
/// to play, bundle and cache instead. `Err` — the caller falls back to the
/// source (transcoding is an optimization, not a gate).
///
/// Runs COM/WMF on the calling thread; callers must not hold the UI thread.
pub fn transcode_for_display(
    src: &Path,
    cache_dir: &Path,
    target_w: u32,
    target_h: u32,
) -> std::result::Result<Option<PathBuf>, String> {
    let _com = ComThreadGuard::init().map_err(|e| format!("COM init: {e}"))?;
    let _mf = MfSessionGuard::init().map_err(|e| format!("MFStartup: {e}"))?;

    let probe = probe_video(src).map_err(|e| format!("probe {}: {e}", src.display()))?;
    let out = cover_size(probe.width, probe.height, target_w, target_h)
        .unwrap_or((probe.width & !1, probe.height & !1));
    let bitrate = wallpaper_bitrate(out.0, out.1);
    // Unknown duration → treat the source as heavy so it still gets squeezed.
    let src_bits = probe
        .file_len
        .saturating_mul(8)
        .saturating_mul(10_000_000)
        .checked_div(probe.duration_hns)
        .unwrap_or(u64::MAX);
    // Fully compliant already: fits the display, is H.264, and is within 1.5x
    // of the bitrate a transcode would produce — re-encoding only burns a
    // generation of quality (this is also what makes bundled .cde videos
    // apply instantly on a same-resolution machine).
    if out.0 == probe.width
        && out.1 == probe.height
        && probe.is_h264
        && src_bits <= bitrate as u64 * 3 / 2
    {
        tracing::info!(
            video = %src.display(),
            "wmf transcode: source already display-sized and lean; using as-is"
        );
        return Ok(None);
    }

    std::fs::create_dir_all(cache_dir).map_err(|e| format!("cache dir: {e}"))?;
    // MFCreateSinkWriterFromURL needs a fully-qualified path — a relative
    // cache_dir makes it return MF_E_NOT_FOUND.
    let cache_dir = std::path::absolute(cache_dir).map_err(|e| format!("cache dir: {e}"))?;
    let cache_dir = cache_dir.as_path();
    let name = optimized_name(src, probe.file_len, target_w, target_h);
    let dest = cache_dir.join(&name);
    if dest.is_file() {
        tracing::info!(video = %dest.display(), "wmf transcode: reusing cached optimized video");
        return Ok(Some(dest));
    }
    // Must keep the .mp4 extension — the sink writer picks its container
    // media sink from it; `.tmp` fails MFCreateSinkWriterFromURL outright.
    let tmp = cache_dir.join(format!("{name}.part.mp4"));

    match unsafe { transcode_inner(src, &tmp, &probe, out.0, out.1, bitrate) } {
        Ok(()) => {
            std::fs::rename(&tmp, &dest).map_err(|e| format!("rename {}: {e}", dest.display()))?;
            // Siblings from other sources/monitors are orphans — `video-*`
            // files are owned by this cache; `background.mp4` theme imports
            // are not.
            if let Ok(rd) = std::fs::read_dir(cache_dir) {
                for e in rd.flatten() {
                    let p = e.path();
                    if p != dest
                        && p.file_name()
                            .and_then(|n| n.to_str())
                            .is_some_and(|n| n.starts_with("video-") && n.ends_with(".mp4"))
                    {
                        let _ = std::fs::remove_file(&p);
                    }
                }
            }
            tracing::info!(
                src = %src.display(),
                dest = %dest.display(),
                out = ?out,
                bitrate,
                "wmf transcode: optimized video wallpaper written"
            );
            Ok(Some(dest))
        }
        Err(e) => {
            let _ = std::fs::remove_file(&tmp);
            Err(format!("transcode {}: {e}", src.display()))
        }
    }
}

/// Decode → VP scale → H.264 encode, all on the calling thread.
/// SAFETY: standard COM/WMF calls on objects owned by this function.
unsafe fn transcode_inner(
    src: &Path,
    tmp: &Path,
    probe: &VideoProbe,
    out_w: u32,
    out_h: u32,
    bitrate: u32,
) -> std::result::Result<(), String> {
    // --- reader: decode + scale via the video processor ---------------------
    let rattrs = unsafe {
        let mut a: Option<IMFAttributes> = None;
        MFCreateAttributes(&mut a, 3)
            .ok()
            .map_err(|e| e.to_string())?;
        a.ok_or_else(|| "MFCreateAttributes returned null".to_string())?
    };
    unsafe {
        rattrs
            .SetUINT32(&MF_READWRITE_ENABLE_HARDWARE_TRANSFORMS, 1)
            .ok()
            .map_err(|e| e.to_string())?;
        rattrs
            .SetUINT32(&MF_SOURCE_READER_ENABLE_VIDEO_PROCESSING, 1)
            .ok()
            .map_err(|e| e.to_string())?;
        // Scaling (not just color conversion) goes through the *advanced* VP —
        // the basic flag alone makes `SetCurrentMediaType` reject a foreign
        // frame size with MF_E_INVALIDMEDIATYPE.
        rattrs
            .SetUINT32(&MF_SOURCE_READER_ENABLE_ADVANCED_VIDEO_PROCESSING, 1)
            .ok()
            .map_err(|e| e.to_string())?;
    }
    let url = to_wide(&src.to_string_lossy());
    let reader: IMFSourceReader = unsafe {
        MFCreateSourceReaderFromURL(PCWSTR(url.as_ptr()), &rattrs).map_err(|e| e.to_string())?
    };
    unsafe {
        let _ = reader.SetStreamSelection(MF_SOURCE_READER_ALL_STREAMS as u32, false);
        reader
            .SetStreamSelection(MF_SOURCE_READER_FIRST_VIDEO_STREAM as u32, true)
            .ok()
            .map_err(|e| e.to_string())?;
    }
    // The software VP refuses frame-size changes in SetCurrentMediaType
    // (MF_E_INVALIDMEDIATYPE even at the native size) — it only does color
    // conversion. Scaling therefore happens in the sink writer, which
    // auto-inserts the Resizer MFT between its input and output types.
    // Ask the reader for NV12 at the NATIVE size; fall back to RGB32.
    let set = unsafe {
        reader.SetCurrentMediaType(
            MF_SOURCE_READER_FIRST_VIDEO_STREAM as u32,
            None,
            &reader_output_type(
                &probe.native,
                &MF_VIDEO_FORMAT_NV12,
                probe.width,
                probe.height,
            )?,
        )
    };
    if set.is_err() {
        unsafe {
            reader
                .SetCurrentMediaType(
                    MF_SOURCE_READER_FIRST_VIDEO_STREAM as u32,
                    None,
                    &reader_output_type(
                        &probe.native,
                        &MF_VIDEO_FORMAT_RGB32,
                        probe.width,
                        probe.height,
                    )?,
                )
                .ok()
                .map_err(|e| format!("SetCurrentMediaType: {e}"))?;
        }
    }
    // Whatever the reader actually negotiated drives the sink's input type —
    // guessing invites a second INVALIDMEDIATYPE.
    let current = unsafe {
        reader
            .GetCurrentMediaType(MF_SOURCE_READER_FIRST_VIDEO_STREAM as u32)
            .map_err(|e| format!("GetCurrentMediaType: {e}"))?
    };
    let decoded_subtype = unsafe {
        current
            .GetGUID(&MF_MT_SUBTYPE)
            .map_err(|e| format!("GetGUID(SUBTYPE): {e}"))?
    };
    let (dec_w, dec_h) = unsafe {
        unpack_u32x2(current.GetUINT64(&MF_MT_FRAME_SIZE))
            .map_err(|e| format!("GetUINT64(FRAME_SIZE): {e}"))?
    };

    // --- sink writer: hardware-capable H.264 -------------------------------
    let wattrs = unsafe {
        let mut a: Option<IMFAttributes> = None;
        MFCreateAttributes(&mut a, 2)
            .ok()
            .map_err(|e| e.to_string())?;
        a.ok_or_else(|| "MFCreateAttributes returned null".to_string())?
    };
    unsafe {
        wattrs
            .SetUINT32(&MF_READWRITE_ENABLE_HARDWARE_TRANSFORMS, 1)
            .ok()
            .map_err(|e| e.to_string())?;
    }
    let out_url = to_wide(&tmp.to_string_lossy());
    let sink = unsafe {
        MFCreateSinkWriterFromURL(PCWSTR(out_url.as_ptr()), core::ptr::null_mut(), &wattrs)
            .map_err(|e| format!("MFCreateSinkWriterFromURL: {e}"))?
    };
    let out_type = video_media_type(
        &MF_VIDEO_FORMAT_H264,
        out_w,
        out_h,
        probe.fps_num,
        probe.fps_den,
        bitrate,
    )?;
    // Sink input describes the samples as they ARRIVE (native size); the
    // resizer MFT it inserts bridges to the H.264 output's target size.
    let in_type = video_media_type(
        &decoded_subtype,
        dec_w,
        dec_h,
        probe.fps_num,
        probe.fps_den,
        0,
    )?;
    let stream = unsafe {
        sink.AddStream(&out_type)
            .map_err(|e| format!("AddStream(H264): {e}"))
    }?;
    unsafe {
        sink.SetInputMediaType(stream, &in_type, None)
            .map_err(|e| format!("SetInputMediaType: {e}"))?;
        sink.BeginWriting()
            .map_err(|e| format!("BeginWriting: {e}"))?;
    }

    // --- pump --------------------------------------------------------------
    let mut first_ts: Option<i64> = None;
    loop {
        let (mut flags, mut ts) = (0u32, 0i64);
        let mut sample = None;
        unsafe {
            reader
                .ReadSample(
                    MF_SOURCE_READER_FIRST_VIDEO_STREAM as u32,
                    0,
                    None,
                    Some(&mut flags),
                    Some(&mut ts),
                    Some(&mut sample),
                )
                .ok()
                .map_err(|e| e.to_string())?;
        }
        if flags & MF_SOURCE_READERF_ERROR != 0 {
            return Err("source reader reported MF_SOURCE_READERF_ERROR".into());
        }
        if flags & MF_SOURCE_READERF_ENDOFSTREAM != 0 {
            break;
        }
        let Some(s) = sample else { continue };
        let base = *first_ts.get_or_insert(ts);
        unsafe {
            s.SetSampleTime((ts - base).max(0))
                .ok()
                .map_err(|e| format!("SetSampleTime: {e}"))?;
            sink.WriteSample(stream, &s)
                .map_err(|e| format!("WriteSample: {e}"))?;
        }
    }
    unsafe { sink.Finalize().map_err(|e| format!("Finalize: {e}")) }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Real-file transcode: `TEST_VIDEO=<mp4>` + optional `TEST_OUT=<dir>`.
    /// Probes the output back and asserts frame size + that it plays smaller.
    /// `TEST_VIDEO=C:\path\to\video.mp4`
    #[test]
    #[ignore]
    fn transcodes_a_real_file() {
        let src = std::env::var("TEST_VIDEO").expect("TEST_VIDEO not set");
        let src = Path::new(&src);
        assert!(src.is_file());
        let dir = std::env::var("TEST_OUT")
            .map(PathBuf::from)
            .unwrap_or_else(|_| std::env::temp_dir().join("coredesk-transcode-test"));
        let out = transcode_for_display(src, &dir, 1920, 1080)
            .unwrap()
            .expect("expected a transcode, got compliance skip");
        assert!(out.is_file());
        let _com = ComThreadGuard::init().unwrap();
        let _mf = MfSessionGuard::init().unwrap();
        let p2 = probe_video(&out).unwrap();
        assert_eq!((p2.width, p2.height), (1920, 1080));
        assert!(p2.is_h264);
        eprintln!(
            "optimized: {} ({} bytes, {}x{})",
            out.display(),
            p2.file_len,
            p2.width,
            p2.height
        );
    }

    /// A display-sized lean H.264 file must skip re-encoding entirely.
    /// `TEST_VIDEO=<already-optimized mp4>`
    #[test]
    #[ignore]
    fn skips_compliant_source() {
        let src = std::env::var("TEST_VIDEO").expect("TEST_VIDEO not set");
        let dir = std::env::temp_dir().join("coredesk-transcode-test2");
        let out = transcode_for_display(Path::new(&src), &dir, 1920, 1080).unwrap();
        assert!(out.is_none(), "expected compliance skip, got {out:?}");
    }
}
