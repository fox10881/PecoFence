//! Product identity and narrowly scoped compatibility with pre-rename installs.

use std::env::VarError;
use std::ffi::OsString;

pub const NAME: &str = "CoreDesk Engine";
/// Internal identifier (directories, pipes, mutexes) — the display name is `NAME`.
pub const ID: &str = "CoreDeskEngine";
/// Microsoft Store product id (Partner Center, packaging/msix/identity.json).
pub const STORE_PRODUCT_ID: &str = "9MV6WG3XNWSX";
pub const REPOSITORY_URL: &str = "https://github.com/fox10881/PecoFence";
/// Previous AppData folder names, newest first: PecoFence was the identity
/// before CoreDesk Engine, OpenFence before PecoFence. Configuration migrates
/// forward from whichever one still holds saved data.
pub const LEGACY_DATA_DIRS: &[&str] = &["PecoFence", "OpenFence"];
pub const LEGACY_DATA_DIR: &str = "OpenFence";
pub const LEGACY_AUTOSTART: &[&str] = &["PecoFence", "openFence"];
pub const LEGACY_MUTEX: &[&str] = &["PecoFence", "openFence"];
/// Environment prefix chain: `COREDESK_X` wins, then the older
/// `PECOFENCE_X`/`OPENFENCE_X` overrides.
const ENV_PREFIXES: &[&str] = &["COREDESK_", "PECOFENCE_", "OPENFENCE_"];

fn lookup_env(name: &str, lookup: impl Fn(&str) -> Option<OsString>) -> Option<OsString> {
    if let Some(v) = lookup(name) {
        return Some(v);
    }
    for prefix in ENV_PREFIXES {
        if let Some(suffix) = name.strip_prefix(prefix) {
            for alt in ENV_PREFIXES.iter().filter(|p| *p != prefix) {
                if let Some(v) = lookup(&format!("{alt}{suffix}")) {
                    return Some(v);
                }
            }
        }
    }
    None
}

/// Canonical variables take priority; old environment overrides keep working.
pub fn var_os(name: &str) -> Option<OsString> {
    lookup_env(name, |key| std::env::var_os(key))
}

pub fn var(name: &str) -> Result<String, VarError> {
    var_os(name)
        .ok_or(VarError::NotPresent)?
        .into_string()
        .map_err(VarError::NotUnicode)
}

/// Hold every generational name so a renamed build and any older executable
/// cannot concurrently manage the same desktop. Named test instances remain
/// independent of the main app.
pub fn instance_mutex_names(instance: Option<&str>) -> Vec<String> {
    let suffix = instance_suffix(instance);
    let mut names = Vec::with_capacity(LEGACY_MUTEX.len() + 1);
    names.push(format!(r"Local\{ID}.SingleInstance{suffix}"));
    for legacy in LEGACY_MUTEX {
        names.push(format!(r"Local\{legacy}.SingleInstance{suffix}"));
    }
    names
}

/// Named pipe the CLI (`coredesk-cli`) talks to: `CoreDeskEngine.<user SID>[.<instance>]`.
/// Pipe names are machine-wide, so the SID keeps two signed-in accounts apart, while the
/// same user in another session (ssh) still reaches their own instance.
pub fn ipc_pipe_name(user_sid: &str, instance: Option<&str>) -> String {
    format!(r"\\.\pipe\{ID}.{user_sid}{}", instance_suffix(instance))
}

/// [`log_file_name`] suffixes: this run's log and the rotated-out part before it (the
/// previous run's, or the previous 1 MiB when a run outgrows the file).
pub const LOG: &str = "";
pub const PREVIOUS_LOG: &str = ".prev";

/// Log file in `%LOCALAPPDATA%\CoreDeskEngine`: `coredesk[.name]<suffix>.log`, one set per
/// instance.
pub fn log_file_name(instance: Option<&str>, suffix: &str) -> String {
    format!("coredesk{}{suffix}.log", instance_suffix(instance))
}

/// `.name` for a named (test) instance, empty for the main one; whitespace-only = main.
fn instance_suffix(instance: Option<&str>) -> String {
    instance
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .map(|value| format!(".{value}"))
        .unwrap_or_default()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_environment_overrides_legacy_without_mutating_process_environment() {
        let both = |name: &str| match name {
            "COREDESK_INSTANCE" => Some(OsString::from("new")),
            "PECOFENCE_INSTANCE" => Some(OsString::from("mid")),
            "OPENFENCE_INSTANCE" => Some(OsString::from("old")),
            _ => None,
        };
        assert_eq!(lookup_env("COREDESK_INSTANCE", both), Some("new".into()));
        assert_eq!(
            lookup_env("COREDESK_INSTANCE", |name| {
                match name {
                    "PECOFENCE_INSTANCE" => Some(OsString::from("mid")),
                    "OPENFENCE_INSTANCE" => Some(OsString::from("old")),
                    _ => None,
                }
            }),
            Some("mid".into())
        );
        assert_eq!(
            lookup_env("COREDESK_INSTANCE", |name| {
                (name == "OPENFENCE_INSTANCE").then(|| OsString::from("legacy"))
            }),
            Some("legacy".into())
        );
        assert_eq!(
            lookup_env("COREDESK_INSTANCE", |name| {
                (name == "COREDESK_INSTANCE").then(OsString::new)
            }),
            Some(OsString::new())
        );
        assert_eq!(lookup_env("UNRELATED", both), None);
    }

    #[test]
    fn legacy_and_current_instance_names_use_the_same_normalized_suffix() {
        let main = instance_mutex_names(None);
        assert_eq!(main[0], r"Local\CoreDeskEngine.SingleInstance");
        assert_eq!(main[1], r"Local\PecoFence.SingleInstance");
        assert_eq!(main[2], r"Local\openFence.SingleInstance");
        assert_eq!(main, instance_mutex_names(Some("  ")));
        let named = instance_mutex_names(Some(" test "));
        assert_eq!(named[0], r"Local\CoreDeskEngine.SingleInstance.test");
        assert_eq!(named[2], r"Local\openFence.SingleInstance.test");
        assert_ne!(main, named);
    }

    #[test]
    fn pipe_name_carries_the_user_sid_and_the_instance_suffix() {
        let sid = "S-1-5-21-1-2-3-1001";
        assert_eq!(
            ipc_pipe_name(sid, None),
            r"\\.\pipe\CoreDeskEngine.S-1-5-21-1-2-3-1001"
        );
        assert_eq!(ipc_pipe_name(sid, Some(" ")), ipc_pipe_name(sid, None));
        assert_eq!(
            ipc_pipe_name(sid, Some(" test ")),
            r"\\.\pipe\CoreDeskEngine.S-1-5-21-1-2-3-1001.test"
        );
        assert_ne!(
            ipc_pipe_name(sid, None),
            ipc_pipe_name("S-1-5-21-1-2-3-1002", None)
        );
    }

    #[test]
    fn log_names_follow_the_instance_suffix() {
        assert_eq!(log_file_name(None, LOG), "coredesk.log");
        assert_eq!(log_file_name(Some(" "), PREVIOUS_LOG), "coredesk.prev.log");
        assert_eq!(
            log_file_name(Some("test"), PREVIOUS_LOG),
            "coredesk.test.prev.log"
        );
    }
}
