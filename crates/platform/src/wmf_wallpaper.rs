//! Native Windows Media Foundation (WMF) desktop video wallpaper engine and WIC JPEG encoder.
//!
//! Renders H.264 `.mp4` videos directly onto the desktop background layer (`0x052C` to `Progman`
//! to spawn `WorkerW`) and automatically pauses decoding/rendering when a full-screen application
//! or game is in the foreground or when the desktop is completely obscured (0% CPU/GPU while
//! hidden).

use crate::bindings::*;
use crate::desktop;
use crate::wide::to_wide;
use crate::wmf_bindings::{
    GUID_ContainerFormatJpeg, GUID_WICPixelFormat24bppBGR, GetParent, HALFTONE, IMF2DBuffer,
    IMFAttributes, IMFMediaBuffer, IMFSample, IMFSourceReader, IMFTransform, IWICBitmapEncoder,
    IWICBitmapFrameEncode, MF_MT_DEFAULT_STRIDE, MF_MT_FRAME_RATE, MF_MT_FRAME_SIZE,
    MF_MT_MAJOR_TYPE, MF_MT_SUBTYPE, MF_SOURCE_READER_ALL_STREAMS,
    MF_SOURCE_READER_FIRST_VIDEO_STREAM, MF_SOURCE_READERF_CURRENTMEDIATYPECHANGED,
    MF_SOURCE_READERF_ENDOFSTREAM, MF_SOURCE_READERF_ERROR, MF_VERSION, MFCreateAttributes,
    MFCreateMediaType, MFCreateSourceReaderFromURL, MFMediaType_Video, MFSTARTUP_NOSOCKET,
    MFShutdown, MFStartup, PROPVARIANT, SRCCOPY, SetParent, SetStretchBltMode, StretchDIBits,
    VARTYPE, WICBitmapEncoderNoCache,
};
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Condvar, Mutex, mpsc};
use std::time::{Duration, Instant};
use windows_core::{Error, GUID, Interface, PCWSTR, Result};

const WM_SPAWN_WORKER: u32 = 0x052C;
const SMTO_NORMAL: u32 = 0x0000;
const WM_PRINT: u32 = 0x0317;
const WM_PRINTCLIENT: u32 = 0x0318;
const SM_CXVIRTUALSCREEN: i32 = 78;
const SM_CYVIRTUALSCREEN: i32 = 79;
const RPC_E_CHANGED_MODE: i32 = 0x8001_0106u32 as i32;

/// Size of the thumbnail kept in `latest_frame` (used for `.peco` export previews): small
/// enough that keeping it around costs ~1 MB instead of a full-resolution frame.
const THUMB_W: usize = 640;
const THUMB_H: usize = 360;

// WMF GUIDs for source reader attributes and video subtypes.
const MF_SOURCE_READER_ENABLE_VIDEO_PROCESSING: GUID =
    GUID::from_u128(0xfb394f3d_ccf1_42ee_bbb3_f9b845d5681d);
const MF_SOURCE_READER_ENABLE_ADVANCED_VIDEO_PROCESSING: GUID =
    GUID::from_u128(0xed812c83_b66e_4322_b50a_d201c90328ff);
const MF_READWRITE_ENABLE_HARDWARE_TRANSFORMS: GUID =
    GUID::from_u128(0xa634a91c_822b_41b9_a494_4de4643612b0);
const MF_READWRITE_DISABLE_CONVERTERS: GUID =
    GUID::from_u128(0x98d5b065_1374_4847_8d5d_31520fee7156);
const MF_LOW_LATENCY: GUID = GUID::from_u128(0x9c27891a_ed7a_40e1_88e8_b22727a024ee);

// FourCC-based WMF Video Format GUIDs: XXXXXXXX-0000-0010-8000-00AA00389B71
const MF_VIDEO_FORMAT_RGB32: GUID = GUID::from_u128(0x00000016_0000_0010_8000_00aa00389b71);
const MF_VIDEO_FORMAT_ARGB32: GUID = GUID::from_u128(0x00000015_0000_0010_8000_00aa00389b71);
const MF_VIDEO_FORMAT_NV12: GUID = GUID::from_u128(0x3231564e_0000_0010_8000_00aa00389b71);
const MF_VIDEO_FORMAT_H264: GUID = GUID::from_u128(0x34363248_0000_0010_8000_00aa00389b71);
const MF_VIDEO_FORMAT_H264_LOWER: GUID = GUID::from_u128(0x34363268_0000_0010_8000_00aa00389b71);
const MF_VIDEO_FORMAT_H264_ES: GUID = GUID::from_u128(0x3f40f4f0_5622_4ff8_b6d8_a17a584bee5e);
const MF_VIDEO_FORMAT_AVC1: GUID = GUID::from_u128(0x31435641_0000_0010_8000_00aa00389b71);
const MF_VIDEO_FORMAT_AVC1_LOWER: GUID = GUID::from_u128(0x31637661_0000_0010_8000_00aa00389b71);

/// Initializes COM on the current thread if not already initialized (tolerates STA threads
/// where `CoInitializeEx(COINIT_MULTITHREADED)` returns `RPC_E_CHANGED_MODE`).
struct ComThreadGuard {
    uninit: bool,
}

impl ComThreadGuard {
    fn init() -> Result<Self> {
        // SAFETY: plain FFI call; RPC_E_CHANGED_MODE means COM is already initialized as STA.
        let hr = unsafe {
            CoInitializeEx(
                None,
                COINIT_MULTITHREADED as u32 | COINIT_DISABLE_OLE1DDE as u32,
            )
        };
        if hr.0 == RPC_E_CHANGED_MODE {
            Ok(Self { uninit: false })
        } else {
            hr.ok()?;
            Ok(Self { uninit: true })
        }
    }
}

impl Drop for ComThreadGuard {
    fn drop(&mut self) {
        if self.uninit {
            // SAFETY: balances successful CoInitializeEx on this thread.
            unsafe { CoUninitialize() };
        }
    }
}

struct MfSessionGuard;

impl MfSessionGuard {
    fn init() -> Result<Self> {
        // SAFETY: standard Media Foundation startup; paired with MFShutdown in Drop.
        unsafe { MFStartup(MF_VERSION as u32, MFSTARTUP_NOSOCKET as u32).ok()? };
        Ok(Self)
    }
}

impl Drop for MfSessionGuard {
    fn drop(&mut self) {
        // SAFETY: pairs with MFStartup in MfSessionGuard::init.
        unsafe {
            let _ = MFShutdown();
        }
    }
}

/// Decoded BGRA frame from a video or image.
#[derive(Clone)]
pub struct VideoFrame {
    pub width: u32,
    pub height: u32,
    pub bgra: Vec<u8>,
}

fn is_h264_subtype(subtype: &GUID) -> bool {
    *subtype == MF_VIDEO_FORMAT_H264
        || *subtype == MF_VIDEO_FORMAT_H264_LOWER
        || *subtype == MF_VIDEO_FORMAT_H264_ES
        || *subtype == MF_VIDEO_FORMAT_AVC1
        || *subtype == MF_VIDEO_FORMAT_AVC1_LOWER
}

/// The pixel format the source reader was configured to emit: NV12 needs no Media Foundation
/// Video Processor at all (the decoder's native output), so the whole VP surface pool — the
/// largest MF-side allocation — is never created. RGB32 keeps the old path as a fallback for
/// streams whose decoder cannot emit NV12 natively (10-bit sources, unusual profiles).
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum OutputKind {
    Nv12,
    Rgb32,
}

/// Validates that `path` is a readable MP4 file with an H.264 video stream and opens an
/// `IMFSourceReader`. The lean path is tried first (NV12, no video processing); when the
/// decoder refuses NV12 the reader is recreated with the video processor enabled and asked
/// for 32-bit RGB frames instead.
fn open_h264_reader(
    path: &Path,
) -> std::result::Result<(IMFSourceReader, u32, u32, i32, Duration, OutputKind), String> {
    tracing::debug!(path = %path.display(), "wmf: validating and opening MP4 file");
    if !path
        .extension()
        .is_some_and(|e| e.eq_ignore_ascii_case("mp4"))
    {
        tracing::error!(path = %path.display(), "wmf: rejected non-mp4 extension");
        return Err(pecofence_core::i18n::text("仅支持 H.264 编码的 MP4 视频文件").to_string());
    }
    if !path.is_file() {
        tracing::error!(path = %path.display(), "wmf: video file does not exist");
        return Err(pecofence_core::i18n::text("找不到所选的视频文件").to_string());
    }
    let clean_path = path
        .to_string_lossy()
        .trim_start_matches(r"\\?\")
        .to_string();
    match try_open_reader(&clean_path, OutputKind::Nv12) {
        Ok(ok) => Ok(ok),
        Err(lean_err) => {
            tracing::debug!(
                error = %lean_err,
                "wmf: NV12 direct decode refused; retrying with the video processor (RGB32)"
            );
            try_open_reader(&clean_path, OutputKind::Rgb32)
        }
    }
}

fn try_open_reader(
    clean_path: &str,
    kind: OutputKind,
) -> std::result::Result<(IMFSourceReader, u32, u32, i32, Duration, OutputKind), String> {
    let wide = to_wide(clean_path);
    // SAFETY: COM and Media Foundation calls on valid local buffers/interfaces.
    unsafe {
        let mut attrs: Option<IMFAttributes> = None;
        MFCreateAttributes(&mut attrs, 3).ok().map_err(|e| {
            tracing::error!(error = %e, "wmf: MFCreateAttributes failed");
            format!("MFCreateAttributes: {e}")
        })?;
        let attrs = attrs.ok_or_else(|| "MFCreateAttributes returned null".to_string())?;
        if kind == OutputKind::Rgb32 {
            // Format conversion needs the video processor; NV12 is the decoder's own output.
            let _ = attrs.SetUINT32(&MF_READWRITE_ENABLE_HARDWARE_TRANSFORMS, 1);
            let _ = attrs.SetUINT32(&MF_SOURCE_READER_ENABLE_VIDEO_PROCESSING, 1);
            let _ = attrs.SetUINT32(&MF_SOURCE_READER_ENABLE_ADVANCED_VIDEO_PROCESSING, 1);
        } else {
            // Software H.264 decodes straight to NV12 with no converter transform in
            // between: disabling both keeps the DXVA device, the video processor and
            // every extra MFT sample queue out of the process.
            let _ = attrs.SetUINT32(&MF_READWRITE_ENABLE_HARDWARE_TRANSFORMS, 0);
            let _ = attrs.SetUINT32(&MF_READWRITE_DISABLE_CONVERTERS, 1);
        }

        let reader = MFCreateSourceReaderFromURL(PCWSTR(wide.as_ptr()), &attrs).map_err(|e| {
            tracing::error!(path = %clean_path, error = %e, "wmf: MFCreateSourceReaderFromURL failed");
            format!("MFCreateSourceReaderFromURL: {e}")
        })?;
        let _ = reader.SetStreamSelection(MF_SOURCE_READER_ALL_STREAMS as u32, false);
        reader
            .SetStreamSelection(MF_SOURCE_READER_FIRST_VIDEO_STREAM as u32, true)
            .ok()
            .map_err(|e| {
                tracing::error!(error = %e, "wmf: SetStreamSelection(FIRST_VIDEO_STREAM) failed");
                format!("SetStreamSelection: {e}")
            })?;

        // Inspect native video stream media type.
        let native = reader
            .GetNativeMediaType(MF_SOURCE_READER_FIRST_VIDEO_STREAM as u32, 0)
            .map_err(|e| {
                tracing::error!(error = %e, "wmf: GetNativeMediaType failed");
                format!("GetNativeMediaType: {e}")
            })?;
        let major = native.GetGUID(&MF_MT_MAJOR_TYPE).unwrap_or_default();
        let subtype = native.GetGUID(&MF_MT_SUBTYPE).unwrap_or_default();
        tracing::info!(
            path = %clean_path,
            ?major,
            ?subtype,
            is_h264 = is_h264_subtype(&subtype),
            "wmf: inspected native video stream media type"
        );
        if major != MFMediaType_Video || !is_h264_subtype(&subtype) {
            tracing::error!(
                ?major,
                ?subtype,
                "wmf: video stream is not H.264 (expected H264/AVC1)"
            );
            return Err(format!(
                "{} (codec GUID: {subtype:?})",
                pecofence_core::i18n::text("仅支持 H.264 编码的 MP4 视频文件")
            ));
        }

        let out_type = MFCreateMediaType().map_err(|e| e.to_string())?;
        out_type
            .SetGUID(&MF_MT_MAJOR_TYPE, &MFMediaType_Video)
            .ok()
            .map_err(|e| e.to_string())?;
        match kind {
            OutputKind::Nv12 => {
                out_type
                    .SetGUID(&MF_MT_SUBTYPE, &MF_VIDEO_FORMAT_NV12)
                    .ok()
                    .map_err(|e| e.to_string())?;
                reader
                    .SetCurrentMediaType(
                        MF_SOURCE_READER_FIRST_VIDEO_STREAM as u32,
                        None,
                        &out_type,
                    )
                    .ok()
                    .map_err(|e| format!("SetCurrentMediaType(NV12): {e}"))?;
            }
            OutputKind::Rgb32 => {
                let rgb32_hr = out_type
                    .SetGUID(&MF_MT_SUBTYPE, &MF_VIDEO_FORMAT_RGB32)
                    .and_then(|| {
                        reader
                            .SetCurrentMediaType(
                                MF_SOURCE_READER_FIRST_VIDEO_STREAM as u32,
                                None,
                                &out_type,
                            )
                            .ok()
                    });
                if rgb32_hr.is_err() {
                    tracing::debug!(
                        hr = ?rgb32_hr,
                        "wmf: MFVideoFormat_RGB32 output rejected; falling back to MFVideoFormat_ARGB32"
                    );
                    out_type
                        .SetGUID(&MF_MT_SUBTYPE, &MF_VIDEO_FORMAT_ARGB32)
                        .ok()
                        .map_err(|e| e.to_string())?;
                    reader
                        .SetCurrentMediaType(
                            MF_SOURCE_READER_FIRST_VIDEO_STREAM as u32,
                            None,
                            &out_type,
                        )
                        .ok()
                        .map_err(|e| {
                            tracing::error!(error = %e, "wmf: SetCurrentMediaType(ARGB32) failed");
                            format!("SetCurrentMediaType: {e}")
                        })?;
                }
            }
        }

        // A wallpaper has no reordering deadline: low-latency mode tells the decoder MFT to
        // emit frames as soon as they are ready instead of filling its entire internal
        // DPB/output queue first (the difference is well over 100 MB of held sample buffers).
        let guid_null = GUID::zeroed();
        if let Ok(decoder) = reader.GetServiceForStream::<IMFTransform>(
            MF_SOURCE_READER_FIRST_VIDEO_STREAM as u32,
            &guid_null,
        ) && let Ok(mft_attrs) = decoder.GetAttributes()
            && mft_attrs.SetUINT32(&MF_LOW_LATENCY, 1).is_ok()
        {
            tracing::debug!("wmf: decoder MF_LOW_LATENCY enabled");
        }

        let (w, h, stride, interval) = query_stream_metrics(&reader)?;
        tracing::info!(
            width = w,
            height = h,
            stride,
            frame_interval_ms = interval.as_millis(),
            output = ?kind,
            "wmf: source reader ready"
        );
        Ok((reader, w, h, stride, interval, kind))
    }
}

unsafe fn query_stream_metrics(
    reader: &IMFSourceReader,
) -> std::result::Result<(u32, u32, i32, Duration), String> {
    // SAFETY: queries current media type attributes on a live source reader.
    unsafe {
        let current = reader
            .GetCurrentMediaType(MF_SOURCE_READER_FIRST_VIDEO_STREAM as u32)
            .map_err(|e| e.to_string())?;
        let size = current
            .GetUINT64(&MF_MT_FRAME_SIZE)
            .map_err(|e| e.to_string())?;
        let w = (size >> 32) as u32;
        let h = (size & 0xFFFF_FFFF) as u32;
        if w == 0 || h == 0 {
            return Err("invalid video dimensions (0x0)".to_string());
        }
        let stride = current
            .GetUINT32(&MF_MT_DEFAULT_STRIDE)
            .map(|s| s as i32)
            .unwrap_or((w * 4) as i32);
        let interval = current
            .GetUINT64(&MF_MT_FRAME_RATE)
            .ok()
            .and_then(|rate| {
                let num = rate >> 32;
                let den = rate & 0xFFFF_FFFF;
                (num > 0 && den > 0).then(|| Duration::from_nanos((den * 1_000_000_000) / num))
            })
            .unwrap_or(Duration::from_millis(33))
            .clamp(Duration::from_millis(8), Duration::from_millis(100));
        Ok((w, h, stride, interval))
    }
}

/// Rewinds `reader` to timestamp 0 for seamless looping.
unsafe fn rewind_reader(reader: &IMFSourceReader) -> bool {
    let mut prop = PROPVARIANT::default();
    // SAFETY: PROPVARIANT is a plain C union; setting vt = 20 (VT_I8) with zeroed payload is 0i64.
    unsafe {
        (*prop.Anonymous.Anonymous).vt = VARTYPE(20);
        let guid_null = GUID::zeroed();
        let hr = reader.SetCurrentPosition(&guid_null, &prop);
        if hr.is_err() {
            tracing::error!(?hr, "wmf: SetCurrentPosition(0) rewind failed");
            return false;
        }
        true
    }
}

/// Reads the next video frame from `reader` into `out_bgra` (top-down 32bpp with alpha = 255).
/// `IMFSample` and `IMFMediaBuffer` are dropped the moment their pixels are copied out, so the
/// decoder's surface pool never accumulates frames between reads.
unsafe fn read_next_frame(
    reader: &IMFSourceReader,
    kind: OutputKind,
    width: &mut u32,
    height: &mut u32,
    stride: &mut i32,
    out_bgra: &mut Vec<u8>,
) -> bool {
    // SAFETY: calls IMFSourceReader::ReadSample and copies rows within locked buffer bounds.
    unsafe {
        for attempt in 0..6 {
            let mut flags: u32 = 0;
            let mut timestamp: i64 = 0;
            let mut sample: Option<IMFSample> = None;
            let hr = reader.ReadSample(
                MF_SOURCE_READER_FIRST_VIDEO_STREAM as u32,
                0,
                None,
                Some(&mut flags),
                Some(&mut timestamp),
                Some(&mut sample),
            );
            if hr.is_err() {
                tracing::error!(?hr, attempt, "wmf: ReadSample returned error");
                return false;
            }
            if (flags & MF_SOURCE_READERF_ERROR) != 0 {
                tracing::error!(flags, "wmf: ReadSample set MF_SOURCE_READERF_ERROR");
                return false;
            }
            if (flags & MF_SOURCE_READERF_CURRENTMEDIATYPECHANGED) != 0
                && let Ok((w, h, s, _)) = query_stream_metrics(reader)
            {
                tracing::debug!(w, h, s, "wmf: media type changed mid-stream");
                *width = w;
                *height = h;
                *stride = s;
            }
            if (flags & MF_SOURCE_READERF_ENDOFSTREAM) != 0 {
                tracing::debug!("wmf: reached end of stream; rewinding to 0");
                if !rewind_reader(reader) {
                    return false;
                }
                continue;
            }
            let Some(sample) = sample else {
                continue;
            };
            let buffer: IMFMediaBuffer = match sample.ConvertToContiguousBuffer() {
                Ok(b) => b,
                Err(e) => {
                    tracing::error!(error = %e, "wmf: ConvertToContiguousBuffer failed");
                    drop(sample);
                    return false;
                }
            };
            let w = *width as usize;
            let h = *height as usize;
            let row_bytes = w * 4;
            out_bgra.resize(row_bytes * h, 255);

            let copied = copy_locked_frame(&buffer, kind, w, h, *stride, out_bgra);
            // The frame is in our own Vec now: release the decoder's sample and buffer
            // immediately instead of letting them linger to the end of the iteration.
            drop(buffer);
            drop(sample);
            if copied {
                return true;
            }
            return false;
        }
        false
    }
}

/// Copies the locked sample contents into `out_bgra`. Returns false when the buffer could not
/// be locked or is smaller than the frame it should hold.
unsafe fn copy_locked_frame(
    buffer: &IMFMediaBuffer,
    kind: OutputKind,
    w: usize,
    h: usize,
    stride: i32,
    out_bgra: &mut [u8],
) -> bool {
    // SAFETY: buffer is a live IMFMediaBuffer; the copy stays inside the locked extent.
    unsafe {
        if let Ok(buf2d) = buffer.cast::<IMF2DBuffer>() {
            let mut scan0: *mut u8 = core::ptr::null_mut();
            let mut pitch: i32 = 0;
            if buf2d.Lock2D(&mut scan0, &mut pitch).is_ok() && !scan0.is_null() {
                match kind {
                    OutputKind::Rgb32 => copy_strided_rows(scan0, pitch, w, h, out_bgra),
                    OutputKind::Nv12 => {
                        nv12_to_bgra(scan0, pitch.unsigned_abs() as usize, w, h, out_bgra)
                    }
                }
                let _ = buf2d.Unlock2D();
                return true;
            }
        }

        let mut ptr: *mut u8 = core::ptr::null_mut();
        let mut cur_len: u32 = 0;
        if buffer.Lock(&mut ptr, None, Some(&mut cur_len)).is_ok() && !ptr.is_null() {
            let pitch = if stride != 0 { stride } else { (w * 4) as i32 };
            let abs_pitch = pitch.unsigned_abs() as usize;
            let needed = match kind {
                OutputKind::Rgb32 => abs_pitch.saturating_mul(h),
                OutputKind::Nv12 => abs_pitch.saturating_mul(h * 3 / 2),
            };
            if (cur_len as usize) >= needed {
                match kind {
                    OutputKind::Rgb32 => copy_strided_rows(ptr, pitch, w, h, out_bgra),
                    OutputKind::Nv12 => nv12_to_bgra(ptr, abs_pitch, w, h, out_bgra),
                }
                let _ = buffer.Unlock();
                return true;
            }
            tracing::error!(
                cur_len,
                needed,
                pitch,
                ?kind,
                "wmf: locked buffer smaller than expected frame size"
            );
            let _ = buffer.Unlock();
        }
        false
    }
}

/// Converts an NV12 frame (`pitch`-strided Y plane followed by the interleaved U/V plane at
/// `pitch * h`) into top-down 32bpp BGRA using BT.601 studio-swing coefficients.
unsafe fn nv12_to_bgra(planes: *const u8, pitch: usize, w: usize, h: usize, out_bgra: &mut [u8]) {
    let y_plane = planes;
    // SAFETY: caller guaranteed the locked buffer spans pitch * (h + h/2) bytes.
    let uv_plane = unsafe { planes.add(pitch * h) };
    for y in 0..h {
        // SAFETY: row reads stay within each plane's pitch-strided extent.
        let (y_row, uv_row) = unsafe { (y_plane.add(y * pitch), uv_plane.add((y / 2) * pitch)) };
        let out_row = &mut out_bgra[y * w * 4..(y + 1) * w * 4];
        let mut x = 0;
        while x < w {
            let ux = x & !1;
            // SAFETY: 4:2:0 chroma has (w+1)/2 pairs per row; ux+1 is the last valid index.
            let (u, v) = unsafe {
                (
                    *uv_row.add(ux) as i32 - 128,
                    *uv_row.add(ux + 1) as i32 - 128,
                )
            };
            // SAFETY: x < w keeps the luma read inside the row.
            let d = 298 * (unsafe { *y_row.add(x) } as i32 - 16);
            let px = &mut out_row[x * 4..x * 4 + 4];
            px[0] = ((d + 516 * u + 128) >> 8).clamp(0, 255) as u8;
            px[1] = ((d - 100 * u - 208 * v + 128) >> 8).clamp(0, 255) as u8;
            px[2] = ((d + 409 * v + 128) >> 8).clamp(0, 255) as u8;
            px[3] = 255;
            x += 1;
        }
    }
}

unsafe fn copy_strided_rows(
    scan0: *const u8,
    pitch: i32,
    width: usize,
    height: usize,
    out_bgra: &mut [u8],
) {
    let row_bytes = width * 4;
    for y in 0..height {
        let src_row = unsafe {
            if pitch >= 0 {
                scan0.add(y * pitch as usize)
            } else {
                scan0.offset((y as isize) * (pitch as isize))
            }
        };
        let dst_off = y * row_bytes;
        let dst_row = &mut out_bgra[dst_off..dst_off + row_bytes];
        // SAFETY: caller verified scan0 + row offset is valid for `row_bytes`.
        unsafe {
            core::ptr::copy_nonoverlapping(src_row, dst_row.as_mut_ptr(), row_bytes);
        }
        for px in dst_row.as_chunks_mut::<4>().0 {
            px[3] = 255;
        }
    }
}

/// Validates that `path` is a valid H.264 MP4 file and decodes its first frame (for preview/thumbnail).
pub fn decode_video_first_frame(path: &Path) -> std::result::Result<VideoFrame, String> {
    let _com = ComThreadGuard::init().map_err(|e| format!("COM init: {e}"))?;
    let _mf = MfSessionGuard::init().map_err(|e| format!("MFStartup: {e}"))?;
    let (reader, mut w, mut h, mut stride, _, kind) = open_h264_reader(path)?;
    let mut bgra = Vec::new();
    // SAFETY: reader is a valid opened IMFSourceReader on this thread.
    let ok = unsafe { read_next_frame(&reader, kind, &mut w, &mut h, &mut stride, &mut bgra) };
    if !ok || bgra.is_empty() {
        return Err(pecofence_core::i18n::text("无法解码此 MP4 视频文件").to_string());
    }
    Ok(VideoFrame {
        width: w,
        height: h,
        bgra,
    })
}

/// Checks via `GetForegroundWindow` and window rects whether the foreground window is a
/// full-screen application/game or a maximized application that completely obscures the desktop.
///
/// Crucially ignores:
/// - Our own process (`PecoFence.Settings`, fence windows) so the user sees the wallpaper while
///   configuring PecoFence.
/// - Explorer desktop/taskbar windows (`Progman`, `WorkerW`, `Shell_TrayWnd`).
/// - Transparent / non-activating / toolwindow overlays (e.g., `NVIDIA GeForce Overlay`
///   `CEF-OSC-WIDGET`) which permanently span `(0, 0, screen_w, screen_h)`.
pub fn should_pause_playback() -> bool {
    let fg_raw = desktop::foreground_window();
    let fg_root = desktop::root_ancestor(fg_raw);
    if fg_root.0.is_null() || !desktop::has_visible_surface(fg_root) {
        return false;
    }

    let shell = desktop::shell_window();
    let shell_pid = desktop::window_pid(shell);
    let our_pid = std::process::id();
    let fg_pid = desktop::window_pid(fg_root);
    if fg_pid == our_pid || fg_pid == shell_pid {
        return false;
    }

    let fg_class = desktop::class_name(fg_root);
    if matches!(
        fg_class.as_str(),
        "Progman"
            | "WorkerW"
            | "Shell_TrayWnd"
            | "Shell_SecondaryTrayWnd"
            | "XamlExplorerHostIslandWindow"
            | "CEF-OSC-WIDGET"
    ) {
        return false;
    }

    // Ignore child windows of Progman/WorkerW or non-activating/transparent overlays.
    // SAFETY: GetParent is a simple query on a live HWND.
    let parent = unsafe { GetParent(crate::wmf_bindings::HWND(fg_root.0)) };
    if !parent.0.is_null() && HWND(parent.0) == shell {
        return false;
    }
    let ex = desktop::ex_style(fg_root);
    if (ex & (WS_EX_TRANSPARENT | WS_EX_NOACTIVATE | WS_EX_TOOLWINDOW) as u32) != 0 {
        return false;
    }

    let fg_rect = crate::window::window_rect(fg_root);
    let mon = crate::monitors::monitor_from_window(fg_root);
    let Some(info) = crate::monitors::query(mon) else {
        return false;
    };

    // 1. Full-screen application or game covering the entire monitor.
    let fullscreen = fg_rect.left <= info.bounds.left
        && fg_rect.top <= info.bounds.top
        && fg_rect.right >= info.bounds.right
        && fg_rect.bottom >= info.bounds.bottom;
    if fullscreen {
        tracing::debug!(
            fg_hwnd = ?fg_root,
            %fg_class,
            fg_pid,
            ?fg_rect,
            mon_bounds = ?info.bounds,
            "wmf: auto-pause triggered by fullscreen foreground window"
        );
        return true;
    }

    // 2. Single-monitor setup where the foreground window is maximized and covers the entire
    // desktop work area.
    let mons = crate::monitors::enumerate();
    if mons.len() <= 1 {
        let w = info.work_area;
        let covers_work = fg_rect.left <= w.left + 8
            && fg_rect.top <= w.top + 8
            && fg_rect.right >= w.right - 8
            && fg_rect.bottom >= w.bottom - 8;
        if covers_work {
            tracing::debug!(
                fg_hwnd = ?fg_root,
                %fg_class,
                fg_pid,
                ?fg_rect,
                work_area = ?w,
                "wmf: auto-pause triggered by maximized foreground window covering desktop work area"
            );
            return true;
        }
    }

    false
}

/// Sends `0x052C` to `Progman` to spawn a `WorkerW` window and attaches `video_hwnd` to the
/// desktop background layer (behind `SHELLDLL_DefView` / desktop icons and fences).
unsafe fn attach_to_desktop_worker(
    video_hwnd: HWND,
) -> std::result::Result<(HWND, i32, i32), String> {
    // SAFETY: standard Win32 window hierarchy calls on Progman / WorkerW and our own HWND.
    unsafe {
        let progman = {
            let p = FindWindowW(windows_core::w!("Progman"), PCWSTR::null());
            if p.0.is_null() {
                desktop::shell_window()
            } else {
                p
            }
        };
        if progman.0.is_null() {
            tracing::error!("wmf: Progman window not found");
            return Err("Progman window not found".to_string());
        }

        // Spawn WorkerW behind the desktop icons via 0x052C.
        let r1 = SendMessageTimeoutW(
            progman,
            WM_SPAWN_WORKER,
            WPARAM(0xD),
            LPARAM(1),
            SMTO_NORMAL,
            1000,
            None,
        );
        let r2 = SendMessageTimeoutW(
            progman,
            WM_SPAWN_WORKER,
            WPARAM(0),
            LPARAM(0),
            SMTO_NORMAL,
            1000,
            None,
        );

        let vw = GetSystemMetrics(SM_CXVIRTUALSCREEN).max(1);
        let vh = GetSystemMetrics(SM_CYVIRTUALSCREEN).max(1);

        // Locate SHELLDLL_DefView and WorkerW for both Modern (24H2+) and Legacy (<=23H2) shells.
        let def_under_progman = FindWindowExW(
            Some(progman),
            None,
            windows_core::w!("SHELLDLL_DefView"),
            PCWSTR::null(),
        );
        let worker_under_progman = FindWindowExW(
            Some(progman),
            None,
            windows_core::w!("WorkerW"),
            PCWSTR::null(),
        );

        let mut top_worker = HWND::default();
        if def_under_progman.0.is_null() {
            for top in desktop::top_level_windows() {
                let def = FindWindowExW(
                    Some(top),
                    None,
                    windows_core::w!("SHELLDLL_DefView"),
                    PCWSTR::null(),
                );
                if !def.0.is_null() {
                    let w =
                        FindWindowExW(None, Some(top), windows_core::w!("WorkerW"), PCWSTR::null());
                    if !w.0.is_null() {
                        top_worker = w;
                        break;
                    }
                }
            }
        }

        let host = if !top_worker.0.is_null() {
            top_worker
        } else if !def_under_progman.0.is_null() {
            progman
        } else if !worker_under_progman.0.is_null() {
            worker_under_progman
        } else {
            progman
        };

        // Ensure WS_EX_LAYERED is armed before parenting so Windows 11 24H2+ Progman
        // (WS_EX_NOREDIRECTIONBITMAP) composites our child surface.
        let _ = SetLayeredWindowAttributes(video_hwnd, COLORREF(0), 255, LWA_ALPHA as u32);
        let prev_parent = SetParent(
            crate::wmf_bindings::HWND(video_hwnd.0),
            Some(crate::wmf_bindings::HWND(host.0)),
        );
        let style = (WS_CHILD | WS_VISIBLE | WS_CLIPSIBLINGS) as isize;
        let ex_style = (WS_EX_LAYERED | WS_EX_TOOLWINDOW | WS_EX_NOACTIVATE) as isize;
        SetWindowLongPtrW(video_hwnd, GWL_STYLE, style);
        SetWindowLongPtrW(video_hwnd, GWL_EXSTYLE, ex_style);
        let lwa_ok = SetLayeredWindowAttributes(video_hwnd, COLORREF(0), 255, LWA_ALPHA as u32);

        let insert_after = if host == progman && !def_under_progman.0.is_null() {
            def_under_progman
        } else {
            HWND_TOP
        };
        let swp_ok = SetWindowPos(
            video_hwnd,
            Some(insert_after),
            0,
            0,
            vw,
            vh,
            (SWP_NOACTIVATE | SWP_SHOWWINDOW | SWP_FRAMECHANGED) as u32,
        );
        let _ = ShowWindow(video_hwnd, SW_SHOWNOACTIVATE);

        tracing::info!(
            ?progman,
            ?def_under_progman,
            ?worker_under_progman,
            ?top_worker,
            ?host,
            ?video_hwnd,
            ?prev_parent,
            spawn_r1 = r1.0,
            spawn_r2 = r2.0,
            lwa_ok = lwa_ok.as_bool(),
            swp_ok = swp_ok.as_bool(),
            vw,
            vh,
            actual_ex_style = format!("{:#x}", desktop::ex_style(video_hwnd)),
            "wmf: attached video window to desktop WorkerW/Progman layer"
        );
        Ok((host, vw, vh))
    }
}

/// The frame shared with the paint path: the decoded frame at video resolution plus the
/// window (destination) size. `StretchDIBits` cover-crops and stretches at blit time, so no
/// screen-sized intermediate buffer is ever kept (~33 MB less at 4K).
struct SharedSurface {
    out_w: i32,
    out_h: i32,
    width: i32,
    height: i32,
    bgra: Vec<u8>,
}

/// Active WMF desktop video wallpaper player. Dropping or calling `stop` cleanly shuts down the
/// playback thread, destroys the `WorkerW` child window, and repaints the desktop background.
pub struct WmfWallpaperPlayer {
    path: PathBuf,
    stop: Arc<AtomicBool>,
    wake: Arc<(Mutex<()>, Condvar)>,
    latest_frame: Arc<Mutex<Option<VideoFrame>>>,
    thread: Option<std::thread::JoinHandle<()>>,
}

impl WmfWallpaperPlayer {
    /// Starts playing `path` on the desktop `WorkerW` layer. Initializes WMF, decodes the first
    /// H.264 frame, attaches the desktop window, and presents Frame #0 on the worker thread
    /// before returning `Ok` (or surfaces the exact failure reason to the caller).
    pub fn start(path: impl Into<PathBuf>) -> std::result::Result<Self, String> {
        let path = path.into();
        tracing::info!(path = %path.display(), "wmf: starting WmfWallpaperPlayer");
        let stop = Arc::new(AtomicBool::new(false));
        let wake = Arc::new((Mutex::new(()), Condvar::new()));
        let latest_frame = Arc::new(Mutex::new(None));
        let (startup_tx, startup_rx) =
            mpsc::sync_channel::<std::result::Result<VideoFrame, String>>(1);

        let stop_thread = stop.clone();
        let wake_thread = wake.clone();
        let latest_thread = latest_frame.clone();
        let video_path = path.clone();

        let thread = std::thread::Builder::new()
            .name("pecofence-wmf-wallpaper".into())
            .spawn(move || {
                run_player_thread(
                    video_path,
                    stop_thread,
                    wake_thread,
                    latest_thread,
                    startup_tx,
                );
            })
            .map_err(|e| {
                tracing::error!(error = %e, "wmf: failed to spawn player thread");
                e.to_string()
            })?;

        match startup_rx.recv_timeout(Duration::from_secs(10)) {
            Ok(Ok(first_frame)) => {
                if let Ok(mut slot) = latest_frame.lock() {
                    *slot = Some(first_frame);
                }
                tracing::info!(path = %path.display(), "wmf: WmfWallpaperPlayer started and rendering");
                Ok(Self {
                    path,
                    stop,
                    wake,
                    latest_frame,
                    thread: Some(thread),
                })
            }
            Ok(Err(err)) => {
                stop.store(true, Ordering::SeqCst);
                let _ = thread.join();
                tracing::error!(path = %path.display(), %err, "wmf: WmfWallpaperPlayer failed during startup");
                Err(err)
            }
            Err(e) => {
                stop.store(true, Ordering::SeqCst);
                tracing::error!(path = %path.display(), error = %e, "wmf: WmfWallpaperPlayer timed out during startup");
                Err(format!("WMF initialization timed out: {e}"))
            }
        }
    }

    pub fn path(&self) -> &Path {
        &self.path
    }

    /// A small (≤640x360) thumbnail of the most recently decoded frame, for `.peco` export
    /// previews. Downscaled on the player thread instead of cloning a full-resolution frame.
    pub fn latest_frame(&self) -> Option<VideoFrame> {
        self.latest_frame.lock().ok().and_then(|g| g.clone())
    }

    pub fn stop(&mut self) {
        self.stop.store(true, Ordering::SeqCst);
        self.wake.1.notify_all();
        if let Some(handle) = self.thread.take() {
            let _ = handle.join();
        }
    }
}

impl Drop for WmfWallpaperPlayer {
    fn drop(&mut self) {
        self.stop();
    }
}

unsafe extern "system" fn video_wnd_proc(
    hwnd: HWND,
    msg: u32,
    wparam: WPARAM,
    lparam: LPARAM,
) -> LRESULT {
    // SAFETY: GWLP_USERDATA holds a valid pointer to `Mutex<SharedSurface>` while the window lives.
    unsafe {
        match msg {
            x if x == WM_ERASEBKGND as u32 => LRESULT(1),
            x if x == WM_PAINT as u32 => {
                let mut ps = PAINTSTRUCT::default();
                let dc = BeginPaint(hwnd, &mut ps);
                paint_surface_dc(hwnd, dc);
                let _ = EndPaint(hwnd, &ps);
                LRESULT(0)
            }
            WM_PRINT | WM_PRINTCLIENT => {
                let dc = HDC(wparam.0 as *mut core::ffi::c_void);
                paint_surface_dc(hwnd, dc);
                LRESULT(0)
            }
            _ => DefWindowProcW(hwnd, msg, wparam, lparam),
        }
    }
}

/// Aspect-fill (cover) crop in source coordinates: the part of a `sw`x`sh` frame that fills
/// `dw`x`dh` without distorting, centred.
fn cover_crop(sw: i32, sh: i32, dw: i32, dh: i32) -> (i32, i32, i32, i32) {
    if dw <= 0 || dh <= 0 || sw <= 0 || sh <= 0 {
        return (0, 0, sw.max(1), sh.max(1));
    }
    if sw as i64 * dh as i64 > dw as i64 * sh as i64 {
        let cw = ((sh as i64 * dw as i64) / dh as i64).clamp(1, sw as i64) as i32;
        ((sw - cw) / 2, 0, cw, sh)
    } else {
        let ch = ((sw as i64 * dh as i64) / dw as i64).clamp(1, sh as i64) as i32;
        (0, (sh - ch) / 2, sw, ch)
    }
}

unsafe fn paint_surface_dc(hwnd: HWND, dc: HDC) {
    // SAFETY: reads GWLP_USERDATA and blits the shared BGRA buffer via StretchDIBits.
    unsafe {
        let ptr = GetWindowLongPtrW(hwnd, GWLP_USERDATA) as *const Mutex<SharedSurface>;
        if ptr.is_null() || dc.0.is_null() {
            return;
        }
        let Ok(surface) = (*ptr).lock() else {
            return;
        };
        if surface.width <= 0 || surface.height <= 0 || surface.bgra.is_empty() {
            return;
        }
        let bmi = crate::wmf_bindings::BITMAPINFO {
            bmiHeader: crate::wmf_bindings::BITMAPINFOHEADER {
                biSize: size_of::<crate::wmf_bindings::BITMAPINFOHEADER>() as u32,
                biWidth: surface.width,
                biHeight: -surface.height,
                biPlanes: 1,
                biBitCount: 32,
                biCompression: BI_RGB as u32,
                ..Default::default()
            },
            ..Default::default()
        };
        let (sx, sy, sw, sh) =
            cover_crop(surface.width, surface.height, surface.out_w, surface.out_h);
        let wmf_dc = crate::wmf_bindings::HDC(dc.0);
        let _ = SetStretchBltMode(wmf_dc, HALFTONE);
        let _ = StretchDIBits(
            wmf_dc,
            0,
            0,
            surface.out_w,
            surface.out_h,
            sx,
            sy,
            sw,
            sh,
            Some(surface.bgra.as_ptr().cast()),
            &bmi,
            DIB_RGB_COLORS as u32,
            SRCCOPY,
        );
    }
}

unsafe fn present_hwnd(hwnd: HWND) {
    // SAFETY: invalidates and immediately blits the latest frame to the window DC.
    unsafe {
        let dc = GetDC(Some(hwnd));
        if !dc.0.is_null() {
            paint_surface_dc(hwnd, dc);
            let _ = ReleaseDC(Some(hwnd), dc);
        }
        let _ = InvalidateRect(Some(hwnd), None, false);
        let _ = GdiFlush();
    }
}

fn run_player_thread(
    path: PathBuf,
    stop: Arc<AtomicBool>,
    wake: Arc<(Mutex<()>, Condvar)>,
    latest_frame: Arc<Mutex<Option<VideoFrame>>>,
    startup_tx: mpsc::SyncSender<std::result::Result<VideoFrame, String>>,
) {
    let _com = match ComThreadGuard::init() {
        Ok(g) => g,
        Err(e) => {
            let _ = startup_tx.send(Err(format!("COM initialization failed: {e}")));
            return;
        }
    };
    let _mf = match MfSessionGuard::init() {
        Ok(g) => g,
        Err(e) => {
            let _ = startup_tx.send(Err(format!("MFStartup failed: {e}")));
            return;
        }
    };
    let (reader, mut vid_w, mut vid_h, mut stride, frame_interval, out_kind) =
        match open_h264_reader(&path) {
            Ok(v) => v,
            Err(e) => {
                let _ = startup_tx.send(Err(e));
                return;
            }
        };

    let mut raw_frame = Vec::new();
    // Decode the very first frame before creating/attaching the window so we know decoding works
    // and can paint Frame #0 immediately upon attachment.
    // SAFETY: reader is live on this thread.
    let first_ok = unsafe {
        read_next_frame(
            &reader,
            out_kind,
            &mut vid_w,
            &mut vid_h,
            &mut stride,
            &mut raw_frame,
        )
    };
    if !first_ok || raw_frame.is_empty() {
        let msg = pecofence_core::i18n::text("无法解码此 MP4 视频文件").to_string();
        tracing::error!(path = %path.display(), "wmf: failed to decode first video frame");
        let _ = startup_tx.send(Err(msg));
        return;
    }

    let class_name = to_wide("PecoFence.WmfWallpaper");
    // SAFETY: Win32 window registration, creation, WMF frame presentation, and cleanup.
    unsafe {
        let instance = GetModuleHandleW(PCWSTR::null());
        let wc = WNDCLASSEXW {
            cbSize: size_of::<WNDCLASSEXW>() as u32,
            style: (CS_HREDRAW | CS_VREDRAW) as u32,
            lpfnWndProc: Some(video_wnd_proc),
            hInstance: instance,
            lpszClassName: PCWSTR(class_name.as_ptr()),
            ..Default::default()
        };
        let _ = RegisterClassExW(&wc);

        let surface = Box::new(Mutex::new(SharedSurface {
            out_w: 0,
            out_h: 0,
            width: 0,
            height: 0,
            bgra: Vec::new(),
        }));
        let surface_ptr = &*surface as *const Mutex<SharedSurface>;

        let hwnd = CreateWindowExW(
            (WS_EX_LAYERED | WS_EX_TOOLWINDOW | WS_EX_NOACTIVATE) as u32,
            PCWSTR(class_name.as_ptr()),
            windows_core::w!("PecoFence Video Wallpaper"),
            WS_POPUP,
            0,
            0,
            1,
            1,
            None,
            None,
            Some(instance),
            None,
        );
        if hwnd.0.is_null() {
            let err = format!("CreateWindowExW failed (GetLastError={})", GetLastError());
            tracing::error!(%err, "wmf: window creation failed");
            let _ = startup_tx.send(Err(err));
            return;
        }
        SetWindowLongPtrW(hwnd, GWLP_USERDATA, surface_ptr as isize);

        let (host, win_w, win_h) = match attach_to_desktop_worker(hwnd) {
            Ok(v) => v,
            Err(e) => {
                SetWindowLongPtrW(hwnd, GWLP_USERDATA, 0);
                let _ = DestroyWindow(hwnd);
                let _ = startup_tx.send(Err(e));
                return;
            }
        };

        // Paint Frame #0 immediately before entering the playback loop: the surface keeps the
        // video-resolution frame and StretchDIBits cover-crops it onto the window at blit time.
        if let Ok(mut surf) = surface.lock() {
            surf.out_w = win_w;
            surf.out_h = win_h;
            surf.width = vid_w as i32;
            surf.height = vid_h as i32;
            surf.bgra.clone_from(&raw_frame);
        }
        present_hwnd(hwnd);
        pump_thread_messages();

        let mut thumb = Vec::new();
        scale_cover_bgra(
            &raw_frame,
            vid_w as usize,
            vid_h as usize,
            &mut thumb,
            THUMB_W,
            THUMB_H,
        );
        let first_vf = VideoFrame {
            width: THUMB_W as u32,
            height: THUMB_H as u32,
            bgra: thumb.clone(),
        };
        let _ = startup_tx.send(Ok(first_vf));

        let mut paused = false;
        let mut last_visibility_check = Instant::now();
        let mut next_frame_at = Instant::now() + frame_interval;
        let mut frame_counter: u64 = 1;

        while !stop.load(Ordering::Relaxed) {
            pump_thread_messages();

            if last_visibility_check.elapsed() >= Duration::from_millis(300) {
                last_visibility_check = Instant::now();
                let now_paused = should_pause_playback();
                if now_paused != paused {
                    paused = now_paused;
                    tracing::info!(paused, frame_counter, "wmf: playback pause state changed");
                    if paused {
                        // Drop every pending decoded frame and hand this process's idle pages
                        // back to Windows while the video cannot be seen anyway.
                        let _ = reader.Flush(MF_SOURCE_READER_FIRST_VIDEO_STREAM as u32);
                        crate::memstats::trim_working_set();
                    }
                }
            }

            if paused {
                // 0% CPU / 0% GPU while desktop is obscured or fullscreen app is active;
                // wake every 80ms only to pump WM_NULL/WM_PRINT messages and check visibility.
                if let Ok(guard) = wake.0.lock() {
                    let _ = wake.1.wait_timeout(guard, Duration::from_millis(80));
                }
                next_frame_at = Instant::now();
                continue;
            }

            if !read_next_frame(
                &reader,
                out_kind,
                &mut vid_w,
                &mut vid_h,
                &mut stride,
                &mut raw_frame,
            ) {
                tracing::error!(
                    frame_counter,
                    "wmf: playback loop stopped after read_next_frame failure"
                );
                break;
            }

            if frame_counter.is_multiple_of(30) {
                scale_cover_bgra(
                    &raw_frame,
                    vid_w as usize,
                    vid_h as usize,
                    &mut thumb,
                    THUMB_W,
                    THUMB_H,
                );
                if let Ok(mut slot) = latest_frame.lock() {
                    *slot = Some(VideoFrame {
                        width: THUMB_W as u32,
                        height: THUMB_H as u32,
                        bgra: thumb.clone(),
                    });
                }
            }

            if let Ok(mut surf) = surface.lock() {
                surf.out_w = win_w;
                surf.out_h = win_h;
                surf.width = vid_w as i32;
                surf.height = vid_h as i32;
                // The previous frame's buffer comes back as the next decode target: the
                // hand-off costs no copy at all.
                std::mem::swap(&mut surf.bgra, &mut raw_frame);
            }
            frame_counter = frame_counter.wrapping_add(1);

            present_hwnd(hwnd);

            next_frame_at += frame_interval;
            let now = Instant::now();
            if next_frame_at > now {
                let wait = next_frame_at - now;
                if let Ok(guard) = wake.0.lock() {
                    let _ = wake.1.wait_timeout(guard, wait);
                }
            } else {
                next_frame_at = now;
            }
        }

        tracing::info!(frame_counter, "wmf: shutting down video wallpaper window");
        SetWindowLongPtrW(hwnd, GWLP_USERDATA, 0);
        let _ = ShowWindow(hwnd, SW_HIDE);
        let _ = SetParent(crate::wmf_bindings::HWND(hwnd.0), None);
        let _ = DestroyWindow(hwnd);
        let _ = InvalidateRect(Some(host), None, true);
        let progman = desktop::shell_window();
        if !progman.0.is_null() {
            let _ = InvalidateRect(Some(progman), None, true);
        }
    }
}

unsafe fn pump_thread_messages() {
    let mut msg = MSG::default();
    // SAFETY: standard non-blocking message pump for this thread's windows.
    unsafe {
        while PeekMessageW(&mut msg, None, 0, 0, PM_REMOVE as u32).as_bool() {
            let _ = TranslateMessage(&msg);
            DispatchMessageW(&msg);
        }
    }
}

/// Scales `src` (`src_w` x `src_h`) into `dst` (`dst_w` x `dst_h`) using aspect-fill (cover).
pub fn scale_cover_bgra(
    src: &[u8],
    src_w: usize,
    src_h: usize,
    dst: &mut Vec<u8>,
    dst_w: usize,
    dst_h: usize,
) {
    if src_w == 0 || src_h == 0 || dst_w == 0 || dst_h == 0 {
        return;
    }
    dst.resize(dst_w * dst_h * 4, 255);
    if src_w == dst_w && src_h == dst_h && src.len() == dst.len() {
        dst.copy_from_slice(src);
        return;
    }

    // Cover crop in source coordinates.
    let (crop_x, crop_y, crop_w, crop_h) = if src_w * dst_h > dst_w * src_h {
        let cw = (src_h * dst_w / dst_h).max(1).min(src_w);
        ((src_w - cw) / 2, 0, cw, src_h)
    } else {
        let ch = (src_w * dst_h / dst_w).max(1).min(src_h);
        (0, (src_h - ch) / 2, src_w, ch)
    };

    for dy in 0..dst_h {
        let sy = (crop_y + dy * crop_h / dst_h).min(src_h - 1);
        let src_row = &src[sy * src_w * 4..(sy + 1) * src_w * 4];
        let dst_row = &mut dst[dy * dst_w * 4..(dy + 1) * dst_w * 4];
        for dx in 0..dst_w {
            let sx = (crop_x + dx * crop_w / dst_w).min(src_w - 1);
            let s = sx * 4;
            let d = dx * 4;
            dst_row[d] = src_row[s];
            dst_row[d + 1] = src_row[s + 1];
            dst_row[d + 2] = src_row[s + 2];
            dst_row[d + 3] = 255;
        }
    }
}

/// Encodes a top-down 32bpp BGRA buffer (`width` x `height`) into JPEG bytes via WIC.
pub fn encode_jpeg(width: u32, height: u32, bgra: &[u8]) -> Result<Vec<u8>> {
    if width == 0 || height == 0 || bgra.len() < (width * height * 4) as usize {
        return Err(Error::from_hresult(E_INVALIDARG));
    }
    let _com = ComThreadGuard::init();
    let tmp_path = std::env::temp_dir().join(format!(
        "pecofence-preview-{}-{}.jpg",
        std::process::id(),
        uuid::Uuid::new_v4()
    ));
    let wide_path = to_wide(&tmp_path.to_string_lossy());

    // Convert 32bpp BGRA to 24bpp BGR (stride aligned to 4 bytes) for the JPEG encoder.
    let stride = (width * 3 + 3) & !3;
    let mut bgr24 = vec![0u8; (stride * height) as usize];
    for y in 0..height as usize {
        let src_row = &bgra[y * (width as usize) * 4..(y + 1) * (width as usize) * 4];
        let dst_row = &mut bgr24[y * (stride as usize)..(y + 1) * (stride as usize)];
        for x in 0..width as usize {
            dst_row[x * 3] = src_row[x * 4];
            dst_row[x * 3 + 1] = src_row[x * 4 + 1];
            dst_row[x * 3 + 2] = src_row[x * 4 + 2];
        }
    }

    // SAFETY: standard WIC stream + JPEG encoder pipeline; file is closed before reading back.
    let encode_res: Result<()> = unsafe {
        let factory: IWICImagingFactory =
            CoCreateInstance(&CLSID_WICImagingFactory, None, CLSCTX_INPROC_SERVER)?;
        let stream: crate::wmf_bindings::IWICStream = factory.CreateStream()?.cast()?;
        stream
            .InitializeFromFilename(PCWSTR(wide_path.as_ptr()), GENERIC_WRITE as u32)
            .ok()?;
        let encoder: IWICBitmapEncoder = factory
            .CreateEncoder(&GUID_ContainerFormatJpeg, core::ptr::null())?
            .cast()?;
        let istream: crate::wmf_bindings::IStream = stream.cast()?;
        encoder.Initialize(&istream, WICBitmapEncoderNoCache).ok()?;
        let mut frame: Option<IWICBitmapFrameEncode> = None;
        encoder
            .CreateNewFrame(&mut frame, core::ptr::null_mut())
            .ok()?;
        let frame = frame.ok_or_else(|| Error::from_hresult(E_FAIL))?;
        frame.Initialize(None).ok()?;
        frame.SetSize(width, height).ok()?;
        let mut pf = GUID_WICPixelFormat24bppBGR;
        frame.SetPixelFormat(&mut pf).ok()?;
        frame
            .WritePixels(height, stride, bgr24.len() as u32, bgr24.as_ptr())
            .ok()?;
        frame.Commit().ok()?;
        encoder.Commit().ok()?;
        Ok(())
    };
    encode_res?;
    let bytes = std::fs::read(&tmp_path).map_err(|_| Error::from_hresult(E_FAIL))?;
    let _ = std::fs::remove_file(&tmp_path);
    Ok(bytes)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn jpeg_encoder_produces_valid_jfif_bytes() {
        let width = 32u32;
        let height = 18u32;
        let mut bgra = vec![0u8; (width * height * 4) as usize];
        for (i, px) in bgra.as_chunks_mut::<4>().0.iter_mut().enumerate() {
            px[0] = (i & 0xFF) as u8;
            px[1] = 120;
            px[2] = 200;
            px[3] = 255;
        }
        let jpg = encode_jpeg(width, height, &bgra).expect("WIC JPEG encode");
        assert!(jpg.len() > 64);
        assert_eq!(&jpg[..2], &[0xFF, 0xD8]);
        assert_eq!(&jpg[jpg.len() - 2..], &[0xFF, 0xD9]);
    }

    #[test]
    fn wmf_memory_attribution() {
        let path = Path::new(r"C:\Users\yahya\Downloads\WALL\Video Project.mp4");
        if !path.is_file() {
            return;
        }
        let mb = || {
            crate::memstats::MemoryStats::current()
                .map(|m| m.private_commit as f64 / (1024.0 * 1024.0))
                .unwrap_or(0.0)
        };
        eprintln!("commit before com/mf: {:.1} MB", mb());
        let _com = ComThreadGuard::init().unwrap();
        let _mf = MfSessionGuard::init().unwrap();
        eprintln!("commit after MFStartup: {:.1} MB", mb());
        {
            let (reader, mut w, mut h, mut s, _, kind) = open_h264_reader(path).unwrap();
            let opened = mb();
            eprintln!("commit after open_h264_reader ({kind:?}): {opened:.1} MB");
            let mut bgra = Vec::new();
            unsafe { read_next_frame(&reader, kind, &mut w, &mut h, &mut s, &mut bgra) };
            eprintln!("commit after frame 1: {:.1} MB", mb());
            for _ in 0..30 {
                unsafe { read_next_frame(&reader, kind, &mut w, &mut h, &mut s, &mut bgra) };
            }
            let after_31 = mb();
            eprintln!("commit after frame 31: {after_31:.1} MB");
            drop(reader);
            // The decoder's internal queue must stay bounded: MF_LOW_LATENCY keeps it to
            // tens of MB; the unbounded queue used to pass +190 MB after 31 frames.
            assert!(
                after_31 - opened < 120.0,
                "decoder queue grew unboundedly: {after_31:.1} - {opened:.1} MB"
            );
        }
        eprintln!("commit after reader drop: {:.1} MB", mb());
    }

    #[test]
    fn com_thread_guard_succeeds_even_on_sta_threads() {
        let _ole = crate::com::OleGuard::init().unwrap();
        let _guard = ComThreadGuard::init().expect("ComThreadGuard must tolerate STA");
    }

    #[test]
    fn nv12_neutral_chroma_decodes_as_gray() {
        // 4x2 pixels, pitch 4: luma values 16..235 with neutral (128,128) chroma must come
        // out as R=G=B (no colour tint) with alpha 255.
        let (w, h, pitch) = (4usize, 2usize, 4usize);
        let mut planes = vec![0u8; pitch * h + pitch * (h / 2)];
        for y in 0..h {
            for x in 0..w {
                planes[y * pitch + x] = (16 + y * w * 20 + x * 20) as u8;
            }
        }
        for b in &mut planes[pitch * h..] {
            *b = 128;
        }
        let mut bgra = vec![0u8; w * h * 4];
        unsafe { nv12_to_bgra(planes.as_ptr(), pitch, w, h, &mut bgra) };
        for (i, px) in bgra.as_chunks::<4>().0.iter().enumerate() {
            let (y_exp, _) = (i / w, i % w);
            let y = 16 + y_exp * w * 20 + (i % w) * 20;
            let gray = ((298 * (y as i32 - 16) + 128) >> 8).clamp(0, 255) as u8;
            assert_eq!(&px[..3], &[gray; 3], "pixel {i} not neutral gray");
            assert_eq!(px[3], 255);
        }
        // Saturated chroma must differ across channels (a real colour, not gray).
        planes[pitch * h] = 255; // U max
        planes[pitch * h + 1] = 128;
        unsafe { nv12_to_bgra(planes.as_ptr(), pitch, w, h, &mut bgra) };
        assert!(bgra[0] > bgra[2], "U should push blue above red");
    }
}
