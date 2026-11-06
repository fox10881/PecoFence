//! coredesk-watchdog: waits for the main process to exit and, if it died without restoring the
//! desktop icons (marker file still present), sends the "show desktop icons" toggle itself.
//!
//! Usage: `coredesk-watchdog <parent-pid> <marker-path>`

#![windows_subsystem = "windows"]

use coredesk_platform::{desktop, process, shell_icons, window};
use std::path::PathBuf;

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let Some(pid) = args.first().and_then(|a| a.parse::<u32>().ok()) else {
        return;
    };
    let Some(marker) = args.get(1).map(PathBuf::from) else {
        return;
    };

    // Block until the parent exits (or fails to open → it is already gone).
    process::wait_for_process_exit(pid);

    if !marker.exists() {
        return; // clean shutdown restored the icons already
    }
    // Give Explorer a moment, then restore if still hidden. Explorer may be restarting or hung,
    // so keep trying for a while; if it never works the marker stays, and the next start adopts
    // the hidden icons and restores them at exit. A CoreDesk Engine started meanwhile takes them
    // over: the icons and the marker are its own then.
    let instance = coredesk_core::brand::var("COREDESK_INSTANCE").ok();
    let mutex = coredesk_core::brand::instance_mutex_names(instance.as_deref())
        .into_iter()
        .next()
        .unwrap_or_default();
    std::thread::sleep(std::time::Duration::from_millis(300));
    for attempt in 0..10 {
        if attempt > 0 {
            std::thread::sleep(std::time::Duration::from_secs(1));
        }
        if window::SingleInstance::is_held(&mutex) {
            return;
        }
        let restored = !shell_icons::desktop_icons_hidden()
            || desktop::resolve_icon_host(desktop::detect_generation()).is_some_and(|host| {
                shell_icons::set_desktop_icons_hidden(false, host.def_view, host.host)
                    == Some(false)
            });
        if restored {
            let _ = std::fs::remove_file(&marker);
            return;
        }
    }
}
