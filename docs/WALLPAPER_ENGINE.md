# Wallpaper Engine personal build

This fork adds live Wallpaper Engine backgrounds to **Liquid Glass**. Version
`0.1.5-we.11` is based on upstream PecoFence 0.1.5. The GPU optics now use
continuous superellipse corners, a flat face, curved edge displacement, subtle
interior frost and narrow directional glints. The material and Windows compositor
clip share the same silhouette. Whole-face magnification and the previous broad
highlight/bounce layers have been removed. No RGB dispersion or full-face black
overlay is applied. Geometry and masks remain cached between wallpaper frames.
See [GLASS_RENDERING.md](GLASS_RENDERING.md) for formulas, source attribution and
validation limits. This is an approximation, not a proven pixel-perfect iOS match.

## Supported path

Verified on Windows 11 at 2560 × 1440, with the existing
**Hide real desktop icons** setting enabled. Windows Graphics Capture reads the
actual GPU-composited desktop root containing Wallpaper Engine. Modern Explorer
embeds WPE as a child window, which cannot be captured directly by CreateForWindow.
The root is eligible only when its real icon layer is hidden. Top-level fences and
foreground applications are outside that root and were absent from the captured
web, video and native DX11 scene frames. This is window capture, not a full-screen
screenshot or a read of the Windows wallpaper file.

Discovery recognizes `WPEDesktopCEFWindow`, `WPEDesktopDX11Window` and
`WPEVideoWallpaper`. A persistent worker was exercised through scene → web → video
→ scene, delivering 77/80/80/81 frames during the four-second phases; saved images
were inspected to verify actual wallpaper content, and the probe exited normally.
Black GPU frames are valid, including transitions. If real icons are visible,
the desktop-root path is refused; the old isolated PrintWindow web path can still
work, but native scene support under that configuration is not claimed.

While WPE remains running, a capture gap keeps the last live frame and retries,
rather than switching to the native Windows wallpaper after 1.5 seconds. A stale
frame is reported as reconnecting in the log. Before the first successful capture,
the initial static material can still be visible. Exiting WPE or disabling live
capture restores the static material. Fluent retains its static-wallpaper pipeline.

The app does not change Wallpaper Engine's playback settings. If it pauses for
maximized/full-screen applications, the glass keeps the current paused frame.

## Runtime behavior

- Enabled automatically for Liquid Glass; no config migration is required.
- Default capture target: 60 FPS. Set `PECOFENCE_WALLPAPER_FPS` before starting the
  app to choose 1–60 FPS. The worker uses monotonic deadlines; long overruns are
  skipped. Actual delivery also depends on capture cost and WPE's playback rate.
- Live updates share the compositor frame clock with fence animations. A 100 ms
  fallback poll detects returning focus; there is no competing 60 Hz UI timer.
- Switching to another application, including a full-screen application, suspends
  live capture immediately and gradually fades liquid material to ordinary window
  transparency. The last captured image becomes invisible; Windows displays the
  real wallpaper underneath, with only a 3.5% white wash. At rest there is no blur,
  refraction, source upload or recurring glass draw. WPE's own playback settings
  still control whether the underlying wallpaper is moving. Focus loss
  never restores the Windows native wallpaper. Startup primes one real WPE frame
  even when the app starts behind another window.
- Returning to the desktop waits for a newly captured frame, then restores optical
  strength with a slow critically damped transition. Incoming/outgoing response
  parameters are 1.8/1.6 seconds, with subtle tails settling at roughly 2.6/2.3
  seconds. A second focus change retargets current strength and velocity smoothly.
  PecoFence popups and an explicitly requested Peek remain interactive; a full-screen
  PecoFence window also suspends ordinary desktop rendering.
- Set `PECOFENCE_LIVE_WALLPAPER=0` before starting to use the original static
  pipeline. An explicit `--wallpaper` override also disables live capture.
- Capture runs on a dedicated worker with a capacity-one mailbox. A blocked
  `PrintWindow` cannot block the UI. Shutdown allows up to 500 ms to drain GPU sessions
  and only joins a completed worker. The activated GraphicsCapture runtime is pinned
  until process exit to avoid a 24H2 asynchronous DLL unload crash observed in testing.
- Frame fingerprinting and the 8x8 box-averaged contrast sample also run on that
  worker. When a captured surface exactly matches a monitor, the UI moves its
  full-resolution buffer into the upload slot without another full-frame copy
  or averaging pass. Partially overlapping or spanning surfaces retain the crop
  fallback; this is not a GPU shared-texture capture path.
- Live updates redraw only the glass/header when grid/list label ink remains
  unchanged. A foreground-color change or a row/details view redraws the content
  too, preserving adaptive text contrast without repainting every icon each frame.
- Physical window coordinates map directly to monitor pixels, including a
  spanning surface and negative monitor origins. Uncovered monitors keep static
  backgrounds. Multi-monitor operation has not been exercised on physical hardware.
- GPU sessions/GDI surfaces are reused and rediscovered every 500 ms. Failed GPU
  sessions are recreated. Lens geometry is
  reused between frames; unchanged images skip GPU uploads and fence redraws.
- Capture also suspends when no fence window is visible or the theme is Fluent.
- Update links point to this fork, so checking for updates cannot install the
  upstream build over this personal modification.

## Verification

```powershell
cargo build --locked --release -p pecofence -p pecofence-watchdog
cargo build --locked --release -p pecofence-cli
cargo fmt --all --check
cargo clippy -p pecofence -p pecofence-platform --all-targets -- -D warnings
cargo clippy -p pecofence-render --all-targets -- -D warnings
cargo test -p pecofence-render --lib
cargo test -p pecofence-core updates::tests
cargo run -p pecofence-platform --example live_wallpaper_probe -- .cache/live-probe
cargo run -p pecofence-platform --example live_wallpaper_probe -- .cache/gpu-probe --gpu
```

The diagnostic writes two real wallpaper frames, checks dimensions and opaque
BGRA pixels, and exercises worker delivery and suspension. It reports whether
the wallpaper moved; a paused wallpaper cannot validate motion.

`wallpaper_switch_probe` is an explicit integration tool that changes installed
wallpapers using WPE's CLI and restores the original on exit. Arguments are WPE
executable, original project, web project, video project, output directory. It
keeps one worker alive across all phases and saves each source image for inspection.
Uniform gray GPU rendering is used to check material brightness independently of
wallpaper color. GPU tests also exercise neutral frosting, opacity and source
replacement. Edge contours are weak and local; user-selected opacity/tint and
floating-fence backing can still alter the overall appearance.

On the development machine, the capture worker delivered 80 frames in four
seconds, and the two visible-desktop samples differed. An isolated application
instance displayed the captured rain wallpaper with refracting glass edges and
survived switching Fluent → Liquid Glass. The installed build was then replaced
after backing up the original executable and configuration; the user confirmed
the real desktop fences display the Wallpaper Engine wallpaper. The we.4 optics
were compared using actual GPU renders over captured rain and sunset frames.
Offscreen comparisons use 150% DPI; the installed we.4 window was checked at
125% DPI. An isolated native app exercised full height, folding/unfolding,
resizing and a simulated lost device with successful recovery, then exited
normally. The installed release connected to live WPE and its screenshot was
inspected against the current SpongeBob wallpaper. The production configuration
matched its saved copy after installation.
Regression checks cover DPI invariance, matching clip/optical geometry, corner
continuity, the bevel-to-flat-face transition and narrow directional glints.
These checks do not establish the user's visual acceptance or native iOS fidelity.

The we.5 native input and paired fold timings are recorded in GLASS_RENDERING.md.
Manual collapse cancels pending peek timers and stays collapsed on hover; an
automatically collapsed screen-edge shelf still supports hover expansion.

The we.7 native integration exercised inactive -> early/middle/full restoration,
pause during restoration, return to the real foreground policy, and stable source
upload counts during suspension. The observed middle strength was 0.487; full
restoration reached 1.0. Separate fold checks retained an 800 x 955 pixel content
surface at the 45 pixel folded height and reused it on open; changing the folded
width to 700 caused exactly one new content draw at expansion. These are automated
native-window checks, not a claim about perceived smoothness on every machine.

The final we.7 GPU capture probe received 240 worker frames in 4004 ms after
warmup at 2560 x 1440, and its two wallpaper samples differed. The worker reads
back only the newest queued GPU texture; older queued frames are closed without
an unnecessary full-screen GPU readback. Frame delivery is a local observation,
not a guarantee that every wallpaper animates at 60 FPS.

A five-second sample of `0.1.5-we.1` with five installed fences measured about 1.4% of total CPU
capacity on a 20-logical-processor machine and a 109 MB working set. This is a
short local observation, not a guarantee for other wallpapers or hardware.

## Install and revert

Keep a copy of the original PecoFence program directory and
`%APPDATA%\PecoFence` before replacing an existing install. Exit PecoFence, then
replace `pecofence.exe`, `pecofence-watchdog.exe`, and `pecofence-cli.exe` together.
The existing `WebView2Loader.dll` and configuration remain compatible.

To revert, exit PecoFence and restore the three original binaries. Restore the
saved data directory only if you also want to discard later layout/settings
changes. Restart PecoFence from its existing shortcut.

For an independent portable installation, extract the **entire** portable ZIP and
start `pecofence.exe --portable`. See [PORTABLE.md](PORTABLE.md) for data paths.
