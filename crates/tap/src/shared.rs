//! State shared between the command window and the XAML visual-tree watcher:
//! the diagnostics objects handed over by `IObjectWithSite::SetSite`, the
//! host-death wait, and the command window handle.

use crate::bindings::*;
use crate::win32::{HANDLE, HWND};
use std::sync::Mutex;

pub struct Shared {
    /// The XAML diagnostics object handed to `IObjectWithSite::SetSite`.
    pub diagnostics: Option<IXamlDiagnostics>,
    /// `IVisualTreeService3` on the same object; kept to `Unadvise` on unload.
    pub tree_service: Option<IVisualTreeService3>,
    /// Our registered callback; held so the framework never sees a dangling ref.
    pub watcher: Option<IVisualTreeServiceCallback2>,
    /// Wait handle from `RegisterWaitForSingleObject` watching the host process.
    pub host_wait: HANDLE,
    /// Open handle to the host process — kept alive for the wait's duration.
    pub host_process: HANDLE,
    /// Handle of our command window, once created.
    pub command_window: HWND,
}

// The Mutex serializes every access; individual COM objects are only invoked
// on the thread that owns them (diagnostics calls from the watcher thread).
unsafe impl Send for Shared {}

static SHARED: Mutex<Shared> = Mutex::new(Shared {
    diagnostics: None,
    tree_service: None,
    watcher: None,
    host_wait: core::ptr::null_mut(),
    host_process: core::ptr::null_mut(),
    command_window: HWND(core::ptr::null_mut()),
});

pub fn lock() -> std::sync::MutexGuard<'static, Shared> {
    match SHARED.lock() {
        Ok(g) => g,
        Err(p) => p.into_inner(),
    }
}
