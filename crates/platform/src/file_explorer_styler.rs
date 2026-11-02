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
/// `FELT` = a `PECO-FES1` layout wire (UTF-8, mux engine); `FELR` resets it.
const CDS_EBM_LOAD: usize = u32::from_be_bytes(*b"EBML") as usize;
const CDS_EBM_UNLOAD: usize = u32::from_be_bytes(*b"EBMU") as usize;
const CDS_FE_RESET: usize = u32::from_be_bytes(*b"FESR") as usize;
const CDS_FE_LAYOUT: usize = u32::from_be_bytes(*b"FELT") as usize;
const CDS_FE_LAYOUT_RESET: usize = u32::from_be_bytes(*b"FELR") as usize;

/// The `clearWinUIBg` XAML rule set — `S Background=Transparent` on the
/// WinUI 3 island's own containers (tab strip, command bar, gallery,
/// details grid, breadcrumb bar, overflow presenter, flyouts). EBM can
/// only place material *behind* the islands; these nested XAML elements
/// still paint their own opaque `Background` brush on top, which DWM
/// cannot reach — only the mux styler can clear it. Only emitted while an
/// EBM effect is armed: without material behind them a transparent
/// background would read as a hole instead of glass.
const CLEAR_WINUI_BG_RULES: &[&str] = &[
    "FileExplorerExtensions.FileExplorerTabControl",
    "FileExplorerExtensions.FileExplorerTabControl > Grid",
    "Microsoft.UI.Xaml.Controls.TabView",
    "Microsoft.UI.Xaml.Controls.Grid#TabContainerGrid",
    "FileExplorerExtensions.CommandBarControl_Wave1",
    "FileExplorerExtensions.CommandBarControl_Wave1 > Grid, Grid#CommandBarControlRootGrid",
    "Microsoft.UI.Xaml.Controls.CommandBar#FileExplorerCommandBar",
    "Microsoft.UI.Xaml.Controls.CommandBar#FileExplorerCommandBar > Grid#LayoutRoot > Grid#ContentRoot",
    "Microsoft.UI.Xaml.Controls.Grid#PART_LayoutRoot",
    "Microsoft.UI.Xaml.Controls.Grid#ContentRoot",
    "Microsoft.UI.Xaml.Controls.Grid#RootContainer",
    "Microsoft.UI.Xaml.Controls.Grid#DetailsViewControlRootGrid",
    "Microsoft.UI.Xaml.Controls.Grid#HomeViewRootGrid",
    "FileExplorerExtensions.GalleryViewControl#GalleryViewControl > Grid",
    "Microsoft.UI.Xaml.Controls.Grid#GalleryRootGrid",
    "Microsoft.UI.Xaml.Controls.StackPanel#DetailsViewThumbnail",
    "Microsoft.UI.Xaml.Controls.StackPanel#DetailsViewThumbnail > Microsoft.UI.Xaml.Controls.Grid",
    "Microsoft.UI.Xaml.Controls.TabViewItem > Microsoft.UI.Xaml.Controls.Grid#LayoutRoot > Microsoft.UI.Xaml.Controls.Grid#TabContainer",
    "Microsoft.UI.Xaml.Controls.CommandBarOverflowPresenter#SecondaryItemsControl > Microsoft.UI.Xaml.Controls.Grid#LayoutRoot",
    "Microsoft.UI.Xaml.Controls.Grid#OuterOverflowContentRootV2",
    "Microsoft.UI.Xaml.Controls.MenuFlyoutPresenter > Microsoft.UI.Xaml.Controls.Border",
    "Microsoft.UI.Xaml.Controls.Primitives.NavigationViewItemPresenter#NavigationViewItemPresenter > Microsoft.UI.Xaml.Controls.Grid#LayoutRoot",
    "FileExplorerExtensions.FirstCrumbStackPanelControl#FirstCrumbStackPanel",
];

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
    /// Toolbar/icon arrangement — `""` = Windows Default. Pure layout
    /// rules only (`fe_layouts`); independent of the material above.
    pub layout: &'a str,
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
    /// Selected layout preset id (`""` = none armed).
    layout: String,
    /// Bumped every apply — the XAML wire re-sends only when this trails.
    xaml_epoch: u64,
    /// pid → (cmd window, XAML epoch delivered).
    xaml_delivered: std::collections::HashMap<u32, (usize, u64)>,
}

static FE: Mutex<Option<Fe>> = Mutex::new(None);
/// Whether a File Explorer effect is armed — read lock-free.
static ARMED: AtomicBool = AtomicBool::new(false);
/// Whether the XAML leg (layout preset and/or `clearWinUIBg` clears) is
/// armed — drives `ensure()` even when the material is stock (`effect=""`).
static XAML_ARMED: AtomicBool = AtomicBool::new(false);

/// `true` while a File Explorer effect or the XAML leg is armed.
pub fn is_armed() -> bool {
    ARMED.load(Ordering::Acquire) || XAML_ARMED.load(Ordering::Acquire)
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

/// Deploys the vendored DLL into the residents dir. `Some(dst)` is only
/// returned when the file on disk is byte-identical to the vendored copy —
/// a locked (loaded-in-explorer) but CORRECT DLL is accepted, a locked
/// STALE one is a hard failure: LoadLibrary-ing stale bytes would arm an
/// unknown engine. When the vendored source is missing entirely the
/// previously-deployed copy is used with a warning (it was written by a
/// prior run of this same installer), and `None` only when nothing exists.
fn deploy_dll(dir: &Path) -> Option<PathBuf> {
    let dst = dir.join("ExplorerBlurMica.dll");
    let dst_bytes = std::fs::read(&dst).ok();
    let src = ebm_source_path().and_then(|src| std::fs::read(&src).ok());
    match src {
        Some(src_bytes) => match dst_bytes {
            Some(d) if d == src_bytes => {
                tracing::debug!(
                    dst = %dst.display(),
                    "deploy_dll: destination already matches the vendored DLL — copy skipped"
                );
                Some(dst)
            }
            _ => {
                if let Err(e) = std::fs::write(&dst, &src_bytes) {
                    tracing::warn!(
                        dst = %dst.display(),
                        %e,
                        "ExplorerBlurMica.dll deploy failed — destination differs and cannot be rewritten (locked by a stale load?)"
                    );
                    return None;
                }
                tracing::debug!(
                    dst = %dst.display(),
                    bytes = src_bytes.len(),
                    "deploy_dll: destination written"
                );
                Some(dst)
            }
        },
        None => {
            if dst_bytes.is_some() {
                tracing::warn!(
                    "ExplorerBlurMica.dll missing beside pecofence.exe — using previously deployed copy"
                );
                Some(dst)
            } else {
                tracing::warn!(
                    dst = %dst.display(),
                    "ExplorerBlurMica.dll unavailable — not beside pecofence.exe and never deployed"
                );
                None
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

/// The `PECO-FES1` XAML wire: the layout preset's pure positioning rules,
/// then — when `clearWinUIBg` is on AND an EBM effect is armed —
/// `S Background=Transparent` clears for the nested WinUI containers the
/// layouts expose (`CLEAR_WINUI_BG_RULES`). `None` when neither wants a
/// wire. Clear rules are appended before the wire's terminal `E` marker —
/// the resident parser stops reading there.
fn build_xaml_wire(fe: &Fe) -> Option<String> {
    let mut out = if fe.layout.is_empty() {
        String::new()
    } else {
        pecofence_core::fe_layouts::layout_wire(&fe.layout)?
    };
    if !fe.clear_winui_bg || fe.effect.is_empty() {
        return (!out.is_empty()).then_some(out);
    }
    if out.is_empty() {
        out.push_str("PECO-FES1\n");
    } else if let Some(pos) = out.rfind("\nE\n") {
        out.truncate(pos + 1);
    }
    for target in CLEAR_WINUI_BG_RULES {
        out.push_str("T ");
        out.push_str(target);
        out.push_str("\nS Background=Transparent\n");
    }
    out.push_str("E\n");
    Some(out)
}

/// Fans `EBML` and the `PECO-FES1` XAML wire out to every
/// cabinet-owning explorer pid — **only** to pids that are new, whose
/// resident restarted (cmd hwnd changed), or whose epoch trails the
/// current config. This is the anti-storm rule: `ensure()` calls us
/// every anchor tick but wires only fire on change. The two legs are
/// independent: XAML delivery never touches the EBM pipeline and runs
/// even when `effect=""`.
fn deliver_all() {
    let wide: Vec<u8> = residents_dir()
        .map(|d| d.join("ExplorerBlurMica.dll"))
        .filter(|p| p.is_file())
        .map(|dll| {
            dll.display()
                .to_string()
                .encode_utf16()
                .chain(core::iter::once(0))
                .flat_map(|c| c.to_le_bytes())
                .collect()
        })
        .unwrap_or_default();
    let anchors = cabinet_anchors();
    tracing::debug!(
        anchors = anchors.len(),
        dll_ready = !wide.is_empty(),
        "deliver_all: fanning out to cabinet-owning explorer pids"
    );
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
        // EBM leg — only while an effect is armed.
        if !fe.effect.is_empty() && !wide.is_empty() {
            let current = (cmd.0 as usize, fe.epoch);
            if fe.delivered.get(&pid) != Some(&current) {
                deliver_ebm(cmd, &wide);
                tracing::debug!(pid, epoch = fe.epoch, "deliver_all: EBML sent");
                fe.delivered.insert(pid, current);
            }
        }
        // XAML leg — a `PECO-FES1` wire through the same resident; the
        // mux engine applies it to the WinUI3 islands.
        if let Some(wire) = build_xaml_wire(fe) {
            let current = (cmd.0 as usize, fe.xaml_epoch);
            if fe.xaml_delivered.get(&pid) != Some(&current) {
                let _ = crate::tap::send_copydata(cmd, CDS_FE_LAYOUT, wire.as_bytes());
                tracing::debug!(pid, bytes = wire.len(), "deliver_all: FELT wire sent");
                fe.xaml_delivered.insert(pid, current);
            }
        }
    }
    crate::tap::prune_extra_residents(&live_pids);
}

/// Stores the layout preset id — validation only. Arming/delivery runs in
/// `sync_xaml_leg` + `deliver_all`, which the caller runs once every field
/// the wire depends on (`layout`, `effect`, `clear_winui_bg`) is settled;
/// wiring mid-apply would send a stale `clearWinUIBg` gate.
fn apply_layout(layout: &str) {
    let valid = layout.is_empty() || pecofence_core::fe_layouts::layout_wire(layout).is_some();
    if !valid {
        tracing::warn!(layout, "unknown feLayout preset — keeping stock");
    }
    let layout = if valid { layout } else { "" };
    let mut guard = lock();
    let fe = guard.get_or_insert_with(Fe::default);
    fe.layout = layout.to_string();
    fe.xaml_epoch += 1;
    tracing::debug!(layout, "apply_layout: stored");
}

/// Recomputes the XAML leg after `layout`/`effect`/`clear_winui_bg`
/// change. Armed = a layout preset is selected, or `clearWinUIBg` is on
/// while an effect is armed. Disarmed fires one `FELR` broadcast so every
/// resident restores the elements it touched, and clears delivery
/// bookkeeping so a later re-arm re-wires.
fn sync_xaml_leg() {
    let armed = lock()
        .as_ref()
        .is_some_and(|fe| !fe.layout.is_empty() || (fe.clear_winui_bg && !fe.effect.is_empty()));
    XAML_ARMED.store(armed, Ordering::Release);
    if !armed {
        let residents = crate::tap::resident_windows_all();
        tracing::debug!(
            residents = residents.len(),
            "sync_xaml_leg: disarmed, FELR broadcast"
        );
        for hwnd in residents {
            let _ = crate::tap::send_copydata(hwnd, CDS_FE_LAYOUT_RESET, &[]);
        }
        if let Some(fe) = lock().as_mut() {
            fe.xaml_delivered.clear();
        }
    } else {
        tracing::debug!("sync_xaml_leg: armed");
    }
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

/// Applies an EBM `effect` + flags + RGBA palettes + the toolbar/icon
/// `layout`. The XAML leg runs unconditionally (`effect=""` still gets the
/// layout wire — minus the clear rules, which need material behind them);
/// `effect=""` additionally unloads EBM everywhere.
/// Returns `false` when the deploy or the `config.ini` write failed so
/// settings can revert.
pub fn apply_effect(cfg: &FeGlassConfig) -> bool {
    tracing::debug!(
        effect = cfg.effect,
        layout = cfg.layout,
        clear_address = cfg.clear_address,
        clear_bar_bg = cfg.clear_bar_bg,
        clear_winui_bg = cfg.clear_winui_bg,
        show_line = cfg.show_line,
        "apply_effect"
    );
    // The layout leg is committed first and is INDEPENDENT of the EBM
    // material below — a deploy/config failure must not take it down.
    apply_layout(cfg.layout);
    // Runs the XAML leg sync + delivery — used by every path, including
    // effect-leg failures, so a valid layout change always lands.
    fn finish_xaml() {
        // Legacy resident disarm BEFORE delivery — an older armed
        // `fe_glass` would otherwise fight the wires.
        disarm_legacy();
        sync_xaml_leg();
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
        finish_xaml();
        return false;
    }
    let Some(dir) = residents_dir() else {
        tracing::warn!("ExplorerBlurMica apply failed — %LOCALAPPDATA% residents dir unavailable");
        finish_xaml();
        return false;
    };
    // Deploy + config.ini are committed BEFORE touching shared state: on
    // failure `fe.effect` still holds the last applied value and the ini
    // on disk still matches it — no rollback needed.
    let deploy_ok = deploy_dll(&dir).is_some();
    let ini_ok = deploy_ok && {
        match write_config_ini(&dir, cfg) {
            Ok(()) => true,
            Err(e) => {
                tracing::warn!(
                    ini = %dir.join("config.ini").display(),
                    %e,
                    "ExplorerBlurMica apply failed — config.ini write"
                );
                false
            }
        }
    };
    if !ini_ok {
        finish_xaml();
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
        fe.xaml_epoch += 1; // `clearWinUIBg`/`effect` gate the wire contents
    }
    ARMED.store(true, Ordering::Release);
    finish_xaml();
    tracing::info!(
        effect = cfg.effect,
        "ExplorerBlurMica armed via resident relay"
    );
    true
}

/// EBM leg only — `EBMU` broadcast unloads the material everywhere and
/// clears its delivery bookkeeping. The XAML leg then re-evaluates: a
/// still-selected layout re-wires WITHOUT the clear rules (no material
/// behind them anymore); with no layout it fully disarms via `FELR`.
fn reset_ebm() -> bool {
    let residents = crate::tap::resident_windows_all();
    tracing::debug!(residents = residents.len(), "reset_ebm: EBMU broadcast");
    for hwnd in residents {
        let _ = crate::tap::send_copydata(hwnd, CDS_EBM_UNLOAD, &[]);
    }
    disarm_legacy();
    if let Some(fe) = lock().as_mut() {
        fe.effect.clear();
        fe.delivered.clear();
        fe.xaml_epoch += 1; // clear rules drop out of the wire
    }
    ARMED.store(false, Ordering::Release);
    sync_xaml_leg();
    if XAML_ARMED.load(Ordering::Acquire) {
        deliver_all();
    }
    true
}

/// `Windows Default` — full disarm: EBM unload + layout reset + legacy
/// glass disarm. Used by shutdown; the settings Apply path goes through
/// `apply_effect` (which keeps the armed layout).
pub fn reset() -> bool {
    apply_layout("");
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
    fn xaml_wire_appends_clear_rules_only_with_effect() {
        // No layout + no clear → no wire at all.
        let fe = Fe::default();
        assert!(build_xaml_wire(&fe).is_none());

        // Clear flag without an armed effect must NOT emit clears — a
        // transparent container with no material behind it reads as a hole.
        let fe = Fe {
            clear_winui_bg: true,
            ..Default::default()
        };
        assert!(build_xaml_wire(&fe).is_none());

        // Effect + clear → every clear target gets `S Background=Transparent`.
        let fe = Fe {
            effect: "blur".into(),
            clear_winui_bg: true,
            ..Default::default()
        };
        let wire = build_xaml_wire(&fe).unwrap();
        assert!(wire.starts_with("PECO-FES1\n"));
        assert!(wire.ends_with("E\n"), "terminal marker must stay last");
        for target in CLEAR_WINUI_BG_RULES {
            assert!(wire.contains(&format!("T {target}\n")), "missing {target}");
        }
        assert_eq!(
            wire.matches("S Background=Transparent\n").count(),
            CLEAR_WINUI_BG_RULES.len()
        );

        // Layout-only wire: preset rules present, zero clear rules.
        let fe = Fe {
            layout: "Tabless".into(),
            ..Default::default()
        };
        let wire = build_xaml_wire(&fe).unwrap();
        assert!(wire.contains("Grid.Row="));
        assert!(!wire.contains("Background"));

        // Layout + clear + effect → preset rules first, clears appended
        // before the terminal `E`.
        let fe = Fe {
            effect: "acrylic".into(),
            clear_winui_bg: true,
            layout: "Tabless".into(),
            ..Default::default()
        };
        let wire = build_xaml_wire(&fe).unwrap();
        assert!(wire.contains("Grid.Row="));
        assert!(wire.ends_with("E\n"));
        assert_eq!(
            wire.matches("S Background=Transparent\n").count(),
            CLEAR_WINUI_BG_RULES.len()
        );
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
            layout: "",
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
