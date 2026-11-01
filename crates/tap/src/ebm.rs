//! ExplorerBlurMica relay — the vendored `ExplorerBlurMica.dll` is a COM
//! shell extension whose `DllMain` self-installs its hooks when loaded
//! into `explorer.exe` (it refuses other processes by returning FALSE).
//! FolderExtensions registration needs HKLM/admin, so instead of COM the
//! platform sends the DLL's path over `EBML` and we `LoadLibraryW` it
//! in-process — identical engine, zero admin, per-pid coverage wherever
//! a resident already lives. `EBMU` unloads it (`ExplorerBlurMica.Detach`
//! restores originals).

use crate::win32;
use core::sync::atomic::{AtomicIsize, Ordering};
use windows_core::PCWSTR;

/// HMODULE of the EBM image this resident loaded — 0 when unloaded.
/// `DllMain` re-entry semantics make double-load dangerous, so the
/// handle itself is the guard.
static EBM_MODULE: AtomicIsize = AtomicIsize::new(0);

/// `EBML` payload = UTF-16 NUL-terminated path to `ExplorerBlurMica.dll`.
pub fn load(bytes: &[u8]) {
    if EBM_MODULE.load(Ordering::Acquire) != 0 {
        return;
    }
    if bytes.len() < 4 || bytes.len() % 2 != 0 {
        return;
    }
    // COPYDATA carries the UTF-16 buffer including its terminator.
    let h = unsafe { win32::LoadLibraryW(PCWSTR(bytes.as_ptr() as *const u16)) };
    if h.is_null() {
        crate::tap_log!("ebm: LoadLibraryW failed");
        return;
    }
    EBM_MODULE.store(h as isize, Ordering::Release);
    crate::tap_log!("ebm: loaded");
}

/// `EBMU` — unload the image; EBM's `DllMain` detach path restores the
/// windows it styled.
pub fn unload() {
    let h = EBM_MODULE.swap(0, Ordering::AcqRel);
    if h != 0 {
        unsafe {
            let _ = win32::FreeLibrary(h as win32::HMODULE);
        }
        crate::tap_log!("ebm: unloaded");
    }
}

/// Unload on our own teardown (`WM_TAP_UNLOAD` / `DllMain` detach) so a
/// disabled effect doesn't outlive the resident that armed it.
pub fn shutdown() {
    unload();
}
