//! Host-side controller for the File Explorer material — drives the real
//! [ExplorerBlurMica](https://github.com/Maplespe/ExplorerBlurMica)
//! extension binary (vendored `third_party/ebm/ExplorerBlurMica.dll`,
//! LGPLv3). Upstream's `*.cpp` sources are private, so the release binary
//! is the only bit-exact engine; this module loads THE DLL ITSELF rather
//! than re-implementing its pipeline.
//!
//! Delivery — `EBML`/`EBMU` COPYDATA through the `pecofence_tap`
//! resident already injected in each explorer process (the retired
//! `PECO-FES1` layout/XAML wire is gone; `FELR` remains as a
//! reset-only broadcast for residents still carrying it):
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
/// `FELR` = reset the retired `PECO-FES1` mux wire — broadcast-only, so a
/// resident carrying a layout/clear from an older build restores stock
/// chrome. The wire itself (`FELT`) is gone: layout presets were removed.
const CDS_EBM_LOAD: usize = u32::from_be_bytes(*b"EBML") as usize;
const CDS_EBM_UNLOAD: usize = u32::from_be_bytes(*b"EBMU") as usize;
const CDS_FE_RESET: usize = u32::from_be_bytes(*b"FESR") as usize;
const CDS_FE_LAYOUT_RESET: usize = u32::from_be_bytes(*b"FELR") as usize;

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
    /// Ini-generation counter — bumps ONLY when a `config.ini`-relevant
    /// field changes (effect/flags/rgba). A trailing pid gets `EBMU`+`EBML`
    /// (fresh config read); layout-only and identical applies bump nothing,
    /// so routine delivery never unloads the live module.
    epoch: u64,
    /// pid → (cmd window, epoch delivered). `ensure()` only wires pids
    /// that are new, stale-epoch, or whose resident restarted.
    delivered: std::collections::HashMap<u32, (usize, u64)>,
    /// The deployed `ExplorerBlurMica.dll` path `EBML` hands to residents
    /// (hash-versioned — an engine upgrade reloads through a fresh path).
    dll_path: String,
    /// Last `deliver_all` fan-out signature — the debug line logs only when
    /// pids/readiness/epochs change, not every 2 s `ensure()` tick.
    delivery_sig: String,
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

/// The vendored engine compiled into the binary — the canonical source for
/// deployment: a next-to-exe file is not part of the contract (installers,
/// `--portable` copies and dev layouts stage it differently). The 658 KB
/// `include_bytes!` is cheap insurance: on a machine where the sidecar file
/// was never staged, the material still deploys.
static EBM_DLL: &[u8] = include_bytes!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../third_party/ebm/ExplorerBlurMica.dll"
));

/// Source bytes to deploy. `PECOFENCE_EBM_DLL=<path>` overrides for engine
/// development only — anything loaded into explorer.exe must come from an
/// explicit opt-in or the bytes this build was tested with.
fn ebm_source_bytes() -> Option<Vec<u8>> {
    match std::env::var("PECOFENCE_EBM_DLL") {
        Ok(p) => match std::fs::read(&p) {
            Ok(b) => Some(b),
            Err(e) => {
                // A set-but-unreadable override must not silently fall back —
                // the developer asked for specific bytes.
                tracing::warn!(path = %p, error = %e, "PECOFENCE_EBM_DLL unreadable; using vendored engine");
                Some(EBM_DLL.to_vec())
            }
        },
        Err(_) => Some(EBM_DLL.to_vec()),
    }
}

/// FNV-1a over the engine bytes — labels the deploy dir; every content change
/// gets a new path (never an overwrite of a DLL a live explorer has mapped).
fn fnv1a64(bytes: &[u8]) -> u64 {
    let mut h = 0xcbf2_9ce4_8422_2325u64;
    for &b in bytes {
        h = (h ^ u64::from(b)).wrapping_mul(0x0000_0100_0000_01b3);
    }
    h
}

/// `residents/ebm-<hash>` — the deployed engine's versioned home. A new build
/// lands in a NEW path: a running explorer's mapped DLL is never under the
/// write target, and `EBMU`+`EBML` swaps engines live in the same pid.
fn ebm_deploy_dir(bytes: &[u8]) -> Option<PathBuf> {
    Some(residents_dir()?.join(format!("ebm-{:016x}", fnv1a64(bytes))))
}

/// Deploys the vendored DLL into its hash-versioned dir. `Some(dst)` is only
/// returned when the file on disk is byte-identical to the source — a
/// byte-equal destination is accepted without rewriting; a mismatch is
/// rewritten (new path ⇒ no live-share lock to fight), and any write/verify
/// failure is a hard error: LoadLibrary-ing stale bytes would arm an
/// unknown engine.
fn deploy_dll() -> Option<PathBuf> {
    let src = ebm_source_bytes()?;
    let dir = ebm_deploy_dir(&src)?;
    let dst = dir.join("ExplorerBlurMica.dll");
    match std::fs::read(&dst) {
        Ok(d) if d == src => {
            tracing::debug!(
                dst = %dst.display(),
                "deploy_dll: destination already matches the vendored DLL — copy skipped"
            );
            Some(dst)
        }
        _ => {
            if std::fs::create_dir_all(&dir).is_err() || std::fs::write(&dst, &src).is_err() {
                tracing::warn!(
                    dst = %dst.display(),
                    "ExplorerBlurMica.dll deploy failed — destination cannot be written"
                );
                return None;
            }
            match std::fs::read(&dst) {
                Ok(d) if d == src => {
                    tracing::debug!(
                        dst = %dst.display(),
                        bytes = src.len(),
                        "deploy_dll: destination written"
                    );
                    Some(dst)
                }
                _ => {
                    tracing::warn!(
                        dst = %dst.display(),
                        "ExplorerBlurMica.dll deploy failed — written bytes failed verification"
                    );
                    None
                }
            }
        }
    }
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

/// Builds the `config.ini` contents for `cfg` — byte-format identical
/// to upstream's `[config]` + `[light]` + `[dark]` layout.
fn config_ini_contents(cfg: &FeGlassConfig) -> String {
    let l = parse_rgba(cfg.light_rgba).unwrap_or((220, 220, 220, 160));
    let d = parse_rgba(cfg.dark_rgba).unwrap_or((0, 0, 0, 120));
    let b = |v: bool| if v { "true" } else { "false" };
    format!(
        "[config]\r\neffect={}\r\nclearAddress={}\r\nclearBarBg={}\r\nclearWinUIBg={}\r\nshowLine={}\r\n[light]\r\nr={}\r\ng={}\r\nb={}\r\na={}\r\n[dark]\r\nr={}\r\ng={}\r\nb={}\r\na={}\r\n",
        effect_id(cfg.effect).unwrap_or(1),
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
    )
}

/// Writes `config.ini` beside the deployed DLL. Identical existing bytes
/// are skipped (EBM may hold the file open read-only — a skipped write
/// can never hit a sharing violation); a real write failure surfaces the
/// io error to the caller with the path attached.
fn write_config_ini(dir: &Path, cfg: &FeGlassConfig) -> std::io::Result<()> {
    let path = dir.join("config.ini");
    let ini = config_ini_contents(cfg);
    match std::fs::read(&path) {
        Ok(existing) if existing == ini.as_bytes() => {
            tracing::debug!(
                ini = %path.display(),
                "config.ini already matches the requested config — write skipped"
            );
            Ok(())
        }
        Ok(_) | Err(_) => {
            let bytes = ini.len();
            std::fs::write(&path, ini).inspect(|_| {
                tracing::debug!(ini = %path.display(), bytes, "config.ini written");
            })
        }
    }
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
/// whose ini-generation trails. This is the anti-storm rule: `ensure()`
/// calls us every anchor tick but wires only fire on change.
fn deliver_all() {
    let wide: Vec<u8> = lock()
        .as_ref()
        .map(|fe| fe.dll_path.clone())
        .filter(|p| !p.is_empty() && Path::new(p).is_file())
        .map(|dll| {
            dll.encode_utf16()
                .chain(core::iter::once(0))
                .flat_map(|c| c.to_le_bytes())
                .collect()
        })
        .unwrap_or_default();
    let anchors = cabinet_anchors();
    // State-change signature: same pids + same readiness + same epoch means
    // this tick delivers nothing new — the debug line repeats otherwise.
    {
        let mut pids: Vec<u32> = anchors.iter().map(|(pid, _)| *pid).collect();
        pids.sort_unstable();
        let sig = {
            let mut guard = lock();
            let Some(fe) = guard.as_mut() else {
                return;
            };
            format!(
                "{}|{}|{}|{:?}",
                fe.epoch,
                !wide.is_empty(),
                anchors.len(),
                pids
            )
        };
        let mut guard = lock();
        if let Some(fe) = guard.as_mut()
            && fe.delivery_sig != sig
        {
            fe.delivery_sig = sig;
            drop(guard);
            tracing::debug!(
                anchors = anchors.len(),
                dll_ready = !wide.is_empty(),
                "deliver_all: fanning out to cabinet-owning explorer pids"
            );
        }
    }
    let mut live_pids = Vec::new();
    for (pid, anchor) in anchors {
        live_pids.push(pid);
        let Some(cmd) = crate::tap::ensure_resident_at(anchor) else {
            tracing::debug!(pid, "deliver_all: no resident for cabinet pid — skipped");
            continue;
        };
        let mut guard = lock();
        let Some(fe) = guard.as_mut() else {
            return;
        };
        if !fe.effect.is_empty() && !wide.is_empty() {
            let current = (cmd.0 as usize, fe.epoch);
            if fe.delivered.get(&pid) != Some(&current) {
                deliver_ebm(cmd, &wide);
                tracing::debug!(pid, epoch = fe.epoch, "deliver_all: EBML sent");
                fe.delivered.insert(pid, current);
            }
        }
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

/// One-shot reset for the retired `PECO-FES1` mux wire. Older builds (and
/// an already-injected resident carrying a layout/clear wire) keep their
/// mux styling until told otherwise — `FELR` restores stock chrome. Sent
/// unconditionally on every apply: a pid that never got the wire ignores it.
fn disarm_xaml() {
    let residents = crate::tap::resident_windows_all();
    if !residents.is_empty() {
        tracing::debug!(
            residents = residents.len(),
            "FELR broadcast: retired mux wire reset"
        );
    }
    for hwnd in residents {
        let _ = crate::tap::send_copydata(hwnd, CDS_FE_LAYOUT_RESET, &[]);
    }
}

// ------------------------------------------------------------- public ----

/// Applies an EBM `effect` + flags + RGBA palettes.
///
/// EBM lifecycle rule: `config.ini` is read once per `LoadLibrary`, so a
/// real config change reaches an already-loaded pid only through
/// `EBMU`+`EBML` — a mid-session `FreeLibrary` whose detach races live
/// detours. That unload is therefore confined to the pids whose ini-
/// generation trails: `epoch` bumps only when effect/flags/rgba actually
/// change. An identical apply delivers nothing and never disturbs the
/// material.
/// Returns `false` when the deploy or the `config.ini` write failed so
/// settings can revert.
pub fn apply_effect(cfg: &FeGlassConfig) -> bool {
    tracing::debug!(
        effect = cfg.effect,
        clear_address = cfg.clear_address,
        clear_bar_bg = cfg.clear_bar_bg,
        clear_winui_bg = cfg.clear_winui_bg,
        show_line = cfg.show_line,
        "apply_effect"
    );
    // Every path ends here: legacy fe_glass disarm (old builds' detours
    // would fight EBM), the retired mux-wire reset (FELR — restores stock
    // chrome in residents still carrying a layout/clear wire), then
    // delivery to every covered pid.
    fn finish() {
        disarm_legacy();
        disarm_xaml();
        deliver_all();
    }
    if cfg.effect.is_empty() {
        return reset_ebm();
    }
    if effect_id(cfg.effect).is_none() {
        tracing::warn!(
            effect = cfg.effect,
            "ExplorerBlurMica apply failed — unknown effect id"
        );
        finish();
        return false;
    }
    // What the ini would actually change — `epoch` bumps only then, so an
    // identical apply never triggers an unload+reload.
    let ini_changed = {
        let guard = lock();
        match guard.as_ref() {
            Some(fe) => {
                fe.effect != cfg.effect
                    || fe.clear_address != cfg.clear_address
                    || fe.clear_bar_bg != cfg.clear_bar_bg
                    || fe.clear_winui_bg != cfg.clear_winui_bg
                    || fe.show_line != cfg.show_line
                    || fe.light_rgba != cfg.light_rgba
                    || fe.dark_rgba != cfg.dark_rgba
            }
            None => true,
        }
    };
    if !ini_changed && ARMED.load(Ordering::Acquire) {
        tracing::debug!("apply_effect: EBM config unchanged — material leg skipped");
        finish();
        return true;
    }
    let Some(dir) = residents_dir() else {
        tracing::warn!("ExplorerBlurMica apply failed — %LOCALAPPDATA% residents dir unavailable");
        finish();
        return false;
    };
    // Deploy + config.ini are committed BEFORE touching shared state: on
    // failure `fe.effect` still holds the last applied value and the ini
    // on disk still matches it — no rollback needed. `config.ini` sits in
    // the deployed DLL's own versioned dir — EBM reads its module dir.
    let Some(dll_path) = deploy_dll() else {
        finish();
        return false;
    };
    let ini_dir = dll_path.parent().unwrap_or(&dir).to_path_buf();
    if let Err(e) = write_config_ini(&ini_dir, cfg) {
        tracing::warn!(
            ini = %ini_dir.join("config.ini").display(),
            %e,
            "ExplorerBlurMica apply failed — config.ini write"
        );
        finish();
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
        fe.dll_path = dll_path.display().to_string();
        fe.epoch += 1; // pids whose generation trails reload EBM once
    }
    ARMED.store(true, Ordering::Release);
    finish();
    tracing::info!(
        effect = cfg.effect,
        "ExplorerBlurMica armed via resident relay"
    );
    true
}

/// `EBMU` broadcast unloads the material everywhere and clears its
/// delivery bookkeeping; the retired mux wire gets its reset too so stock
/// chrome comes back with the stock material.
fn reset_ebm() -> bool {
    let residents = crate::tap::resident_windows_all();
    tracing::debug!(residents = residents.len(), "reset_ebm: EBMU broadcast");
    for hwnd in residents {
        let _ = crate::tap::send_copydata(hwnd, CDS_EBM_UNLOAD, &[]);
    }
    disarm_legacy();
    disarm_xaml();
    if let Some(fe) = lock().as_mut() {
        fe.effect.clear();
        fe.delivered.clear();
    }
    ARMED.store(false, Ordering::Release);
    true
}

/// `Windows Default` — full disarm: EBM unload + mux/legacy resets.
/// Used by shutdown; the settings Apply path goes through `apply_effect`.
pub fn reset() -> bool {
    reset_ebm()
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
        let cfg = FeGlassConfig {
            effect: "acrylic",
            clear_address: true,
            clear_bar_bg: false,
            clear_winui_bg: true,
            show_line: true,
            light_rgba: "#A0DCDCDC",
            dark_rgba: "#78000000",
        };
        write_config_ini(&dir, &cfg).unwrap();
        // Identical contents are skipped on a second apply — an EBM-held
        // read handle must never turn into a sharing violation.
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
