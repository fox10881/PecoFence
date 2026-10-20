//! Explicit integration probe: switches the given installed wallpapers, restores the
//! original, and verifies one persistent worker survives web/video/scene transitions.
//! Args: WPE-exe original-project web-project video-project output-directory
use pecofence_platform::live_wallpaper::{CapturedWallpaper, LiveWallpaper};
use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
    path::Path,
    process::Command,
    time::{Duration, Instant},
};

fn switch(exe: &str, file: &str) {
    let status = Command::new(exe)
        .args(["-control", "openWallpaper", "-file", file])
        .status()
        .unwrap();
    assert!(status.success());
}

fn save(path: &Path, frame: &CapturedWallpaper) {
    let mut bytes = Vec::new();
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
    std::fs::write(path, bytes).unwrap();
}

struct Restore<'a> {
    exe: &'a str,
    file: &'a str,
}
impl Drop for Restore<'_> {
    fn drop(&mut self) {
        switch(self.exe, self.file);
    }
}

fn main() {
    pecofence_platform::window::set_process_dpi_awareness_v2();
    let args: Vec<_> = std::env::args().collect();
    assert_eq!(args.len(), 6);
    let output = Path::new(&args[5]);
    std::fs::create_dir_all(output).unwrap();
    let _restore = Restore {
        exe: &args[1],
        file: &args[2],
    };
    let worker = LiveWallpaper::new();
    worker.set_enabled(true);
    let mut hashes = Vec::new();
    for (label, file) in [
        ("scene-before", &args[2]),
        ("web", &args[3]),
        ("video", &args[4]),
        ("scene-restored", &args[2]),
    ] {
        switch(&args[1], file);
        let start = Instant::now();
        let mut frames = 0;
        let mut last = None;
        while start.elapsed() < Duration::from_secs(4) {
            if let Some(frame) = worker.take_latest() {
                frames += 1;
                last = frame.images.into_iter().next();
            }
            std::thread::sleep(Duration::from_millis(10));
        }
        let frame = last.expect("No wallpaper frame after switch");
        save(&output.join(format!("{label}.bmp")), &frame);
        let mut hash = DefaultHasher::new();
        frame.bgra.hash(&mut hash);
        hashes.push(hash.finish());
        println!(
            "phase={label} frames={frames} engine_running={} hash={:016x}",
            worker.engine_running(),
            hash.finish()
        );
        assert!(frames >= 20, "worker stopped across {label} transition");
    }
    assert!(hashes[0] != hashes[1] && hashes[1] != hashes[2] && hashes[2] != hashes[3]);
    println!("persistent_worker_web_video_scene_switch_passed=true");
}
