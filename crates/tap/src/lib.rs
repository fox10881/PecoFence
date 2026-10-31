//! `pecofence_tap.dll` — the resident XAML-styling engine injected into
//! `explorer.exe` (taskbar themes), `StartMenuExperienceHost.exe` (Start
//! Menu themes) and `ShellExperienceHost.exe`/`ShellHost.exe` (Notification
//! Center themes), modelled on TranslucentTB's `ExplorerTAP` + Windhawk's
//! `visualtreewatcher`.
//!
//! Injected by `pecofence.exe` through a `WH_CALLWNDPROC` hook (the
//! `CallWndProc` export is the injection vector). On attach it:
//!  1. opens a message-only command window (`PecoFenceTAP` /
//!     `PecoFenceTAPSM`) the host delivers `PECO-TBS1`/`PECO-SMS1` theme
//!     wires to over `WM_COPYDATA`;
//!  2. initializes XAML Diagnostics (`InitializeXamlDiagnosticsEx`) so a
//!     `TAPSite` COM object hands us the visual-tree service that feeds
//!     `styler`;
//!  3. restores every original property value when the host exits, resets,
//!     or sends `PecoFence.TAP.Unload`.

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
mod shared;
mod site;
#[allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals,
    clippy::all
)]
mod styler;
#[allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals,
    clippy::all
)]
mod styler_bindings;
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
        if is_explorer_process() || is_smeh_process() || is_nch_process() {
            // Self-pin: the host unhooks right after our command window
            // appears, and `InitializeXamlDiagnosticsEx` only pins us later,
            // inside `SetSite`. Without our own pin the gap between
            // `UnhookWindowsHookEx` and `SetSite` lets the refcount hit zero
            // — the resident unloaded ~40ms after every inject and the whole
            // cycle repeated each host tick. The inert-resident design makes
            // a permanent pin correct anyway.
            const GET_MODULE_HANDLE_EX_FLAG_FROM_ADDRESS: u32 = 0x4;
            const GET_MODULE_HANDLE_EX_FLAG_PIN: u32 = 0x1;
            let mut pinned = core::ptr::null_mut();
            unsafe {
                let _ = win32::GetModuleHandleExW(
                    GET_MODULE_HANDLE_EX_FLAG_FROM_ADDRESS | GET_MODULE_HANDLE_EX_FLAG_PIN,
                    windows_core::PCWSTR(hinstance as *const u16),
                    &mut pinned,
                );
            }
            tap_log!("DllMain attach hmod={hinstance:?} smeh={}", is_smeh());
            std::thread::spawn(bootstrap);
        }
    } else if reason == 0 && (is_explorer_process() || is_smeh_process() || is_nch_process()) {
        // DLL_PROCESS_DETACH — only meaningful inside a host; the host-side
        // `FreeLibrary(tap.lib)` in pecofence.exe would otherwise log the
        // same line and look like a resident unload.
        tap_log!("DllMain DETACH smeh={} nch={}", is_smeh(), is_nch_process());
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

/// The exported win-event procedure — the `WINEVENT_INCONTEXT` injection
/// vector for suspended AppContainer hosts: the OS maps this DLL into the
/// target when it fires its first accessibility event (i.e. exactly when a
/// frozen host wakes to render). The proc itself is a no-op; `DllMain` did
/// the bootstrap at map time.
///
/// # Safety
/// Called by Windows inside the hooked process with valid arguments.
#[unsafe(no_mangle)]
pub unsafe extern "system" fn WinEventProc(
    _hook: *mut core::ffi::c_void,
    _event: u32,
    _hwnd: win32::HWND,
    _object: i32,
    _child: i32,
    _tid: u32,
    _time: u32,
) {
}

fn process_name_is(target: &str) -> bool {
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
    let want: Vec<u16> = target.encode_utf16().collect();
    // `|0x20` lowercases ASCII letters — enough for a suffix compare.
    name.len() >= want.len()
        && name[name.len() - want.len()..]
            .iter()
            .zip(&want)
            .all(|(a, b)| (*a | 0x20) == *b)
}

fn is_explorer_process() -> bool {
    process_name_is("explorer.exe")
}

/// `true` inside `StartMenuExperienceHost.exe` — the Start Menu styler's
/// host process. Checked once; the process image never changes.
pub(crate) fn is_smeh() -> bool {
    static IS_SMEH: std::sync::OnceLock<bool> = std::sync::OnceLock::new();
    *IS_SMEH.get_or_init(|| process_name_is("startmenuexperiencehost.exe"))
}

fn is_smeh_process() -> bool {
    is_smeh()
}

/// `true` inside `ShellExperienceHost.exe` or `ShellHost.exe` — the
/// Notification Center styler's host processes.
pub(crate) fn is_nch() -> bool {
    static IS_NCH: std::sync::OnceLock<bool> = std::sync::OnceLock::new();
    *IS_NCH.get_or_init(|| {
        process_name_is("shellexperiencehost.exe") || process_name_is("shellhost.exe")
    })
}

fn is_nch_process() -> bool {
    is_nch()
}

fn bootstrap() {
    // MTA for the threads we own; the XAML callbacks arrive on the UI thread
    // the diagnostics service marshals to, so this only affects our own calls.
    unsafe {
        let _ = win32::RoInitialize(0);
    }
    std::thread::spawn(command::run);
    std::thread::spawn(site::install);
}
