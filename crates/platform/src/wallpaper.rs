//! Per-monitor wallpaper discovery (`IDesktopWallpaper`) and WIC decoding to a small BGRA buffer.

use crate::bindings::*;
use crate::wide::to_wide;
use std::path::{Path, PathBuf};
use windows_core::{Interface, PCWSTR, Result};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Position {
    Center,
    Tile,
    Stretch,
    Fit,
    Fill,
    Span,
}

#[derive(Clone, Debug)]
pub struct MonitorWallpaper {
    /// Device path id used by `IDesktopWallpaper` (`\\?\DISPLAY#...`).
    pub monitor_id: String,
    /// Monitor rectangle in virtual-screen coordinates.
    pub rect: RECT,
    /// Image path, if a picture wallpaper is set for this monitor: Windows' own copy of the
    /// picture when the file it was set from is gone.
    pub path: Option<PathBuf>,
}

#[derive(Clone, Debug)]
pub struct WallpaperSnapshot {
    pub position: Position,
    /// Solid background colour (RGB) shown where the picture does not cover.
    pub background: [u8; 3],
    pub monitors: Vec<MonitorWallpaper>,
}

fn take_string(pwstr: windows_core::PWSTR) -> String {
    if pwstr.is_null() {
        return String::new();
    }
    // SAFETY: the shell allocated this NUL-terminated string with CoTaskMemAlloc.
    unsafe {
        let s = pwstr.to_string().unwrap_or_default();
        CoTaskMemFree(pwstr.0.cast());
        s
    }
}

impl WallpaperSnapshot {
    /// Fingerprint the same snapshot used to build the backdrops, including monitor geometry.
    /// File metadata is re-read so callers can reject a file changed during decoding.
    pub fn signature(&self) -> String {
        let mut sig = format!("{:?}|{:?}", self.position, self.background);
        for m in &self.monitors {
            sig.push('|');
            sig.push_str(&m.monitor_id);
            sig.push_str(&format!(
                "@{},{},{},{}",
                m.rect.left, m.rect.top, m.rect.right, m.rect.bottom
            ));
            if let Some(p) = &m.path {
                sig.push('=');
                sig.push_str(&p.to_string_lossy());
                if let Ok(meta) = std::fs::metadata(p) {
                    use std::os::windows::fs::MetadataExt;
                    sig.push_str(&format!("#{}#{}", meta.file_size(), meta.last_write_time()));
                }
            }
        }
        sig
    }
}

/// Queries a fingerprint of the current wallpaper. Requires COM on this thread.
pub fn signature() -> Option<String> {
    Some(query().ok()?.signature())
}

/// A cheap hint for detecting missed desktop notifications. Never use it as the wallpaper
/// itself: Explorer may publish the new picture slightly after changing this value.
pub fn desktop_id() -> Option<[u8; 16]> {
    let mut id = [0u8; 16];
    let mut size = id.len() as u32;
    // RRF_RT_REG_BINARY constrains the type; RegGetValueW opens the current key on each read,
    // so this also survives Explorer replacing a key that a registry watcher had opened.
    let status = unsafe {
        RegGetValueW(
            HKEY_CURRENT_USER,
            windows_core::w!(r"Software\Microsoft\Windows\CurrentVersion\Explorer\VirtualDesktops"),
            windows_core::w!("CurrentVirtualDesktop"),
            0x08,
            None,
            Some(id.as_mut_ptr().cast()),
            Some(&mut size),
        )
    };
    (status.0 == 0 && size == 16).then_some(id)
}

const DESKTOPS_KEY: &str = r"Software\Microsoft\Windows\CurrentVersion\Explorer\VirtualDesktops";

/// Reads a registry value of `kind` (RRF_RT_*) into a byte buffer.
fn registry_value(key: &str, value: &str, kind: u32) -> Option<Vec<u8>> {
    let key = to_wide(key);
    let value = to_wide(value);
    let mut size = 0u32;
    // SAFETY: size query, then a read into a buffer of that size; strings outlive the calls.
    unsafe {
        let status = RegGetValueW(
            HKEY_CURRENT_USER,
            PCWSTR(key.as_ptr()),
            PCWSTR(value.as_ptr()),
            kind,
            None,
            None,
            Some(&mut size),
        );
        if status.0 != 0 || size == 0 {
            return None;
        }
        let mut data = vec![0u8; size as usize];
        let status = RegGetValueW(
            HKEY_CURRENT_USER,
            PCWSTR(key.as_ptr()),
            PCWSTR(value.as_ptr()),
            kind,
            None,
            Some(data.as_mut_ptr().cast()),
            Some(&mut size),
        );
        (status.0 == 0).then(|| {
            data.truncate(size as usize);
            data
        })
    }
}

/// `{XXXXXXXX-XXXX-XXXX-XXXX-XXXXXXXXXXXX}` for a GUID stored in its binary layout.
fn guid_string(id: &[u8; 16]) -> String {
    let d1 = u32::from_le_bytes([id[0], id[1], id[2], id[3]]);
    let d2 = u16::from_le_bytes([id[4], id[5]]);
    let d3 = u16::from_le_bytes([id[6], id[7]]);
    let tail: String = id[8..].iter().map(|b| format!("{b:02X}")).collect();
    format!(
        "{{{d1:08X}-{d2:04X}-{d3:04X}-{}-{}}}",
        &tail[..4],
        &tail[4..]
    )
}

/// Every virtual desktop in task-view order with the picture Explorer keeps for it (`None`
/// when the desktop has no picture of its own). Like [`desktop_id`], an Explorer detail: a
/// hint for preparing backdrops, never what is displayed.
pub fn desktop_wallpapers() -> Vec<([u8; 16], Option<PathBuf>)> {
    const RRF_RT_REG_SZ: u32 = 0x02;
    const RRF_RT_REG_BINARY: u32 = 0x08;
    let Some(ids) = registry_value(DESKTOPS_KEY, "VirtualDesktopIDs", RRF_RT_REG_BINARY) else {
        return Vec::new();
    };
    ids.as_chunks::<16>()
        .0
        .iter()
        .map(|chunk| {
            let id = *chunk;
            let key = format!(r"{DESKTOPS_KEY}\Desktops\{}", guid_string(&id));
            let path =
                registry_value(&key, "Wallpaper", RRF_RT_REG_SZ).and_then(|data| utf16_path(&data));
            (id, path)
        })
        .collect()
}

/// The NUL-terminated UTF-16LE path at the start of `data`, `None` when it is empty.
fn utf16_path(data: &[u8]) -> Option<PathBuf> {
    let wide: Vec<u16> = data
        .as_chunks::<2>()
        .0
        .iter()
        .map(|c| u16::from_le_bytes([c[0], c[1]]))
        .take_while(|&c| c != 0)
        .collect();
    (!wide.is_empty()).then(|| PathBuf::from(String::from_utf16_lossy(&wide)))
}

/// The copy Windows made of the picture `path` when it was set as the wallpaper, for a
/// picture deleted or moved since (a wallpaper app that set it from a temporary file): the
/// desktop keeps showing that copy. `Themes\TranscodedWallpaper` is the picture of all
/// monitors, `Themes\Transcoded_000`, `_001` … one per monitor when they show different
/// pictures; `Control Panel\Desktop\TranscodedImageCache` (`_000` …) records each copy's
/// source path, in UTF-16 from byte 24.
fn windows_copy(path: &Path) -> Option<PathBuf> {
    const RRF_RT_REG_BINARY: u32 = 0x08;
    let themes = PathBuf::from(std::env::var_os("APPDATA")?).join(r"Microsoft\Windows\Themes");
    let wanted = path.to_string_lossy();
    std::fs::read_dir(themes).ok()?.flatten().find_map(|entry| {
        let name = entry.file_name().into_string().ok()?;
        let value = match name.strip_prefix("Transcoded_") {
            Some(index) => format!("TranscodedImageCache_{index}"),
            None if name == "TranscodedWallpaper" => "TranscodedImageCache".into(),
            None => return None,
        };
        let data = registry_value(r"Control Panel\Desktop", &value, RRF_RT_REG_BINARY)?;
        let source = utf16_path(data.get(24..)?)?;
        source
            .to_string_lossy()
            .eq_ignore_ascii_case(&wanted)
            .then(|| entry.path())
    })
}

/// The virtual desktop `hwnd` is on, `None` when Explorer does not say (the desktop itself,
/// windows shown on every desktop, a failed call). Requires COM on this thread.
pub fn window_desktop(hwnd: HWND) -> Option<u128> {
    use std::mem::ManuallyDrop;
    // Never released at exit: thread-local destructors run inside ExitProcess, where releasing
    // Explorer's object waits for an RPC reply that never comes. The process then hangs for
    // good, holding the single-instance mutex, and PecoFence cannot start again.
    thread_local! {
        static MANAGER: std::cell::RefCell<Option<ManuallyDrop<IVirtualDesktopManager>>> =
            const { std::cell::RefCell::new(None) };
    }
    MANAGER.with_borrow_mut(|manager| {
        if manager.is_none() {
            // SAFETY: standard COM activation.
            *manager = unsafe { CoCreateInstance(&VirtualDesktopManager, None, CLSCTX_ALL as u32) }
                .ok()
                .map(ManuallyDrop::new);
        }
        // SAFETY: COM call on a live interface.
        match unsafe { manager.as_ref()?.GetWindowDesktopId(hwnd) } {
            Ok(id) => Some(id.to_u128()).filter(|&id| id != 0),
            Err(_) => {
                // Explorer restarted, perhaps: ask a new object next time.
                drop(manager.take().map(ManuallyDrop::into_inner));
                None
            }
        }
    })
}

/// Whether a virtual-desktop switch can change the picture: more than one desktop, and they
/// do not all keep the same one (one without a picture of its own differs from one with;
/// Windows 10 keeps none per desktop). A hint like [`desktop_wallpapers`].
pub fn desktops_differ() -> bool {
    let desktops = desktop_wallpapers();
    desktops.len() > 1 && desktops.iter().any(|(_, path)| *path != desktops[0].1)
}

/// Watch both wallpaper settings and Explorer's virtual-desktop state. The latter is only
/// an opportunistic trigger (an Explorer implementation detail); callers must keep a poll
/// fallback and re-query IDesktopWallpaper before displaying anything.
pub fn watch_changes(on_change: impl Fn() + Send + 'static) -> Result<()> {
    // The two watcher threads serialize access to a callback that need only be Send.
    let on_change = std::sync::Arc::new(std::sync::Mutex::new(on_change));
    let mut result = Err(windows_core::Error::from_hresult(E_FAIL));
    for (path, subtree, name) in [
        (r"Control Panel\Desktop", false, "pecofence-wallpaper-watch"),
        (
            r"Software\Microsoft\Windows\CurrentVersion\Explorer\VirtualDesktops",
            true,
            "pecofence-desktop-watch",
        ),
    ] {
        let callback = on_change.clone();
        match watch_registry_key(path, subtree, name, move || {
            if let Ok(callback) = callback.lock() {
                callback();
            }
        }) {
            Ok(()) => result = Ok(()),
            Err(error) => tracing::warn!(path, %error, "wallpaper change source unavailable"),
        }
    }
    result
}

/// What `RegNotifyChangeKeyValue` returns once the watched key was deleted.
const ERROR_KEY_DELETED: i32 = 1018;

fn open_notify_key(path: &[u16]) -> Option<HKEY> {
    let mut key = HKEY::default();
    // SAFETY: out-pointer references a local; the string outlives the call.
    let status = unsafe {
        RegOpenKeyExW(
            HKEY_CURRENT_USER,
            PCWSTR(path.as_ptr()),
            None,
            ACCESS_MASK(KEY_NOTIFY as u32),
            &mut key,
        )
    };
    (status.0 == 0).then_some(key)
}

fn watch_registry_key(
    path: &str,
    subtree: bool,
    name: &str,
    on_change: impl Fn() + Send + 'static,
) -> Result<()> {
    let key_path = to_wide(path);
    let Some(key) = open_notify_key(&key_path) else {
        return Err(windows_core::Error::from_hresult(E_FAIL));
    };
    let raw = key.0 as isize;
    let thread = std::thread::Builder::new()
        .name(name.into())
        .stack_size(64 * 1024)
        .spawn(move || {
            let mut key = HKEY(raw as *mut core::ffi::c_void);
            tracing::debug!(?key, subtree, "wallpaper registry watcher started");
            loop {
                // SAFETY: synchronous wait on a key we opened; returns when a value changes.
                let st = unsafe {
                    RegNotifyChangeKeyValue(
                        key,
                        subtree,
                        REG_NOTIFY_CHANGE_LAST_SET as u32,
                        None,
                        false,
                    )
                };
                if st.0 == 0 {
                    tracing::debug!(?key, "wallpaper registry change received");
                    on_change();
                    continue;
                }
                // SAFETY: closing the key we opened.
                unsafe {
                    let _ = RegCloseKey(key);
                }
                if st.0 != ERROR_KEY_DELETED {
                    tracing::warn!(status = st.0, ?key, "wallpaper registry watcher stopped");
                    break;
                }
                // Explorer replaced the key: watch the one that is there now, and report the
                // changes missed meanwhile.
                key = loop {
                    std::thread::sleep(std::time::Duration::from_secs(2));
                    if let Some(key) = open_notify_key(&key_path) {
                        break key;
                    }
                };
                tracing::info!(?key, "wallpaper registry watcher reopened its key");
                on_change();
            }
        });
    if thread.is_err() {
        // No thread took ownership of the key when spawning failed.
        unsafe {
            let _ = RegCloseKey(key);
        }
    }
    thread
        .map(|_| ())
        .map_err(|_| windows_core::Error::from_hresult(E_FAIL))
}

pub fn query() -> Result<WallpaperSnapshot> {
    // SAFETY: standard COM activation.
    let dw: IDesktopWallpaper =
        unsafe { CoCreateInstance(&DesktopWallpaper, None, CLSCTX_ALL as u32)? };
    // SAFETY: COM calls on a live interface.
    unsafe {
        let position = match dw.GetPosition()? {
            DWPOS_CENTER => Position::Center,
            DWPOS_TILE => Position::Tile,
            DWPOS_STRETCH => Position::Stretch,
            DWPOS_FIT => Position::Fit,
            DWPOS_SPAN => Position::Span,
            _ => Position::Fill,
        };
        let bg = dw.GetBackgroundColor()?.0;
        let background = [
            (bg & 0xff) as u8,
            ((bg >> 8) & 0xff) as u8,
            ((bg >> 16) & 0xff) as u8,
        ];

        let count = dw.GetMonitorDevicePathCount()?;
        let mut monitors = Vec::with_capacity(count as usize);
        for i in 0..count {
            let id = take_string(dw.GetMonitorDevicePathAt(i)?);
            let id_w = to_wide(&id);
            // Explorer retains disconnected outputs and returns E_FAIL for their rectangle.
            // They are not part of the visible wallpaper and must not fail the whole read.
            let rect = dw.GetMonitorRECT(PCWSTR(id_w.as_ptr())).unwrap_or_default();
            // A failed query on a live monitor is not a solid-colour wallpaper. Preserve
            // that distinction so a switch-in-progress cannot replace good pixels with black.
            let path = if rect.right > rect.left && rect.bottom > rect.top {
                let path = take_string(dw.GetWallpaper(PCWSTR(id_w.as_ptr()))?);
                (!path.is_empty()).then(|| {
                    let path = PathBuf::from(path);
                    if path.is_file() {
                        path
                    } else {
                        windows_copy(&path).unwrap_or(path)
                    }
                })
            } else {
                None
            };
            monitors.push(MonitorWallpaper {
                monitor_id: id,
                rect,
                path,
            });
        }
        Ok(WallpaperSnapshot {
            position,
            background,
            monitors,
        })
    }
}

/// `IDesktopWallpaper::SetWallpaper` for one monitor — Explorer's own per-monitor route;
/// it does not broadcast `SPI_SETDESKWALLPAPER`. `monitor_id` is the `\\?\DISPLAY#…`
/// device path a [`query`] snapshot reports.
pub fn set_monitor_wallpaper(monitor_id: &str, path: &Path) -> Result<()> {
    // SAFETY: standard COM activation.
    let dw: IDesktopWallpaper =
        unsafe { CoCreateInstance(&DesktopWallpaper, None, CLSCTX_ALL as u32)? };
    let id_w = to_wide(monitor_id);
    let path_w = path.as_os_str().encode_wide_nul();
    // SAFETY: both pointers are NUL-terminated buffers that outlive the call.
    unsafe { dw.SetWallpaper(PCWSTR(id_w.as_ptr()), PCWSTR(path_w.as_ptr())) }.ok()
}

/// `DWPOS_FILL` for every monitor — a wallpaper composited at exact monitor pixels lands
/// 1:1 only under Fill; Center/Tile would shift or repeat it.
pub fn set_position_fill() -> Result<()> {
    // SAFETY: standard COM activation.
    let dw: IDesktopWallpaper =
        unsafe { CoCreateInstance(&DesktopWallpaper, None, CLSCTX_ALL as u32)? };
    // SAFETY: COM call on a live interface.
    unsafe { dw.SetPosition(DWPOS_FILL) }.ok()
}

/// Decoded opaque BGRA pixels.
#[derive(Clone, Debug)]
pub struct DecodedImage {
    pub width: u32,
    pub height: u32,
    pub bgra: Vec<u8>,
}

/// Just the image's own pixel size — decoder + frame `GetSize`, no pixel copy.
pub fn image_dimensions(path: &Path) -> Result<(u32, u32)> {
    let wide: Vec<u16> = path.as_os_str().encode_wide_nul();
    // SAFETY: standard WIC pipeline; every interface is owned for the call.
    unsafe {
        let factory: IWICImagingFactory =
            CoCreateInstance(&CLSID_WICImagingFactory, None, CLSCTX_INPROC_SERVER)?;
        let decoder = factory.CreateDecoderFromFilename(
            PCWSTR(wide.as_ptr()),
            core::ptr::null(),
            GENERIC_READ,
            WICDecodeMetadataCacheOnDemand,
        )?;
        let frame = decoder.GetFrame(0)?;
        let src: IWICBitmapSource = frame.cast()?;
        let (mut w, mut h) = (0u32, 0u32);
        src.GetSize(&mut w, &mut h).ok()?;
        if w == 0 || h == 0 {
            return Err(windows_core::Error::from_hresult(E_FAIL));
        }
        Ok((w, h))
    }
}

/// Decodes `path` to exactly `width` x `height`, aspect ratio ignored — the anisotropic
/// stretch the adaptive theme engine uses for both the blurred underlay and the
/// capped-stretch asset.
pub fn decode_stretched(path: &Path, width: u32, height: u32) -> Result<DecodedImage> {
    let (w, h) = (width.max(1), height.max(1));
    decode(path, |_, _| ((w, h), [0, 0, w, h]))
}

/// Decodes `path` and scales it (preserving aspect ratio) so it fits within
/// `max_width` x `max_height`. The scaler is attached directly to the frame so the full-size
/// bitmap is never materialized.
pub fn decode_scaled(path: &Path, max_width: u32, max_height: u32) -> Result<DecodedImage> {
    decode(path, |w, h| {
        let scale = (max_width as f32 / w as f32)
            .min(max_height as f32 / h as f32)
            .min(1.0);
        let dst_w = ((w as f32 * scale).round() as u32).max(1);
        let dst_h = ((h as f32 * scale).round() as u32).max(1);
        ((dst_w, dst_h), [0, 0, dst_w, dst_h])
    })
}

/// Decodes `path` at its own size and keeps at most `max_width` x `max_height` pixels of it:
/// the middle when `centered`, else the top-left corner. That is all a monitor of that size
/// shows of a centred or a tiled picture.
pub fn decode_native(
    path: &Path,
    max_width: u32,
    max_height: u32,
    centered: bool,
) -> Result<DecodedImage> {
    decode(path, |w, h| {
        let (cw, ch) = (w.min(max_width.max(1)), h.min(max_height.max(1)));
        let (x, y) = if centered {
            ((w - cw) / 2, (h - ch) / 2)
        } else {
            (0, 0)
        };
        ((w, h), [x, y, cw, ch])
    })
}

/// The WIC pipeline of both: `size` maps the picture's size to the size it is scaled to and
/// the part of the scaled picture (x, y, width, height) that is copied out.
fn decode(
    path: &Path,
    size: impl FnOnce(u32, u32) -> ((u32, u32), [u32; 4]),
) -> Result<DecodedImage> {
    let wide: Vec<u16> = path.as_os_str().encode_wide_nul();
    // SAFETY: standard WIC pipeline; every interface is owned for the duration of the call.
    unsafe {
        let factory: IWICImagingFactory =
            CoCreateInstance(&CLSID_WICImagingFactory, None, CLSCTX_INPROC_SERVER)?;
        let decoder = factory.CreateDecoderFromFilename(
            PCWSTR(wide.as_ptr()),
            core::ptr::null(),
            GENERIC_READ,
            WICDecodeMetadataCacheOnDemand,
        )?;
        let frame = decoder.GetFrame(0)?;
        let frame_src: IWICBitmapSource = frame.cast()?;
        let (mut w, mut h) = (0u32, 0u32);
        frame_src.GetSize(&mut w, &mut h).ok()?;
        if w == 0 || h == 0 {
            return Err(windows_core::Error::from_hresult(E_FAIL));
        }
        let ((dst_w, dst_h), [x, y, out_w, out_h]) = size(w, h);

        let scaler = factory.CreateBitmapScaler()?;
        scaler
            .Initialize(&frame_src, dst_w, dst_h, WICBitmapInterpolationModeFant)
            .ok()?;
        let converter = factory.CreateFormatConverter()?;
        converter
            .Initialize(
                &scaler,
                &GUID_WICPixelFormat32bppPBGRA,
                WICBitmapDitherTypeNone,
                None,
                0.0,
                WICBitmapPaletteTypeMedianCut,
            )
            .ok()?;
        let source: IWICBitmapSource = converter.cast()?;
        let stride = out_w * 4;
        let mut bgra = vec![0u8; (stride * out_h) as usize];
        let rect = WICRect {
            X: x as i32,
            Y: y as i32,
            Width: out_w as i32,
            Height: out_h as i32,
        };
        // The generated wrapper mis-projects CopyPixels' buffer as a return value; call the
        // vtable slot directly.
        (Interface::vtable(&source).CopyPixels)(
            Interface::as_raw(&source),
            &rect,
            stride,
            bgra.len() as u32,
            bgra.as_mut_ptr(),
        )
        .ok()?;
        Ok(DecodedImage {
            width: out_w,
            height: out_h,
            bgra,
        })
    }
}

/// Separable box blur on premultiplied BGRA, `passes` horizontal+vertical rounds of
/// `radius` — three passes read as a gaussian. Edges clamp by index saturation.
fn blur_bgra(bgra: &mut [u8], w: usize, h: usize, radius: usize, passes: usize) {
    if w == 0 || h == 0 || radius == 0 {
        return;
    }
    let mut tmp = vec![0u8; bgra.len()];
    for _ in 0..passes {
        // Horizontal.
        for y in 0..h {
            let row = y * w;
            for x in 0..w {
                let mut acc = [0u32; 4];
                let lo = x.saturating_sub(radius);
                let hi = (x + radius).min(w - 1);
                for sx in lo..=hi {
                    let s = (row + sx) * 4;
                    acc[0] += bgra[s] as u32;
                    acc[1] += bgra[s + 1] as u32;
                    acc[2] += bgra[s + 2] as u32;
                    acc[3] += bgra[s + 3] as u32;
                }
                let n = (hi - lo + 1) as u32;
                let d = (row + x) * 4;
                tmp[d] = (acc[0] / n) as u8;
                tmp[d + 1] = (acc[1] / n) as u8;
                tmp[d + 2] = (acc[2] / n) as u8;
                tmp[d + 3] = (acc[3] / n) as u8;
            }
        }
        // Vertical.
        for x in 0..w {
            for y in 0..h {
                let mut acc = [0u32; 4];
                let lo = y.saturating_sub(radius);
                let hi = (y + radius).min(h - 1);
                for sy in lo..=hi {
                    let s = (sy * w + x) * 4;
                    acc[0] += tmp[s] as u32;
                    acc[1] += tmp[s + 1] as u32;
                    acc[2] += tmp[s + 2] as u32;
                    acc[3] += tmp[s + 3] as u32;
                }
                let n = (hi - lo + 1) as u32;
                let d = (y * w + x) * 4;
                bgra[d] = (acc[0] / n) as u8;
                bgra[d + 1] = (acc[1] / n) as u8;
                bgra[d + 2] = (acc[2] / n) as u8;
                bgra[d + 3] = (acc[3] / n) as u8;
            }
        }
    }
}

/// Bilinear-resample BGRA `src` (sw×sh) into a new dw×dh buffer.
fn bilinear_resize(src: &[u8], sw: usize, sh: usize, dw: usize, dh: usize) -> Vec<u8> {
    let mut out = vec![0u8; dw * dh * 4];
    for y in 0..dh {
        let fy = (y as f32 + 0.5) * sh as f32 / dh as f32 - 0.5;
        let y0 = fy.floor().max(0.0) as usize;
        let y1 = (y0 + 1).min(sh - 1);
        let ty = (fy - y0 as f32).clamp(0.0, 1.0);
        for x in 0..dw {
            let fx = (x as f32 + 0.5) * sw as f32 / dw as f32 - 0.5;
            let x0 = fx.floor().max(0.0) as usize;
            let x1 = (x0 + 1).min(sw - 1);
            let tx = (fx - x0 as f32).clamp(0.0, 1.0);
            let d = (y * dw + x) * 4;
            for c in 0..4 {
                let s00 = src[(y0 * sw + x0) * 4 + c] as f32;
                let s10 = src[(y0 * sw + x1) * 4 + c] as f32;
                let s01 = src[(y1 * sw + x0) * 4 + c] as f32;
                let s11 = src[(y1 * sw + x1) * 4 + c] as f32;
                let v = s00 * (1.0 - tx) * (1.0 - ty)
                    + s10 * tx * (1.0 - ty)
                    + s01 * (1.0 - tx) * ty
                    + s11 * tx * ty;
                out[d + c] = v.round().clamp(0.0, 255.0) as u8;
            }
        }
    }
    out
}

/// The adaptive theme engine's Contain composite for one monitor: the asset stretched
/// to the full `dst` frame and heavily blurred as the base layer, with the uncropped
/// asset uniformly scaled to fit drawn centered at `bg` — the classic
/// blurred pillarbox, generated entirely from the same image. `bg` is the
/// `[left, top, right, bottom]` rect `adaptive_fit` returned.
pub fn adaptive_wallpaper(
    src: &Path,
    dst_w: u32,
    dst_h: u32,
    bg: [i32; 4],
) -> Result<DecodedImage> {
    // Blur at 1/8 resolution, then upscale: indistinguishable at pillarbox distances
    // and ~64x cheaper than blurring the full frame.
    let small = decode_stretched(src, (dst_w / 8).max(1), (dst_h / 8).max(1))?;
    let mut px = small.bgra;
    let radius = (small.width.min(small.height) as usize / 20).clamp(6, 48);
    blur_bgra(
        &mut px,
        small.width as usize,
        small.height as usize,
        radius,
        3,
    );
    let mut canvas = bilinear_resize(
        &px,
        small.width as usize,
        small.height as usize,
        dst_w as usize,
        dst_h as usize,
    );
    let (bw, bh) = ((bg[2] - bg[0]).max(1) as u32, (bg[3] - bg[1]).max(1) as u32);
    let fg = decode_stretched(src, bw, bh)?;
    let (dw, dh) = (dst_w as usize, dst_h as usize);
    for y in 0..bh as usize {
        let dy = bg[1] as usize + y;
        if dy >= dh {
            break;
        }
        for x in 0..bw as usize {
            let dx = bg[0] as usize + x;
            if dx >= dw {
                break;
            }
            let s = (y * bw as usize + x) * 4;
            let d = (dy * dw + dx) * 4;
            canvas[d..d + 4].copy_from_slice(&fg.bgra[s..s + 4]);
        }
    }
    Ok(DecodedImage {
        width: dst_w,
        height: dst_h,
        bgra: canvas,
    })
}

/// Blurred-backdrop-only composite from an already-decoded BGRA frame (video first
/// frames — files go through [`adaptive_wallpaper`]): downscale to 1/8, blur, upscale
/// to `dst_w x dst_h`. The result is meant to sit UNDER a video/cover blit — the region
/// the media draws on top of is never visible, so no sharp foreground pass runs.
pub fn blurred_backdrop_bgra(src: &[u8], sw: usize, sh: usize, dw: usize, dh: usize) -> Vec<u8> {
    let (tw, th) = ((dw / 8).max(1), (dh / 8).max(1));
    let mut small = bilinear_resize(src, sw, sh, tw, th);
    let radius = (tw.min(th) / 20).clamp(6, 48);
    blur_bgra(&mut small, tw, th, radius, 3);
    bilinear_resize(&small, tw, th, dw, dh)
}

trait EncodeWideNul {
    fn encode_wide_nul(&self) -> Vec<u16>;
}

impl EncodeWideNul for std::ffi::OsStr {
    fn encode_wide_nul(&self) -> Vec<u16> {
        use std::os::windows::ffi::OsStrExt;
        self.encode_wide().chain(std::iter::once(0)).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn registry_watcher_delivers_a_real_value_change() {
        use std::{sync::mpsc, time::Duration};
        use windows_sys::Win32::System::Registry as registry;
        let path = format!(
            r"Software\PecoFenceWallpaperWatchTest-{}",
            std::process::id()
        );
        let wide_path = to_wide(&path);
        let value = to_wide("change");
        let mut key = core::ptr::null_mut();
        unsafe {
            assert_eq!(
                registry::RegCreateKeyW(registry::HKEY_CURRENT_USER, wide_path.as_ptr(), &mut key,),
                0
            );
        }
        let (send, receive) = mpsc::channel();
        watch_registry_key(&path, false, "wallpaper-test", move || {
            let _ = send.send(());
        })
        .unwrap();
        let mut delivered = false;
        for version in 0..20u32 {
            unsafe {
                assert_eq!(
                    registry::RegSetValueExW(
                        key,
                        value.as_ptr(),
                        0,
                        registry::REG_DWORD,
                        version.to_le_bytes().as_ptr(),
                        4,
                    ),
                    0
                );
            }
            if receive.recv_timeout(Duration::from_millis(50)).is_ok() {
                delivered = true;
                break;
            }
        }
        unsafe {
            registry::RegCloseKey(key);
            registry::RegDeleteTreeW(registry::HKEY_CURRENT_USER, wide_path.as_ptr());
        }
        assert!(
            delivered,
            "real registry writes did not wake the wallpaper watcher"
        );
    }

    #[test]
    fn desktop_ids_format_as_their_registry_key_names() {
        let id = [
            0x39, 0xF8, 0x5B, 0xB3, 0x0F, 0xB5, 0xC7, 0x40, 0xAB, 0x31, 0xB2, 0xF7, 0xDB, 0xD5,
            0x5B, 0x32,
        ];
        assert_eq!(guid_string(&id), "{B35BF839-B50F-40C7-AB31-B2F7DBD55B32}");
    }

    #[test]
    fn cached_wallpaper_tracks_monitor_placement_and_size() {
        let mut snapshot = WallpaperSnapshot {
            position: Position::Fill,
            background: [20, 30, 40],
            monitors: vec![MonitorWallpaper {
                monitor_id: "display-a".into(),
                rect: RECT {
                    left: 0,
                    top: 0,
                    right: 1920,
                    bottom: 1080,
                },
                path: None,
            }],
        };
        let initial = snapshot.signature();
        assert_eq!(initial, snapshot.clone().signature());
        snapshot.monitors[0].rect.left = -1920;
        snapshot.monitors[0].rect.right = 0;
        assert_ne!(initial, snapshot.signature());
        let moved = snapshot.signature();
        snapshot.monitors[0].rect.bottom = 1200;
        assert_ne!(moved, snapshot.signature());
    }

    #[test]
    fn fit_and_solid_colour_changes_invalidate_cached_pixels() {
        let mut snapshot = WallpaperSnapshot {
            position: Position::Fill,
            background: [0, 0, 0],
            monitors: Vec::new(),
        };
        let initial = snapshot.signature();
        snapshot.position = Position::Fit;
        assert_ne!(initial, snapshot.signature());
        let fit = snapshot.signature();
        snapshot.background = [255, 255, 255];
        assert_ne!(fit, snapshot.signature());
    }

    /// `cargo test -p pecofence-platform wall_export_probe -- --ignored --nocapture` —
    /// applies a real image from `C:\Users\yahya\Downloads\WALL` as the desktop wallpaper
    /// (like the user's "switch back to a static image"), then verifies every provider the
    /// theme exporter uses reports that image and that a `.peco` written from it bundles
    /// the image with `video_wallpaper_path` cleared.
    #[test]
    #[ignore = "changes the real desktop wallpaper"]
    fn wall_export_probe() {
        let img = std::env::var("WALL_PROBE_IMAGE")
            .map(std::path::PathBuf::from)
            .unwrap_or_else(|_| {
                std::path::PathBuf::from(
                    r"C:\Users\yahya\Downloads\WALL\Gemini_Generated_Image_91tt0891tt0891tt.jpg",
                )
            });
        assert!(img.is_file(), "WALL asset missing: {}", img.display());
        // Apply it exactly as the theme-import restore path does.
        assert!(crate::sysparams::set_desktop_wallpaper(
            &img.to_string_lossy()
        ));

        // Provider chain as `current_background_source` consults it (no live player).
        let spi = crate::sysparams::desktop_wallpaper();
        eprintln!("SPI_GETDESKWALLPAPER -> {spi:?}");
        let _com = crate::com::OleGuard::init().expect("OLE init");
        let snap = query().expect("IDesktopWallpaper query");
        let via_idw = snap.monitors.iter().find_map(|m| m.path.clone());
        eprintln!("IDesktopWallpaper -> {via_idw:?}");

        let bg = spi
            .as_deref()
            .map(std::path::PathBuf::from)
            .filter(|p| p.is_file())
            .or(via_idw.filter(|p| p.is_file()));
        let bg = bg.expect("a provider must report the static image");
        eprintln!("bundled source = {}", bg.display());
        // The image applied must be what gets bundled (TranscodedWallpaper counts — it is
        // Windows' own copy of the same picture).
        let is_image = bg
            .extension()
            .and_then(|e| e.to_str())
            .is_none_or(|e| !e.eq_ignore_ascii_case("mp4"));
        assert!(is_image, "provider still reports video: {}", bg.display());

        // The export payload path, end to end: entry → archive → extract → verify.
        let mut cfg = pecofence_core::Config::default();
        cfg.settings.video_wallpaper_path = Some(r"C:\stale\video.mp4".into());
        let (name, data) =
            pecofence_core::peco_theme::background_entry(&mut cfg, &bg).expect("bg entry");
        assert!(name.starts_with("background."));
        assert!(!name.ends_with(".mp4"));
        assert_eq!(cfg.settings.video_wallpaper_path, None);

        // The preview decode the export performs on the same source.
        let preview = decode_scaled(&bg, 640, 360).expect("preview decode");
        eprintln!("preview {}x{} from {name}", preview.width, preview.height);

        let cfg_json = pecofence_core::ConfigStore::to_json(&cfg).expect("json");
        assert!(!cfg_json.contains("stale\\video.mp4"));
        let dir = std::env::temp_dir().join("peco-wall-probe");
        let _ = std::fs::create_dir_all(&dir);
        let peco = dir.join("wall-image.peco");
        pecofence_core::peco_theme::write_peco_file(
            &peco,
            &cfg_json,
            None,
            Some((&name, &data)),
            b"\xFF\xD8\xFF\xE0preview",
        )
        .expect("write .peco");
        let ex = pecofence_core::peco_theme::extract_peco_to_temp(&peco).expect("extract");
        let bundled = ex.background_path.expect("background bundled");
        eprintln!("extracted background = {}", bundled.display());
        assert_eq!(std::fs::read(&bundled).unwrap(), data);
        let _ = std::fs::remove_dir_all(&ex.temp_dir);
    }

    /// `cargo test -p pecofence-platform wall_explorer_apply -- --ignored` —
    /// applies a WALL image through `IDesktopWallpaper::SetWallpaper`, the path File
    /// Explorer's "Set as desktop background" takes. Unlike `SystemParametersInfo` this
    /// does not broadcast `SPI_SETDESKWALLPAPER`, so it exercises the app's
    /// snapshot-diff retirement rather than the broadcast hook.
    #[test]
    #[ignore = "changes the real desktop wallpaper"]
    fn wall_explorer_apply_probe() {
        let img = std::env::var("WALL_PROBE_IMAGE")
            .map(std::path::PathBuf::from)
            .unwrap_or_else(|_| {
                std::path::PathBuf::from(
                    r"C:\Users\yahya\Downloads\WALL\A_highly_detailed_photorealistic_interior_of_a_dark_moody_ro.webp",
                )
            });
        assert!(img.is_file(), "WALL asset missing: {}", img.display());
        let _com = crate::com::OleGuard::init().expect("OLE init");
        let dw: IDesktopWallpaper =
            unsafe { CoCreateInstance(&DesktopWallpaper, None, CLSCTX_ALL as u32) }
                .expect("IDesktopWallpaper");
        let count = unsafe { dw.GetMonitorDevicePathCount() }.expect("monitor count");
        let img_w = crate::wide::to_wide(img.to_str().unwrap());
        for i in 0..count {
            let id = take_string(unsafe { dw.GetMonitorDevicePathAt(i) }.expect("id"));
            let id_w = crate::wide::to_wide(&id);
            let hr = unsafe { dw.SetWallpaper(PCWSTR(id_w.as_ptr()), PCWSTR(img_w.as_ptr())) };
            assert!(hr.is_ok(), "SetWallpaper failed: {hr:?}");
        }
        eprintln!("applied via IDesktopWallpaper: {}", img.display());
        let spi = crate::sysparams::desktop_wallpaper();
        eprintln!("SPI_GETDESKWALLPAPER now -> {spi:?}");
    }

    /// The adaptive composite end-to-end on a synthetic asset: a 16:9 two-tone image on a
    /// 32:9 frame. The asset is drawn centered at its capped rect; both margins are painted
    /// from a blurred copy (non-black), while the asset region itself keeps the sharp edge.
    #[test]
    fn adaptive_wallpaper_pillarboxes() {
        let _com = crate::com::OleGuard::init().expect("OLE init");
        let (w, h) = (160usize, 90usize);
        let mut px = vec![0u8; w * h * 4];
        for y in 0..h {
            for x in 0..w {
                let i = (y * w + x) * 4;
                let left = x < w / 2;
                px[i] = if left { 220 } else { 30 }; // B
                px[i + 1] = if left { 50 } else { 140 }; // G
                px[i + 2] = if left { 20 } else { 220 }; // R
                px[i + 3] = 255;
            }
        }
        let jpg = crate::wmf_wallpaper::encode_jpeg(w as u32, h as u32, &px).expect("jpeg");
        let dir = std::env::temp_dir().join(format!("peco-adaptive-test-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        let src = dir.join("src.jpg");
        std::fs::write(&src, &jpg).unwrap();

        let (dw, dh) = (320u32, 90u32); // 32:9 target — strict Contain
        let fit = pecofence_core::adaptive::adaptive_fit(160, 90, dw, dh, true);
        let bg = pecofence_core::adaptive::bg_rect(&fit, dw, dh);
        assert!(bg[0] > 0 && bg[2] < dw as i32, "expected side bars: {bg:?}");
        let img = adaptive_wallpaper(&src, dw, dh, bg).expect("composite");
        assert_eq!((img.width, img.height), (dw, dh));

        let cy = ((bg[1] + bg[3]) / 2) as usize;
        // Inside the drawn asset's left quarter: the sharp dark-blue survives.
        let in_x = bg[0] as usize + (bg[2] - bg[0]) as usize / 4;
        let s = (cy * dw as usize + in_x) * 4;
        assert!(
            img.bgra[s] > 140 && img.bgra[s + 2] < 140,
            "asset pixel lost its color: {:?}",
            &img.bgra[s..s + 3]
        );
        // Far-left pillarbox: painted by the blurred copy, not left black.
        let m = (cy * dw as usize + 4) * 4;
        let margin = img.bgra[m] as u32 + img.bgra[m + 1] as u32 + img.bgra[m + 2] as u32;
        assert!(margin > 90, "pillarbox margin is black");
        let _ = std::fs::remove_dir_all(&dir);
    }
}
