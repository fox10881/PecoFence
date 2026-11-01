//! Host-side controller for the File Explorer material — drives the real
//! [ExplorerBlurMica](https://github.com/Maplespe/ExplorerBlurMica)
//! extension binary (vendored `third_party/ebm/ExplorerBlurMica.dll`,
//! LGPLv3). Upstream's `*.cpp` sources are private, so the release binary
//! is the only bit-exact engine; this module loads THE DLL ITSELF rather
//! than re-implementing its pipeline.
//!
//! Delivery — `EBML`/`EBMU` COPYDATA through the `pecofence_tap`
//! resident already injected in each explorer process:
//!
//! ```text
//! apply_effect → deploy DLL + write config.ini → per cabinet-owning
//!   explorer pid: ensure_resident_at → EBML <utf16 dll path>
//!     → resident LoadLibraryW(path) → EBM's DllMain self-installs
//! reset       → EBMU broadcast to every resident window
//!     → resident FreeLibrary → DllMain detach restores originals
//! ```
//!
//! Why not COM registration: `Drive\shellex\FolderExtensions` entries
//! are only honoured under `HKLM` (admin) — upstream's `register.cmd`
//! elevates for exactly that reason. A `LoadLibraryW` lands the same
//! `DllMain` in the same process with zero admin and covers
//! separate-process explorers wherever a resident can be injected.
//!
//! `config.ini` sits beside the deployed DLL — EBM reads it from its
//! own module directory (`[config]` effect/flags, `[light]`/`[dark]`
//! RGBA; it handles the system theme switch itself).

use std::path::{Path, PathBuf};
use std::sync::Mutex;
use std::sync::atomic::{AtomicBool, Ordering};
use windows_core::PCWSTR;

windows_core::link!("user32.dll" "system" fn GetClassNameW(hwnd: crate::bindings::HWND, lpclassname: windows_core::PWSTR, nmaxcount: i32) -> i32);
windows_core::link!("advapi32.dll" "system" fn RegOpenKeyExW(hkey: isize, subkey: PCWSTR, options: u32, sam: u32, result: *mut isize) -> i32);
windows_core::link!("advapi32.dll" "system" fn RegCloseKey(hkey: isize) -> i32);

/// `COPYDATASTRUCT.dwData` values — must match `crates/tap/src/command.rs`.
/// `EBML` = load `ExplorerBlurMica.dll` (payload: UTF-16 path + NUL);
/// `EBMU` = unload it. `FESR` = the legacy `fe_glass` reset, kept so a
/// still-armed resident from an older build disarms its detours before
/// EBM takes over (one broadcast per apply — not periodic).
const CDS_EBM_LOAD: usize = u32::from_be_bytes(*b"EBML") as usize;
const CDS_EBM_UNLOAD: usize = u32::from_be_bytes(*b"EBMU") as usize;
const CDS_FE_RESET: usize = u32::from_be_bytes(*b"FESR") as usize;

/// UI-facing EBM configuration — mirrors `config.ini`'s `[config]` flags
/// plus the `[light]`/`[dark]` RGBA strings (`#AARRGGBB`).
pub struct FeGlassConfig<'a> {
    /// `""` = off; `"blur"`, `"acrylic"`, `"mica"`, `"blurclear"`,
    /// `"micaalt"` — EBM `effect=` 0..4.
    pub effect: &'a str,
    pub clear_address: bool,
    pub clear_bar_bg: bool,
    pub clear_winui_bg: bool,
    pub show_line: bool,
    pub light_rgba: &'a str,
    pub dark_rgba: &'a str,
}

/// Owned snapshot of the last applied config — `ensure()` re-delivers it
/// to newly-appearing explorer pids; `epoch` bumps on every apply so a
/// config change reloads EBM in already-covered pids (it reads
/// `config.ini` once per `LoadLibrary`).
#[derive(Default)]
struct Fe {
    effect: String,
    clear_address: bool,
    clear_bar_bg: bool,
    clear_winui_bg: bool,
    show_line: bool,
    light_rgba: String,
    dark_rgba: String,
    /// Bumped every `apply_effect` — a pid whose delivery epoch trails
    /// gets `EBMU`+`EBML` (fresh config read) rather than a skip.
    epoch: u64,
    /// pid → (cmd window, epoch delivered). `ensure()` only wires pids
    /// that are new, stale-epoch, or whose resident restarted.
    delivered: std::collections::HashMap<u32, (usize, u64)>,
}

static FE: Mutex<Option<Fe>> = Mutex::new(None);
/// Whether a File Explorer effect is armed — read lock-free.
static ARMED: AtomicBool = AtomicBool::new(false);

/// `true` while a File Explorer effect is armed.
pub fn is_armed() -> bool {
    ARMED.load(Ordering::Acquire)
}

fn lock() -> std::sync::MutexGuard<'static, Option<Fe>> {
    FE.lock().unwrap_or_else(|p| p.into_inner())
}

fn widestr(s: &str) -> Vec<u16> {
    s.encode_utf16().chain(core::iter::once(0)).collect()
}

/// EBM `effect` name → `config.ini` integer.
fn effect_id(effect: &str) -> Option<u32> {
    match effect {
        "blur" => Some(0),
        "acrylic" => Some(1),
        "mica" => Some(2),
        "blurclear" => Some(3),
        "micaalt" => Some(4),
        _ => None,
    }
}

/// `%LOCALAPPDATA%\PecoFence\residents` — same home the tap resident
/// uses; stable for the resident-side `LoadLibraryW` to read.
fn residents_dir() -> Option<PathBuf> {
    let dir = PathBuf::from(std::env::var_os("LOCALAPPDATA")?)
        .join("PecoFence")
        .join("residents");
    std::fs::create_dir_all(&dir).ok()?;
    Some(dir)
}

/// `ExplorerBlurMica.dll` shipped beside the running exe (build.rs /
/// `make-windows.ps1` place it there).
fn ebm_source_path() -> Option<PathBuf> {
    let exe = std::env::current_exe().ok()?;
    let p = exe.with_file_name("ExplorerBlurMica.dll");
    p.is_file().then_some(p)
}

/// Copies the vendored DLL into the residents dir when missing or
/// different. A copy failure is only fatal when no deployed copy exists
/// at all — a loaded (locked) DLL is already a working EBM.
fn deploy_dll(dir: &Path) -> Option<PathBuf> {
    let dst = dir.join("ExplorerBlurMica.dll");
    if let Some(src) = ebm_source_path() {
        let same = match (std::fs::read(&src), std::fs::read(&dst)) {
            (Ok(a), Ok(b)) => a == b,
            _ => false,
        };
        if !same && let Err(e) = std::fs::copy(&src, &dst) {
            tracing::warn!(%e, "ExplorerBlurMica.dll deploy failed (locked?)");
        }
    }
    dst.is_file().then_some(dst)
}

/// `#AARRGGBB` (or `#RRGGBB`) → `(r, g, b, a)` for `config.ini`.
fn parse_rgba(s: &str) -> Option<(u8, u8, u8, u8)> {
    let hex = s.strip_prefix('#').unwrap_or(s);
    let v = match hex.len() {
        8 => u32::from_str_radix(hex, 16).ok()?,
        6 => u32::from_str_radix(hex, 16).ok()? | 0xFF00_0000,
        _ => return None,
    };
    Some((
        ((v >> 16) & 0xFF) as u8,
        ((v >> 8) & 0xFF) as u8,
        (v & 0xFF) as u8,
        (v >> 24) as u8,
    ))
}

/// Writes `config.ini` beside the deployed DLL — byte-format identical
/// to upstream's `[config]` + `[light]` + `[dark]` layout.
fn write_config_ini(dir: &Path, cfg: &Fe) -> std::io::Result<()> {
    let l = parse_rgba(&cfg.light_rgba).unwrap_or((220, 220, 220, 160));
    let d = parse_rgba(&cfg.dark_rgba).unwrap_or((0, 0, 0, 120));
    let b = |v: bool| if v { "true" } else { "false" };
    let ini = format!(
        "[config]\r\neffect={}\r\nclearAddress={}\r\nclearBarBg={}\r\nclearWinUIBg={}\r\nshowLine={}\r\n[light]\r\nr={}\r\ng={}\r\nb={}\r\na={}\r\n[dark]\r\nr={}\r\ng={}\r\nb={}\r\na={}\r\n",
        effect_id(&cfg.effect).unwrap_or(1),
        b(cfg.clear_address),
        b(cfg.clear_bar_bg),
        b(cfg.clear_winui_bg),
        b(cfg.show_line),
        l.0,
        l.1,
        l.2,
        l.3,
        d.0,
        d.1,
        d.2,
        d.3,
    );
    std::fs::write(dir.join("config.ini"), ini)
}

// ------------------------------------------------------------- cabinets ----

/// Every `CabinetWClass` that is a real File Explorer window — frames
/// with a `WorkerW` child are the legacy Control Panel shell, which has
/// no XAML islands and isn't targeted.
fn cabinet_windows() -> Vec<crate::bindings::HWND> {
    struct Ctx {
        out: Vec<crate::bindings::HWND>,
    }
    unsafe extern "system" fn cb(
        hwnd: crate::bindings::HWND,
        lparam: crate::bindings::LPARAM,
    ) -> windows_core::BOOL {
        let _ = std::panic::catch_unwind(core::panic::AssertUnwindSafe(|| {
            let ctx = unsafe { &mut *(lparam.0 as *mut Ctx) };
            let mut cls = [0u16; 64];
            let n = unsafe { GetClassNameW(hwnd, windows_core::PWSTR(cls.as_mut_ptr()), 64) };
            if n <= 0 || String::from_utf16_lossy(&cls[..n as usize]) != "CabinetWClass" {
                return;
            }
            let worker = widestr("WorkerW");
            let legacy = unsafe {
                crate::bindings::FindWindowExW(
                    Some(hwnd),
                    None,
                    PCWSTR(worker.as_ptr()),
                    PCWSTR::null(),
                )
            };
            if !legacy.0.is_null() {
                return;
            }
            ctx.out.push(hwnd);
        }));
        windows_core::BOOL(1)
    }
    let mut ctx = Ctx { out: Vec::new() };
    unsafe {
        let _ = crate::bindings::EnumWindows(
            Some(cb),
            crate::bindings::LPARAM(&mut ctx as *mut Ctx as isize),
        );
    }
    ctx.out
}

/// One anchor `CabinetWClass` per owning explorer pid — inject targets
/// for `ensure_resident_at` (separate-process explorers need their own
/// resident; the shell explorer's resident can't reach them).
fn cabinet_anchors() -> Vec<(u32, crate::bindings::HWND)> {
    let mut seen = std::collections::HashSet::new();
    let mut out = Vec::new();
    for hwnd in cabinet_windows() {
        let mut pid = 0u32;
        unsafe {
            let _ = crate::bindings::GetWindowThreadProcessId(hwnd, Some(&mut pid));
        }
        if pid != 0 && seen.insert(pid) {
            out.push((pid, hwnd));
        }
    }
    out
}

// ------------------------------------------------------------- delivery ----

/// Sends `EBMU`+`EBML` to one pid's resident. Unload-first makes a
/// config change re-read `config.ini` in already-loaded pids; it is a
/// no-op where EBM was never loaded.
fn deliver_ebm(cmd: crate::bindings::HWND, dll_path_utf16: &[u8]) {
    let _ = crate::tap::send_copydata(cmd, CDS_EBM_UNLOAD, &[]);
    let _ = crate::tap::send_copydata(cmd, CDS_EBM_LOAD, dll_path_utf16);
}

/// Fans `EBML` out to every cabinet-owning explorer pid — **only** to
/// pids that are new, whose resident restarted (cmd hwnd changed), or
/// whose epoch trails the current config. This is the anti-storm rule:
/// `ensure()` calls us every anchor tick but wires only fire on change.
fn deliver_all() {
    let Some(dll) = residents_dir().map(|d| d.join("ExplorerBlurMica.dll")) else {
        return;
    };
    if !dll.is_file() {
        return;
    }
    let wide: Vec<u8> = dll
        .display()
        .to_string()
        .encode_utf16()
        .chain(core::iter::once(0))
        .flat_map(|c| c.to_le_bytes())
        .collect();
    let anchors = cabinet_anchors();
    let mut live_pids = Vec::new();
    for (pid, anchor) in anchors {
        live_pids.push(pid);
        let Some(cmd) = crate::tap::ensure_resident_at(anchor) else {
            continue;
        };
        let mut guard = lock();
        let Some(fe) = guard.as_mut() else {
            return;
        };
        let current = (cmd.0 as usize, fe.epoch);
        if fe.delivered.get(&pid) == Some(&current) {
            continue; // already loaded under this config — silent tick
        }
        deliver_ebm(cmd, &wide);
        fe.delivered.insert(pid, current);
    }
    crate::tap::prune_extra_residents(&live_pids);
}

/// One-shot disarm for legacy residents: an older app build may have
/// left `fe_glass` armed inside explorer — its DWM/uxtheme detours would
/// fight the EBM extension over the same surfaces. `FESR` restores
/// originals; no-op when nothing is armed.
fn disarm_legacy() {
    for hwnd in crate::tap::resident_windows_all() {
        let _ = crate::tap::send_copydata(hwnd, CDS_FE_RESET, &[]);
    }
}

// ------------------------------------------------------------- public ----

/// Applies an EBM `effect` + flags + RGBA palettes. `effect=""` →
/// `reset()` (Windows Default). Returns `false` when the deploy or the
/// `config.ini` write failed so settings can revert.
pub fn apply_effect(cfg: &FeGlassConfig) -> bool {
    if cfg.effect.is_empty() {
        return reset();
    }
    if effect_id(cfg.effect).is_none() {
        return false;
    }
    {
        let mut guard = lock();
        let fe = guard.get_or_insert_with(Fe::default);
        fe.effect = cfg.effect.to_string();
        fe.clear_address = cfg.clear_address;
        fe.clear_bar_bg = cfg.clear_bar_bg;
        fe.clear_winui_bg = cfg.clear_winui_bg;
        fe.show_line = cfg.show_line;
        fe.light_rgba = cfg.light_rgba.to_string();
        fe.dark_rgba = cfg.dark_rgba.to_string();
        fe.epoch += 1; // every live pid reloads EBM under the new config
        let dir = match residents_dir() {
            Some(d) => d,
            None => return false,
        };
        if deploy_dll(&dir).is_none() || write_config_ini(&dir, fe).is_err() {
            tracing::warn!("ExplorerBlurMica apply failed (deploy/config)");
            return false;
        }
    }
    ARMED.store(true, Ordering::Release);
    // Legacy resident disarm BEFORE EBM lands — belt for the upgrade
    // path; harmless when nothing is armed.
    disarm_legacy();
    deliver_all();
    tracing::info!(
        effect = cfg.effect,
        "ExplorerBlurMica armed via resident relay"
    );
    true
}

/// `Windows Default` — `EBMU` broadcast unloads EBM everywhere; also
/// disarms any legacy resident glass.
pub fn reset() -> bool {
    for hwnd in crate::tap::resident_windows_all() {
        let _ = crate::tap::send_copydata(hwnd, CDS_EBM_UNLOAD, &[]);
    }
    disarm_legacy();
    if let Some(fe) = lock().as_mut() {
        fe.effect.clear();
        fe.delivered.clear();
    }
    ARMED.store(false, Ordering::Release);
    true
}

/// Anchor-tick liveness: re-injects + re-delivers `EBML` to explorer
/// pids that appeared since the last pass (new cabinet process), whose
/// resident restarted, or whose config epoch trails. No wires fire when
/// nothing changed — no periodic Unadvise/Advise or LoadLibrary churn.
pub fn ensure() {
    if !is_armed() {
        return;
    }
    deliver_all();
}

/// App shutdown — unload EBM everywhere so Explorer returns to stock;
/// the deployed files stay for the next run.
pub fn shutdown() {
    let _ = reset();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn effect_names_match_config_ini() {
        assert_eq!(effect_id("blur"), Some(0));
        assert_eq!(effect_id("acrylic"), Some(1));
        assert_eq!(effect_id("mica"), Some(2));
        assert_eq!(effect_id("blurclear"), Some(3));
        assert_eq!(effect_id("micaalt"), Some(4));
        assert_eq!(effect_id(""), None);
        assert_eq!(effect_id("bogus"), None);
    }

    #[test]
    fn rgba_parses_aarrggbb() {
        assert_eq!(parse_rgba("#A0DCDCDC"), Some((0xDC, 0xDC, 0xDC, 0xA0)));
        assert_eq!(parse_rgba("#78000000"), Some((0, 0, 0, 0x78)));
        assert_eq!(parse_rgba("FF0000"), Some((0xFF, 0, 0, 0xFF)));
        assert_eq!(parse_rgba("#xyz"), None);
        assert_eq!(parse_rgba(""), None);
    }

    #[test]
    fn config_ini_matches_upstream_layout() {
        let dir = std::env::temp_dir().join("pecofence_ebm_cfg_test");
        std::fs::create_dir_all(&dir).unwrap();
        let cfg = Fe {
            effect: "acrylic".into(),
            clear_address: true,
            clear_bar_bg: false,
            clear_winui_bg: true,
            show_line: true,
            light_rgba: "#A0DCDCDC".into(),
            dark_rgba: "#78000000".into(),
            ..Default::default()
        };
        write_config_ini(&dir, &cfg).unwrap();
        let ini = std::fs::read_to_string(dir.join("config.ini")).unwrap();
        assert!(ini.contains("[config]\r\neffect=1"));
        assert!(ini.contains("clearAddress=true"));
        assert!(ini.contains("clearBarBg=false"));
        assert!(ini.contains("clearWinUIBg=true"));
        assert!(ini.contains("showLine=true"));
        assert!(ini.contains("[light]\r\nr=220\r\ng=220\r\nb=220\r\na=160"));
        assert!(ini.contains("[dark]\r\nr=0\r\ng=0\r\nb=0\r\na=120"));
        let _ = std::fs::remove_dir_all(&dir);
    }
}
