//! Settings host window + page protocol: state JSON, toasts, message handling, config adoption,
//! import / export / restore, settings diff application.

use super::*;
use std::sync::atomic::{AtomicU64, Ordering};

/// Largest snapping gap honoured, in DIPs; `snapping.gapPx` is clamped to `0..=` this.
pub(super) const MAX_SNAP_GAP_DIP: i32 = 64;

/// `snapping.gapPx` as the gap (DIPs) fence windows keep to each other and the work-area edges
/// while snapping.
pub(super) fn snap_gap_dip(snapping: &coredesk_core::SnappingSettings) -> i32 {
    snapping.gap_px.clamp(0, MAX_SNAP_GAP_DIP)
}

/// A script error on the settings page (`pageError`): its JavaScript error type and position
/// go to the log as fixed names and numbers; the page never sends the message, which can
/// quote what the user typed.
/// The widget dictionary's view of the *active* design's element params —
/// `{"time":{…},"day":{…},"date":{…}}` with each populated field a knob
/// the page applies. `{}` when nothing is configured.
fn clock_els_json(s: &coredesk_core::Settings) -> serde_json::Value {
    serde_json::to_value(s.active_widget_params().elements)
        .unwrap_or_else(|_| serde_json::json!({}))
}

/// `{"t":"clockStyle"}` post — design id plus the resolved element params
/// and the adaptive master flag so the page renders modder styling on
/// every design swap and fresh `clockReady`.
fn clock_style_json(s: &coredesk_core::Settings) -> String {
    serde_json::json!({
        "t": "clockStyle",
        "style": s.clock_style,
        "adaptive": s.clock_adaptive_color,
        "use24Hour": s.clock_use_24_hour,
        "els": clock_els_json(s),
    })
    .to_string()
}

/// Overlay sidecar-on-disk authored deltas under `s`'s in-memory dicts,
/// run before resolving/applying so a hand-edit made while the app ran is
/// honored without a restart.
///
/// Reconciliation is the same per-entry tri-state the save path uses,
/// driven by [`ConfigStore::sidecar_diff`]: an entry whose authored view
/// changed on disk *and* whose in-memory view still equals the recorded
/// baseline is a pure hand-edit — including a revert-to-baseline, which
/// produces no delta and used to be invisible — so the memory entry is
/// rebuilt from the disk delta (`None` delta = defaults). An entry the
/// app itself touched since the baseline (theme apply, IPC, runtime
/// mirror) keeps memory; untouched entries keep pending runtime edits.
///
/// Without a recorded baseline (`sidecar_diff` → `None`, e.g. a store
/// that never loaded) the legacy fill-if-unset merge runs instead.
fn merge_sidecar_deltas(s: &mut coredesk_core::Settings, store: &coredesk_core::ConfigStore) {
    use coredesk_core::config_store as cs;
    let empty = serde_json::json!({});
    if let Some(diff) = store.sidecar_diff(cs::LIVE_EFFECTS_CONFIG_FILE, cs::EFFECTS_DICT_KEY) {
        for id in &diff.changed {
            let untouched = s.effect_params.get(id).is_none_or(|p| {
                cs::memory_authored_view(
                    cs::EFFECTS_DICT_KEY,
                    id,
                    &serde_json::to_value(p).unwrap_or_else(|_| empty.clone()),
                ) == diff
                    .baseline
                    .get(id)
                    .cloned()
                    .unwrap_or_else(|| empty.clone())
            });
            if !untouched {
                continue;
            }
            let params = diff
                .deltas
                .get(id)
                .and_then(|d| serde_json::from_value::<coredesk_core::EffectParams>(d.clone()).ok())
                .map(|mut p| {
                    p.sanitize();
                    p
                })
                .unwrap_or_default();
            s.effect_params.insert(id.clone(), params);
        }
    } else {
        for (id, p) in
            cs::read_effect_sidecar_deltas(&store.dir().join(cs::LIVE_EFFECTS_CONFIG_FILE))
        {
            s.effect_params.entry(id).or_default().merge_disk(&p);
        }
    }
    if let Some(diff) = store.sidecar_diff(cs::WIDGET_CONFIG_FILE, cs::WIDGET_DICT_KEY) {
        for id in &diff.changed {
            let untouched = s.widget_params.get(id).is_none_or(|p| {
                cs::memory_authored_view(
                    cs::WIDGET_DICT_KEY,
                    id,
                    &serde_json::to_value(p).unwrap_or_else(|_| empty.clone()),
                ) == diff
                    .baseline
                    .get(id)
                    .cloned()
                    .unwrap_or_else(|| empty.clone())
            });
            if !untouched {
                continue;
            }
            let mut params = diff
                .deltas
                .get(id)
                .and_then(|d| serde_json::from_value::<coredesk_core::WidgetParams>(d.clone()).ok())
                .map(|mut p| {
                    p.sanitize();
                    p
                })
                .unwrap_or_default();
            // `bounds`/`adaptiveColor` mirror the visible widget — the
            // file can't snap the active design back mid-edit.
            if *id == s.clock_style
                && let Some(old) = s.widget_params.get(id)
            {
                params.bounds = old.bounds.or(params.bounds);
                params.adaptive_color = old.adaptive_color.or(params.adaptive_color);
            }
            s.widget_params.insert(id.clone(), params);
        }
    } else {
        for (id, p) in cs::read_widget_sidecar_deltas(&store.dir().join(cs::WIDGET_CONFIG_FILE)) {
            s.widget_params.entry(id).or_default().merge_disk(&p);
        }
    }
}

/// `{"t":"clockTheme"}` post — `sample` `Some` carries the probe result
/// (accent + backdrop luminance); `None` is the params/adaptive push used
/// on toggles and dict edits, which leaves the palette fields untouched.
fn clock_theme_json(
    s: &coredesk_core::Settings,
    sample: Option<&backdrop_probe::Sample>,
) -> String {
    let mut v = serde_json::json!({
        "t": "clockTheme",
        "adaptive": s.clock_adaptive_color,
        "use24Hour": s.clock_use_24_hour,
        "els": clock_els_json(s),
    });
    if let Some(sample) = sample {
        let o = v.as_object_mut().unwrap();
        o.insert("isLightBackdrop".into(), (sample.luminance > 150).into());
        o.insert("luminance".into(), sample.luminance.into());
        o.insert(
            "accent".into(),
            format!(
                "#{:02x}{:02x}{:02x}",
                sample.accent[0], sample.accent[1], sample.accent[2]
            )
            .into(),
        );
    }
    v.to_string()
}

fn log_page_error(v: &serde_json::Value) {
    const KINDS: &[&str] = &[
        "Error",
        "TypeError",
        "ReferenceError",
        "RangeError",
        "SyntaxError",
        "URIError",
        "EvalError",
        "AggregateError",
    ];
    let text = |key: &str| v.get(key).and_then(|k| k.as_str()).unwrap_or("");
    let kind = KINDS
        .iter()
        .find(|&&k| k == text("kind"))
        .copied()
        .unwrap_or("other");
    let file = text("file");
    let script = if file.ends_with("i18n.js") {
        "i18n.js"
    } else if file.is_empty() {
        "none"
    } else {
        "settings page"
    };
    let number = |key: &str| v.get(key).and_then(|n| n.as_u64()).unwrap_or(0);
    tracing::warn!(
        what = kind,
        at = script,
        line = number("line"),
        col = number("col"),
        "settings page script error"
    );
}

/// Where a wholesale configuration came from: names its toast and the undo snapshot.
#[derive(Clone, Copy)]
pub(super) enum ConfigSource {
    Import,
    Backup,
    Theme,
    /// The last theme re-fitted on a contain/cover switch — same machinery, its own toast.
    Retune,
}

impl ConfigSource {
    fn done(self) -> &'static str {
        match self {
            Self::Import => coredesk_core::i18n::text("已导入配置"),
            Self::Backup => coredesk_core::i18n::text("已恢复备份"),
            Self::Theme => coredesk_core::i18n::text("已导入主题"),
            Self::Retune => coredesk_core::i18n::text("已应用背景显示方式"),
        }
    }

    fn before(self) -> &'static str {
        match self {
            Self::Import | Self::Theme | Self::Retune => coredesk_core::i18n::text("导入配置前"),
            Self::Backup => coredesk_core::i18n::text("恢复备份前"),
        }
    }
}

/// Room for one of the page's automatic "…前" snapshots. Like the CLI's `auto_snapshot_slot`,
/// it never evicts a snapshot the user saved: a full list loses its oldest automatic one (the
/// page's or the CLI's) instead, and `false` means none is left to drop, so none is taken.
/// `protect` (the snapshot about to be restored) always stays. Automatic names saved in another
/// UI language count as the user's.
pub(super) fn auto_snapshot_room(
    snapshots: &mut Vec<coredesk_core::Snapshot>,
    protect: Option<uuid::Uuid>,
) -> bool {
    let auto = [
        coredesk_core::i18n::text("恢复快照前"),
        ConfigSource::Import.before(),
        ConfigSource::Backup.before(),
    ];
    if snapshots.len() >= coredesk_core::MAX_SNAPSHOTS
        && let Some(i) = snapshots.iter().position(|s| {
            Some(s.id) != protect
                && (s.is_auto_backup == Some(true)
                    || s.name.starts_with(super::ipc::AUTO_SNAPSHOT_PREFIX)
                    || auto.contains(&s.name.as_str()))
        })
    {
        snapshots.remove(i);
    }
    snapshots.len() < coredesk_core::MAX_SNAPSHOTS
}

/// What the gallery calls an automatic "…前" backup. Entries saved by this build store the
/// i18n key, which localizes on read; anything older (a CLI `auto-cli-…` name, a label
/// frozen in the language it was saved in) gets the generic restore label, same as before.
pub(super) fn auto_backup_label(name: &str) -> String {
    let key = match name {
        "恢复快照前" => "恢复快照前",
        "导入配置前" => "导入配置前",
        "恢复备份前" => "恢复备份前",
        _ => "恢复快照前",
    };
    coredesk_core::i18n::text(key).to_string()
}

fn snapshot_limit_text() -> String {
    coredesk_core::i18n::format(
        "快照已达上限（{0} 个），未保存新快照。请删除不需要的快照。",
        &[coredesk_core::MAX_SNAPSHOTS.to_string()],
    )
}

/// Remaps an imported `.cde` theme's layout monitor IDs to `work_areas` when the theme was
/// saved on a different monitor setup, preserving every fence's normalized coordinates and size.
fn remap_theme_monitors(
    cfg: &mut coredesk_core::Config,
    work_areas: &[coredesk_core::geometry::WorkArea],
) {
    if work_areas.is_empty() || cfg.layout_for(work_areas).is_some() {
        return;
    }
    let idx = cfg.last_shown().unwrap_or(0);
    let Some(layout) = cfg.layouts.get_mut(idx) else {
        return;
    };
    let mut old_ids: Vec<String> = layout
        .fingerprint
        .iter()
        .map(|m| m.device_path.clone())
        .collect();
    for f in &layout.fences {
        if !f.geometry.monitor.is_empty() && !old_ids.contains(&f.geometry.monitor) {
            old_ids.push(f.geometry.monitor.clone());
        }
    }
    for f in &mut layout.fences {
        let mapped_idx = old_ids
            .iter()
            .position(|id| id == &f.geometry.monitor)
            .unwrap_or(0);
        let target_area = work_areas.get(mapped_idx).unwrap_or(&work_areas[0]);
        f.geometry.monitor = target_area.device_path.clone();
    }
    layout.fingerprint = work_areas
        .iter()
        .map(coredesk_core::MonitorIdentity::of)
        .collect();
}

/// Serializes player teardown vs. the next pipeline's allocation: the join
/// before alloc still holds, just paid on a worker thread instead of the UI
/// thread that was logging `slow command` for every wallpaper switch.
static VIDEO_SWAP_LOCK: Mutex<()> = Mutex::new(());
/// Bumped on every wallpaper-intent change; a worker returning with a stale
/// epoch disposes its fresh player itself — the UI thread never joins it.
static VIDEO_SWAP_EPOCH: AtomicU64 = AtomicU64::new(0);

/// Post-install suppression window for the video-wallpaper retire checks: a
/// shell rebuild concurrent with our launch can write a fresh `Transcoded_*`
/// cache — indistinguishable from a user apply by mtime alone — so for the
/// first seconds after install a differing static report keeps the video.
const VIDEO_RETIRE_GRACE: Duration = Duration::from_secs(30);

/// Consecutive shell-up start failures before `ensure_video_wallpaper` stops
/// auto-retrying — a genuinely broken file would otherwise churn a worker
/// spawn plus first-frame decode on every poll, forever.
const MAX_VIDEO_START_FAILURES: u8 = 8;

/// The static wallpaper recorded when the video player installed, plus the
/// recording instants. A reported file predating the install cannot be the
/// user's response to the video — it is the shell re-asserting a cached
/// transcode: Explorer re-points `IDesktopWallpaper` at differently-named
/// `Transcoded_*` files during every rebuild, including the rebuild that
/// overlaps our own cold boot (where no `shell_respawn_wall` marker exists
/// yet). The instants carry the pre-existing-file check and the grace window.
#[derive(Debug)]
pub(super) struct StaticBaseline {
    path: PathBuf,
    mtime: Option<std::time::SystemTime>,
    recorded_at: std::time::SystemTime,
    recorded_instant: std::time::Instant,
}

/// A `WmfWallpaperPlayer::start` in flight on a worker; `poll_video_swap`
/// installs the result only while the epoch still matches config.
pub(super) struct VideoSwap {
    epoch: u64,
    /// The swap's full effective target — a pending swap for the identical
    /// target is kept instead of spawning a duplicate (a theme apply calls
    /// `sync_video_wallpaper` several times; each spawn used to invalidate the
    /// previous worker, killing fresh players at `frame_counter=1`).
    path: PathBuf,
    letterbox: bool,
    /// Recovery after shell loss vs. a user pick — the accept path logs/toasts
    /// them differently.
    reattach: bool,
    toast: bool,
    rx: std::sync::mpsc::Receiver<
        std::result::Result<coredesk_platform::wmf_wallpaper::WmfWallpaperPlayer, Option<String>>,
    >,
}

/// A `wmf_transcode::transcode_for_display` run in flight on a worker.
/// `poll_video_transcode` flips `video_wallpaper_path` to the optimized file
/// when it lands — while a transcode is pending, `sync_video_wallpaper` and
/// `ensure_video_wallpaper` hold off starting the player entirely.
pub(super) struct VideoTranscode {
    epoch: u64,
    /// The original pick — `video_wallpaper_path` holds it until the optimized
    /// file lands.
    src: PathBuf,
    toast: bool,
    rx: std::sync::mpsc::Receiver<std::result::Result<Option<PathBuf>, String>>,
}

/// Worker-side transcode: decodes, scales to `target` physical px, re-encodes
/// H.264 into `cache_dir`. `Ok(None)` = source already compliant; `Ok(Some)` =
/// the optimized file; `Err` = caller falls back to the source.
fn spawn_video_transcode(
    src: PathBuf,
    cache_dir: PathBuf,
    target: (u32, u32),
) -> std::sync::mpsc::Receiver<std::result::Result<Option<PathBuf>, String>> {
    let (tx, rx) = std::sync::mpsc::channel();
    let _ = std::thread::Builder::new()
        .name("coredesk-video-transcode".into())
        .spawn(move || {
            let _ = tx.send(coredesk_platform::wmf_transcode::transcode_for_display(
                &src, &cache_dir, target.0, target.1,
            ));
        });
    rx
}

/// Stops a player off the UI thread (stop+join ≈70 ms), serialized with swap
/// workers so a teardown never races the next pipeline's allocation.
fn reap_video_player(player: coredesk_platform::wmf_wallpaper::WmfWallpaperPlayer) {
    let _ = std::thread::Builder::new()
        .name("coredesk-video-reap".into())
        .spawn(move || {
            let _g = VIDEO_SWAP_LOCK.lock().unwrap_or_else(|e| e.into_inner());
            drop(player);
        });
}

/// Worker-side swap: stops the old player (join off the UI thread), starts the
/// new one. `Err(None)` marks a superseded request — not a failure to report.
fn spawn_video_swap(
    old: Option<coredesk_platform::wmf_wallpaper::WmfWallpaperPlayer>,
    path: PathBuf,
    letterbox: bool,
    epoch: u64,
) -> std::sync::mpsc::Receiver<
    std::result::Result<coredesk_platform::wmf_wallpaper::WmfWallpaperPlayer, Option<String>>,
> {
    let (tx, rx) = std::sync::mpsc::channel();
    let _ = std::thread::Builder::new()
        .name("coredesk-video-swap".into())
        .spawn(move || {
            let _g = VIDEO_SWAP_LOCK.lock().unwrap_or_else(|e| e.into_inner());
            drop(old);
            let result =
                coredesk_platform::wmf_wallpaper::WmfWallpaperPlayer::start(&path, letterbox)
                    .map_err(Some);
            let result = match result {
                Ok(player) if VIDEO_SWAP_EPOCH.load(Ordering::SeqCst) != epoch => {
                    drop(player);
                    Err(None)
                }
                other => other,
            };
            let _ = tx.send(result);
        });
    rx
}

impl App {
    /// Replaces the configuration wholesale (import / backup) and rebuilds everything.
    pub(super) fn adopt_config(&mut self, mut cfg: coredesk_core::Config, source: ConfigSource) {
        self.end_peek_now();
        // An external config (`.cde` archive or imported file) can carry
        // unknown styling ids or corrupt geometry from another version or a
        // hand edit — normalize before anything diffs/applies it, or the
        // shell surfaces get fed values this build can't honour.
        cfg.sanitize_external();
        if cfg.settings.desktop_effect != "none"
            && crate::effects_engine::Effect::from_id(&cfg.settings.desktop_effect).is_none()
        {
            cfg.settings.desktop_effect = "none".into();
        }
        let old_settings = self.state.config.settings.clone();
        self.state.replace_config(cfg);
        if let Some(desk) = shell::user_desktop() {
            self.state.migrate_desktop_path(&desk);
        }
        // What the file (plus the desktop-path migration) wants the settings to be. The Store
        // edition's startup switch is Windows': a file does not change it (nor open its page).
        let mut new_settings = self.state.config.settings.clone();
        if coredesk_platform::process::is_packaged() {
            new_settings.autostart = old_settings.autostart;
        }
        if !matches!(source, ConfigSource::Theme | ConfigSource::Retune) {
            // A plain config import/backup restore severs the link to whatever theme
            // described the old setup — never trust a path a foreign file carried.
            new_settings.active_theme_path = None;
        }
        self.sync_desktop_if_available("config adopted");
        // `.cde` manifests bind fence contents to this desktop's actual files (names that
        // match nothing are dropped); a theme's folder portal whose directory is missing
        // here degrades to a virtual fence rather than flashing "folder not found".
        let desktop_dirs: Vec<PathBuf> = [shell::user_desktop(), shell::public_desktop()]
            .into_iter()
            .flatten()
            .collect();
        self.state.bind_theme_members(
            &desktop_dirs,
            matches!(source, ConfigSource::Theme | ConfigSource::Retune),
        );
        self.relayout_from_state();
        // The file's portals need enumerating; runtime state of the replaced ones is pruned.
        self.refresh_portals();
        // Re-apply the settings through the diff path so the anchor / Run-key side effects
        // (hide_real_icons, quick_hide.enabled, show_desktop, autostart) and the ctx.behavior
        // cells update exactly as if the user had changed them on the page.
        self.state.config.settings = old_settings;
        self.apply_settings(new_settings);
        // apply_settings only refreshes on a visual / icon diff; a wholesale swap (per-fence
        // tint, layouts) needs these regardless.
        self.sync_peek_hotkey();
        self.sync_video_wallpaper();
        self.apply_icon_variant();
        self.refresh_visuals(true);
        // A theme apply rebuilds the desktop's composition band (wallpaper
        // swap, effects re-open) — the WebView's companion surface can lose
        // its alpha to that churn and paint the widget opaque black. The
        // fail-safe soft restart respawns it transparent at its live rect.
        if matches!(source, ConfigSource::Theme | ConfigSource::Retune) {
            self.soft_restart_clock();
        }
        self.state.save_if_dirty();
        self.push_settings_state();
        self.settings_toast(source.done());
    }

    /// Lazily creates the shared WebView2 environment both hosts use;
    /// `false` means the caller should abandon the open (error reported).
    fn ensure_web_env(&mut self) -> bool {
        if self.web_env.is_some() {
            return true;
        }
        match WebEnvironment::create() {
            Ok(env) => {
                self.web_env = Some(env);
                true
            }
            // WebView2's wait for its callback pumps messages and ends with an error when
            // it takes a WM_QUIT (the session ends): quit after all.
            Err(_) if window::quit_posted() => {
                window::post_quit(0);
                false
            }
            Err(e) => {
                tracing::error!(error = %e, "WebView2 environment failed");
                if let Some(t) = &self.tray {
                    t.show_info(
                        "CoreDesk Engine",
                        coredesk_core::i18n::text(
                            "无法创建 WebView2 环境，请确认已安装 Microsoft Edge WebView2 运行时。",
                        ),
                        true,
                    );
                }
                false
            }
        }
    }

    pub(super) fn open_settings(&mut self) {
        self.query_store_startup();
        if let Some(h) = &self.settings {
            window::show_normal(h.hwnd());
            window::bring_to_front(h.hwnd());
            return;
        }
        if !self.ensure_web_env() {
            return;
        }
        let liquid_glass = self.ctx.theme.borrow().liquid_glass;
        match SettingsHost::open(
            &self.settings_class,
            self.web_env.as_ref().unwrap(),
            self.theme_mode,
            liquid_glass,
            self.queue.clone(),
        ) {
            Ok(mut host) => {
                // Caption icon: the same drawn fence glyph as the tray, at 16/32 DIP.
                let scale = monitors::dpi_for_window(host.hwnd()).max(96) as f32 / 96.0;
                let accent = self.ctx.theme.borrow().accent_rgb8();
                let dark = self.theme_mode == ThemeMode::Dark;
                let small = (16.0 * scale).round() as i32;
                let big = (32.0 * scale).round() as i32;
                host.set_icons(
                    (small, tray_icon_image(small, accent, dark)),
                    (big, tray_icon_image(big, accent, dark)),
                );
                self.settings = Some(host);
                self.sync_app_glass();
            }
            Err(_) if window::quit_posted() => window::post_quit(0),
            Err(e) => {
                tracing::error!(error = %e, "settings window failed");
                if let Some(t) = &self.tray {
                    t.show_info(
                        "CoreDesk Engine",
                        coredesk_core::i18n::text("无法打开设置窗口（WebView2 初始化失败），请检查 Microsoft Edge WebView2 运行时。"),
                        true,
                    );
                }
            }
        }
    }

    /// `#AARRGGBB` → `(a, r, g, b)` — the settings model's stored form;
    /// `dwm::sync_app_glass` repacks it for the accent policy.
    fn parse_argb(s: &str) -> Option<(u8, u8, u8, u8)> {
        let h = s.trim().strip_prefix('#').unwrap_or(s.trim());
        let v = u32::from_str_radix(h, 16).ok()?;
        match h.len() {
            8 => Some(((v >> 24) as u8, (v >> 16) as u8, (v >> 8) as u8, v as u8)),
            6 => Some((0xFF, (v >> 16) as u8, (v >> 8) as u8, v as u8)),
            _ => None,
        }
    }

    /// Mirrors the armed ExplorerBlurMica look onto our own windows: the host
    /// HWND frame gets the real DWM material (DWMSBT for mica/micaalt, the
    /// accent policy for acrylic/blur/blurclear) and each WebView's sheet is
    /// re-blended toward the tint — an HWND-hosted WebView can't composite
    /// alpha through to the backdrop, so the client carries the colour.
    /// Called whenever `fe_*` lands, on theme-mode switches (the light/dark
    /// palette differs), and on `WM_DWMCOMPOSITIONCHANGED`.
    pub(super) fn sync_app_glass(&mut self) {
        let s = &self.state.config.settings;
        let rgba = if systheme::apps_use_light_theme() {
            s.fe_light_rgba.clone()
        } else {
            s.fe_dark_rgba.clone()
        };
        let effect = s.fe_effect.clone();
        self.apply_glass(&effect, &rgba);
    }

    /// The piece `sync_app_glass` splits out so `apply_settings` can feed it
    /// the *incoming* values — the committed config still holds the old
    /// material while the diff is being applied.
    fn apply_glass(&self, effect: &str, rgba: &str) {
        let tint = Self::parse_argb(rgba);
        tracing::debug!(
            effect,
            tint = rgba,
            settings_open = self.settings.is_some(),
            fe_glass_open = self.fe_glass.is_some(),
            "app glass: mirroring EBM material onto own windows"
        );
        if let Some(h) = &self.settings {
            h.set_glass(effect, tint);
        }
        if let Some(h) = &self.fe_glass {
            h.set_glass(effect, tint);
        }
    }

    /// Desktop clock widget: `""`/`"none"` closes it; any other design id
    /// swaps the page's design in place on the single widget window — its
    /// position and size are untouched. Called from `apply_settings`' diff
    /// and once at startup for a persisted style.
    pub(super) fn apply_clock_style(&mut self, style: &str) {
        let s = style.trim();
        if s.is_empty() || s == "none" {
            if let Some(h) = self.clock.take() {
                h.close();
            }
            window::kill_timer(self.control.hwnd(), super::TIMER_CLOCK_PROBE);
            return;
        }
        if let Some(h) = &self.clock {
            h.post_json(&clock_style_json(&self.state.config.settings));
            // Re-assert the transparent surface — cheap insurance for the
            // alpha-loss case the soft restart also covers.
            h.rearm_transparency();
            self.probe_clock_backdrop();
            return;
        }
        if !self.ensure_web_env() {
            return;
        }
        let bounds = self.state.config.settings.clock_bounds;
        match crate::clock_host::ClockHost::open(
            &self.clock_class,
            self.web_env.as_ref().unwrap(),
            bounds,
            self.queue.clone(),
        ) {
            Ok(host) => {
                self.clock = Some(host);
                // Probe cadence lives for as long as the widget does;
                // `clockReady` fires the first sample once the page listens.
                window::set_timer(self.control.hwnd(), super::TIMER_CLOCK_PROBE, 2_500);
            }
            Err(e) => tracing::error!(error = %e, "clock widget failed"),
        }
    }

    /// Soft restart for the widget: destroys it and respawns at its *live*
    /// rect — the window's own rect, not the config's, so a restart mid-drag
    /// still lands it exactly where it was. The `.cde` theme-apply fail-safe
    /// and the widget's 「تحديث」 context menu both run this; the fresh page's
    /// `clockReady` re-arms the persisted style, locale, and backdrop probe.
    pub(super) fn soft_restart_clock(&mut self) {
        let Some(h) = self.clock.take() else {
            return;
        };
        let r = window::window_rect(h.hwnd());
        let live = [r.left, r.top, r.right - r.left, r.bottom - r.top];
        if live[2] > 0 && live[3] > 0 {
            self.state.config.settings.clock_bounds = Some(live);
        }
        // Right-click refresh is the modder's "pick up my edits" gesture —
        // pull sidecar deltas authored while the app ran before rebuilding.
        merge_sidecar_deltas(&mut self.state.config.settings, &self.state.store);
        h.close();
        let style = self.state.config.settings.clock_style.clone();
        self.apply_clock_style(&style);
    }

    /// Native desktop particle/weather overlay. `"none"` tears the overlay
    /// down to zero resources; a real effect opens (or rebuilds) it and joins
    /// the anchor's desktop-surface band — directly above the icon host (and
    /// any video wallpaper, which is a child of that host) and below fences.
    /// Called from `apply_settings`' diff and once at startup.
    pub(super) fn apply_desktop_effect(&mut self, s: &coredesk_core::Settings) {
        let Some(host) = &self.effects else {
            return;
        };
        let before = host.hwnd();
        host.apply(
            &s.desktop_effect,
            s.desktop_effect_intensity as f32 / 100.0,
            s.resolve_effect_params(&s.desktop_effect).as_ref(),
        );
        let after = host.hwnd();
        if before == after {
            return;
        }
        if let Some(a) = self.anchor.borrow_mut().as_mut() {
            if let Some(h) = before {
                a.unregister_fence(h);
            }
            if let Some(h) = after {
                a.register_desktop_surface(h);
            }
            a.reanchor("desktop effects changed");
        }
    }

    /// Adaptive backdrop contrast: spawns the off-thread sampler for the
    /// widget's current rect. No-ops unless a widget exists and the setting
    /// is on — the 2.5s timer may keep firing with no widget and costs a
    /// no-op; `apply_clock_theme` re-checks the flag before posting.
    pub(super) fn probe_clock_backdrop(&mut self) {
        let Some(h) = &self.clock else {
            return;
        };
        if !self.state.config.settings.clock_adaptive_color {
            return;
        }
        let (rect, widget) = (window::window_rect(h.hwnd()), h.hwnd().0 as isize);
        let control = self.control.hwnd().0 as isize;
        std::thread::Builder::new()
            .name("coredesk-clock-probe".into())
            .spawn(move || {
                let _com = coredesk_platform::com::MtaGuard::init();
                let sample =
                    backdrop_probe::sample(HWND(widget as _), rect, &monitors::enumerate());
                let boxed = Box::into_raw(Box::new(sample));
                window::post_message(HWND(control as _), WM_APP_CLOCK_PROBE, 0, boxed as isize);
            })
            .ok();
    }

    /// A finished sample: paint the widget's theme message when adaptive
    /// mode is still armed (a toggle-off since the probe started wins).
    pub(super) fn apply_clock_theme(&mut self, sample: backdrop_probe::Sample) {
        let Some(h) = &self.clock else {
            return;
        };
        if !self.state.config.settings.clock_adaptive_color {
            return;
        }
        h.post_json(&clock_theme_json(
            &self.state.config.settings,
            Some(&sample),
        ));
    }

    /// The dedicated ExplorerBlurMica editor (settings card's 高级设置 button):
    /// a singleton — a second click foregrounds the open window, maximized or not.
    pub(super) fn open_fe_glass(&mut self) {
        if let Some(h) = &self.fe_glass {
            window::show_normal(h.hwnd());
            window::bring_to_front(h.hwnd());
            return;
        }
        if !self.ensure_web_env() {
            return;
        }
        match crate::fe_glass_host::FeGlassHost::open(
            &self.fe_glass_class,
            self.web_env.as_ref().unwrap(),
            self.theme_mode,
            self.queue.clone(),
        ) {
            Ok(mut host) => {
                // Same drawn fence glyph as the settings caption, at 16/32 DIP.
                let scale = monitors::dpi_for_window(host.hwnd()).max(96) as f32 / 96.0;
                let accent = self.ctx.theme.borrow().accent_rgb8();
                let dark = self.theme_mode == ThemeMode::Dark;
                let small = (16.0 * scale).round() as i32;
                let big = (32.0 * scale).round() as i32;
                host.set_icons(
                    (small, tray_icon_image(small, accent, dark)),
                    (big, tray_icon_image(big, accent, dark)),
                );
                self.fe_glass = Some(host);
                self.sync_app_glass();
            }
            Err(_) if window::quit_posted() => window::post_quit(0),
            Err(e) => {
                tracing::error!(error = %e, "fe glass window failed");
                self.settings_error(coredesk_core::i18n::text(
                    "无法打开设置窗口（WebView2 初始化失败），请检查 Microsoft Edge WebView2 运行时。",
                ));
            }
        }
    }

    /// The `feState` payload the glass editor renders from — the seven `fe_*`
    /// fields plus `systemLight` (`AppsUseLightTheme`), which decides which
    /// RGBA row is editable and which palette the preview tints with.
    pub(super) fn fe_glass_state_json(&self) -> String {
        let localization = coredesk_core::i18n::ui_payload();
        let s = &self.state.config.settings;
        serde_json::json!({
            "type": "feState",
            "locale": localization["locale"],
            "translations": localization["translations"],
            "systemLight": systheme::apps_use_light_theme(),
            "fe": {
                "effect": s.fe_effect,
                "clearAddress": s.fe_clear_address,
                "clearBarBg": s.fe_clear_bar_bg,
                "clearWinuiBg": s.fe_clear_winui_bg,
                "showLine": s.fe_show_line,
                "lightRgba": s.fe_light_rgba,
                "darkRgba": s.fe_dark_rgba,
            },
        })
        .to_string()
    }

    /// Signature of the `feState` payload — everything the page renders from
    /// that can change beneath it: the fields, the active-theme flag, and the
    /// language (the document localizes itself from `translations`).
    fn fe_glass_sig(&self) -> String {
        let s = &self.state.config.settings;
        format!(
            "{}|{}|{}|{}|{}|{}|{}|{}|{}",
            s.fe_effect,
            s.fe_clear_address,
            s.fe_clear_bar_bg,
            s.fe_clear_winui_bg,
            s.fe_show_line,
            s.fe_light_rgba,
            s.fe_dark_rgba,
            systheme::apps_use_light_theme(),
            coredesk_core::i18n::ui_payload()["locale"],
        )
    }

    /// Pushes `feState` to the open glass editor — but only when what it would
    /// show changed since the last push. Otherwise an unrelated settings
    /// commit would keep resetting the page's unsaved draft mid-edit.
    pub(super) fn push_fe_glass_state(&self) {
        let Some(h) = &self.fe_glass else {
            return;
        };
        let sig = self.fe_glass_sig();
        if *self.fe_glass_pushed.borrow() == sig {
            return;
        }
        *self.fe_glass_pushed.borrow_mut() = sig;
        h.post_json(&self.fe_glass_state_json());
    }

    /// The page's Apply: adopt the seven `fe_*` fields, run the normal
    /// settings diff (`apply_effect` writes `config.ini` and reloads EBM in
    /// each cabinet pid), then restart Explorer so every surface re-reads the
    /// new material — the same refresh upstream's `register.cmd` performs.
    /// On a no-change apply nothing is restarted.
    fn apply_fe_glass(&mut self, v: &serde_json::Value) {
        let old = self.state.config.settings.clone();
        let mut new = old.clone();
        // Missing keys keep the old value — a partial/malformed payload
        // must not wipe fields (an absent `layout` would persist "" and the
        // next feState push would snap the dropdown back to Windows Default).
        let str_of = |k: &str, d: &str| -> String {
            v.get(k).and_then(|x| x.as_str()).unwrap_or(d).to_string()
        };
        let bool_of = |k: &str, d: bool| v.get(k).and_then(|x| x.as_bool()).unwrap_or(d);
        new.fe_effect = str_of("effect", &old.fe_effect);
        new.fe_clear_address = bool_of("clearAddress", old.fe_clear_address);
        new.fe_clear_bar_bg = bool_of("clearBarBg", old.fe_clear_bar_bg);
        new.fe_clear_winui_bg = bool_of("clearWinuiBg", old.fe_clear_winui_bg);
        new.fe_show_line = bool_of("showLine", old.fe_show_line);
        new.fe_light_rgba = str_of("lightRgba", &old.fe_light_rgba);
        new.fe_dark_rgba = str_of("darkRgba", &old.fe_dark_rgba);
        let wanted = new.fe_effect.clone();
        self.apply_settings(new);
        let applied = self.state.config.settings.fe_effect == wanted;
        if let Some(h) = &self.fe_glass {
            let (text, error) = if !applied {
                (
                    coredesk_core::i18n::text("资源管理器背景效果未能更改，请稍后重试。")
                        .to_string(),
                    true,
                )
            } else {
                // No toast on a no-op; applied changes get a plain confirmation —
                // the resident reloads the EBM config live, so no Explorer restart
                // is needed (the manual failsafe action below still offers one).
                (coredesk_core::i18n::text("已应用").to_string(), false)
            };
            h.post_json(
                &serde_json::json!({ "type": "feToast", "text": text, "error": error }).to_string(),
            );
        }
    }

    pub(super) fn settings_state_json(&self) -> String {
        let localization = coredesk_core::i18n::ui_payload();
        let fences: Vec<serde_json::Value> = self
            .state
            .fences()
            .iter()
            .map(|f| self.fence_options_json(f))
            .collect();
        let mem_mb = coredesk_platform::memstats::MemoryStats::current()
            .map(|m| m.private_working_set as f64 / (1024.0 * 1024.0))
            .unwrap_or(0.0);
        let snapshots: Vec<serde_json::Value> = self
            .state
            .config
            .snapshots
            .iter()
            .map(|s| {
                serde_json::json!({
                    "id": s.id,
                    "name": if s.is_auto_backup == Some(true) {
                        auto_backup_label(&s.name)
                    } else {
                        s.name.clone()
                    },
                    "ts": s.ts,
                    "date": coredesk_platform::fileinfo::format_local_datetime(s.ts),
                    "fenceCount": super::ipc::snapshot_dto(s, &self.state.work_areas).fence_count,
                    "wallpaperPath": s.wallpaper_path,
                    "isAutoBackup": s.is_auto_backup == Some(true),
                })
            })
            .collect();
        let themes = self.themes_json();
        let monitors: Vec<serde_json::Value> = self
            .monitor_labels()
            .into_iter()
            .map(|(id, label)| serde_json::json!({ "id": id, "label": label }))
            .collect();
        let accent = {
            let [r, g, b] = self.ctx.theme.borrow().accent_rgb8();
            format!("#{r:02X}{g:02X}{b:02X}")
        };
        serde_json::json!({
            "type": "state",
            "locale": localization["locale"],
            "translations": localization["translations"],
            "settings": self.state.config.settings,
            "desktopIconsHidden": coredesk_platform::shell_icons::desktop_icons_hidden(),
            "rules": self.state.config.rules,
            "fences": fences,
            "snapshots": snapshots,
            "themes": themes,
            "monitors": monitors,
            "tintPalette": fence_options::tint_palette_json(),
            "version": env!("CARGO_PKG_VERSION"),
            "storeEdition": coredesk_platform::process::is_packaged(),
            "configPath": self.state.config_path_on_disk().to_string_lossy(),
            "memoryMb": mem_mb,
            "itemCount": self.state.workspace_item_count(),
            "themeMode": if self.theme_mode == ThemeMode::Dark { "dark" } else { "light" },
            "accent": accent,
            "update": self.update_json(),
            "peekHotkey": self.peek_hotkey_json(),
            "themeDirty": self.theme_dirty.get(),
            // Start Menu Style dropdown: only {id, label} reach the page — the
            // converted rule payload stays behind start_menu_themes::theme().
            "startMenuStyles": coredesk_core::start_menu_themes::style_options_json(),
            // Taskbar Style dropdown — same isolation for the taskbar catalog.
            "taskbarStyles": coredesk_core::taskbar_themes::style_options_json(),
            // Notification Center Style dropdown — same isolation again.
            "notificationCenterStyles": coredesk_core::notification_center_themes::style_options_json(),
        })
        .to_string()
    }

    pub(super) fn push_settings_state(&self) {
        if let Some(h) = &self.settings {
            h.post_json(&self.settings_state_json());
        }
        self.push_fe_glass_state();
    }

    /// File watcher/navigation updates must not rebuild controls while the user is
    /// editing a name or has a select popup open.
    pub(super) fn push_workspace_summary(&self) {
        if let Some(h) = &self.settings {
            h.post_json(
                &serde_json::json!({
                    "type": "workspaceSummary",
                    "fenceCount": self.state.fences().len(),
                    "itemCount": self.state.workspace_item_count(),
                })
                .to_string(),
            );
        }
    }

    /// Call after mutating `self.state.config.settings` anywhere other than apply_settings:
    /// persists the change and keeps an open settings page in sync so its next
    /// whole-object patchSettings does not revert it.
    pub(super) fn settings_mutated(&mut self) {
        self.state.mark_dirty();
        self.schedule_save();
        self.push_settings_state();
    }

    /// Store edition: reads Windows' startup switch for the package on a worker; the result comes
    /// back through `WM_APP_STARTUP_TASK` to [`Self::finish_store_startup`].
    pub(super) fn query_store_startup(&self) {
        if !coredesk_platform::process::is_packaged() {
            return;
        }
        let result = self.store_startup.clone();
        let control = self.control.hwnd().0 as isize;
        let _ = std::thread::Builder::new()
            .name("coredesk-startup-task".into())
            .spawn(
                move || match coredesk_platform::store::startup_task_enabled() {
                    Ok(on) => {
                        result.store(1 + on as u8, std::sync::atomic::Ordering::Relaxed);
                        window::post_message(
                            HWND(control as *mut core::ffi::c_void),
                            WM_APP_STARTUP_TASK,
                            0,
                            0,
                        );
                    }
                    Err(e) => tracing::warn!(error = %e, "reading the startup task failed"),
                },
            );
    }

    /// 「随 Windows 启动」 follows the switch Windows keeps for the Store package.
    pub(super) fn finish_store_startup(&mut self) {
        let on = match self
            .store_startup
            .load(std::sync::atomic::Ordering::Relaxed)
        {
            1 => false,
            2 => true,
            _ => return,
        };
        if self.state.config.settings.autostart != on {
            tracing::info!(on, "startup switch follows Windows");
            self.state.config.settings.autostart = on;
            self.settings_mutated();
        }
    }

    pub(super) fn settings_toast(&self, text: &str) {
        if let Some(h) = &self.settings {
            h.post_json(&serde_json::json!({ "type": "toast", "text": text }).to_string());
        }
    }

    fn settings_error(&self, text: &str) {
        // The toast is the whole UI surface — without this line a failed write
        // or rejected apply is undiagnosable in the log.
        tracing::error!(error = %text, "settings error shown to user");
        if let Some(h) = &self.settings {
            h.post_json(
                &serde_json::json!({ "type": "toast", "text": text, "error": true }).to_string(),
            );
        }
    }

    pub(super) fn on_settings_message(&mut self, json: &str) {
        let Ok(v) = serde_json::from_str::<serde_json::Value>(json) else {
            return;
        };
        match v.get("type").and_then(|t| t.as_str()) {
            Some("ready") => {
                tracing::info!("settings: page ready");
                if let Some(h) = self.settings.as_mut() {
                    h.ready = true;
                }
                // A reload in the middle of recording a shortcut never says it ended.
                self.resume_peek_hotkey();
                self.push_settings_state();
                // The first `ready` shows the window; a reload's leaves it as the user left it.
                if let Some(h) = self
                    .settings
                    .as_ref()
                    .filter(|h| !desktop::is_visible(h.hwnd()))
                {
                    window::show_normal(h.hwnd());
                    window::bring_to_front(h.hwnd());
                }
                if let Some(fence) = self.settings_focus_fence.take() {
                    self.post_show_fence(fence);
                }
                self.show_pending_feedback();
                // The initial `set_glass` post can land before the page's
                // message listener exists — re-push now that it is live.
                self.sync_app_glass();
                // Same race for the window state used by the custom
                // titlebar's maximize/restore glyph.
                if let Some(h) = self.settings.as_ref() {
                    h.post_json(&format!(
                        "{{\"t\":\"wstate\",\"max\":{}}}",
                        coredesk_platform::frameless::is_zoomed(h.hwnd())
                    ));
                }
            }
            // The glass editor's own protocol — fe_glass.html is the only page
            // that speaks it, so routing by type stays unambiguous.
            Some("feReady") => {
                // Fresh page: force the state push (the sig guard would skip a
                // repeated payload), then reveal the window maximized.
                self.fe_glass_pushed.borrow_mut().clear();
                self.push_fe_glass_state();
                if let Some(h) = &self.fe_glass {
                    window::show_maximized(h.hwnd());
                    window::bring_to_front(h.hwnd());
                }
                // Same subscription race as the settings page — the first
                // `set_glass` post can precede the listener.
                self.sync_app_glass();
                if let Some(h) = self.fe_glass.as_ref() {
                    h.post_json(&format!(
                        "{{\"t\":\"wstate\",\"max\":{}}}",
                        coredesk_platform::frameless::is_zoomed(h.hwnd())
                    ));
                }
            }
            // The clock widget page is live: arm its design, then reveal.
            Some("clockReady") => {
                if let Some(h) = &self.clock {
                    h.post_json(&clock_style_json(&self.state.config.settings));
                    window::show_normal(h.hwnd());
                }
                self.probe_clock_backdrop();
            }
            // Drag/resize ended — persist the widget's new bounds.
            Some("clockMoved") => {
                let get = |k: &str| v.get(k).and_then(|n| n.as_i64()).unwrap_or(0) as i32;
                self.state.config.settings.clock_bounds =
                    Some([get("x"), get("y"), get("w"), get("h")]);
                self.schedule_save();
                self.probe_clock_backdrop();
            }
            Some("feApply") => self.apply_fe_glass(&v),
            Some("feCancel") => {
                if let Some(h) = self.fe_glass.take() {
                    self.fe_glass_pushed.borrow_mut().clear();
                    h.close();
                }
            }
            Some("setFence") => self.on_set_fence(&v),
            Some("ringFence") => {
                let host = v
                    .get("id")
                    .and_then(|i| i.as_str())
                    .and_then(|i| uuid::Uuid::parse_str(i).ok())
                    .map(|id| self.state.host_of(id));
                self.set_settings_ring(host);
            }
            Some("patchSettings") => {
                let Some(new_settings) = v.get("settings").cloned() else {
                    return;
                };
                match serde_json::from_value::<coredesk_core::Settings>(new_settings) {
                    Ok(new_settings) => self.apply_settings(new_settings),
                    Err(e) => tracing::warn!(error = %e, "bad settings from page"),
                }
            }
            Some("setRules") => {
                if let Some(rules) = v.get("rules").cloned()
                    && let Ok(rules) = serde_json::from_value::<coredesk_core::RuleSet>(rules)
                {
                    self.set_rules(rules);
                }
            }
            Some("action") => match v.get("name").and_then(|n| n.as_str()) {
                Some("applyRules") => {
                    let entries = shell::enumerate_desktop();
                    let moved = self.state.apply_rules_all(&entries);
                    self.refresh_all();
                    self.schedule_save();
                    self.settings_toast(&coredesk_core::i18n::format(
                        "已按规则整理 {0} 个项目",
                        &[moved.to_string()],
                    ));
                }
                Some("applyRulesForce") => {
                    // Force-apply wipes every manual desktop pin and user assignment:
                    // keep a way back, same as `rules apply` (auto snapshots rotate
                    // their own kind).
                    let pinned = !self.state.config.layouts[self.state.layout]
                        .desktop_items
                        .is_empty()
                        || self.state.fences().iter().any(|f| {
                            f.items
                                .iter()
                                .any(|r| r.assigned_by == coredesk_core::AssignedBy::User)
                        });
                    if pinned {
                        self.auto_snapshot();
                    }
                    let entries = shell::enumerate_desktop();
                    let moved = self.state.apply_rules_force_all(&entries);
                    self.refresh_all();
                    self.schedule_save();
                    self.settings_toast(&coredesk_core::i18n::format(
                        "已按规则整理 {0} 个项目",
                        &[moved.to_string()],
                    ));
                }
                Some("addTemplate") => {
                    if let Some(template) = v
                        .get("template")
                        .and_then(|t| t.as_str())
                        .and_then(coredesk_core::rules::Template::parse)
                    {
                        self.add_template(template);
                    }
                }
                Some("copyFenceSettings") => {
                    let fence = |key: &str| {
                        v.get(key)
                            .and_then(|i| i.as_str())
                            .and_then(|i| uuid::Uuid::parse_str(i).ok())
                    };
                    if let (Some(target), Some(source)) = (fence("id"), fence("source")) {
                        self.copy_fence_settings(target, source);
                    }
                }
                Some("openConfigFolder") => {
                    // Explorer opens Documents for a path it cannot see: hand it the real one.
                    use std::os::windows::process::CommandExt;
                    let path = self.state.config_path_on_disk();
                    let mut explorer = std::process::Command::new("explorer.exe");
                    if path.is_file() {
                        // Quoted as a whole (what `arg` does once a space appears), the
                        // `/select` argument opens Documents too.
                        explorer.raw_arg(format!("/select,\"{}\"", path.display()));
                    } else {
                        explorer.arg(path.parent().unwrap_or(&path));
                    }
                    let _ = explorer.spawn();
                }
                Some("pickVideoWallpaper") => self.pick_video_wallpaper(),
                Some("applyVideoWallpaper") => {
                    if let Some(p) = v.get("path").and_then(|p| p.as_str()) {
                        self.apply_video_wallpaper(Path::new(p));
                    }
                }
                Some("exportTheme") => self.export_theme(),
                Some("exportThemeTo") => {
                    if let Some(p) = v.get("path").and_then(|p| p.as_str()) {
                        self.export_theme_to(Path::new(p));
                    }
                }
                Some("importTheme") => self.import_theme(),
                Some("importThemeFrom") => {
                    if let Some(p) = v.get("path").and_then(|p| p.as_str()) {
                        self.import_theme_from(Path::new(p));
                    }
                }
                Some("applyTheme") => {
                    if let Some(p) = v.get("path").and_then(|p| p.as_str()) {
                        self.request_theme_switch(Path::new(p));
                    }
                }
                Some("themeSwitchDecision") => {
                    let decision = v.get("decision").and_then(|d| d.as_str()).unwrap_or("");
                    let path = v.get("path").and_then(|p| p.as_str()).map(PathBuf::from);
                    match (decision, path) {
                        ("save", Some(p)) => {
                            // Save the live state into the ACTIVE theme's archive first,
                            // then switch — the file keeps its name, nothing is appended.
                            if !self.overwrite_active_theme() {
                                return;
                            }
                            self.apply_theme_from_library(&p);
                        }
                        ("discard", Some(p)) => self.apply_theme_from_library(&p),
                        _ => {} // "cancel" (or a malformed message) leaves everything as is
                    }
                }
                Some("saveSetup") => {
                    let name = v.get("themeName").and_then(|n| n.as_str()).unwrap_or("");
                    // Themes always bundle the active background — no opt-out.
                    self.save_theme_to_library(name);
                }
                Some("openThemesFolder") => {
                    let dir = self.themes_dir();
                    let _ = std::fs::create_dir_all(&dir);
                    let _ = std::process::Command::new("explorer.exe").arg(&dir).spawn();
                }
                Some("exportItem") => match v.get("kind").and_then(|k| k.as_str()) {
                    Some("theme") => {
                        if let Some(p) = v.get("path").and_then(|p| p.as_str()) {
                            self.export_library_theme(Path::new(p));
                        }
                    }
                    Some("snapshot") => {
                        if let Some(id) = v
                            .get("id")
                            .and_then(|i| i.as_str())
                            .and_then(|i| uuid::Uuid::parse_str(i).ok())
                        {
                            self.export_snapshot_theme(id);
                        }
                    }
                    _ => {}
                },
                Some("deleteTheme") => {
                    if let Some(p) = v.get("path").and_then(|p| p.as_str()) {
                        self.delete_library_theme(Path::new(p));
                    }
                }
                Some("openUrl") => {
                    if let Some(url) = v.get("url").and_then(|u| u.as_str())
                        && (url.starts_with("https://cloudconvert.com")
                            || url.starts_with("https://www.freeconvert.com")
                            || url.starts_with("https://freeconvert.com"))
                    {
                        let _ = shell::shell_execute(Path::new(url), None, None);
                    }
                }
                Some("saveSnapshot") => {
                    // A full list refuses a new snapshot (as the CLI does) instead of
                    // dropping the oldest one.
                    if self.state.config.snapshots.len() >= coredesk_core::MAX_SNAPSHOTS {
                        self.settings_error(&snapshot_limit_text());
                    } else {
                        let name = v.get("snapshotName").and_then(|n| n.as_str()).unwrap_or("");
                        let include_wallpaper = v
                            .get("includeWallpaper")
                            .and_then(|f| f.as_bool())
                            .unwrap_or(false);
                        let wallpaper = include_wallpaper
                            .then(coredesk_platform::sysparams::desktop_wallpaper)
                            .flatten();
                        self.state.save_snapshot(name, wallpaper);
                        self.schedule_save();
                        self.push_settings_state();
                        self.settings_toast(coredesk_core::i18n::text("已保存快照"));
                    }
                }
                Some("restoreSnapshot") => {
                    if let Some(id) = v
                        .get("id")
                        .and_then(|i| i.as_str())
                        .and_then(|i| uuid::Uuid::parse_str(i).ok())
                        && self.state.config.snapshots.iter().any(|s| s.id == id)
                    {
                        // Snapshot of the current state first, so "restore" is reversible;
                        // a list full of the user's own snapshots has no room for it, so
                        // nothing is restored. The fixed-id restore backup overwrites itself
                        // and needs the room only until it is first taken.
                        let has_backup = self
                            .state
                            .config
                            .snapshots
                            .iter()
                            .any(|s| s.id == coredesk_core::AUTO_BACKUP_SNAPSHOT_ID);
                        if !has_backup
                            && !auto_snapshot_room(&mut self.state.config.snapshots, Some(id))
                        {
                            self.settings_error(&snapshot_limit_text());
                            return;
                        }
                        let wallpaper = self.state.snapshot_wallpaper(id);
                        self.state.restore_snapshot_with_backup(id, "恢复快照前");
                        self.end_peek_now();
                        self.relayout_from_state();
                        // Portals restored with the snapshot need enumerating; stale
                        // runtime state of the replaced ones is pruned on the way.
                        self.refresh_portals();
                        // A snapshot saved with a wallpaper restores it too; a missing file
                        // or a failing shell reports but never blocks the layout restore.
                        if let Some(path) = wallpaper {
                            if !self.apply_static_wallpaper(&path) {
                                tracing::warn!(path, "restoreSnapshot: wallpaper not applied");
                            }
                        }
                        self.schedule_save();
                        self.push_settings_state();
                        self.settings_toast(coredesk_core::i18n::text("已恢复快照"));
                    }
                }
                Some("deleteSnapshot") => {
                    if let Some(id) = v
                        .get("id")
                        .and_then(|i| i.as_str())
                        .and_then(|i| uuid::Uuid::parse_str(i).ok())
                        && self.state.delete_snapshot(id)
                    {
                        self.schedule_save();
                        self.push_settings_state();
                    }
                }
                Some("renameSnapshot") => {
                    if let (Some(id), Some(name)) = (
                        v.get("id")
                            .and_then(|i| i.as_str())
                            .and_then(|i| uuid::Uuid::parse_str(i).ok()),
                        v.get("snapshotName").and_then(|n| n.as_str()),
                    ) {
                        match super::ipc::checked_name("snapshot name", name) {
                            Ok(name) => {
                                if self.state.rename_snapshot(id, &name) {
                                    self.schedule_save();
                                    self.push_settings_state();
                                }
                            }
                            Err(e) => self.settings_error(&e.message),
                        }
                    }
                }
                Some("swapMonitors") => {
                    let a = v
                        .get("a")
                        .and_then(|x| x.as_str())
                        .unwrap_or("")
                        .to_string();
                    let b = v
                        .get("b")
                        .and_then(|x| x.as_str())
                        .unwrap_or("")
                        .to_string();
                    if !a.is_empty() && !b.is_empty() {
                        self.swap_monitors(&a, &b);
                    }
                }
                Some(name @ ("feedbackInfo" | "feedbackResult")) => {
                    self.on_feedback_action(name, &v);
                }
                Some("pageError") => log_page_error(&v),
                Some(name @ ("checkUpdate" | "installUpdate" | "openReleasePage")) => {
                    self.on_update_action(name);
                }
                Some("setPeekHotkey") => {
                    let number = |key: &str| v.get(key).and_then(|n| n.as_u64()).unwrap_or(0);
                    self.set_peek_hotkey_from_page(number("mods") as u32, number("vk") as u32);
                }
                Some("peekHotkeyRecording") => {
                    if v.get("on").and_then(|o| o.as_bool()).unwrap_or(false) {
                        self.pause_peek_hotkey();
                    } else {
                        self.resume_peek_hotkey();
                    }
                }
                Some("openFeGlass") => self.open_fe_glass(),
                Some("restartExplorer") => {
                    // Failsafe for a stuck shell (no UI button — refresh is the
                    // daily action). Kill the shell and let Windows' own
                    // respawn (or our detached spawn) bring it back; the video
                    // player's pending-reattach loop rides out the gap.
                    self.settings_toast(coredesk_core::i18n::text("正在重启资源管理器…"));
                    std::thread::spawn(coredesk_platform::tap::restart_explorer);
                }
                Some("repairIcons") => self.set_desktop_icons_hidden(false),
                Some("hideDesktopIcons") => self.set_desktop_icons_hidden(true),
                Some("rateInStore") => self.open_store_rating(),
                Some("openManual") => self.open_manual(),
                Some("openRepository") => {
                    let url = coredesk_core::brand::REPOSITORY_URL;
                    let _ = shell::shell_execute(std::path::Path::new(url), None, None);
                }
                _ => {}
            },
            _ => {}
        }
    }

    /// Replaces the rule set (settings page / CLI). A folder portal never shows `fence.items`:
    /// a rule routing into one would make matched items vanish from every fence, so such
    /// targets fall back to the inbox.
    pub(super) fn set_rules(&mut self, mut rules: coredesk_core::RuleSet) {
        let targets = std::iter::once(&mut rules.default_target)
            .chain(rules.list.iter_mut().map(|r| &mut r.target));
        for t in targets {
            if let Target::Fence(id) = *t
                && self
                    .state
                    .fence(id)
                    .is_some_and(|f| f.kind == FenceKind::FolderPortal)
            {
                *t = Target::Inbox;
            }
        }
        self.state.config.rules = rules;
        self.state.mark_dirty();
        self.schedule_save();
    }

    /// Both rescue buttons and the tray use the same idempotent operation. A second
    /// request must still repair Explorer's state if it changed outside CoreDeskEngine.
    pub(super) fn set_desktop_icons_hidden(&mut self, hidden: bool) {
        if self.apply_desktop_icons_hidden(hidden) {
            self.state.config.settings.hide_real_icons = hidden;
            self.settings_mutated();
            self.desktop_icons_setting_changed();
            self.settings_toast(if hidden {
                coredesk_core::i18n::text("Windows 桌面图标已重新隐藏。")
            } else {
                coredesk_core::i18n::text("桌面图标已显示，可点击“重新隐藏桌面图标”恢复隐藏。")
            });
        } else {
            self.push_settings_state();
            self.settings_error(coredesk_core::i18n::text(
                "桌面图标状态未能更改，请等待资源管理器恢复后重试。",
            ));
        }
    }

    /// The special desktop items (Recycle Bin, ...) enter or leave the inbox fence together
    /// with the hide-real-icons setting: resync now rather than at the next folder event.
    pub(super) fn desktop_icons_setting_changed(&mut self) {
        self.sync_desktop_if_available("hide-real-icons toggled");
        self.refresh_all();
        self.push_workspace_summary();
    }

    fn apply_desktop_icons_hidden(&self, hidden: bool) -> bool {
        let mut guard = self.anchor.borrow_mut();
        let Some(a) = guard.as_mut() else {
            return false;
        };
        if hidden {
            return a.hide_desktop_icons();
        }
        a.restore_desktop_icons();
        if coredesk_platform::shell_icons::desktop_icons_hidden()
            && let Some(h) = desktop::resolve_icon_host(a.generation)
        {
            let _ =
                coredesk_platform::shell_icons::set_desktop_icons_hidden(false, h.def_view, h.host);
            // Clear our recovery marker only after Explorer actually restored the icons.
            a.restore_desktop_icons();
        }
        !coredesk_platform::shell_icons::desktop_icons_hidden()
    }

    /// Applies a full settings object from the page, reacting to what changed.
    pub(super) fn apply_settings(&mut self, mut new: coredesk_core::Settings) {
        let old = self.state.config.settings.clone();
        if new == old {
            return;
        }
        // Hand-edits to the sidecar files made while the app runs: entries
        // the app never touched since the recorded baseline are rebuilt
        // from disk (a revert-to-baseline counts), so a modder's tweak —
        // or a manual reset — applies on the next apply gesture without a
        // restart; merge-on-save then keeps it on disk instead of
        // stomping it.
        merge_sidecar_deltas(&mut new, &self.state.store);
        // Layered widget dictionary: a style switch first mirrors the
        // outgoing design's live placement into its dict slot (per-design
        // persistence), then the incoming design's entry resolves onto the
        // runtime fields so the bounds/adaptive diffs below see user-layer
        // values. A same-style dict change (import/theme) resolves without
        // the mirror — the file's entry wins, not the live state.
        if new.clock_style != old.clock_style && coredesk_core::is_clock_widget_id(&old.clock_style)
        {
            let mut mirrored = coredesk_core::WidgetParams::from_runtime(&old);
            // `from_runtime` has no live element state — keep the incoming
            // dict's authored elements for the outgoing design.
            mirrored.elements = new
                .widget_params
                .get(&old.clock_style)
                .map(|p| p.elements.clone())
                .unwrap_or_default();
            new.widget_params.insert(old.clock_style.clone(), mirrored);
        }
        if new.clock_style != old.clock_style || new.widget_params != old.widget_params {
            new.resolve_widget_params();
        }
        if new.autostart != old.autostart {
            if let Err(e) =
                coredesk_platform::autostart::set_product_enabled(new.autostart, self.portable)
            {
                tracing::warn!(error = %e, "autostart change failed");
                new.autostart = old.autostart;
                self.settings_error(coredesk_core::i18n::text(
                    "未能更改“随 Windows 启动”，请稍后重试。",
                ));
            } else if coredesk_platform::process::is_packaged() {
                // Store installs: Windows owns the startup task, so hand the user the switch.
                // The switch here keeps showing Windows' state; it is read again when the user
                // comes back to this window.
                new.autostart = old.autostart;
                let _ = shell::shell_execute(
                    std::path::Path::new("ms-settings:startupapps"),
                    None,
                    None,
                );
            }
        }
        if new.hide_real_icons != old.hide_real_icons
            && !self.apply_desktop_icons_hidden(new.hide_real_icons)
        {
            new.hide_real_icons = old.hide_real_icons;
            self.settings_error(coredesk_core::i18n::text(
                "桌面图标状态未能更改，请稍后重试。",
            ));
        }
        if new.start_menu_style != old.start_menu_style {
            // The dropdown only offers catalog ids; an unknown id here means a
            // hand-edited patchSettings — same treatment as a failed write.
            let id = new.start_menu_style.as_str();
            let applied = coredesk_core::start_menu_themes::is_style_id(id)
                && coredesk_platform::start_menu::apply_start_menu_theme(id);
            if !applied {
                new.start_menu_style.clone_from(&old.start_menu_style);
                self.settings_error(coredesk_core::i18n::text(
                    "开始菜单样式未能更改，请稍后重试。",
                ));
            }
        }
        if new.taskbar_style != old.taskbar_style {
            let id = new.taskbar_style.as_str();
            let applied = coredesk_core::taskbar_themes::is_style_id(id)
                && coredesk_platform::taskbar_styler::apply(id);
            if !applied {
                new.taskbar_style.clone_from(&old.taskbar_style);
                self.settings_error(coredesk_core::i18n::text(
                    "任务栏样式未能更改，请稍后重试。",
                ));
            }
        }
        if new.notification_center_style != old.notification_center_style {
            let id = new.notification_center_style.as_str();
            let applied = coredesk_core::notification_center_themes::is_style_id(id)
                && coredesk_platform::notification_center::apply(id);
            if !applied {
                new.notification_center_style
                    .clone_from(&old.notification_center_style);
                self.settings_error(coredesk_core::i18n::text(
                    "通知中心样式未能更改，请稍后重试。",
                ));
            }
        }
        // Themes carry the fe_* Explorer-material recipe now: the archive's
        // values land here and this diff applies them. Archives packed by
        // older builds carry the stripped defaults (`fe_effect=""`), which
        // correctly read as "Windows Default" and disarm to stock.
        let fe_changed = new.fe_effect != old.fe_effect
            || new.fe_clear_address != old.fe_clear_address
            || new.fe_clear_bar_bg != old.fe_clear_bar_bg
            || new.fe_clear_winui_bg != old.fe_clear_winui_bg
            || new.fe_show_line != old.fe_show_line
            || new.fe_light_rgba != old.fe_light_rgba
            || new.fe_dark_rgba != old.fe_dark_rgba;
        if fe_changed {
            let applied = coredesk_platform::file_explorer_styler::apply_effect(
                &coredesk_platform::file_explorer_styler::FeGlassConfig {
                    effect: new.fe_effect.as_str(),
                    clear_address: new.fe_clear_address,
                    clear_bar_bg: new.fe_clear_bar_bg,
                    clear_winui_bg: new.fe_clear_winui_bg,
                    show_line: new.fe_show_line,
                    light_rgba: new.fe_light_rgba.as_str(),
                    dark_rgba: new.fe_dark_rgba.as_str(),
                },
            );
            // The same material now mirrors onto our own windows — feed the
            // incoming values: `self.state.config.settings` still holds the
            // old material until the diff lands below.
            let rgba = if systheme::apps_use_light_theme() {
                &new.fe_light_rgba
            } else {
                &new.fe_dark_rgba
            };
            self.apply_glass(&new.fe_effect, rgba);
            if !applied {
                // Deploy/config.ini failure — revert so the page shows
                // what is actually running.
                new.fe_effect.clone_from(&old.fe_effect);
                self.settings_error(coredesk_core::i18n::text(
                    "资源管理器背景效果未能更改，请稍后重试。",
                ));
            }
        }
        // Clock bounds ride `.cde` themes: adopt them before the style diff
        // so a fresh `ClockHost::open` spawns at the themed spot (state still
        // holds `old` during this diff — `apply_clock_style` reads it).
        self.state.config.settings.clock_bounds = new.clock_bounds;
        // Same early-sync for the widget dictionary surface: `apply_clock_style`
        // resolves element params via `active_widget_params()`, and the
        // probe/apply paths read `clock_adaptive_color` off state.
        self.state.config.settings.clock_style = new.clock_style.clone();
        self.state.config.settings.clock_adaptive_color = new.clock_adaptive_color;
        self.state.config.settings.clock_use_24_hour = new.clock_use_24_hour;
        self.state.config.settings.widget_params = new.widget_params.clone();
        if new.clock_style != old.clock_style {
            self.apply_clock_style(&new.clock_style);
        }
        if new.clock_bounds != old.clock_bounds {
            // A live widget is moved in place; a theme without bounds returns
            // it to the default centred spot. Closed widgets pick bounds up
            // on their next open via the state write above.
            if let Some(h) = &self.clock {
                let (x, y, w, hh) = new
                    .clock_bounds
                    .map(|[x, y, w, h]| (x, y, w, h))
                    .unwrap_or_else(crate::clock_host::default_bounds);
                let _ = window::set_window_bounds(h.hwnd(), x, y, w, hh);
                self.probe_clock_backdrop();
            }
        }
        if new.desktop_effect != old.desktop_effect
            || new.desktop_effect_intensity != old.desktop_effect_intensity
            || new.effect_params != old.effect_params
        {
            self.apply_desktop_effect(&new);
        }
        if new.clock_adaptive_color != old.clock_adaptive_color {
            if new.clock_adaptive_color {
                self.probe_clock_backdrop();
            } else if let Some(h) = &self.clock {
                // Adaptive off → manual element colors take over; the page
                // also falls back to its dark palette.
                h.post_json(&clock_theme_json(&new, None));
            }
        }
        // Global 12/24-hour toggle: same params-only push — the page
        // re-formats the readout in place (AM/PM nodes re-shown/hidden).
        if new.clock_use_24_hour != old.clock_use_24_hour
            && let Some(h) = &self.clock
        {
            h.post_json(&clock_theme_json(&new, None));
        }
        // Same-style dictionary edit (import / theme apply): push the
        // resolved element params. A style switch already carries them in
        // the `clockStyle` post above.
        if new.widget_params != old.widget_params && new.clock_style == old.clock_style {
            if let Some(h) = &self.clock {
                h.post_json(&clock_theme_json(&new, None));
            }
        }
        let icons_toggled = new.hide_real_icons != old.hide_real_icons;
        if new.icons.shortcut_arrows != old.icons.shortcut_arrows {
            // The setting covers the real desktop too (its description says
            // so): show → drop the `Shell Icons\29` override; hide → write
            // Microsoft's transparent stock icon.
            let shown = new.icons.shortcut_arrows;
            std::thread::spawn(move || {
                coredesk_platform::shell::set_link_overlay_shown(shown);
            });
        }
        if new.quick_hide.enabled != old.quick_hide.enabled
            && let Some(a) = self.anchor.borrow_mut().as_mut()
        {
            a.quick_hide_enabled = new.quick_hide.enabled;
            // Switched off while the fences are hidden: a double-click can no longer show them.
            if !a.quick_hide_enabled && a.fences_hidden() {
                a.toggle_fences_hidden();
            }
        }
        if new.show_desktop != old.show_desktop
            && let Some(a) = self.anchor.borrow_mut().as_mut()
        {
            a.behavior = match new.show_desktop {
                ShowDesktopSetting::KeepVisible => ShowDesktopBehavior::KeepVisible,
                ShowDesktopSetting::HideWithDesktop => ShowDesktopBehavior::HideWithDesktop,
            };
        }
        if new.zorder != old.zorder
            && let Some(a) = self.anchor.borrow_mut().as_mut()
        {
            a.mode = match new.zorder {
                ZOrderSetting::InsertAboveHost => ZMode::InsertAboveHost,
                ZOrderSetting::HwndBottom => ZMode::HwndBottom,
            };
            a.reanchor("zorder changed");
        }
        self.ctx.behavior.hover_peek.set(new.roll_up.hover_peek);
        self.ctx.behavior.snapping.set(new.snapping.enabled);
        self.ctx
            .behavior
            .snap_gap_dip
            .set(snap_gap_dip(&new.snapping));
        self.ctx
            .behavior
            .size_to_cells
            .set(new.snapping.size_to_cells);
        self.ctx.behavior.guide_lines.set(new.snapping.guide_lines);
        self.ctx
            .behavior
            .click_to_expand
            .set(new.roll_up.click_to_expand);
        self.ctx
            .behavior
            .title_on_hover
            .set(new.roll_up.title_on_hover);
        self.ctx.behavior.title_align.set(new.title_align);
        self.ctx.behavior.tab_bar.set(new.tab_bar);
        self.ctx.behavior.label_color.set(new.label_color);
        self.ctx
            .behavior
            .hide_inactive_scrollbar
            .set(new.roll_up.hide_inactive_scrollbar);
        self.ctx
            .behavior
            .switch_tab_on_hover
            .set(new.roll_up.switch_tab_on_hover);
        self.ctx
            .behavior
            .hide_count_and_chevron
            .set(new.roll_up.hide_count_and_chevron);
        // Only whole rows show with 「调整大小时按整行整列对齐」 on: redraw.
        let chrome_changed = new.roll_up.title_on_hover != old.roll_up.title_on_hover
            || new.title_align != old.title_align
            || new.label_color != old.label_color
            || new.roll_up.hide_inactive_scrollbar != old.roll_up.hide_inactive_scrollbar
            || new.roll_up.hide_count_and_chevron != old.roll_up.hide_count_and_chevron
            || new.snapping.size_to_cells != old.snapping.size_to_cells;
        if chrome_changed {
            for w in self.fences.values() {
                w.redraw();
            }
        }
        if new.tab_bar != old.tab_bar {
            // The tab column takes (or gives back) width beside the items, and an auto-height
            // fence grows to show every tab in it.
            let hosts: Vec<FenceId> = self.fences.keys().copied().collect();
            for host in hosts {
                if let Some(w) = self.fences.get(&host) {
                    w.sync_tab_bar();
                }
                self.apply_auto_height(host);
            }
        }
        self.ctx
            .behavior
            .backdrop
            .set(backdrop_mode_for(new.backdrop));
        let visual_changed = new.theme != old.theme
            || new.theme_style != old.theme_style
            || new.backdrop != old.backdrop;
        let icons_changed = new.icons != old.icons;
        let push_changed = new.roll_up.push_neighbors != old.roll_up.push_neighbors;
        let inbox_hiding_changed =
            new.hide_inbox_when_icons_shown != old.hide_inbox_when_icons_shown;
        let fit_changed = new.contain_background != old.contain_background;
        // 「调整大小时按整行整列对齐」 switched on keeps every rectangle: the rows fill any
        // height with whole icons (redrawn above), and rounding would undo fences lined up.
        self.state.config.settings = new;
        self.refresh_language();
        self.state.mark_dirty();
        self.schedule_save();
        self.sync_peek_hotkey();
        if icons_toggled {
            self.desktop_icons_setting_changed();
        }
        if icons_changed {
            self.apply_icon_variant();
        }
        if visual_changed {
            self.refresh_visuals(true);
        }
        if push_changed {
            // Pushed fences go back at once when it is switched off.
            self.reflow_pushed();
        }
        if inbox_hiding_changed {
            self.sync_inbox_window();
        }
        if fit_changed {
            // Canvas re-anchoring and image refit run here; the video player's
            // cover/contain restart rides the sync right below.
            self.retune_background_fit();
        }
        self.sync_video_wallpaper();
        self.push_settings_state();
    }

    /// Starts or stops the WMF desktop video wallpaper player to match
    /// `settings.video_wallpaper_path`. The swap runs on a worker: `stop`'s join
    /// and `start`'s first-frame wait are seconds-class calls the UI thread
    /// never pays for; `poll_video_swap` installs the result when it lands.
    pub(super) fn sync_video_wallpaper(&mut self) {
        self.poll_video_transcode();
        self.poll_video_swap();
        // A pick's transcode is still running — nothing plays until it lands
        // and `video_wallpaper_path` flips to the optimized file.
        if self.video_transcode.is_some() {
            return;
        }
        let wanted = self
            .state
            .config
            .settings
            .video_wallpaper_path
            .clone()
            .filter(|s| !s.is_empty());
        let Some(path_str) = wanted else {
            self.invalidate_video_swap();
            if let Some(p) = self.wmf_player.take() {
                reap_video_player(p);
            }
            self.static_under_video = None;
            return;
        };
        let path = PathBuf::from(&path_str);
        let letterbox = self.contain_background();
        if self
            .video_swap
            .as_ref()
            .is_some_and(|s| s.path == path && s.letterbox == letterbox)
        {
            return; // the wanted target is already in flight — keep it
        }
        if self
            .wmf_player
            .as_ref()
            .is_some_and(|p| p.path() == path && p.is_alive() && p.letterbox() == letterbox)
        {
            return;
        }
        let old = self.wmf_player.take();
        self.static_under_video = None;
        if !path.is_file() {
            self.invalidate_video_swap();
            if let Some(p) = old {
                reap_video_player(p);
            }
            return;
        }
        let epoch = VIDEO_SWAP_EPOCH.fetch_add(1, Ordering::SeqCst) + 1;
        self.video_swap = Some(VideoSwap {
            epoch,
            path: path.clone(),
            letterbox,
            reattach: false,
            toast: false,
            rx: spawn_video_swap(old, path, letterbox, epoch),
        });
    }

    /// Wallpaper intent changed again — an in-flight swap's result is now stale:
    /// the worker sees the newer epoch and disposes its player itself.
    fn invalidate_video_swap(&mut self) {
        VIDEO_SWAP_EPOCH.fetch_add(1, Ordering::SeqCst);
        self.video_swap = None;
        self.video_transcode = None;
        self.pending_video_toast = false;
        self.video_start_failures = 0;
    }

    /// Installs a finished swap when it still matches the config. Runs off the
    /// `check_wallpaper` poll — a few hundred ms after `sync_video_wallpaper`
    /// returned instantly instead of ~200 ms of teardown+start on the UI thread.
    pub(super) fn poll_video_swap(&mut self) {
        let Some(swap) = &self.video_swap else {
            return;
        };
        match swap.rx.try_recv() {
            Ok(Ok(player)) => {
                let swap = self.video_swap.take().unwrap();
                let still_wanted = self
                    .state
                    .config
                    .settings
                    .video_wallpaper_path
                    .as_deref()
                    .map(PathBuf::from)
                    .as_deref()
                    == Some(player.path())
                    && player.letterbox() == self.contain_background();
                if still_wanted && VIDEO_SWAP_EPOCH.load(Ordering::SeqCst) == swap.epoch {
                    self.static_under_video =
                        Self::static_wallpaper_now().map(|(path, mtime)| StaticBaseline {
                            path,
                            mtime,
                            recorded_at: std::time::SystemTime::now(),
                            recorded_instant: std::time::Instant::now(),
                        });
                    self.video_start_failures = 0;
                    self.wmf_player = Some(player);
                    self.capture_live_wallpapers(Duration::ZERO);
                    if swap.reattach {
                        tracing::info!("wmf video wallpaper re-attached after desktop restart");
                    }
                    if swap.toast || std::mem::take(&mut self.pending_video_toast) {
                        self.settings_toast(coredesk_core::i18n::text("已将视频设为桌面壁纸"));
                    }
                } else {
                    reap_video_player(player);
                }
            }
            Ok(Err(Some(error))) => {
                let toast = self.video_swap.take().is_some_and(|s| s.toast);
                // Only failures while the shell is up count toward the
                // auto-retry cap — a missing desktop layer at cold boot is
                // transient and `ensure_video_wallpaper` keeps retrying it.
                if !coredesk_platform::desktop::shell_window().0.is_null() {
                    self.video_start_failures = self.video_start_failures.saturating_add(1);
                }
                tracing::error!(
                    %error,
                    failures = self.video_start_failures,
                    "WMF video wallpaper failed to start"
                );
                // A user pick surfaces the reason; boot/re-attach retries
                // stay quiet — the next poll may heal them.
                if toast {
                    self.settings_error(&coredesk_core::i18n::format(
                        "无法使用此文件：{0}",
                        std::slice::from_ref(&error),
                    ));
                }
            }
            Ok(Err(None)) => {
                self.video_swap = None; // superseded by a newer request
            }
            Err(std::sync::mpsc::TryRecvError::Empty) => {}
            Err(std::sync::mpsc::TryRecvError::Disconnected) => {
                self.video_swap = None;
            }
        }
    }

    /// Installs a finished transcode: flips `video_wallpaper_path` to the
    /// optimized file when the pick it was made for is still the configured
    /// one. `Ok(None)` (source already lean) and `Err` both leave the path on
    /// the source — the swap pipeline plays it untouched either way, so the
    /// toast rides along through `pending_video_toast`.
    pub(super) fn poll_video_transcode(&mut self) {
        let Some(t) = &self.video_transcode else {
            return;
        };
        match t.rx.try_recv() {
            Ok(res) => {
                let t = self.video_transcode.take().unwrap();
                let fresh = VIDEO_SWAP_EPOCH.load(Ordering::SeqCst) == t.epoch
                    && self
                        .state
                        .config
                        .settings
                        .video_wallpaper_path
                        .as_deref()
                        .map(PathBuf::from)
                        == Some(t.src.clone());
                match res {
                    Ok(Some(optimized)) if fresh => {
                        self.state.config.settings.video_wallpaper_path =
                            Some(optimized.to_string_lossy().into_owned());
                        self.settings_mutated();
                        if t.toast {
                            self.pending_video_toast = true;
                        }
                    }
                    Ok(Some(_)) => {} // superseded pick — named cache file stays for reuse
                    Ok(None) => {
                        if t.toast {
                            self.pending_video_toast = true;
                        }
                    }
                    Err(error) => {
                        tracing::warn!(%error, "video transcode failed; playing source as-is");
                        if fresh && t.toast {
                            self.pending_video_toast = true;
                        }
                    }
                }
            }
            Err(std::sync::mpsc::TryRecvError::Empty) => {}
            Err(std::sync::mpsc::TryRecvError::Disconnected) => {
                self.video_transcode = None;
            }
        }
    }

    /// Self-heal for the desktop video wallpaper: an Explorer restart destroys the
    /// WorkerW/Progman layer the video window is parented to, so the player thread
    /// exits while `video_wallpaper_path` still names the file. When the shell is
    /// back up, `WmfWallpaperPlayer::start` re-runs the `0x052C` WorkerW spawn and
    /// re-attaches — driven from the periodic `check_wallpaper` so any cause of
    /// Explorer loss (our Restart Explorer failsafe, a crash, a user taskkill)
    /// recovers without a settings round-trip. Failures stay quiet: a shell that
    /// is mid-boot retries on the next poll.
    pub(super) fn ensure_video_wallpaper(&mut self) {
        self.poll_video_transcode();
        self.poll_video_swap();
        // Transcode in flight — the player starts once the path flips.
        if self.video_transcode.is_some() {
            return;
        }
        let Some(path_str) = self
            .state
            .config
            .settings
            .video_wallpaper_path
            .clone()
            .filter(|s| !s.is_empty())
        else {
            return;
        };
        // A swap for this exact target is already in flight — keep it; a swap
        // for a different target gets superseded by the spawn below.
        let path = PathBuf::from(&path_str);
        let letterbox = self.contain_background();
        if self
            .video_swap
            .as_ref()
            .is_some_and(|s| s.path == path && s.letterbox == letterbox)
        {
            return;
        }
        if self.wmf_player.as_ref().is_some_and(|p| p.is_alive()) {
            return;
        }
        // Shell down (GetShellWindow is a live query) — attach would just fail;
        // the next poll retries once Progman exists again.
        if coredesk_platform::desktop::shell_window().0.is_null() {
            return;
        }
        // Consecutive shell-up failures (unreadable file, dead pipeline) give
        // up quietly until wallpaper intent changes — every attempt costs a
        // worker spawn and a first-frame decode.
        if self.video_start_failures >= MAX_VIDEO_START_FAILURES {
            return;
        }
        let old = self.wmf_player.take();
        self.static_under_video = None;
        if !path.is_file() {
            if let Some(p) = old {
                reap_video_player(p);
            }
            return;
        }
        let epoch = VIDEO_SWAP_EPOCH.fetch_add(1, Ordering::SeqCst) + 1;
        self.video_swap = Some(VideoSwap {
            epoch,
            path: path.clone(),
            letterbox,
            reattach: true,
            toast: false,
            rx: spawn_video_swap(old, path, letterbox, epoch),
        });
    }

    /// Applies a static image as the desktop wallpaper and retires the video engine:
    /// choosing a static background must stop a still-playing video and clear its stored
    /// path so exports and restarts don't resurrect it. Returns `set_desktop_wallpaper`'s
    /// result.
    pub(super) fn apply_static_wallpaper(&mut self, path: &str) -> bool {
        self.invalidate_video_swap();
        if let Some(p) = self.wmf_player.take() {
            reap_video_player(p);
        }
        self.static_under_video = None;
        if self
            .state
            .config
            .settings
            .video_wallpaper_path
            .take()
            .is_some()
        {
            self.settings_mutated();
        }
        coredesk_platform::sysparams::set_desktop_wallpaper(path)
    }

    /// `WM_SETTINGCHANGE`/`SPI_SETDESKWALLPAPER`: a static wallpaper was applied — by us
    /// (`apply_static_wallpaper` already cleared state, so the guard makes it a no-op) or
    /// outside the app (Settings > Personalization, Explorer's "Set as desktop background").
    /// An outside change means wallpaper intent moved to the image, so a running or
    /// configured video yields — otherwise it keeps covering the image and exports keep
    /// bundling it.
    /// `SPI_SETDESKWALLPAPER` broadcast — a static image took the desktop.
    /// The video yields only when the reported image genuinely differs from
    /// the baseline recorded at player start: a shell restart can re-assert
    /// the same wallpaper, and clearing `video_wallpaper_path` on that
    /// transient made restart kill the video permanently. With no baseline
    /// recorded the broadcast itself is the evidence (a real apply).
    pub(super) fn on_wallpaper_changed(&mut self) {
        if self.wmf_player.is_none() && self.state.config.settings.video_wallpaper_path.is_none() {
            return;
        }
        let now = Self::static_wallpaper_now();
        let differs = match (self.static_under_video.as_ref(), now.as_ref()) {
            (Some(b), Some((p, m))) => (b.path.as_path(), b.mtime) != (p.as_path(), *m),
            _ => true,
        };
        if !differs {
            tracing::debug!(
                wallpaper = ?now,
                "wallpaper signal re-asserted the same image; keeping video intent"
            );
            return;
        }
        if self.is_shell_reassert(&now) {
            tracing::debug!(
                wallpaper = ?now,
                "wallpaper signal inside shell-rebuild window; keeping video"
            );
            return;
        }
        if let Some(b) = self.static_under_video.as_ref() {
            // The reported file predates the player's baseline — Explorer
            // re-asserted a pre-existing transcode cache, not a user apply
            // (a real apply writes a fresh `Transcoded_*` file).
            if now
                .as_ref()
                .and_then(|(_, m)| *m)
                .is_some_and(|m| m <= b.recorded_at)
            {
                tracing::debug!(
                    wallpaper = ?now,
                    "wallpaper signal predates video baseline; keeping video intent"
                );
                return;
            }
            // Inside the post-install window a fresh transcode from a
            // concurrent shell rebuild is indistinguishable from a user
            // apply — keep the video; a real apply re-arrives later.
            if b.recorded_instant.elapsed() < VIDEO_RETIRE_GRACE {
                tracing::debug!(
                    wallpaper = ?now,
                    "wallpaper signal inside post-install grace; keeping video intent"
                );
                return;
            }
        }
        if coredesk_platform::wallpaper::slideshow_active() {
            // A running Windows slideshow broadcasts SETDESKWALLPAPER on every
            // rotation — a new path/mtime that is not a user apply. Picking a
            // single picture exits slideshow mode, so real applies still land.
            tracing::debug!(
                wallpaper = ?now,
                "wallpaper signal differs but a slideshow is running; keeping video"
            );
            return;
        }
        if coredesk_platform::wallpaper::desktops_differ() {
            tracing::debug!(
                applied = ?now,
                "wallpaper signal differs but virtual desktops differ; keeping video"
            );
            return;
        }
        tracing::info!(
            player_alive = self.wmf_player.as_ref().is_some_and(|p| p.is_alive()),
            configured = ?self.state.config.settings.video_wallpaper_path,
            "external wallpaper change; retiring video wallpaper"
        );
        self.invalidate_video_swap();
        if let Some(p) = self.wmf_player.take() {
            reap_video_player(p);
        }
        self.static_under_video = None;
        self.state.config.settings.video_wallpaper_path = None;
        // `settings_mutated` — not bare `schedule_save` — or the cleared path never
        // reaches disk (`save_if_dirty` writes only flagged state) and the next
        // launch resurrects the video.
        self.settings_mutated();
    }

    /// Inside the shell-rebuild window after `TaskbarCreated`, a wallpaper
    /// signal resolving to a file that already existed at respawn time is
    /// the shell re-asserting the desktop — not a user apply. Only a file
    /// written after the respawn counts as a genuine new change. `None`
    /// mtime / unresolvable paths err on keeping the video.
    fn is_shell_reassert(&self, now: &Option<(PathBuf, Option<std::time::SystemTime>)>) -> bool {
        let Some(respawn) = self.shell_respawn_wall else {
            return false;
        };
        let Ok(since) = respawn.elapsed() else {
            return false;
        };
        if since > Duration::from_secs(60) {
            return false;
        }
        match now {
            Some((_, Some(mtime))) => mtime.duration_since(respawn).is_err(),
            _ => true,
        }
    }

    /// The static wallpaper `(path, file mtime)` as `IDesktopWallpaper` resolves it —
    /// the same provider `follow_snapshot` compares `static_under_video` against, so a
    /// SPI↔IDW divergence (`TranscodedWallpaper` vs the source path) cannot look like a
    /// user apply. The mtime tells a shell re-assert (pre-existing file) from a user pick.
    fn static_wallpaper_now() -> Option<(PathBuf, Option<std::time::SystemTime>)> {
        coredesk_platform::wallpaper::query()
            .ok()
            .and_then(|q| q.monitors.into_iter().find_map(|m| m.path))
            .filter(|p| p.is_file())
            .or_else(|| {
                coredesk_platform::sysparams::desktop_wallpaper()
                    .map(PathBuf::from)
                    .filter(|p| p.is_file())
            })
            .map(|p| {
                let mtime = std::fs::metadata(&p).and_then(|m| m.modified()).ok();
                (p, mtime)
            })
    }

    /// A static image applied over a running video retires it — for applies the
    /// `SPI_SETDESKWALLPAPER` broadcast misses (Explorer's "Set as desktop background"
    /// goes through `IDesktopWallpaper`). Detected by the reported static path differing
    /// from `static_under_video`, recorded when the player started. `desktops_differ`
    /// guards virtual-desktop switches, which legitimately report another image;
    /// `is_shell_reassert` covers the transient inside a shell-rebuild window, and the
    /// baseline's `recorded_at`/grace window covers shell rebuilds that overlap our own
    /// launch (a `Transcoded_*` rename or a fresh cache write at cold boot).
    pub(super) fn retire_video_for_static(&mut self, static_path: Option<&Path>) {
        let player_alive = self.wmf_player.as_ref().is_some_and(|p| p.is_alive());
        let (Some(baseline), Some(path)) = (self.static_under_video.as_ref(), static_path) else {
            return;
        };
        let now = (
            path.to_path_buf(),
            std::fs::metadata(path).and_then(|m| m.modified()).ok(),
        );
        if !player_alive || (now.0 == baseline.path && now.1 == baseline.mtime) {
            return;
        }
        if self.is_shell_reassert(&Some(now.clone())) {
            tracing::debug!(
                applied = %now.0.display(),
                "static wallpaper differs but inside shell-rebuild window; keeping video"
            );
            return;
        }
        // The reported file predates the player's baseline — Explorer
        // re-pointed IDesktopWallpaper at a pre-existing `Transcoded_*`
        // cache (its per-monitor naming differs from the SPI path's).
        if now.1.is_some_and(|m| m <= baseline.recorded_at) {
            tracing::debug!(
                applied = %now.0.display(),
                "static wallpaper report predates video baseline; keeping video"
            );
            return;
        }
        // Inside the post-install window a fresh transcode from a
        // concurrent shell rebuild is indistinguishable from a user apply.
        if baseline.recorded_instant.elapsed() < VIDEO_RETIRE_GRACE {
            tracing::debug!(
                applied = %now.0.display(),
                "static wallpaper changed inside post-install grace; keeping video"
            );
            return;
        }
        if coredesk_platform::wallpaper::slideshow_active() {
            // Slideshow rotation, not a user apply — see `on_wallpaper_changed`.
            tracing::debug!(
                applied = %now.0.display(),
                "static wallpaper differs but a slideshow is running; keeping video"
            );
            return;
        }
        if coredesk_platform::wallpaper::desktops_differ() {
            tracing::debug!(
                applied = %now.0.display(),
                "static wallpaper changed but virtual desktops differ; keeping video"
            );
            return;
        }
        tracing::info!(
            applied = %now.0.display(),
            was = %baseline.path.display(),
            video = %self.wmf_player.as_ref().map(|p| p.path().display().to_string()).unwrap_or_default(),
            "static image applied over video wallpaper; retiring player"
        );
        self.invalidate_video_swap();
        if let Some(p) = self.wmf_player.take() {
            reap_video_player(p);
        }
        self.static_under_video = None;
        self.state.config.settings.video_wallpaper_path = None;
        self.settings_mutated();
    }

    pub(super) fn pick_video_wallpaper(&mut self) {
        let owner = self.settings.as_ref().map(|h| h.hwnd());
        match coredesk_platform::filedialog::open_mp4(
            owner,
            coredesk_core::i18n::text("将视频设为桌面壁纸"),
        ) {
            Ok(Some(path)) => self.apply_video_wallpaper(&path),
            Ok(None) => {}
            Err(e) => self.settings_error(&coredesk_core::i18n::format(
                "无法打开文件对话框：{0}",
                &[e.to_string()],
            )),
        }
    }

    pub(super) fn apply_video_wallpaper(&mut self, path: &Path) {
        tracing::info!(path = %path.display(), "applying video wallpaper from settings");
        let letterbox = self.contain_background();
        // The same pick is already in flight (e.g. a theme apply racing a user
        // pick) — keep it and make sure its completion toasts.
        if let Some(t) = self.video_transcode.as_mut().filter(|t| t.src == path) {
            t.toast = true;
            return;
        }
        if let Some(s) = self
            .video_swap
            .as_mut()
            .filter(|s| s.path == path && s.letterbox == letterbox)
        {
            s.toast = true;
            return;
        }
        self.invalidate_video_swap();
        if let Some(p) = self.wmf_player.take() {
            reap_video_player(p);
        }
        self.static_under_video = None;
        // The config holds the original until the optimized file lands —
        // `poll_video_transcode` flips it, and the swap pipeline starts the
        // player from there. `.cde` exports therefore always bundle the
        // downscaled asset, never the 4K source.
        self.state.config.settings.video_wallpaper_path = Some(path.to_string_lossy().into_owned());
        self.settings_mutated();
        let epoch = VIDEO_SWAP_EPOCH.fetch_add(1, Ordering::SeqCst) + 1;
        let Some(dir) = self
            .state
            .config_path()
            .parent()
            .map(|p| p.join("imported_wallpapers"))
        else {
            // No cache dir to hold an optimized asset — play the source as before.
            self.video_swap = Some(VideoSwap {
                epoch,
                path: path.to_path_buf(),
                letterbox,
                reattach: false,
                toast: true,
                rx: spawn_video_swap(None, path.to_path_buf(), letterbox, epoch),
            });
            return;
        };
        // The largest connected display's physical pixels — transcoding to
        // any bigger frame would only inflate the bundle.
        let (tw, th) = self
            .state
            .work_areas
            .iter()
            .map(|a| a.screen_px())
            .map(|[w, h]| (w.max(0) as u32, h.max(0) as u32))
            .max_by_key(|(w, h)| w * h)
            .unwrap_or((1920, 1080));
        self.video_transcode = Some(VideoTranscode {
            epoch,
            src: path.to_path_buf(),
            toast: true,
            rx: spawn_video_transcode(path.to_path_buf(), dir, (tw, th)),
        });
    }

    pub(super) fn export_theme(&mut self) {
        let owner = self.settings.as_ref().map(|h| h.hwnd());
        let name = format!(
            "coredesk-theme-{}.cde",
            coredesk_core::config_store::today_yyyy_mm_dd()
        );
        match coredesk_platform::filedialog::save_cde(
            owner,
            coredesk_core::i18n::text("导出为 .cde 主题"),
            &name,
        ) {
            Ok(Some(path)) => self.export_theme_to(&path),
            Ok(None) => {}
            Err(e) => self.settings_error(&coredesk_core::i18n::format(
                "无法打开保存对话框：{0}",
                &[e.to_string()],
            )),
        }
    }

    /// The active desktop background media: the playing WMF video, else the desktop
    /// wallpaper image — only paths that still exist. `video_wallpaper_path` keeps the
    /// last-used path even after playback stopped or a static wallpaper took over, so the
    /// live player — not the config string — decides.
    fn current_background_source(&mut self) -> Option<PathBuf> {
        let spi = coredesk_platform::sysparams::desktop_wallpaper()
            .map(PathBuf::from)
            .filter(|p| p.is_file());
        let idw = spi.is_none().then(|| {
            coredesk_platform::wallpaper::query()
                .ok()
                .and_then(|q| q.monitors.into_iter().find_map(|m| m.path))
                .filter(|p| p.is_file())
        });
        let static_src = spi.clone().or_else(|| idw.clone().flatten());
        // A static image applied after the video started overrides it — including
        // Explorer applies that broadcast nothing. The comparison uses the same
        // IDW-resolved provider as the recorded baseline (`static_wallpaper_now`).
        self.retire_video_for_static(
            Self::static_wallpaper_now()
                .as_ref()
                .map(|(p, _)| p.as_path()),
        );
        let player = self.wmf_player.as_ref();
        let player_alive = player.is_some_and(|p| p.is_alive());
        let player_path = player.map(|p| p.path().to_path_buf());
        let player_file = player_path.as_ref().is_some_and(|p| p.is_file());
        // A swap in flight IS the video background — the player handle is None
        // for its whole duration, so callers deciding "is the background the
        // video?" (retune, theme pack) must see the pending target, not SPI's
        // static fallback. Missing this made a mid-swap retune retire the video.
        let swap_path = self
            .video_swap
            .as_ref()
            .map(|s| s.path.clone())
            .filter(|p| p.is_file());
        let chosen = if player_alive && player_file {
            player_path.clone()
        } else {
            swap_path.clone().or(static_src.clone())
        };
        tracing::info!(
            player_present = player.is_some(),
            player_alive,
            player_path = ?player_path,
            player_file,
            swap_pending = ?swap_path,
            spi_wallpaper = ?spi,
            idw_wallpaper = ?idw.as_ref().map(|o| o.as_ref().map(|p| p.display().to_string())),
            static_under_video = ?self.static_under_video,
            configured_video = ?self.state.config.settings.video_wallpaper_path,
            chosen = ?chosen,
            "background source evaluated"
        );
        chosen
    }

    /// Serializes `cfg` and builds a `.cde` payload: an optional `background.<ext>` entry
    /// copied from `bg_source` plus a 640x360 `preview.jpg` of the layout rendered over a
    /// frame of that background (`preview_frame` wins when the caller has live pixels).
    /// The returned [`SplitConfigJson`] carries `config.json` minus the modder keys plus
    /// the `Widget_config.json`/`Live_Effects_config.json` sidecar documents — all three
    /// are packed into the archive.
    fn cde_payload(
        &self,
        cfg: &coredesk_core::Config,
        bg_source: Option<&Path>,
        preview_frame: Option<(u32, u32, Vec<u8>)>,
    ) -> std::result::Result<
        (
            coredesk_core::config_store::SplitConfigJson,
            Option<(String, Vec<u8>)>,
            Vec<u8>,
        ),
        String,
    > {
        let mut cfg = cfg.clone();
        cfg.make_theme_portable();
        // A bundle whose background is not the video must not resurrect a stale
        // `video_wallpaper_path` on import — it only survives when a video is embedded.
        let video_bg = bg_source
            .and_then(|p| p.extension())
            .is_some_and(|e| e.eq_ignore_ascii_case("mp4"));
        if !video_bg {
            if cfg.settings.video_wallpaper_path.is_some() {
                tracing::info!(
                    stale = ?cfg.settings.video_wallpaper_path,
                    "clearing video_wallpaper_path for non-video export"
                );
            }
            cfg.settings.video_wallpaper_path = None;
        }
        // Portal contents are runtime data, not config.items: their names become the
        // manifest so a machine without that folder restores a virtual fence holding
        // whatever desktop files match instead of showing "folder not found".
        for layout in &mut cfg.layouts {
            for fence in &mut layout.fences {
                if fence.kind == coredesk_core::FenceKind::FolderPortal {
                    fence.contained_files = self
                        .state
                        .unsorted_items_of(fence)
                        .filter_map(|it| it.manifest_name())
                        .collect();
                }
            }
        }
        let mut bg_entry: Option<(String, Vec<u8>)> = None;
        let mut frame = preview_frame;
        if let Some(src) = bg_source.filter(|p| p.is_file()) {
            bg_entry = coredesk_core::cde_theme::background_entry(&mut cfg, src);
            match &bg_entry {
                Some((name, data)) => tracing::info!(
                    asset_type = if video_bg { "video" } else { "image" },
                    path = %src.display(),
                    entry = %name,
                    bytes = data.len(),
                    "packing theme asset"
                ),
                None => {
                    tracing::warn!(path = %src.display(), "theme background unreadable; none packed")
                }
            }
            if frame.is_none() {
                frame = if src
                    .extension()
                    .and_then(|e| e.to_str())
                    .is_some_and(|e| e.eq_ignore_ascii_case("mp4"))
                {
                    coredesk_platform::wmf_wallpaper::decode_video_first_frame(src)
                        .ok()
                        .map(|f| (f.width, f.height, f.bgra))
                } else {
                    coredesk_platform::wallpaper::decode_scaled(src, 640, 360)
                        .ok()
                        .map(|d| (d.width, d.height, d.bgra))
                };
            }
        } else {
            tracing::info!(
                ?bg_source,
                "packing theme asset: none — no usable background"
            );
        }
        let config_json = coredesk_core::ConfigStore::to_json(&cfg).map_err(|e| e.to_string())?;
        let split = coredesk_core::config_store::split_config_json(&config_json);
        let bg_ref = frame.as_ref().map(|(w, h, px)| (*w, *h, px.as_slice()));
        let preview_bgra =
            coredesk_core::cde_theme::render_layout_preview_bgra(&cfg, bg_ref, 640, 360);
        let preview_jpg = coredesk_platform::wmf_wallpaper::encode_jpeg(640, 360, &preview_bgra)
            .map_err(|e| e.to_string())?;
        Ok((split, bg_entry, preview_jpg))
    }

    /// Writes the split config trio + `bg_entry`/`preview_jpg` to `dest` and reports through the page.
    fn write_theme_archive(
        &mut self,
        dest: &Path,
        split: &coredesk_core::config_store::SplitConfigJson,
        meta_json: Option<&str>,
        bg_entry: Option<(String, Vec<u8>)>,
        preview_jpg: &[u8],
    ) -> bool {
        let bg_slice = bg_entry.as_ref().map(|(n, b)| (n.as_str(), b.as_slice()));
        let sidecars: [(&str, &str); 2] = [
            (
                coredesk_core::config_store::WIDGET_CONFIG_FILE,
                split.widget.as_str(),
            ),
            (
                coredesk_core::config_store::LIVE_EFFECTS_CONFIG_FILE,
                split.effects.as_str(),
            ),
        ];
        tracing::info!(
            dest = %dest.display(),
            background = ?bg_slice.map(|(n, b)| (n, b.len())),
            config_bytes = split.main.len(),
            preview_bytes = preview_jpg.len(),
            "writing .cde archive"
        );
        match coredesk_core::cde_theme::write_cde_file(
            dest,
            &split.main,
            &sidecars,
            meta_json,
            bg_slice,
            preview_jpg,
        ) {
            Ok(()) => {
                self.settings_toast(&coredesk_core::i18n::format(
                    "已导出到 {0}",
                    &[format!("{}", dest.display())],
                ));
                true
            }
            Err(e) => {
                self.settings_error(&coredesk_core::i18n::format(
                    "导出失败：{0}",
                    &[e.to_string()],
                ));
                false
            }
        }
    }

    pub(super) fn export_theme_to(&mut self, dest: &Path) {
        self.state.save_if_dirty();
        let bg = self.current_background_source();
        tracing::info!(dest = %dest.display(), background = ?bg, "theme export: background source selected");
        // The live frame backs the preview only when the video is the bundled background —
        // a stopped player still holds its last frame, which would paint over the image.
        let live = bg
            .as_deref()
            .and_then(|p| p.extension())
            .is_some_and(|e| e.eq_ignore_ascii_case("mp4"))
            .then(|| {
                self.wmf_player
                    .as_ref()
                    .and_then(|p| p.latest_frame())
                    .map(|f| (f.width, f.height, f.bgra))
            })
            .flatten();
        let meta = self.theme_meta(bg.as_deref());
        let meta_json = meta
            .source
            .is_some()
            .then(|| serde_json::to_string(&meta).ok())
            .flatten();
        match self.cde_payload(&self.state.config, bg.as_deref(), live) {
            Ok((split, bg_entry, preview_jpg)) => {
                self.write_theme_archive(
                    dest,
                    &split,
                    meta_json.as_deref(),
                    bg_entry,
                    &preview_jpg,
                );
            }
            Err(e) => self.settings_error(&coredesk_core::i18n::format("导出失败：{0}", &[e])),
        }
    }

    /// File stem usable for a `.cde` filename: path-hostile characters become `-`, leading
    /// dots/spaces are trimmed, and an empty result falls back to "Theme <date> <time>".
    fn sanitize_theme_stem(name: &str) -> String {
        let mut out = String::with_capacity(name.len());
        for c in name.trim().chars().take(80) {
            if c.is_control() || matches!(c, '<' | '>' | ':' | '"' | '/' | '\\' | '|' | '?' | '*') {
                out.push('-');
            } else {
                out.push(c);
            }
        }
        let clean = out.trim_matches([' ', '.', '-']).to_string();
        if clean.is_empty() {
            let stamp =
                coredesk_platform::fileinfo::format_local_datetime(coredesk_core::now_unix())
                    .replace(['/', '\\', ':'], "-");
            format!("{} {}", coredesk_core::i18n::text("主题"), stamp)
        } else {
            clean
        }
    }

    /// A free `stem.cde` / `stem (n).cde` path inside the theme library.
    fn unique_theme_path(&self, stem: &str) -> PathBuf {
        let dir = self.themes_dir();
        let _ = std::fs::create_dir_all(&dir);
        let mut dest = dir.join(format!("{stem}.cde"));
        if dest.exists() {
            for i in 2..1000 {
                let candidate = dir.join(format!("{stem} ({i}).cde"));
                if !candidate.exists() {
                    dest = candidate;
                    break;
                }
            }
        }
        dest
    }

    /// Adaptive-engine metadata for a `.cde` export: the monitor the theme was captured
    /// on, where its background asset lands there under the capped fit, and every fence's
    /// rect normalized onto that asset — so fences re-attach to the same visual feature
    /// (shelves, frame graphics) wherever the theme is applied. Empty when no bundled
    /// background can be measured.
    fn theme_meta(&self, bg_source: Option<&Path>) -> coredesk_core::adaptive::ThemeMeta {
        use coredesk_core::adaptive as ad;
        use coredesk_core::geometry;
        let mut meta = ad::ThemeMeta::default();
        let areas = &self.state.work_areas;
        let Some(primary) = areas.first() else {
            return meta;
        };
        // The asset's own pixel size: a live video reports its decoded frame, a file its
        // WIC/MF header. Best-effort `source.asset` telemetry — a video's drawn rect does
        // not depend on it at all.
        let asset = bg_source.and_then(|p| {
            if p.extension().is_some_and(|e| e.eq_ignore_ascii_case("mp4")) {
                self.wmf_player
                    .as_ref()
                    .and_then(|pl| pl.latest_frame())
                    .map(|f| (f.width, f.height))
                    .or_else(|| {
                        coredesk_platform::wmf_wallpaper::decode_video_first_frame(p)
                            .ok()
                            .map(|f| (f.width, f.height))
                    })
            } else {
                coredesk_platform::wallpaper::image_dimensions(p).ok()
            }
        });
        // Media-anchored canvas needs the asset's own pixel dims — without them there is
        // no media space to normalize against, so the theme ships with no metadata.
        let Some((aw, ah)) = asset else {
            return meta;
        };
        // Where the asset is actually drawn on a `w x h` monitor (telemetry for
        // `source.bg_rect`): strict Contain draws the whole frame centered and
        // uncropped; strict Cover fills the monitor and lets the platform crop it.
        let contain = self.contain_background();
        let drawn_rect = |w: u32, h: u32| -> [i32; 4] {
            if contain {
                ad::bg_rect(&ad::adaptive_fit(aw, ah, w, h, true), w, h)
            } else {
                [0, 0, w as i32, h as i32]
            }
        };
        // The media→monitor transform per panel: cover maps through the symmetric
        // center crop (media-agnostic — video blit and Windows "Fill" crop alike);
        // contain maps the whole frame onto the fitted rect.
        let media_map = |w: u32, h: u32| {
            if contain {
                ad::image_map(aw, ah, w, h)
            } else {
                ad::video_map(aw, ah, w, h)
            }
        };
        let layout_idx = self.state.config.last_shown().unwrap_or(0);
        let Some(layout) = self.state.config.layouts.get(layout_idx) else {
            return meta;
        };
        // The "source monitor" is the first fenced monitor — where the shelf art sits;
        // single-monitor themes collapse to the primary display.
        let source_area = layout
            .fences
            .first()
            .and_then(|f| geometry::resolve_work_area(&f.geometry, areas))
            .unwrap_or(primary);
        let [pw, ph] = source_area.screen_px().map(|v| v.max(0) as u32);
        meta.source = Some(ad::SourceCanvas {
            resolution: [pw, ph],
            dpi: source_area.dpi,
            bg_rect: drawn_rect(pw, ph),
            asset: asset.map(|(w, h)| [w, h]),
        });
        // Each fence is measured against the media frame: its monitor-relative px rect
        // runs through the monitor's media→screen map inverted, yielding normalized
        // media coords glued to asset pixels rather than screen positions.
        for fence in &layout.fences {
            let Some(work) = geometry::resolve_work_area(&fence.geometry, areas) else {
                continue;
            };
            let [mw, mh] = work.screen_px().map(|v| v.max(0) as u32);
            let map = media_map(mw, mh);
            let px = geometry::window_rect(&fence.geometry, work);
            let mon = [
                px.left - work.mon_left,
                px.top - work.mon_top,
                px.right - work.mon_left,
                px.bottom - work.mon_top,
            ];
            meta.canvas
                .insert(fence.id.to_string(), ad::to_media_canvas(mon, &map, aw, ah));
        }
        meta.canvas_space = ad::CanvasSpace::Media;
        meta
    }

    /// Adaptive theme import: maps each fence's stored coordinate onto its target
    /// monitor — media-anchored themes invert the monitor's media→screen transform so
    /// fences land on the same asset pixel across crops; legacy screen-anchored themes
    /// map onto the drawn asset rect. Then the hardcoded spacing compensation runs.
    /// Returns the measured asset size so the caller can composite pillarboxed
    /// wallpapers; `None` when the theme carries no usable canvas (legacy files fall
    /// through to the ordinary path).
    fn apply_adaptive_layout(
        &mut self,
        cfg: &mut coredesk_core::Config,
        meta: &coredesk_core::adaptive::ThemeMeta,
        bg: Option<&Path>,
        is_video: bool,
    ) -> Option<(u32, u32)> {
        use coredesk_core::adaptive as ad;
        let source = meta.source.as_ref()?;
        let media_anchored = meta.canvas_space == ad::CanvasSpace::Media;
        // Media-anchored coords need the asset's own dims — probe the bundled file,
        // then the dims saved in the metadata. A screen-anchored video needs none (the
        // monitor itself is the canvas); images always need them for the capped fit.
        let meta_asset = source.asset.map(|[w, h]| (w, h));
        let asset = if is_video {
            bg.and_then(|p| {
                coredesk_platform::wmf_wallpaper::decode_video_first_frame(p)
                    .ok()
                    .map(|f| (f.width, f.height))
            })
            .or(meta_asset)
        } else {
            bg.and_then(|p| coredesk_platform::wallpaper::image_dimensions(p).ok())
                .or(meta_asset)
        };
        if asset.is_none() && (!is_video || media_anchored) {
            return None;
        }
        let areas = &self.state.work_areas;
        let contain = self.contain_background();
        // Canvas re-anchoring (screen space only): themes saved while the drawn rect
        // meant something else — the capped hybrid fit, or a letterboxed video's
        // contain rect — carry normalized coords against that rect. `source.bg_rect`
        // records the rect the canvas was built on; when it disagrees with what the
        // media truly draws now (the whole source monitor in Cover, the contain rect
        // in Contain), renormalize each entry through px first so old archives land
        // correctly.
        let src_fix = if !media_anchored {
            let res = source.resolution;
            let true_src = if contain {
                asset.map(|(aw, ah)| {
                    ad::bg_rect(
                        &ad::adaptive_fit(aw, ah, res[0], res[1], true),
                        res[0],
                        res[1],
                    )
                })
            } else {
                Some([0, 0, res[0] as i32, res[1] as i32])
            };
            true_src.and_then(|t| (source.bg_rect != t).then_some((source.bg_rect, t)))
        } else {
            None
        };
        let mut placed = 0usize;
        for layout in &mut cfg.layouts {
            for fence in &mut layout.fences {
                let Some(n) = meta.canvas.get(&fence.id.to_string()).copied() else {
                    continue;
                };
                let Some(work) = coredesk_core::geometry::resolve_work_area(&fence.geometry, areas)
                else {
                    continue;
                };
                let [mw, mh] = work.screen_px().map(|v| v.max(0) as u32);
                let px = if media_anchored {
                    // Media-anchored: invert the monitor's media→screen map so the fence
                    // lands on the same media pixel — shelves keep their fences across
                    // crops. A fence whose media region this panel crops out clamps
                    // onto the nearest edge instead of going off-screen.
                    let (aw, ah) = asset.unwrap_or((1, 1));
                    let map = if contain {
                        ad::image_map(aw, ah, mw, mh)
                    } else {
                        ad::video_map(aw, ah, mw, mh)
                    };
                    let r = ad::from_media_canvas(n, &map, aw, ah);
                    let (w, h) = ((r[2] - r[0]).max(8), (r[3] - r[1]).max(8));
                    let l = r[0].clamp(0, (mw as i32 - w).max(0));
                    let t = r[1].clamp(0, (mh as i32 - h).max(0));
                    [l, t, l + w.min(mw as i32), t + h.min(mh as i32)]
                } else {
                    let n = match src_fix {
                        Some((old_bg, new_bg)) => ad::to_canvas(ad::from_canvas(n, old_bg), new_bg),
                        None => n,
                    };
                    // The physical screen rect the asset is actually drawn into: the
                    // contain rect in Contain, the whole monitor in Cover — the
                    // center crop is what the platform sees, media-agnostic.
                    let fbg = if contain {
                        match asset {
                            Some((aw, ah)) => {
                                ad::bg_rect(&ad::adaptive_fit(aw, ah, mw, mh, true), mw, mh)
                            }
                            None => [0, 0, mw as i32, mh as i32],
                        }
                    } else {
                        [0, 0, mw as i32, mh as i32]
                    };
                    ad::from_canvas(n, fbg)
                };
                let rect = coredesk_core::geometry::PxRect {
                    left: px[0] + work.mon_left,
                    top: px[1] + work.mon_top,
                    right: px[2] + work.mon_left,
                    bottom: px[3] + work.mon_top,
                };
                let mut g = coredesk_core::geometry::normalize(rect, work);
                // Canvas coordinates are anchored to the wallpaper, which is drawn on the
                // monitor — a work-area change (a taskbar style swap, auto-hide toggling)
                // moves nothing under it. The picked edge-gap anchor would re-anchor the
                // fence to the new work area and slide it off the art; anchoring each axis
                // to the edge the taskbar does NOT reserve keeps the absolute position.
                g.anchor = coredesk_core::geometry::canvas_anchor(&g, work);
                g.skew_left = fence.geometry.skew_left;
                g.skew_right = fence.geometry.skew_right;
                // Spacing compensation is decoupled from the background engine: the
                // multiplier is strictly the inverse OS display-scale ratio
                // (source_dpi / target_dpi), rounded to one decimal place so
                // floating-point error never jitters the gaps. Icon sizes stay locked
                // to the theme's exact pixels to match the wallpaper art.
                //   300%→200%: 1.5x · 200%→300%: 0.67→0.7x · 100%→300%: 0.33→0.3x
                let factor = ((source.dpi as f32 / work.dpi.max(1) as f32) * 10.0).round() / 10.0;
                if factor != 1.0 {
                    for v in [&mut fence.view.spacing_x, &mut fence.view.spacing_y]
                        .into_iter()
                        .flatten()
                    {
                        *v = (*v * factor).clamp(0.0, 256.0);
                    }
                }
                fence.geometry = g;
                placed += 1;
            }
        }
        tracing::info!(
            placed,
            canvas = meta.canvas.len(),
            space = ?meta.canvas_space,
            asset = ?asset,
            source_dpi = source.dpi,
            "adaptive theme: canvas coordinates applied"
        );
        asset
    }

    /// Applies a theme image to every monitor through `IDesktopWallpaper` (Explorer's own
    /// per-monitor route). Cover hands the raw asset to Windows "Fill" (its native
    /// uniform crop); Contain composites the frame centered over a blurred copy of
    /// itself — shelves inside the image keep their canvas mapping either way.
    fn apply_adaptive_wallpapers(&mut self, bg: &Path, asset_w: u32, asset_h: u32) {
        use coredesk_core::adaptive as ad;
        // Same retirement the plain route performs — the video engine yields to images.
        self.invalidate_video_swap();
        if let Some(p) = self.wmf_player.take() {
            reap_video_player(p);
        }
        self.static_under_video = None;
        if self
            .state
            .config
            .settings
            .video_wallpaper_path
            .take()
            .is_some()
        {
            self.settings_mutated();
        }
        let snap = coredesk_platform::wallpaper::query().ok();
        let dir = self
            .state
            .config_path()
            .parent()
            .map(|p| p.join("imported_wallpapers"));
        if let Some(d) = &dir {
            let _ = std::fs::create_dir_all(d);
        }
        let contain = self.contain_background();
        let mut applied = 0usize;
        for (i, area) in self.state.work_areas.clone().iter().enumerate() {
            let [mw, mh] = area.screen_px().map(|v| v.max(0) as u32);
            let path = match ad::adaptive_fit(asset_w, asset_h, mw, mh, contain) {
                // Cover: the raw asset — Windows "Fill" does the uniform
                // scale-and-crop natively, so no composite is needed.
                ad::AspectFit::Cover => bg.to_path_buf(),
                ad::AspectFit::Capped { bg: rect, .. } => dir
                    .as_ref()
                    .and_then(|d| {
                        let dest = d.join(format!("adaptive-{i}.jpg"));
                        coredesk_platform::wallpaper::adaptive_wallpaper(bg, mw, mh, rect)
                            .and_then(|img| {
                                coredesk_platform::wmf_wallpaper::encode_jpeg(
                                    img.width, img.height, &img.bgra,
                                )
                                .map(|jpg| (dest, jpg))
                            })
                            .ok()
                            .and_then(|(dest, jpg)| std::fs::write(&dest, &jpg).ok().map(|_| dest))
                    })
                    .unwrap_or_else(|| bg.to_path_buf()),
            };
            // `IDesktopWallpaper` monitor ids differ from `WorkArea::device_path`; the
            // snapshot's px rect is the join key.
            let id = snap.as_ref().and_then(|s| {
                s.monitors.iter().find_map(|m| {
                    (m.rect.left == area.mon_left
                        && m.rect.top == area.mon_top
                        && m.rect.right == area.mon_right
                        && m.rect.bottom == area.mon_bottom)
                        .then(|| m.monitor_id.clone())
                })
            });
            match id {
                Some(id) => {
                    if let Err(e) = coredesk_platform::wallpaper::set_monitor_wallpaper(&id, &path)
                    {
                        tracing::warn!(error = %e, "adaptive wallpaper: set failed");
                    } else {
                        applied += 1;
                    }
                }
                None => {
                    let _ = coredesk_platform::sysparams::set_desktop_wallpaper(
                        &path.to_string_lossy(),
                    );
                    applied += 1;
                }
            }
        }
        // Fill is mandatory under the canvas math: both Cover (raw asset, OS crops)
        // and the Contain composite are drawn for the exact monitor rect, and a
        // remembered Center/Tile position would misplace them.
        let _ = coredesk_platform::wallpaper::set_position_fill();
        tracing::info!(applied, "adaptive wallpapers committed");
    }

    /// `settings.containBackground`: strict Contain + blurred margins when on,
    /// strict Cover + crop when off — the whole background engine reads this.
    fn contain_background(&self) -> bool {
        self.state.config.settings.contain_background
    }

    /// The contain/cover toggle flipped: re-run the adaptive engine against the
    /// live state — canvas fences re-anchor to the new drawn rect and every
    /// monitor's image refits — committed once on the flip, never per keystroke.
    /// A live video's cover/contain flip is handled by the `sync_video_wallpaper`
    /// the caller always runs next.
    fn retune_background_fit(&mut self) {
        let contain = self.state.config.settings.contain_background;
        tracing::info!(contain, "background fit mode applied");
        let Some(bg) = self.current_background_source() else {
            return;
        };
        let is_video = bg
            .extension()
            .is_some_and(|e| e.eq_ignore_ascii_case("mp4"));
        if let Some(meta) = self.last_theme_meta.clone() {
            let mut cfg = self.state.config.clone();
            let asset = self.apply_adaptive_layout(&mut cfg, &meta, Some(&bg), is_video);
            // Video's return is its probed dims, not whether fences moved — a
            // screen-anchored canvas may still have been re-placed, so always adopt.
            if is_video || asset.is_some() {
                self.adopt_config(cfg, ConfigSource::Retune);
            }
            if !is_video && let Some((aw, ah)) = asset {
                self.apply_adaptive_wallpapers(&bg, aw, ah);
            }
            return;
        }
        // No theme canvas — the wallpaper itself still refits (cover ↔ contain).
        if !is_video && let Ok((aw, ah)) = coredesk_platform::wallpaper::image_dimensions(&bg) {
            self.apply_adaptive_wallpapers(&bg, aw, ah);
        }
    }

    /// Packs the live configuration (meta canvas + background + preview) into `dest` —
    /// the shared writer behind library saves, the dirty-prompt Save and the daily
    /// auto-update. `false` after the error is reported to the page.
    fn write_current_theme(&mut self, dest: &Path) -> bool {
        let cfg = self.state.config.clone();
        let bg = self.current_background_source();
        let live = bg
            .as_deref()
            .and_then(|p| p.extension())
            .is_some_and(|e| e.eq_ignore_ascii_case("mp4"))
            .then(|| {
                self.wmf_player
                    .as_ref()
                    .and_then(|p| p.latest_frame())
                    .map(|f| (f.width, f.height, f.bgra))
            })
            .flatten();
        let meta = self.theme_meta(bg.as_deref());
        let (split, bg_entry, preview_jpg) = match self.cde_payload(&cfg, bg.as_deref(), live) {
            Ok(p) => p,
            Err(e) => {
                self.settings_error(&coredesk_core::i18n::format("保存主题失败：{0}", &[e]));
                return false;
            }
        };
        let bg_slice = bg_entry.as_ref().map(|(n, b)| (n.as_str(), b.as_slice()));
        let meta_json = meta
            .source
            .is_some()
            .then(|| serde_json::to_string(&meta).ok())
            .flatten();
        let sidecars: [(&str, &str); 2] = [
            (
                coredesk_core::config_store::WIDGET_CONFIG_FILE,
                split.widget.as_str(),
            ),
            (
                coredesk_core::config_store::LIVE_EFFECTS_CONFIG_FILE,
                split.effects.as_str(),
            ),
        ];
        match coredesk_core::cde_theme::write_cde_file(
            dest,
            &split.main,
            &sidecars,
            meta_json.as_deref(),
            bg_slice,
            &preview_jpg,
        ) {
            Ok(()) => true,
            Err(e) => {
                self.settings_error(&coredesk_core::i18n::format(
                    "保存主题失败：{0}",
                    &[e.to_string()],
                ));
                false
            }
        }
    }

    /// Gallery "Save Current Setup": the live configuration becomes a `.cde` in the theme
    /// library — a card with a generated thumbnail. The active video or wallpaper is always
    /// embedded; there is no way to save a theme without its background. The saved file
    /// becomes the active theme: it is the current setup's own archive now.
    fn save_theme_to_library(&mut self, name: &str) {
        tracing::info!(theme = name, "theme library save requested");
        // Meta first: `My Setup` becomes `My Setup_3840x2160_100%.cde` — the source
        // canvas rides in the filename so the card says what the theme was authored on.
        let bg = self.current_background_source();
        let meta = self.theme_meta(bg.as_deref());
        let mut stem = Self::sanitize_theme_stem(name);
        if let Some(s) = &meta.source {
            stem.push('_');
            stem.push_str(&coredesk_core::adaptive::source_suffix(s));
        }
        let dest = self.unique_theme_path(&stem);
        if !self.write_current_theme(&dest) {
            return;
        }
        self.state.config.settings.active_theme_path = Some(dest.to_string_lossy().into_owned());
        self.settings_mutated();
        // The new archive now describes the live state — clean baseline (clear AFTER the
        // mutation plumbing ran: schedule_save re-arms the flag).
        self.theme_dirty.set(false);
        self.settings_toast(coredesk_core::i18n::text("已保存主题"));
    }

    /// `path` is a `.cde` file directly inside the theme library — the only paths a gallery
    /// card may ever hand to delete/copy operations.
    fn theme_in_library(&self, path: &Path) -> bool {
        let dir = self.themes_dir();
        let canon = |p: &Path| std::fs::canonicalize(p).unwrap_or_else(|_| p.to_path_buf());
        path.parent()
            .is_some_and(|parent| canon(parent) == canon(&dir))
            && path
                .extension()
                .is_some_and(|e| e.eq_ignore_ascii_case("cde"))
    }

    /// Card "Export .cde" on a library theme: the archive already is the `.cde` — offer a copy.
    fn export_library_theme(&mut self, path: &Path) {
        if !self.theme_in_library(path) {
            return;
        }
        let stem = path
            .file_stem()
            .map(|s| s.to_string_lossy().into_owned())
            .filter(|s| !s.is_empty())
            .unwrap_or_else(|| "theme".to_string());
        let owner = self.settings.as_ref().map(|h| h.hwnd());
        match coredesk_platform::filedialog::save_cde(
            owner,
            coredesk_core::i18n::text("导出为 .cde 主题"),
            &format!("{stem}.cde"),
        ) {
            Ok(Some(dest)) => match std::fs::copy(path, &dest) {
                Ok(_) => self.settings_toast(&coredesk_core::i18n::format(
                    "已导出到 {0}",
                    &[format!("{}", dest.display())],
                )),
                Err(e) => self.settings_error(&coredesk_core::i18n::format(
                    "导出失败：{0}",
                    &[e.to_string()],
                )),
            },
            Ok(None) => {}
            Err(e) => self.settings_error(&coredesk_core::i18n::format(
                "无法打开保存对话框：{0}",
                &[e.to_string()],
            )),
        }
    }

    /// Card "Export .cde" on a layout snapshot: packs the snapshot's stored layouts (with the
    /// current visual settings) and its captured wallpaper into a fresh `.cde` archive.
    fn export_snapshot_theme(&mut self, id: uuid::Uuid) {
        let Some(snap) = self
            .state
            .config
            .snapshots
            .iter()
            .find(|s| s.id == id)
            .cloned()
        else {
            return;
        };
        let name = if snap.is_auto_backup == Some(true) {
            auto_backup_label(&snap.name)
        } else {
            snap.name.clone()
        };
        let suggested = format!("{}.cde", Self::sanitize_theme_stem(&name));
        let owner = self.settings.as_ref().map(|h| h.hwnd());
        let dest = match coredesk_platform::filedialog::save_cde(
            owner,
            coredesk_core::i18n::text("导出为 .cde 主题"),
            &suggested,
        ) {
            Ok(Some(d)) => d,
            Ok(None) => return,
            Err(e) => {
                self.settings_error(&coredesk_core::i18n::format(
                    "无法打开保存对话框：{0}",
                    &[e.to_string()],
                ));
                return;
            }
        };
        let mut cfg = self.state.config.clone();
        cfg.layouts = snap.layouts.clone();
        let bg = snap
            .wallpaper_path
            .as_deref()
            .map(PathBuf::from)
            .filter(|p| p.is_file());
        tracing::info!(
            snapshot = %snap.name,
            snapshot_wallpaper = ?snap.wallpaper_path,
            background = ?bg,
            "snapshot export: background source selected"
        );
        match self.cde_payload(&cfg, bg.as_deref(), None) {
            Ok((split, bg_entry, preview_jpg)) => {
                self.write_theme_archive(&dest, &split, None, bg_entry, &preview_jpg);
            }
            Err(e) => self.settings_error(&coredesk_core::i18n::format("导出失败：{0}", &[e])),
        }
    }

    pub(super) fn import_theme(&mut self) {
        let owner = self.settings.as_ref().map(|h| h.hwnd());
        match coredesk_platform::filedialog::open_cde(owner, coredesk_core::i18n::text("导入主题"))
        {
            Ok(Some(path)) => self.import_theme_from(&path),
            Ok(None) => {}
            Err(e) => self.settings_error(&coredesk_core::i18n::format(
                "无法打开文件对话框：{0}",
                &[e.to_string()],
            )),
        }
    }

    /// The `.cde` theme library folder next to `config.json` (portable and packaged stores
    /// get their own, like `imported_wallpapers/`).
    pub(super) fn themes_dir(&self) -> PathBuf {
        self.state
            .config_path()
            .parent()
            .unwrap_or(Path::new("."))
            .join("themes")
    }

    /// One-time library migration: PecoFence-era `*.peco` archives are the same
    /// uncompressed-ZIP container as `.cde`, so adoption is a rename. `active_theme_path`
    /// moves with the file it pointed at; anything already migrated is left alone.
    pub(crate) fn migrate_theme_library(&mut self) {
        let dir = self.themes_dir();
        let Ok(rd) = std::fs::read_dir(&dir) else {
            return;
        };
        for entry in rd.flatten() {
            let src = entry.path();
            if !src
                .extension()
                .is_some_and(|e| e.eq_ignore_ascii_case("peco"))
            {
                continue;
            }
            let mut dest = src.with_extension("cde");
            if dest.exists() {
                // Both spellings already exist: keep the .cde, drop the stale .peco
                // when identical, otherwise park the .peco beside it under a suffix.
                if std::fs::read(&src).ok() == std::fs::read(&dest).ok() {
                    let _ = std::fs::remove_file(&src);
                    continue;
                }
                let stem = dest
                    .file_stem()
                    .map(|s| s.to_os_string())
                    .unwrap_or_default();
                for i in 2..100 {
                    let candidate = dest.with_file_name({
                        let mut s = stem.clone();
                        s.push(format!(" ({i})"));
                        s
                    });
                    if !candidate.exists() {
                        dest = candidate;
                        break;
                    }
                }
            }
            if let Err(e) = std::fs::rename(&src, &dest) {
                tracing::warn!(error = %e, from = %src.display(), "theme library .peco migration skipped");
                continue;
            }
            tracing::info!(from = %src.display(), to = %dest.display(), "theme library migrated .peco -> .cde");
            if self
                .state
                .config
                .settings
                .active_theme_path
                .as_deref()
                .is_some_and(|p| Path::new(p) == src)
            {
                self.state.config.settings.active_theme_path =
                    Some(dest.to_string_lossy().into_owned());
                self.state.mark_dirty();
            }
        }
    }

    /// The theme gallery's cards: every `*.cde` in the library with its embedded
    /// `preview.jpg` as a data URI (empty string when the archive has none).
    fn themes_json(&self) -> Vec<serde_json::Value> {
        let mut files: Vec<PathBuf> = std::fs::read_dir(self.themes_dir())
            .map(|rd| {
                rd.filter_map(|e| e.ok().map(|e| e.path()))
                    .filter(|p| p.extension().is_some_and(|e| e.eq_ignore_ascii_case("cde")))
                    .collect()
            })
            .unwrap_or_default();
        files.sort_by_key(|p| p.file_name().map(|n| n.to_ascii_lowercase()));
        files
            .into_iter()
            .map(|p| {
                let name = p
                    .file_stem()
                    .map(|s| s.to_string_lossy().into_owned())
                    .unwrap_or_default();
                let preview = coredesk_core::cde_theme::read_cde_preview(&p)
                    .map(|jpg| {
                        format!(
                            "data:image/jpeg;base64,{}",
                            coredesk_core::base64::encode(&jpg)
                        )
                    })
                    .unwrap_or_default();
                // The gallery sorts cards newest-first; a .cde's moment is its mtime.
                let (ts, date) = std::fs::metadata(&p)
                    .and_then(|m| m.modified())
                    .ok()
                    .and_then(|t| t.duration_since(std::time::UNIX_EPOCH).ok())
                    .map(|d| {
                        let ts = d.as_secs().min(i64::MAX as u64) as i64;
                        (ts, coredesk_platform::fileinfo::format_local_datetime(ts))
                    })
                    .unwrap_or_default();
                serde_json::json!({
                    "name": name,
                    "path": p.to_string_lossy(),
                    "preview": preview,
                    "ts": ts,
                    "date": date,
                })
            })
            .collect()
    }

    /// Copies an imported `.cde` into the theme library so it appears as a card. A file
    /// already inside the library, or one whose bytes match an existing entry, is kept as is;
    /// name collisions get a " (n)" suffix. Returns the library path that represents the
    /// theme — the anchor `active_theme_path` records (never the original file outside
    /// the library).
    fn store_theme_copy(&self, src: &Path) -> PathBuf {
        let dir = self.themes_dir();
        let canon = |p: &Path| std::fs::canonicalize(p).unwrap_or_else(|_| p.to_path_buf());
        if src
            .parent()
            .is_some_and(|parent| canon(parent) == canon(&dir))
        {
            return src.to_path_buf();
        }
        if std::fs::create_dir_all(&dir).is_err() {
            return src.to_path_buf();
        }
        let stem = src
            .file_stem()
            .map(|s| s.to_string_lossy().into_owned())
            .filter(|s| !s.is_empty())
            .unwrap_or_else(|| "theme".to_string());
        let same_bytes = |a: &Path, b: &Path| {
            std::fs::read(a)
                .ok()
                .zip(std::fs::read(b).ok())
                .is_some_and(|(x, y)| x == y)
        };
        let mut dest = dir.join(format!("{stem}.cde"));
        if dest.exists() {
            if same_bytes(src, &dest) {
                return dest;
            }
            for i in 2..100 {
                let candidate = dir.join(format!("{stem} ({i}).cde"));
                if !candidate.exists() {
                    dest = candidate;
                    break;
                }
                if same_bytes(src, &candidate) {
                    return candidate;
                }
            }
        }
        if let Err(e) = std::fs::copy(src, &dest) {
            tracing::warn!(error = %e, dest = %dest.display(), "theme library copy failed");
        }
        dest
    }

    /// Applies a `.cde` already living in the theme library (a gallery card click).
    pub(super) fn apply_theme_from_library(&mut self, path: &Path) {
        self.apply_theme_path(path, false);
    }

    /// Gallery Apply on a theme card. With unsaved layout changes AND a known active
    /// theme, the page is asked first — Save writes the live state into the active
    /// archive before switching, Don't Save discards them, Cancel aborts. Anything
    /// else applies instantly. Without a page to answer there is nothing to ask, so
    /// the apply proceeds.
    fn request_theme_switch(&mut self, path: &Path) {
        if self.theme_dirty.get()
            && self.state.config.settings.active_theme_path.is_some()
            && let Some(h) = &self.settings
        {
            h.post_json(
                &serde_json::json!({
                    "type": "themeSwitchPrompt",
                    "path": path.to_string_lossy(),
                })
                .to_string(),
            );
            return;
        }
        self.apply_theme_from_library(path);
    }

    /// Writes the live state over the ACTIVE theme's own archive — the file keeps its
    /// name and its card, nothing new is created. `false` (after reporting the problem)
    /// means there is no active theme or the write failed; the caller then aborts the
    /// pending switch so the user can decide again.
    fn overwrite_active_theme(&mut self) -> bool {
        let Some(p) = self.state.config.settings.active_theme_path.clone() else {
            return false;
        };
        let dest = PathBuf::from(&p);
        if !self.theme_in_library(&dest) {
            return false;
        }
        if !self.write_current_theme(&dest) {
            return false;
        }
        tracing::info!(theme = %dest.display(), "active theme overwritten in place");
        self.theme_dirty.set(false);
        self.push_settings_state();
        true
    }

    /// Once a day, rewrites the active theme's archive in place with the current layout —
    /// the file's mtime is the clock, so restarts neither skip nor double-write. Never
    /// creates a file: a card deleted behind our back is simply left alone.
    pub(super) fn maybe_auto_update_theme(&mut self) {
        if !self.state.config.settings.theme_auto_update_daily {
            return;
        }
        let Some(p) = self.state.config.settings.active_theme_path.clone() else {
            return;
        };
        let path = PathBuf::from(&p);
        if !self.theme_in_library(&path) || !path.is_file() {
            return;
        }
        let today = coredesk_core::config_store::today_yyyy_mm_dd();
        let file_day = std::fs::metadata(&path)
            .and_then(|m| m.modified())
            .ok()
            .and_then(|t| t.duration_since(std::time::UNIX_EPOCH).ok())
            .map(|d| coredesk_core::config_store::yyyy_mm_dd_of(d.as_secs() as i64));
        if file_day.as_deref() == Some(today.as_str()) {
            return;
        }
        tracing::info!(theme = %path.display(), "daily auto-update: rewriting active theme");
        if self.write_current_theme(&path) {
            self.theme_dirty.set(false);
            // The card's date (its mtime) just moved — refresh the gallery quietly.
            self.push_settings_state();
        }
    }

    /// Deletes a `.cde` from the theme library. Only files directly inside the library
    /// folder are removable — a page-supplied path can never reach outside it. Removing
    /// the active theme's archive severs its anchor: nothing remains to overwrite.
    pub(super) fn delete_library_theme(&mut self, path: &Path) {
        if self.theme_in_library(path) {
            let _ = std::fs::remove_file(path);
            let canon = |p: &Path| std::fs::canonicalize(p).unwrap_or_else(|_| p.to_path_buf());
            if self
                .state
                .config
                .settings
                .active_theme_path
                .as_deref()
                .is_some_and(|p| canon(Path::new(p)) == canon(path))
            {
                self.state.config.settings.active_theme_path = None;
                self.state.mark_dirty();
                self.schedule_save();
            }
        }
        self.push_settings_state();
    }

    pub(crate) fn import_theme_from(&mut self, path: &Path) {
        self.apply_theme_path(path, true);
    }

    /// Shared import/apply flow: `save_to_library` distinguishes a file the user picked
    /// (`importTheme`, kept as a gallery card) from a card that was clicked (`applyTheme`).
    fn apply_theme_path(&mut self, path: &Path, save_to_library: bool) {
        let extracted = match coredesk_core::cde_theme::extract_cde_to_temp(path) {
            Ok(ex) => ex,
            Err(e) => {
                self.settings_error(&coredesk_core::i18n::format(
                    "无法使用此文件：{0}",
                    std::slice::from_ref(&e),
                ));
                return;
            }
        };

        let mut cfg = match self.state.parse_import_file(&extracted.config_path) {
            Ok(c) => c,
            Err(e) => {
                let _ = std::fs::remove_dir_all(&extracted.temp_dir);
                self.settings_error(&coredesk_core::i18n::format(
                    "无法使用此文件：{0}",
                    std::slice::from_ref(&e),
                ));
                return;
            }
        };

        // Persist the extracted background media next to config.json so it survives temp cleanup.
        let mut persisted_bg: Option<PathBuf> = None;
        if let Some(bg_temp) = &extracted.background_path {
            let ext = bg_temp
                .extension()
                .and_then(|e| e.to_str())
                .unwrap_or("jpg");
            let dest_dir = self
                .state
                .config_path()
                .parent()
                .unwrap_or(Path::new("."))
                .join("imported_wallpapers");
            let _ = std::fs::create_dir_all(&dest_dir);
            // A fresh name per import: a fixed `theme-background.mp4` collides with the
            // player still holding the previous video open (the copy can fail or rewrite
            // bytes under a live reader), and `sync_video_wallpaper` keys restarts on the
            // path — identical paths kept the old video playing under the new theme.
            let dest_bg =
                dest_dir.join(format!("theme-background-{}.{}", uuid::Uuid::new_v4(), ext));
            if std::fs::copy(bg_temp, &dest_bg).is_ok() {
                // Retire the previous theme's media (best effort — a still-playing
                // file stays until the next import sweeps it).
                for entry in std::fs::read_dir(&dest_dir).into_iter().flatten().flatten() {
                    let name = entry.file_name();
                    let stale = name
                        .to_str()
                        .is_some_and(|n| n.starts_with("theme-background-"));
                    if stale && entry.path() != dest_bg {
                        let _ = std::fs::remove_file(entry.path());
                    }
                }
                persisted_bg = Some(dest_bg);
            }
        }
        let _ = std::fs::remove_dir_all(&extracted.temp_dir);

        let is_video = persisted_bg
            .as_ref()
            .and_then(|p| p.extension())
            .is_some_and(|e| e.eq_ignore_ascii_case("mp4"));

        if is_video {
            cfg.settings.video_wallpaper_path = persisted_bg
                .as_ref()
                .map(|p| p.to_string_lossy().into_owned());
        } else if persisted_bg.is_some() {
            cfg.settings.video_wallpaper_path = None;
        }

        // A picked file joins the library first: the card it becomes (not the file the
        // user picked outside the library) is the active theme anchor — the file the
        // unsaved-changes Save and the daily auto-update overwrite in place. It rides
        // `cfg` into `adopt_config`, replacing whatever path a foreign theme.json carried.
        let active_path = if save_to_library {
            self.store_theme_copy(path)
        } else {
            path.to_path_buf()
        };
        cfg.settings.active_theme_path = Some(active_path.to_string_lossy().into_owned());
        remap_theme_monitors(&mut cfg, &self.state.work_areas);
        // Adaptive engine: canvas coordinates ride `theme.json`; when present they re-place
        // fences onto the fitted asset and scale the grid by the DPI ratio. Legacy archives
        // have no metadata and keep the remapped geometry untouched. The canvas is also kept
        // for the contain/cover toggle re-fit (a legacy theme clears any stale one).
        self.last_theme_meta = extracted.meta.clone();
        let adaptive_asset = self.last_theme_meta.clone().and_then(|m| {
            self.apply_adaptive_layout(&mut cfg, &m, persisted_bg.as_deref(), is_video)
        });
        self.adopt_config(cfg, ConfigSource::Theme);

        if let Some(bg) = persisted_bg {
            if is_video {
                self.sync_video_wallpaper();
            } else if let Some((aw, ah)) = adaptive_asset {
                self.apply_adaptive_wallpapers(&bg, aw, ah);
            } else {
                let bg_str = bg.to_string_lossy();
                if !self.apply_static_wallpaper(&bg_str) {
                    tracing::warn!(path = %bg.display(), "importTheme: image wallpaper not applied");
                }
            }
        }
        // The apply's own writes do not count as user edits: whatever the incoming theme
        // normalized is the new baseline the dirty flag measures against.
        self.theme_dirty.set(false);
    }

    pub(super) fn refresh_language(&mut self) {
        let language = self
            .state
            .config
            .settings
            .language
            .resolve(coredesk_platform::locale::ui_language());
        if language == coredesk_core::i18n::language() {
            return;
        }
        coredesk_core::i18n::set_language(language);
        tracing::info!(language = language.tag(), "interface language changed");
        if let Some(host) = &self.settings {
            host.update_language();
        }
        if let Some(host) = &self.fe_glass {
            host.update_language();
        }
        if let Some(host) = &self.clock {
            host.update_language();
        }
        for window in self.fences.values() {
            window.redraw();
        }
        self.push_settings_state();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn remapping_theme_monitors_preserves_fence_geometry_on_new_display() {
        let mut cfg = coredesk_core::Config::default();
        let mut f = coredesk_core::Fence::new(
            "Shelf",
            coredesk_core::FenceKind::Virtual,
            coredesk_core::NormGeometry {
                monitor: "OLD_MON#1".into(),
                x: 120.0,
                y: 80.0,
                w: 400.0,
                h: 300.0,
                work_w: 1920.0,
                work_h: 1080.0,
                anchor: coredesk_core::Anchor::LeftTop,
                skew_left: Some(-12.0),
                skew_right: Some(12.0),
            },
        );
        f.view.shelf_3d = Some(true);
        cfg.layouts.push(coredesk_core::Layout {
            fingerprint: vec![coredesk_core::MonitorIdentity {
                device_path: "OLD_MON#1".into(),
                gdi_name: r"\\.\DISPLAY1".into(),
                work_dip: [1920.0, 1080.0],
                dpi: 96,
                screen_px: [1920, 1080],
            }],
            fences: vec![f],
            desktop_items: Default::default(),
            shown: 1,
        });
        let work = vec![coredesk_core::geometry::WorkArea {
            device_path: "NEW_MON#2".into(),
            gdi_name: r"\\.\DISPLAY1".into(),
            left: 0,
            top: 0,
            right: 1920,
            bottom: 1080,
            dpi: 96,
            mon_left: 0,
            mon_top: 0,
            mon_right: 1920,
            mon_bottom: 1080,
        }];
        remap_theme_monitors(&mut cfg, &work);
        assert_eq!(cfg.layouts[0].fences[0].geometry.monitor, "NEW_MON#2");
        assert_eq!(cfg.layouts[0].fences[0].geometry.x, 120.0);
        assert_eq!(cfg.layouts[0].fences[0].geometry.y, 80.0);
        assert_eq!(cfg.layouts[0].fences[0].geometry.w, 400.0);
        assert_eq!(cfg.layouts[0].fences[0].geometry.h, 300.0);
    }
}
