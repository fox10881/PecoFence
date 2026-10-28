//! `pecofence_tap.dll` — the Explorer-side half of the taskbar transparency
//! feature, modelled on TranslucentTB's `ExplorerTAP`.
//!
//! Injected into `explorer.exe` by `pecofence.exe` through a `WH_CALLWNDPROC`
//! hook (the `CallWndProc` export is the injection vector). On attach it:
//!  1. detours `user32!SetWindowCompositionAttribute` in-process so Explorer
//!     cannot overwrite the taskbar accent we apply from the host;
//!  2. opens a message-only command window (`PecoFenceTAP`) the host posts
//!     state changes to;
//!  3. initializes XAML Diagnostics (`InitializeXamlDiagnosticsEx`) so a
//!     `TAPSite` COM object hands us the visual-tree service used to repaint
//!     the taskbar's `BackgroundFill`/`BackgroundStroke` rectangles.

#[allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals,
    clippy::all
)]
mod bindings;
mod command;
#[allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals,
    clippy::all
)]
mod effects;
#[allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals,
    clippy::all
)]
mod fx_bindings;
mod hook;
mod shared;
mod site;
mod win32;

/// Debug-build-only tracing: the DLL swallows COM errors by design (Explorer
/// must never crash for cosmetics), so diagnosis needs a side channel. Each
/// line appends to `%TEMP%\pecofence_tap.log`; release builds compile it out.
/// One shared `File` under a mutex — per-line `open()` raced concurrent
/// writers and could silently drop lines.
#[cfg(debug_assertions)]
pub(crate) fn log(msg: &str) {
    use std::io::Write;
    static LOG_FILE: std::sync::Mutex<Option<std::fs::File>> = std::sync::Mutex::new(None);
    let mut guard = LOG_FILE.lock().unwrap_or_else(|p| p.into_inner());
    if guard.is_none() {
        *guard = std::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(std::env::temp_dir().join("pecofence_tap.log"))
            .ok();
    }
    if let Some(f) = guard.as_mut() {
        let _ = writeln!(f, "{:>8.3?} {msg}", std::time::Instant::now());
        let _ = f.flush();
    }
}

#[cfg(not(debug_assertions))]
#[inline(always)]
pub(crate) fn log(_msg: &str) {}

macro_rules! tap_log {
    ($($t:tt)*) => { $crate::log(&format!($($t)*)) };
}
pub(crate) use tap_log;

use std::sync::atomic::{AtomicPtr, Ordering};

static DLL_INSTANCE: AtomicPtr<core::ffi::c_void> = AtomicPtr::new(core::ptr::null_mut());

pub(crate) fn dll_instance() -> win32::HMODULE {
    DLL_INSTANCE.load(Ordering::Relaxed)
}

/// # Safety
/// Entry point invoked by the loader; `reason` values come from Windows.
#[unsafe(no_mangle)]
pub unsafe extern "system" fn DllMain(
    hinstance: win32::HINSTANCE,
    reason: u32,
    _reserved: *mut core::ffi::c_void,
) -> i32 {
    if reason == win32::DLL_PROCESS_ATTACH {
        DLL_INSTANCE.store(hinstance, Ordering::Relaxed);
        unsafe {
            let _ = win32::DisableThreadLibraryCalls(hinstance);
        }
        if is_explorer_process() {
            std::thread::spawn(bootstrap);
        }
    }
    1
}

/// The exported hook procedure — presence in this DLL is all the injection
/// needs; it never does work itself.
///
/// # Safety
/// Called by Windows from the hooked thread with valid arguments.
#[unsafe(no_mangle)]
pub unsafe extern "system" fn CallWndProc(
    code: i32,
    wparam: win32::WPARAM,
    lparam: win32::LPARAM,
) -> win32::LRESULT {
    unsafe { win32::CallNextHookEx(core::ptr::null_mut(), code, wparam, lparam) }
}

fn is_explorer_process() -> bool {
    let mut path = [0u16; 260];
    let len = unsafe {
        win32::GetModuleFileNameW(
            core::ptr::null_mut(),
            windows_core::PWSTR(path.as_mut_ptr()),
            path.len() as u32,
        )
    };
    if len == 0 {
        return false;
    }
    let name = &path[..len as usize];
    let explorer: Vec<u16> = "explorer.exe".encode_utf16().collect();
    // `|0x20` lowercases ASCII letters — enough for a suffix compare.
    name.len() >= explorer.len()
        && name[name.len() - explorer.len()..]
            .iter()
            .zip(&explorer)
            .all(|(a, b)| (*a | 0x20) == *b)
}

fn bootstrap() {
    // MTA for the threads we own; the XAML callbacks arrive on the UI thread
    // the diagnostics service marshals to, so this only affects our own calls.
    unsafe {
        let _ = win32::RoInitialize(0);
    }
    // The detour goes in first — it must be live before the host posts its
    // first state command so no Explorer write can slip through.
    let _ = hook::install();
    std::thread::spawn(command::run);
    std::thread::spawn(site::install);
}
