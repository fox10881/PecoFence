//! `SetWindowCompositionAttribute` detour, the TranslucentTB `swcadetour.cpp`
//! equivalent. Explorer periodically rewrites the taskbar's `ACCENT_POLICY`;
//! without this hook it fights whatever we applied, strobing between the two
//! states. While our mode is Clear or Blur the hook swallows accent writes
//! aimed at taskbar windows and returns success, exactly like TTB's detour
//! (theirs asks the worker over IPC; ours reads a local atomic — same effect
//! without leaving the process).

use crate::shared::{STATE_NORMAL, TAP_STATE};
use crate::win32;
use retour::RawDetour;
use std::sync::Mutex;
use std::sync::atomic::{AtomicPtr, Ordering};

#[repr(C)]
pub struct WindowCompositionAttribData {
    pub attrib: i32,
    pub pv_data: *mut core::ffi::c_void,
    pub cb_data: u32,
}

pub const WCA_ACCENT_POLICY: i32 = 0x13;

type SwcaFn = unsafe extern "system" fn(
    crate::bindings::HWND,
    *const WindowCompositionAttribData,
) -> windows_core::BOOL;

static DETOUR: Mutex<Option<RawDetour>> = Mutex::new(None);
static TRAMPOLINE: AtomicPtr<()> = AtomicPtr::new(core::ptr::null_mut());
/// Whether the detour is currently armed — every `enable`/`disable` walks and
/// suspends the process's threads, so callers must not pay that per command.
static ARMED: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);

/// True for `Shell_TrayWnd`/`Shell_SecondaryTrayWnd` — the only windows whose
/// accent writes we are allowed to veto.
fn is_taskbar(hwnd: crate::bindings::HWND) -> bool {
    let mut buf = [0u16; 64];
    let len = unsafe {
        win32::GetClassNameW(
            hwnd,
            windows_core::PWSTR(buf.as_mut_ptr()),
            buf.len() as i32,
        )
    };
    if len <= 0 {
        return false;
    }
    let name = &buf[..len as usize];
    name == &windows_core::h!("Shell_TrayWnd")[..]
        || name == &windows_core::h!("Shell_SecondaryTrayWnd")[..]
}

unsafe extern "system" fn swca_detour(
    hwnd: crate::bindings::HWND,
    data: *const WindowCompositionAttribData,
) -> windows_core::BOOL {
    if TAP_STATE.load(Ordering::Acquire) != STATE_NORMAL
        && !data.is_null()
        && unsafe { (*data).attrib } == WCA_ACCENT_POLICY
        && is_taskbar(hwnd)
    {
        // Claim success without forwarding — Explorer's write is discarded.
        return windows_core::BOOL(1);
    }
    let original: SwcaFn = unsafe { core::mem::transmute(TRAMPOLINE.load(Ordering::Acquire)) };
    unsafe { original(hwnd, data) }
}

/// Resolves `user32!SetWindowCompositionAttribute` and detours it in-process.
/// Idempotent — the detour persists across unload/re-inject cycles; only its
/// enabled flag toggles.
pub fn install() -> Result<(), ()> {
    let mut guard = DETOUR.lock().unwrap_or_else(|p| p.into_inner());
    if guard.is_some() {
        return Ok(());
    }
    let user32 = unsafe { win32::GetModuleHandleW(windows_core::w!("user32.dll")) };
    if user32.is_null() {
        return Err(());
    }
    let target = unsafe {
        win32::GetProcAddress(
            user32,
            windows_core::PCSTR(c"SetWindowCompositionAttribute".as_ptr() as *const u8),
        )
    };
    if target.is_null() {
        return Err(());
    }
    let detour =
        unsafe { RawDetour::new(target as *const (), swca_detour as *const ()) }.map_err(|_| ())?;
    TRAMPOLINE.store(
        detour.trampoline() as *const () as *mut (),
        Ordering::Release,
    );
    unsafe {
        detour.enable().map_err(|_| ())?;
    }
    ARMED.store(true, Ordering::Release);
    *guard = Some(detour);
    Ok(())
}

/// Arms or disarms the detour. Disarmed keeps the trampoline in place (a
/// no-op passthrough is cheap) so re-injection never rewrites code — matching
/// "disable the SWCA detour" for Normal without an unsafe teardown.
/// No-op when already in the requested state — `enable`/`disable` suspend
/// every thread in the process, which must not happen per command.
pub fn set_enabled(on: bool) {
    if ARMED.swap(on, Ordering::AcqRel) == on {
        return;
    }
    if let Some(detour) = DETOUR.lock().unwrap_or_else(|p| p.into_inner()).as_ref() {
        unsafe {
            let _ = if on {
                detour.enable()
            } else {
                detour.disable()
            };
        }
    }
}

/// Disarms the detour. `RawDetour` is left allocated afterwards: tearing the
/// trampoline down while an Explorer thread may still sit inside it is unsafe;
/// `disable` restores the original prologue, which is all unload requires.
pub fn uninstall() {
    set_enabled(false);
}
