//! Host-side controller for `pecofence_tap.dll` — the TranslucentTB-style
//! taskbar override. This process injects the DLL into `explorer.exe` with a
//! `WH_CALLWNDPROC` hook on the taskbar thread (TTB's `InjectExplorerTAP`),
//! then drives it through registered window messages:
//!
//! - `"PecoFence.TAP.Command"` (`wParam` = mode, `lParam` = our pid): the DLL
//!   sets its internal state, which flips the in-Explorer
//!   `SetWindowCompositionAttribute` detour on/off and repaints the taskbar's
//!   `BackgroundFill`/`BackgroundStroke` XAML rectangles.
//! - `"PecoFence.TAP.Unload"`: synchronous full teardown — original brushes
//!   back, visual-tree watcher unadvised, detour lifted.
//!
//! The host writes **no** `SetWindowCompositionAttribute` payload at all —
//! TTB's Win 11 TAP path never touches SWCA, and on modern builds every
//! manual policy is a poison: `ACCENT_DISABLED` leaves a permanently black
//! DWM plate, while `ACCENT_ENABLE_TRANSPARENTGRADIENT`/`0` falls back to
//! the solid system accent color. Instead, the in-Explorer detour simply
//! *blocks* Explorer's own accent writes while a XAML mode is armed (the
//! anti-strobe defense), and `Normal` un-arms it — Explorer's native accent
//! then flows again untouched, so nothing black, blue, or stale can sit
//! behind the transparent XAML.
//!
//! The mode is not applied in a loop: once set, the DLL's detour defends it.
//! `ensure()` only re-runs the whole sequence when Explorer restarted (pid
//! changed or the command window vanished).

use crate::bindings::{
    FindWindowW, GetCurrentProcessId, GetModuleFileNameW, GetWindowThreadProcessId, HWND, LPARAM,
    PostMessageW, SendMessageTimeoutW, WPARAM,
};
use std::path::PathBuf;
use std::sync::Mutex;
use windows_core::{BOOL, PCSTR, PCWSTR, PWSTR};

windows_core::link!("kernel32.dll" "system" fn FreeLibrary(hlibmodule: *mut core::ffi::c_void) -> BOOL);
windows_core::link!("kernel32.dll" "system" fn GetProcAddress(hmodule: *mut core::ffi::c_void, lpprocname: PCSTR) -> *mut core::ffi::c_void);
windows_core::link!("kernel32.dll" "system" fn LoadLibraryW(lplibfilename: PCWSTR) -> *mut core::ffi::c_void);
windows_core::link!("kernel32.dll" "system" fn Sleep(dwmilliseconds: u32));
windows_core::link!("user32.dll" "system" fn IsWindow(hwnd: HWND) -> BOOL);
windows_core::link!("user32.dll" "system" fn RegisterWindowMessageW(lpstring: PCWSTR) -> u32);
windows_core::link!("user32.dll" "system" fn SetWindowsHookExW(idhook: i32, lpfn: *const core::ffi::c_void, hmod: *mut core::ffi::c_void, dwthreadid: u32) -> *mut core::ffi::c_void);
windows_core::link!("user32.dll" "system" fn UnhookWindowsHookEx(hhk: *mut core::ffi::c_void) -> BOOL);

const WH_CALLWNDPROC: i32 = 4;
const SMTO_ABORTIFHUNG: u32 = 0x0002;
const SMTO_BLOCK: u32 = 0x0001;
const SMTO_ERRORONEXIT: u32 = 0x0020;

const CMD_NORMAL: usize = 0;
const CMD_CLEAR: usize = 1;
const CMD_BLUR: usize = 2;
const CMD_OPAQUE: usize = 3;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum TapMode {
    #[default]
    Normal,
    Clear,
    Blur,
    Opaque,
}

struct Tap {
    /// Our `WH_CALLWNDPROC` hook while it is still armed (kept until the DLL
    /// has actually been loaded by Explorer; `WH_CALLWNDPROC` hooks stay
    /// installed even after the target loads the DLL, so we unhook once the
    /// command window answers).
    hook: *mut core::ffi::c_void,
    /// Our own load of `pecofence_tap.dll` (keeps the path resolvable for the
    /// hook; Explorer keeps its own mapping).
    lib: *mut core::ffi::c_void,
    /// The in-Explorer command window (`PecoFenceTAP` class).
    cmd_hwnd: HWND,
    /// `RegisterWindowMessage` ids — per-session, registered on both sides.
    msg_cmd: u32,
    msg_unload: u32,
    /// Pid of the Explorer process the DLL is currently loaded in.
    explorer_pid: u32,
    /// The mode the DLL was last told to apply.
    mode: TapMode,
}

// Raw handles cross no threads other than the message pump's; access is
// serialised by the Mutex like the rest of this module's state.
unsafe impl Send for Tap {}

static TAP: Mutex<Option<Tap>> = Mutex::new(None);

fn lock() -> std::sync::MutexGuard<'static, Option<Tap>> {
    TAP.lock().unwrap_or_else(|p| p.into_inner())
}

fn wide(s: &str) -> Vec<u16> {
    s.encode_utf16().chain(core::iter::once(0)).collect()
}

fn find_taskbar() -> Option<(HWND, u32)> {
    let hwnd = unsafe { FindWindowW(PCWSTR(wide("Shell_TrayWnd").as_ptr()), PCWSTR::null()) };
    if hwnd.0.is_null() {
        return None;
    }
    let mut pid = 0u32;
    unsafe {
        GetWindowThreadProcessId(hwnd, Some(&mut pid));
    }
    (pid != 0).then_some((hwnd, pid))
}

fn command_for(mode: TapMode) -> usize {
    match mode {
        TapMode::Normal => CMD_NORMAL,
        TapMode::Clear => CMD_CLEAR,
        TapMode::Blur => CMD_BLUR,
        TapMode::Opaque => CMD_OPAQUE,
    }
}

fn tap_dll_path() -> Option<PathBuf> {
    let mut buf = [0u16; 512];
    let len = unsafe { GetModuleFileNameW(None, PWSTR(buf.as_mut_ptr()), buf.len() as u32) };
    if len == 0 {
        return None;
    }
    let exe = PathBuf::from(String::from_utf16_lossy(&buf[..len as usize]));
    Some(exe.with_file_name("pecofence_tap.dll"))
}

/// Sends a registered message to the in-Explorer command window and waits for
/// it to be processed — ordering guarantee for "state set" before "accent
/// applied".
fn send_cmd(hwnd: HWND, msg: u32, wparam: usize, lparam: isize) -> bool {
    let mut result = 0usize;
    unsafe {
        SendMessageTimeoutW(
            hwnd,
            msg,
            WPARAM(wparam),
            LPARAM(lparam),
            SMTO_ABORTIFHUNG | SMTO_BLOCK | SMTO_ERRORONEXIT,
            2_000,
            Some(&mut result),
        )
        .0 != 0
    }
}

/// Arms the `WH_CALLWNDPROC` hook so Windows maps our DLL into `explorer.exe`,
/// then waits for its command window to appear.
fn inject(tap: &mut Tap, taskbar: HWND) -> bool {
    let path = match tap_dll_path() {
        Some(p) if p.exists() => p,
        _ => return false,
    };
    let wide_path = path
        .to_string_lossy()
        .encode_utf16()
        .chain(core::iter::once(0))
        .collect::<Vec<u16>>();
    let lib = unsafe { LoadLibraryW(PCWSTR(wide_path.as_ptr())) };
    if lib.is_null() {
        return false;
    }
    let proc = unsafe { GetProcAddress(lib, PCSTR(c"CallWndProc".as_ptr() as *const u8)) };
    if proc.is_null() {
        unsafe {
            let _ = FreeLibrary(lib);
        }
        return false;
    }
    let mut tid_pid = 0u32;
    let tid = unsafe { GetWindowThreadProcessId(taskbar, Some(&mut tid_pid)) };
    if tid == 0 {
        unsafe {
            let _ = FreeLibrary(lib);
        }
        return false;
    }
    let hook = unsafe { SetWindowsHookExW(WH_CALLWNDPROC, proc, lib, tid) };
    if hook.is_null() {
        unsafe {
            let _ = FreeLibrary(lib);
        }
        return false;
    }
    tap.hook = hook;
    tap.lib = lib;
    tap.msg_cmd = unsafe { RegisterWindowMessageW(PCWSTR(wide("PecoFence.TAP.Command").as_ptr())) };
    tap.msg_unload =
        unsafe { RegisterWindowMessageW(PCWSTR(wide("PecoFence.TAP.Unload").as_ptr())) };
    // Poke the thread so the hook fires immediately rather than on the next
    // spontaneous message.
    unsafe {
        let _ = PostMessageW(Some(taskbar), 0, WPARAM(0), LPARAM(0));
    }
    // Up to 4s for the DLL to boot its command window inside Explorer.
    let class = wide("PecoFenceTAP");
    for _ in 0..80 {
        let cmd = unsafe { FindWindowW(PCWSTR(class.as_ptr()), PCWSTR::null()) };
        if !cmd.0.is_null() {
            tap.cmd_hwnd = cmd;
            return true;
        }
        unsafe { Sleep(50) };
    }
    false
}

/// Drops the hook + our DLL reference once Explorer's copy is running.
fn disarm(tap: &mut Tap) {
    unsafe {
        if !tap.hook.is_null() {
            let _ = UnhookWindowsHookEx(tap.hook);
            tap.hook = core::ptr::null_mut();
        }
        if !tap.lib.is_null() {
            let _ = FreeLibrary(tap.lib);
            tap.lib = core::ptr::null_mut();
        }
    }
}

/// Full teardown: tell the DLL to restore everything, then forget it.
fn unload_locked(tap: &mut Tap) {
    if !tap.cmd_hwnd.0.is_null() && tap.msg_unload != 0 {
        // `unload()` inside the DLL restores brushes before returning.
        let _ = send_cmd(tap.cmd_hwnd, tap.msg_unload, 0, 0);
    }
    disarm(tap);
    tap.cmd_hwnd = HWND(core::ptr::null_mut());
    tap.explorer_pid = 0;
    tap.mode = TapMode::Normal;
}

/// Applies `mode` to the taskbar. `Normal` performs a complete factory
/// restore and removes the DLL from Explorer; Clear/Blur inject (or reuse)
/// the TAP and defend the accent via the in-Explorer detour.
pub fn apply(mode: TapMode) -> bool {
    let Some((taskbar, pid)) = find_taskbar() else {
        return false;
    };
    let mut guard = lock();
    if mode == TapMode::Normal {
        if let Some(tap) = guard.as_mut() {
            // State back to Normal first so the detour stops swallowing
            // Explorer's writes, then unload — no accent payload: the DLL
            // restores the XAML fills and Explorer's own accent (never
            // touched) simply flows again.
            if tap.msg_cmd != 0 && !tap.cmd_hwnd.0.is_null() {
                let _ =
                    send_cmd(
                        tap.cmd_hwnd,
                        tap.msg_cmd,
                        CMD_NORMAL,
                        unsafe { GetCurrentProcessId() } as isize,
                    );
            }
            unload_locked(tap);
        }
        return true;
    }

    let stale = guard
        .as_ref()
        .map(|t| t.explorer_pid != pid || unsafe { IsWindow(t.cmd_hwnd) }.0 == 0)
        .unwrap_or(true);
    if stale && let Some(mut tap) = guard.take() {
        unload_locked(&mut tap);
    }
    let tap = guard.get_or_insert_with(|| Tap {
        hook: core::ptr::null_mut(),
        lib: core::ptr::null_mut(),
        cmd_hwnd: HWND(core::ptr::null_mut()),
        msg_cmd: 0,
        msg_unload: 0,
        explorer_pid: 0,
        mode: TapMode::Normal,
    });
    if tap.explorer_pid != pid {
        if !inject(tap, taskbar) {
            unload_locked(tap);
            return false;
        }
        disarm(tap);
        tap.explorer_pid = pid;
    }
    if !send_cmd(
        tap.cmd_hwnd,
        tap.msg_cmd,
        command_for(mode),
        unsafe { GetCurrentProcessId() } as isize,
    ) {
        return false;
    }
    tap.mode = mode;
    // No accent write: on Win 11 TAP does the entire effect in XAML — TTB
    // never calls SetWindowCompositionAttribute when its TAP service is
    // active, and every manual policy we tried poisoned the DWM plate
    // (ACCENT_DISABLED -> black, TRANSPARENTGRADIENT/0 -> accent-color).
    true
}

/// Cheap liveness check — call periodically (the anchor check timer does).
/// Re-injects and re-applies when Explorer restarted under a non-Normal mode.
/// Also re-sends the state command when nothing is stale: inside the DLL that
/// re-arms the watcher and retries `InitializeXamlDiagnosticsEx` — the retry
/// cadence for the case where the XAML island wasn't ready at inject time.
pub fn ensure() {
    let mode = match lock().as_ref() {
        Some(t) if t.mode != TapMode::Normal => t.mode,
        _ => return,
    };
    if find_taskbar().is_some() {
        // `apply` skips re-injection when the resident TAP is healthy; the
        // command send is cheap (no repaint when state is unchanged).
        let _ = apply(mode);
    }
}

/// Call on shutdown: restores the taskbar and unloads the DLL if injected.
pub fn shutdown() {
    let mut guard = lock();
    if let Some(mut tap) = guard.take() {
        unload_locked(&mut tap);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// `cargo test -p pecofence-platform tap::tests -- --ignored --test-threads=1`
    /// exercises the real injection against the live Explorer. Runs the cycle
    /// twice: `Normal` posts `WM_TAP_UNLOAD` which leaves a resident,
    /// unadvised TAP in Explorer — cycle two must re-arm through the surviving
    /// command window instead of a fresh `DllMain`.
    #[test]
    #[ignore = "injects pecofence_tap.dll into the live explorer.exe"]
    fn inject_clear_blur_normal() {
        for cycle in 1..=2 {
            assert!(apply(TapMode::Clear), "cycle {cycle}: inject+Clear failed");
            std::thread::sleep(std::time::Duration::from_secs(2));
            assert!(apply(TapMode::Blur), "cycle {cycle}: Blur failed");
            std::thread::sleep(std::time::Duration::from_secs(2));
            assert!(
                apply(TapMode::Normal),
                "cycle {cycle}: Normal/unload failed"
            );
            std::thread::sleep(std::time::Duration::from_secs(1));
        }
    }

    /// `TAP_MODE=clear|blur|normal cargo test ... tap_env` — holds the state
    /// for 12s so external verification can observe the taskbar; the DLL's
    /// host-death watch restores Normal when this process exits.
    #[test]
    #[ignore = "holds a taskbar state for manual verification"]
    fn tap_env() {
        let mode = match std::env::var("TAP_MODE").as_deref() {
            Ok("clear") => TapMode::Clear,
            Ok("blur") => TapMode::Blur,
            Ok("opaque") => TapMode::Opaque,
            _ => TapMode::Normal,
        };
        assert!(apply(mode), "apply({mode:?}) failed");
        // Mirror the app's 2 s anchor-tick `ensure` so the TAP's ixde retry
        // path gets exercised during the hold window.
        for _ in 0..12 {
            std::thread::sleep(std::time::Duration::from_secs(1));
            ensure();
        }
    }

    /// Captures the taskbar window surface via WGC — the taskbar's real composed
    /// output, including the acrylic backdrop baked into its plate. In this
    /// remote environment the taskbar does not composite into the *monitor*
    /// frame, so window capture is the reliable verification path.
    #[test]
    #[ignore = "captures the taskbar surface via WGC during Clear"]
    fn taskbar_surface_probe() {
        use crate::gpu_wallpaper::GpuSurface;
        let Some((taskbar, _)) = find_taskbar() else {
            panic!("no Shell_TrayWnd");
        };
        let mut surf = GpuSurface::new(taskbar).expect("GpuSurface::new failed");
        let sample = |surf: &mut GpuSurface, tag: &str| {
            let mut out = String::new();
            for _ in 0..20 {
                std::thread::sleep(std::time::Duration::from_millis(300));
                if let Ok(Some(frame)) = surf.capture(0, 0) {
                    let (w, h) = (frame.width as usize, frame.height as usize);
                    let mut bmp = Vec::with_capacity(54 + w * h * 4);
                    let data = (54 + w * h * 4) as u32;
                    bmp.extend_from_slice(b"BM");
                    bmp.extend_from_slice(&data.to_le_bytes());
                    bmp.extend_from_slice(&[0; 4]);
                    bmp.extend_from_slice(&54u32.to_le_bytes());
                    bmp.extend_from_slice(&40u32.to_le_bytes());
                    bmp.extend_from_slice(&(w as i32).to_le_bytes());
                    bmp.extend_from_slice(&(-(h as i32)).to_le_bytes()); // top-down
                    bmp.extend_from_slice(&1u16.to_le_bytes());
                    bmp.extend_from_slice(&32u16.to_le_bytes());
                    bmp.extend_from_slice(&[0; 24]);
                    bmp.extend_from_slice(&frame.bgra);
                    let path = std::env::temp_dir().join(format!("tb_win_{tag}.bmp"));
                    let _ = std::fs::write(&path, &bmp);
                    let y = h / 2;
                    for (name, x) in [("center", w / 2), ("left", w / 8), ("right", w * 7 / 8)] {
                        let i = (y * w + x) * 4;
                        let p = &frame.bgra[i..i + 4];
                        out += &format!(" {tag} {name}=BGRA({},{},{},{})", p[0], p[1], p[2], p[3]);
                    }
                    break;
                }
            }
            if out.is_empty() {
                out = format!(" {tag}: no frame");
            }
            out
        };
        std::thread::sleep(std::time::Duration::from_secs(3));
        eprintln!("{}", sample(&mut surf, "before"));
        assert!(apply(TapMode::Clear), "apply(Clear) failed");
        std::thread::sleep(std::time::Duration::from_secs(10));
        eprintln!("{}", sample(&mut surf, "clear"));
        assert!(apply(TapMode::Blur), "apply(Blur) failed");
        std::thread::sleep(std::time::Duration::from_secs(3));
        eprintln!("{}", sample(&mut surf, "blur"));
        assert!(apply(TapMode::Normal), "apply(Normal) failed");
        std::thread::sleep(std::time::Duration::from_secs(3));
        eprintln!("{}", sample(&mut surf, "after"));
    }

    /// Simulates the reported sequence `Normal -> Opaque -> Blur -> Clear ->
    /// Normal` — the failure mode where an `ACCENT_DISABLED` restore left a
    /// permanently black DWM plate. With WGC window capture, `Opaque` should
    /// now read ~0,0,0 (pure-black solid plate, distinct from Normal's ~28)
    /// and `Blur`/`Clear` must not turn permanently black.
    #[test]
    #[ignore = "drives the live taskbar through the reported sequence"]
    fn accent_sequence_probe() {
        use crate::gpu_wallpaper::GpuSurface;
        let Some((taskbar, _)) = find_taskbar() else {
            panic!("no Shell_TrayWnd");
        };
        let mut surf = GpuSurface::new(taskbar).expect("GpuSurface::new failed");
        let sample = |surf: &mut GpuSurface, tag: &str| {
            for _ in 0..20 {
                std::thread::sleep(std::time::Duration::from_millis(300));
                if let Ok(Some(frame)) = surf.capture(0, 0) {
                    let (w, h) = (frame.width as usize, frame.height as usize);
                    let y = h / 2;
                    let mut out = String::new();
                    for (name, x) in [("center", w / 2), ("left", w / 8), ("right", w * 7 / 8)] {
                        let i = (y * w + x) * 4;
                        let p = &frame.bgra[i..i + 4];
                        out += &format!(" {tag} {name}=BGRA({},{},{},{})", p[0], p[1], p[2], p[3]);
                    }
                    eprintln!("{out}");
                    return;
                }
            }
            eprintln!("{tag}: no frame");
        };
        assert!(apply(TapMode::Normal), "apply(Normal) failed");
        std::thread::sleep(std::time::Duration::from_secs(3));
        sample(&mut surf, "normal");
        assert!(apply(TapMode::Opaque), "apply(Opaque) failed");
        std::thread::sleep(std::time::Duration::from_secs(3));
        sample(&mut surf, "opaque");
        assert!(apply(TapMode::Blur), "apply(Blur) failed");
        std::thread::sleep(std::time::Duration::from_secs(3));
        sample(&mut surf, "blur");
        assert!(apply(TapMode::Clear), "apply(Clear) failed");
        std::thread::sleep(std::time::Duration::from_secs(3));
        sample(&mut surf, "clear");
        assert!(apply(TapMode::Normal), "apply(Normal) failed");
        std::thread::sleep(std::time::Duration::from_secs(3));
        sample(&mut surf, "restored");
    }
}
