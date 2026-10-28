//! Command channel: a message-only window inside Explorer that the host
//! process `PostMessage`s state changes to — the lightweight replacement for
//! TranslucentTB's COM `TaskbarAppearanceService` RPC. Also owns the orderly
//! `WM_TAP_UNLOAD` teardown and the host-death fallback
//! (`RestoreAllTaskbarsToDefaultWhenProcessDies`).

use crate::bindings::HWND;
use crate::hook;
use crate::shared::{self, STATE_NORMAL, TAP_STATE};
use crate::win32::*;
use std::sync::atomic::{AtomicU32, Ordering};

pub const TAP_WINDOW_CLASS: &[u16] = &[
    0x50, 0x65, 0x63, 0x6f, 0x46, 0x65, 0x6e, 0x63, 0x65, 0x54, 0x41, 0x50,
    0x00, // "PecoFenceTAP"
];
const CMD_NAME: &[u16] = &[
    0x50, 0x65, 0x63, 0x6f, 0x46, 0x65, 0x6e, 0x63, 0x65, 0x2e, 0x54, 0x41, 0x50, 0x2e, 0x43, 0x6f,
    0x6d, 0x6d, 0x61, 0x6e, 0x64, 0x00, // "PecoFence.TAP.Command"
];
const UNLOAD_NAME: &[u16] = &[
    0x50, 0x65, 0x63, 0x6f, 0x46, 0x65, 0x6e, 0x63, 0x65, 0x2e, 0x54, 0x41, 0x50, 0x2e, 0x55, 0x6e,
    0x6c, 0x6f, 0x61, 0x64, 0x00, // "PecoFence.TAP.Unload"
];

static CMD_MSG: AtomicU32 = AtomicU32::new(0);
static UNLOAD_MSG: AtomicU32 = AtomicU32::new(0);

/// `wParam` values the host sends on the command message.
pub const CMD_NORMAL: usize = 0;
pub const CMD_CLEAR: usize = 1;
pub const CMD_BLUR: usize = 2;
pub const CMD_OPAQUE: usize = 3;

pub fn run() {
    unsafe {
        CMD_MSG.store(RegisterWindowMessageW(pcwstr(CMD_NAME)), Ordering::Release);
        UNLOAD_MSG.store(
            RegisterWindowMessageW(pcwstr(UNLOAD_NAME)),
            Ordering::Release,
        );

        let class = WNDCLASSEXW {
            cbSize: core::mem::size_of::<WNDCLASSEXW>() as u32,
            style: 0,
            lpfnWndProc: Some(wndproc),
            cbClsExtra: 0,
            cbWndExtra: 0,
            hInstance: crate::dll_instance(),
            hIcon: core::ptr::null_mut(),
            hCursor: core::ptr::null_mut(),
            hbrBackground: core::ptr::null_mut(),
            lpszMenuName: windows_core::PCWSTR::null(),
            lpszClassName: pcwstr(TAP_WINDOW_CLASS),
            hIconSm: core::ptr::null_mut(),
        };
        if RegisterClassExW(&class) == 0 {
            return;
        }
        let hwnd = CreateWindowExW(
            0,
            pcwstr(TAP_WINDOW_CLASS),
            windows_core::PCWSTR::null(),
            0,
            0,
            0,
            0,
            0,
            HWND_MESSAGE,
            core::ptr::null_mut(),
            crate::dll_instance(),
            core::ptr::null(),
        );
        if hwnd.0.is_null() {
            return;
        }
        shared::lock().command_window = hwnd;
        crate::tap_log!("command window ready hwnd={:?}", hwnd.0);

        let mut msg = core::mem::zeroed::<MSG>();
        while GetMessageW(&mut msg, HWND(core::ptr::null_mut()), 0, 0).0 > 0 {
            let _ = TranslateMessage(&msg);
            DispatchMessageW(&msg);
        }
        shared::lock().command_window = HWND(core::ptr::null_mut());
    }
}

fn pcwstr(s: &'static [u16]) -> windows_core::PCWSTR {
    windows_core::PCWSTR(s.as_ptr())
}

unsafe extern "system" fn wndproc(hwnd: HWND, msg: u32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    let _ = std::panic::catch_unwind(core::panic::AssertUnwindSafe(|| {
        handle_message(hwnd, msg, wparam, lparam)
    }));
    unsafe { DefWindowProcW(hwnd, msg, wparam, lparam) }
}

fn handle_message(_hwnd: HWND, msg: u32, wparam: WPARAM, lparam: LPARAM) {
    if msg == CMD_MSG.load(Ordering::Acquire) && msg != 0 {
        // First command carries the host pid in lParam: arm the death watch
        // once (TTB's RestoreAllTaskbarsToDefaultWhenProcessDies).
        if lparam != 0 {
            watch_host_process(lparam as u32);
        }
        let state = match wparam {
            CMD_NORMAL => shared::STATE_NORMAL,
            CMD_CLEAR => shared::STATE_CLEAR,
            CMD_BLUR => shared::STATE_BLUR,
            CMD_OPAQUE => shared::STATE_OPAQUE,
            _ => return,
        };
        // Arm/disarm the detour and re-arm the watcher before the state flips,
        // so no Explorer accent write can slip through in between.
        if state != shared::STATE_NORMAL {
            hook::set_enabled(true);
            crate::site::ensure_watching();
        }
        crate::tap_log!("cmd state={state} host_pid={lparam}");
        if TAP_STATE.swap(state, Ordering::AcqRel) != state {
            shared::dispatch_apply(0);
        }
        if state == shared::STATE_NORMAL {
            hook::set_enabled(false);
        }
    } else if msg == UNLOAD_MSG.load(Ordering::Acquire) && msg != 0 {
        crate::tap_log!("unload requested");
        unload();
    }
}

fn watch_host_process(pid: u32) {
    let mut shared = shared::lock();
    if !shared.host_wait.is_null() {
        return; // already watching
    }
    let process = unsafe {
        OpenProcess(
            SYNCHRONIZE | PROCESS_QUERY_LIMITED_INFORMATION,
            windows_core::BOOL(0),
            pid,
        )
    };
    if process.is_null() {
        return;
    }
    let mut wait: HANDLE = core::ptr::null_mut();
    let ok = unsafe {
        RegisterWaitForSingleObject(
            &mut wait,
            process,
            Some(host_died),
            core::ptr::null(),
            INFINITE,
            WT_EXECUTEONLYONCE,
        )
    };
    if ok.0 == 0 {
        unsafe {
            let _ = CloseHandle(process);
        }
        return;
    }
    shared.host_wait = wait;
    // The wait does NOT own `process` — closing it while the wait is pending
    // is undefined behavior and silently kills the death watch. Keep the
    // handle until `host_died`/`unload` closes it.
    shared.host_process = process;
}

/// Host vanished: drop back to Normal so the detour stops swallowing
/// Explorer's writes, restore the XAML brushes, and reset the window accent.
/// The fired wait auto-unregisters (`WT_EXECUTEONLYONCE`); clearing the stored
/// handle lets the *next* host's first command re-arm the death watch.
unsafe extern "system" fn host_died(_context: *mut core::ffi::c_void, _fired: windows_core::BOOL) {
    let _ = std::panic::catch_unwind(core::panic::AssertUnwindSafe(|| {
        TAP_STATE.store(STATE_NORMAL, Ordering::Release);
        shared::dispatch_apply(0);
        hook::set_enabled(false);
        let mut shared = shared::lock();
        shared.host_wait = core::ptr::null_mut();
        if !shared.host_process.is_null() {
            unsafe {
                let _ = CloseHandle(shared.host_process);
            }
            shared.host_process = core::ptr::null_mut();
        }
    }));
}

/// `WM_TAP_UNLOAD`: full factory restore — original brushes back on the UI
/// thread (blocking so the host's `SendMessage` only returns once the taskbar
/// is really back), watcher unadvised, detour disarmed, taskbar accent reset.
///
/// The command window and the DLL stay alive: `InitializeXamlDiagnosticsEx`
/// pins the DLL in Explorer permanently, so a destroyed pump could never be
/// recreated on re-injection — an inert resident is exactly what
/// TranslucentTB keeps. Everything it *applied* is still fully unwound.
fn unload() {
    TAP_STATE.store(STATE_NORMAL, Ordering::Release);
    // Synchronous restore: paint originals before returning.
    shared::dispatch_apply(3_000);
    // Snapshot the service/watcher under the lock, then release it — the
    // unadvise call marshals to the UI thread and must not race callbacks
    // that take `shared`.
    let pair = {
        let mut shared = shared::lock();
        shared.tree_service.clone().zip(shared.watcher.take())
    };
    if let Some((service, watcher)) = pair {
        unsafe {
            let _ = service.UnadviseVisualTreeChange(&*watcher);
        }
    }
    {
        let mut shared = shared::lock();
        if !shared.host_wait.is_null() {
            unsafe {
                let _ = UnregisterWaitEx(shared.host_wait, core::ptr::null_mut());
            }
            shared.host_wait = core::ptr::null_mut();
        }
        if !shared.host_process.is_null() {
            unsafe {
                let _ = CloseHandle(shared.host_process);
            }
            shared.host_process = core::ptr::null_mut();
        }
        let rects = core::mem::take(&mut shared.rects);
        shared.slot_handles.clear();
        shared.bg_handles.clear();
        shared.applied = shared::STATE_NORMAL;
        drop(shared);
        drop(rects); // COM releases happen outside the lock
    }
    hook::uninstall();
}
