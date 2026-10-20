//! Wallpaper Engine capture. Modern Explorer embeds WPE as a child of Progman, which
//! Windows Graphics Capture cannot target directly. Capture that desktop root only when
//! its real icon layer is hidden; top-level fences and foreground apps are not included.
//! Legacy/web surfaces can still use PrintWindow on the worker.

use crate::{bindings::*, desktop, wide::from_wide};
use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
    sync::{
        Arc, Mutex,
        atomic::{AtomicBool, Ordering},
    },
    thread,
    time::{Duration, Instant},
};
use windows_core::{BOOL, Error, PWSTR, Result};

windows_core::link!("user32.dll" "system" fn EnumChildWindows(hwnd: HWND, callback: WNDENUMPROC, data: LPARAM) -> BOOL);
windows_core::link!("user32.dll" "system" fn GetParent(hwnd: HWND) -> HWND);
windows_core::link!("user32.dll" "system" fn PrintWindow(hwnd: HWND, dc: HDC, flags: u32) -> BOOL);
windows_core::link!("kernel32.dll" "system" fn QueryFullProcessImageNameW(process: HANDLE, flags: u32, name: PWSTR, length: *mut u32) -> BOOL);

/// A frame uses physical virtual-screen coordinates and opaque, top-down BGRA pixels.
pub struct CapturedWallpaper {
    pub left: i32,
    pub top: i32,
    pub width: u32,
    pub height: u32,
    pub bgra: Vec<u8>,
}

pub struct WallpaperFrame {
    pub images: Vec<CapturedWallpaper>,
    /// Small, block-averaged contrast images prepared off the UI thread.
    pub samples: Vec<CapturedWallpaper>,
    pub fingerprint: u64,
    pub captured_at: Instant,
}

pub const CONTRAST_SAMPLE_FACTOR: u32 = 8;

fn contrast_sample(image: &CapturedWallpaper) -> CapturedWallpaper {
    let factor = CONTRAST_SAMPLE_FACTOR as usize;
    let (sw, sh) = (image.width as usize, image.height as usize);
    let (width, height) = (sw.div_ceil(factor), sh.div_ceil(factor));
    let mut bgra = Vec::with_capacity(width * height * 4);
    let mut sums = vec![0u32; width * 4];
    for by in 0..height {
        sums.fill(0);
        let rows = by * factor..((by + 1) * factor).min(sh);
        let count = rows.len() as u32;
        for y in rows {
            for (x, px) in image.bgra[y * sw * 4..(y + 1) * sw * 4]
                .as_chunks::<4>()
                .0
                .iter()
                .enumerate()
            {
                for c in 0..4 {
                    sums[x / factor * 4 + c] += px[c] as u32;
                }
            }
        }
        for x in 0..width {
            let n = count * (((x + 1) * factor).min(sw) - x * factor) as u32;
            bgra.extend(
                sums[x * 4..x * 4 + 4]
                    .iter()
                    .map(|s| ((s + n / 2) / n) as u8),
            );
        }
    }
    CapturedWallpaper {
        left: image.left,
        top: image.top,
        width: width as u32,
        height: height as u32,
        bgra,
    }
}

fn wpe_process(hwnd: HWND) -> bool {
    // SAFETY: process is opened read-only; the local name buffer is bounded and the handle
    // is closed on both success and failure.
    unsafe {
        let process = OpenProcess(0x1000, false, desktop::window_pid(hwnd));
        if process.0.is_null() {
            return false;
        }
        let mut name = [0u16; 1024];
        let mut len = name.len() as u32;
        let ok =
            QueryFullProcessImageNameW(process, 0, PWSTR(name.as_mut_ptr()), &mut len).as_bool();
        let _ = CloseHandle(process);
        if !ok {
            return false;
        }
        let name = from_wide(&name[..len as usize]).to_ascii_lowercase();
        name.ends_with("\\wallpaper64.exe") || name.ends_with("\\wallpaper32.exe")
    }
}

fn is_wpe_render_class(class: &str) -> bool {
    class.starts_with("WPEDesktop") || class == "WPEVideoWallpaper"
}

fn is_wpe_descendant(mut hwnd: HWND) -> bool {
    // Bound the ancestry walk in case another process destroys/reparents a window.
    for _ in 0..16 {
        if hwnd.0.is_null() {
            break;
        }
        if is_wpe_render_class(&desktop::class_name(hwnd)) && wpe_process(hwnd) {
            return true;
        }
        // SAFETY: GetParent accepts an HWND; stale handles simply stop the walk.
        hwnd = unsafe { GetParent(hwnd) };
    }
    false
}

unsafe extern "system" fn collect(hwnd: HWND, data: LPARAM) -> BOOL {
    let class = desktop::class_name(hwnd);
    // CEF's intermediate GPU child has the real pixels; its parent prints black. Native
    // scene/video roots can be tried too, but unsupported renderers fall back to static.
    if desktop::is_visible(hwnd)
        && (class == "Intermediate D3D Window" || is_wpe_render_class(&class))
        && is_wpe_descendant(hwnd)
    {
        // SAFETY: discovery passes a live Vec pointer synchronously to EnumWindows/children.
        unsafe { &mut *(data.0 as *mut Vec<HWND>) }.push(hwnd);
    }
    true.into()
}

unsafe extern "system" fn collect_top(hwnd: HWND, data: LPARAM) -> BOOL {
    // SAFETY: synchronous enumeration reuses the same live Vec as collect.
    unsafe {
        let _ = collect(hwnd, data);
        let _ = EnumChildWindows(hwnd, Some(collect), data);
    }
    true.into()
}

fn discover_wpe() -> Vec<HWND> {
    let mut windows = Vec::new();
    // SAFETY: both callbacks complete before the stack-owned Vec goes away.
    unsafe {
        let _ = EnumWindows(
            Some(collect_top),
            LPARAM((&mut windows as *mut Vec<HWND>) as isize),
        );
    }
    // Prefer the CEF surface and suppress its black WPE wrapper for the same rectangle.
    windows.sort_by_key(|h| desktop::class_name(*h) != "Intermediate D3D Window");
    let mut rects = Vec::new();
    windows.retain(|h| {
        let Some(r) = capture_rect(*h) else {
            return false;
        };
        let rect = (r.left, r.top, r.right, r.bottom);
        if rects.contains(&rect) {
            return false;
        }
        rects.push(rect);
        true
    });
    windows
}

fn discover() -> Vec<HWND> {
    let mut targets = Vec::new();
    for hwnd in discover_wpe() {
        let root = desktop::root_ancestor(hwnd);
        let class = desktop::class_name(root);
        let shell = desktop::shell_window();
        let clean_root = (class == "Progman" || class == "WorkerW")
            && desktop::window_pid(root) == desktop::window_pid(shell)
            && crate::shell_icons::desktop_icons_hidden();
        let target = if clean_root { root } else { hwnd };
        if !targets.contains(&target) {
            targets.push(target);
        }
    }
    targets
}

fn engine_running() -> bool {
    desktop::top_level_windows()
        .into_iter()
        .any(|hwnd| desktop::class_name(hwnd) == "WPETrayWindow" && wpe_process(hwnd))
}

fn capture_rect(hwnd: HWND) -> Option<RECT> {
    if !desktop::is_visible(hwnd) {
        return None;
    }
    let mut r = RECT::default();
    // SAFETY: out pointer to a sized local. Refuse enormous/non-desktop allocations.
    if !unsafe { GetWindowRect(hwnd, &mut r) }.as_bool() {
        return None;
    }
    let w = i64::from(r.right) - i64::from(r.left);
    let h = i64::from(r.bottom) - i64::from(r.top);
    (w >= 100 && h >= 100 && w * h <= 33_177_600).then_some(r)
}

struct GdiSurface {
    hwnd: HWND,
    dc: HDC,
    bitmap: HBITMAP,
    old: HGDIOBJ,
    bits: *mut u8,
    width: i32,
    height: i32,
}

impl GdiSurface {
    fn new(hwnd: HWND, r: RECT) -> Result<Self> {
        let (width, height) = (r.right - r.left, r.bottom - r.top);
        // SAFETY: the worker owns all GDI objects and frees them on failure or Drop. The
        // checked geometry defines exactly the DIB allocation and all subsequent slices.
        unsafe {
            let dc = CreateCompatibleDC(None);
            if dc.0.is_null() {
                return Err(Error::from_thread());
            }
            let info = BITMAPINFO {
                bmiHeader: BITMAPINFOHEADER {
                    biSize: size_of::<BITMAPINFOHEADER>() as u32,
                    biWidth: width,
                    biHeight: -height,
                    biPlanes: 1,
                    biBitCount: 32,
                    biCompression: BI_RGB as u32,
                    ..Default::default()
                },
                ..Default::default()
            };
            let mut bits = std::ptr::null_mut();
            let bitmap =
                CreateDIBSection(Some(dc), &info, DIB_RGB_COLORS as u32, &mut bits, None, 0);
            if bitmap.0.is_null() || bits.is_null() {
                let error = Error::from_thread();
                if !bitmap.0.is_null() {
                    let _ = DeleteObject(HGDIOBJ(bitmap.0));
                }
                let _ = DeleteDC(dc);
                return Err(error);
            }
            let old = SelectObject(dc, HGDIOBJ(bitmap.0));
            if old.0.is_null() || old.0 as isize == -1 {
                let error = Error::from_thread();
                let _ = DeleteObject(HGDIOBJ(bitmap.0));
                let _ = DeleteDC(dc);
                return Err(error);
            }
            Ok(Self {
                hwnd,
                dc,
                bitmap,
                old,
                bits: bits.cast(),
                width,
                height,
            })
        }
    }

    fn capture(&self) -> Option<CapturedWallpaper> {
        let r = capture_rect(self.hwnd)?;
        if (r.right - r.left, r.bottom - r.top) != (self.width, self.height)
            || !is_wpe_descendant(self.hwnd)
        {
            return None;
        }
        // SAFETY: selected DIB is live and exclusive to this worker. Clear old pixels so a
        // renderer that stops supporting PrintWindow cannot masquerade as a fresh frame.
        let len = self.width as usize * self.height as usize * 4;
        unsafe {
            std::ptr::write_bytes(self.bits, 0, len);
            if !PrintWindow(self.hwnd, self.dc, 2).as_bool() {
                return None;
            }
            let _ = GdiFlush();
            let pixels = std::slice::from_raw_parts(self.bits, len);
            // Unsupported GPU wrappers often return success with an entirely black DIB.
            // A genuinely black transition keeps the last frame briefly, then falls back.
            if !pixels
                .as_chunks::<4>()
                .0
                .iter()
                .any(|p| p[0] != 0 || p[1] != 0 || p[2] != 0)
            {
                return None;
            }
            let mut bgra = pixels.to_vec();
            for p in bgra.as_chunks_mut::<4>().0 {
                p[3] = 255;
            }
            Some(CapturedWallpaper {
                left: r.left,
                top: r.top,
                width: self.width as u32,
                height: self.height as u32,
                bgra,
            })
        }
    }
}

impl Drop for GdiSurface {
    fn drop(&mut self) {
        // SAFETY: owned objects, restored before deleting the selected bitmap.
        unsafe {
            let _ = SelectObject(self.dc, self.old);
            let _ = DeleteObject(HGDIOBJ(self.bitmap.0));
            let _ = DeleteDC(self.dc);
        }
    }
}

/// Synchronous diagnostic entry point. Production uses the bounded worker below.
enum Backend {
    Gpu(crate::gpu_wallpaper::GpuSurface),
    Gdi(GdiSurface),
}

struct Surface {
    hwnd: HWND,
    width: i32,
    height: i32,
    backend: Backend,
    failed: bool,
}

impl Surface {
    fn new(hwnd: HWND, r: RECT) -> Result<Self> {
        let backend = match crate::gpu_wallpaper::GpuSurface::new(hwnd) {
            Ok(surface) => {
                tracing::info!(
                    handle = hwnd.0 as usize,
                    "Wallpaper Engine Windows Graphics Capture connected"
                );
                Backend::Gpu(surface)
            }
            Err(error) if is_wpe_descendant(hwnd) => {
                tracing::debug!(%error, "GPU child not capturable; trying web PrintWindow");
                Backend::Gdi(GdiSurface::new(hwnd, r)?)
            }
            Err(error) => return Err(error),
        };
        Ok(Self {
            hwnd,
            width: r.right - r.left,
            height: r.bottom - r.top,
            backend,
            failed: false,
        })
    }

    fn capture(&mut self) -> Option<CapturedWallpaper> {
        let r = capture_rect(self.hwnd)?;
        if (r.right - r.left, r.bottom - r.top) != (self.width, self.height) {
            return None;
        }
        match &mut self.backend {
            Backend::Gpu(gpu) => match gpu.capture(r.left, r.top) {
                Ok(frame) => frame,
                Err(error) => {
                    self.failed = true;
                    tracing::warn!(%error, "Wallpaper Engine GPU capture failed; reconnecting");
                    None
                }
            },
            Backend::Gdi(gdi) => gdi.capture(),
        }
    }
}

pub fn capture_once() -> Vec<CapturedWallpaper> {
    let _com = crate::com::MtaGuard::init().ok();
    let mut surfaces: Vec<_> = discover()
        .into_iter()
        .filter_map(|h| Surface::new(h, capture_rect(h)?).ok())
        .collect();
    // GPU sessions asynchronously receive the first frame.
    let until = Instant::now() + Duration::from_secs(1);
    loop {
        let images: Vec<_> = surfaces.iter_mut().filter_map(Surface::capture).collect();
        if !images.is_empty() || surfaces.is_empty() || Instant::now() >= until {
            return images;
        }
        thread::sleep(Duration::from_millis(20));
    }
}

/// GPU capture diagnostic, including native scene wallpapers.
pub fn capture_gpu_once() -> Result<Vec<CapturedWallpaper>> {
    let mut images = Vec::new();
    for hwnd in discover() {
        let mut surface = crate::gpu_wallpaper::GpuSurface::new(hwnd)?;
        let until = Instant::now() + Duration::from_millis(1000);
        while Instant::now() < until {
            let Some(r) = capture_rect(hwnd) else { break };
            if let Some(frame) = surface.capture(r.left, r.top)? {
                images.push(frame);
                break;
            }
            thread::sleep(Duration::from_millis(20));
        }
    }
    Ok(images)
}

struct Shared {
    enabled: AtomicBool,
    stop: AtomicBool,
    engine_running: AtomicBool,
    latest: Mutex<Option<WallpaperFrame>>,
}

/// Capacity-one mailbox: a slow UI drops old frames, never queues unlimited images.
pub struct LiveWallpaper {
    shared: Arc<Shared>,
    thread: Option<thread::JoinHandle<()>>,
    pub interval_ms: u32,
    pub fps: u32,
}

impl Default for LiveWallpaper {
    fn default() -> Self {
        Self::new()
    }
}

impl LiveWallpaper {
    pub fn new() -> Self {
        let fps = std::env::var("PECOFENCE_WALLPAPER_FPS")
            .ok()
            .and_then(|s| s.parse::<u32>().ok())
            .unwrap_or(60)
            .clamp(1, 60);
        let interval_ms = 1000u32.div_ceil(fps);
        let interval = Duration::from_nanos(1_000_000_000 / u64::from(fps));
        let shared = Arc::new(Shared {
            enabled: AtomicBool::new(false),
            stop: AtomicBool::new(false),
            engine_running: AtomicBool::new(false),
            latest: Mutex::new(None),
        });
        let worker = shared.clone();
        let spawned = thread::Builder::new()
            .name("wallpaper-engine-capture".into())
            .spawn(move || {
                let Ok(_com) = crate::com::MtaGuard::init() else {
                    return;
                };
                let mut surfaces: Vec<Surface> = Vec::new();
                let mut discovery = Instant::now() - Duration::from_secs(3);
                let mut deadline = Instant::now();
                while !worker.stop.load(Ordering::Relaxed) {
                    if !worker.enabled.load(Ordering::Relaxed) {
                        surfaces.clear();
                        discovery = Instant::now() - Duration::from_secs(3);
                        thread::sleep(Duration::from_millis(100));
                        deadline = Instant::now();
                        continue;
                    }
                    if discovery.elapsed() >= Duration::from_millis(500) {
                        worker
                            .engine_running
                            .store(engine_running(), Ordering::Relaxed);
                        let found = discover();
                        surfaces.retain(|s| {
                            !s.failed
                                && found.contains(&s.hwnd)
                                && capture_rect(s.hwnd).is_some_and(|r| {
                                    (r.right - r.left, r.bottom - r.top) == (s.width, s.height)
                                })
                        });
                        for hwnd in found {
                            if !surfaces.iter().any(|s| s.hwnd == hwnd)
                                && let Some(r) = capture_rect(hwnd)
                                && let Ok(s) = Surface::new(hwnd, r)
                            {
                                surfaces.push(s);
                            }
                        }
                        discovery = Instant::now();
                    }
                    let images: Vec<_> = surfaces.iter_mut().filter_map(Surface::capture).collect();
                    let captured_at = Instant::now();
                    let mut hash = DefaultHasher::new();
                    for image in &images {
                        (image.left, image.top, image.width, image.height).hash(&mut hash);
                        image.bgra.hash(&mut hash);
                    }
                    let fingerprint = hash.finish();
                    let samples = images.iter().map(contrast_sample).collect();
                    if worker.enabled.load(Ordering::Relaxed)
                        && !images.is_empty()
                        && let Ok(mut latest) = worker.latest.lock()
                    {
                        if worker.enabled.load(Ordering::Relaxed) {
                            *latest = Some(WallpaperFrame {
                                images,
                                samples,
                                fingerprint,
                                captured_at,
                            });
                        }
                    }
                    // Keep a monotonic cadence: a small sleep/readback overrun must
                    // not discard the next whole frame slot. Reset after a full missed
                    // interval so a long stall never causes a burst of catch-up frames.
                    deadline += interval;
                    let now = Instant::now();
                    if now.saturating_duration_since(deadline) >= interval {
                        deadline = now;
                    }
                    thread::sleep(deadline.saturating_duration_since(now));
                }
            });
        let thread = match spawned {
            Ok(handle) => Some(handle),
            Err(error) => {
                tracing::warn!(%error, "live wallpaper worker unavailable");
                None
            }
        };
        Self {
            shared,
            thread,
            interval_ms,
            fps,
        }
    }

    pub fn set_enabled(&self, enabled: bool) {
        self.shared.enabled.store(enabled, Ordering::Relaxed);
        if !enabled && let Ok(mut latest) = self.shared.latest.lock() {
            *latest = None;
        }
    }

    pub fn take_latest(&self) -> Option<WallpaperFrame> {
        self.shared.latest.lock().ok()?.take()
    }

    pub fn engine_running(&self) -> bool {
        self.shared.engine_running.load(Ordering::Relaxed)
    }
}

impl Drop for LiveWallpaper {
    fn drop(&mut self) {
        self.shared.stop.store(true, Ordering::Relaxed);
        // Drain GPU sessions before COM/process teardown. Bound the wait because legacy
        // PrintWindow may be blocked in another process; never join an unfinished worker.
        if let Some(handle) = self.thread.take() {
            let until = Instant::now() + Duration::from_millis(500);
            while !handle.is_finished() && Instant::now() < until {
                thread::sleep(Duration::from_millis(5));
            }
            if handle.is_finished() {
                let _ = handle.join();
            }
        }
    }
}

#[cfg(test)]
mod contrast_tests {
    use super::*;

    #[test]
    fn worker_contrast_sample_averages_partial_blocks_and_preserves_origin() {
        let mut bgra = Vec::new();
        for y in 0..11u8 {
            for x in 0..19u8 {
                bgra.extend_from_slice(&[x, y, x + y, 255]);
            }
        }
        let source = CapturedWallpaper {
            left: -2560,
            top: -80,
            width: 19,
            height: 11,
            bgra,
        };
        let sample = contrast_sample(&source);
        assert_eq!(
            (sample.left, sample.top, sample.width, sample.height),
            (-2560, -80, 3, 2)
        );
        assert_eq!(
            sample.bgra,
            [
                4, 4, 7, 255, 12, 4, 15, 255, 17, 4, 21, 255, 4, 9, 13, 255, 12, 9, 21, 255, 17, 9,
                26, 255
            ]
        );
    }
}
