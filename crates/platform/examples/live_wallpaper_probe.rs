//! Capture actual WPE frames without touching the desktop or application configuration.
//! cargo run -p pecofence-platform --example live_wallpaper_probe -- <output-directory>
use pecofence_platform::live_wallpaper::{
    CapturedWallpaper, LiveWallpaper, capture_gpu_once, capture_once,
};
use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
    path::Path,
    time::{Duration, Instant},
};

fn fingerprint(frame: &CapturedWallpaper) -> u64 {
    let mut h = DefaultHasher::new();
    frame.bgra.hash(&mut h);
    h.finish()
}

fn save_bmp(path: &Path, frame: &CapturedWallpaper) -> std::io::Result<()> {
    let mut bytes = Vec::with_capacity(54 + frame.bgra.len());
    bytes.extend_from_slice(b"BM");
    bytes.extend_from_slice(&(54u32 + frame.bgra.len() as u32).to_le_bytes());
    bytes.extend_from_slice(&[0; 4]);
    bytes.extend_from_slice(&54u32.to_le_bytes());
    bytes.extend_from_slice(&40u32.to_le_bytes());
    bytes.extend_from_slice(&(frame.width as i32).to_le_bytes());
    bytes.extend_from_slice(&(-(frame.height as i32)).to_le_bytes());
    bytes.extend_from_slice(&1u16.to_le_bytes());
    bytes.extend_from_slice(&32u16.to_le_bytes());
    bytes.extend_from_slice(&[0; 24]);
    bytes.extend_from_slice(&frame.bgra);
    std::fs::write(path, bytes)
}

fn main() {
    pecofence_platform::window::set_process_dpi_awareness_v2();
    let _com = pecofence_platform::com::MtaGuard::init().unwrap();
    let gpu = std::env::args().any(|s| s == "--gpu");
    let capture = || {
        if gpu {
            capture_gpu_once().unwrap()
        } else {
            capture_once()
        }
    };
    let output = std::env::args().nth(1).expect("output directory required");
    let output = Path::new(&output);
    std::fs::create_dir_all(output).unwrap();
    let first = capture();
    assert!(
        !first.is_empty(),
        "No supported Wallpaper Engine surface found"
    );
    for (i, frame) in first.iter().enumerate() {
        assert_eq!(
            frame.bgra.len(),
            frame.width as usize * frame.height as usize * 4
        );
        assert!(frame.bgra.as_chunks::<4>().0.iter().all(|p| p[3] == 255));
        save_bmp(&output.join(format!("first-{i}.bmp")), frame).unwrap();
        println!(
            "surface={i} rect=({},{},{},{}) hash={:016x}",
            frame.left,
            frame.top,
            frame.width,
            frame.height,
            fingerprint(frame)
        );
    }
    std::thread::sleep(Duration::from_millis(750));
    let second = capture();
    for (i, frame) in second.iter().enumerate() {
        save_bmp(&output.join(format!("second-{i}.bmp")), frame).unwrap();
    }
    let changed = first
        .iter()
        .zip(&second)
        .any(|(a, b)| fingerprint(a) != fingerprint(b));
    println!("two_samples_changed={changed}");
    if !changed {
        println!("Wallpaper is paused or static; motion validation pending");
    }
    let worker = LiveWallpaper::new();
    worker.set_enabled(true);
    // Exclude capture-session startup from the steady-state cadence sample.
    let warmup = Instant::now();
    while worker.take_latest().is_none() {
        assert!(
            warmup.elapsed() < Duration::from_secs(3),
            "worker warmup timed out"
        );
        std::thread::sleep(Duration::from_millis(5));
    }
    let start = Instant::now();
    let mut frames = 0;
    while start.elapsed() < Duration::from_secs(4) {
        if let Some(frame) = worker.take_latest() {
            assert!(frame.captured_at.elapsed() < Duration::from_secs(1));
            frames += 1;
        }
        std::thread::sleep(Duration::from_millis(5));
    }
    println!(
        "worker_frames={frames} elapsed_ms={} target_fps={}",
        start.elapsed().as_millis(),
        worker.fps
    );
    assert!(
        frames >= 10,
        "Capture worker failed to keep producing frames"
    );
    worker.set_enabled(false);
    std::thread::sleep(Duration::from_millis(250));
    assert!(
        worker.take_latest().is_none(),
        "Disabled worker retained an in-flight frame"
    );
}
