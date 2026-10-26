//! Settings host window + page protocol: state JSON, toasts, message handling, config adoption,
//! import / export / restore, settings diff application.

use super::*;

/// Largest snapping gap honoured, in DIPs; `snapping.gapPx` is clamped to `0..=` this.
pub(super) const MAX_SNAP_GAP_DIP: i32 = 64;

/// `snapping.gapPx` as the gap (DIPs) fence windows keep to each other and the work-area edges
/// while snapping.
pub(super) fn snap_gap_dip(snapping: &pecofence_core::SnappingSettings) -> i32 {
    snapping.gap_px.clamp(0, MAX_SNAP_GAP_DIP)
}

/// A script error on the settings page (`pageError`): its JavaScript error type and position
/// go to the log as fixed names and numbers; the page never sends the message, which can
/// quote what the user typed.
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
}

impl ConfigSource {
    fn done(self) -> &'static str {
        match self {
            Self::Import => pecofence_core::i18n::text("已导入配置"),
            Self::Backup => pecofence_core::i18n::text("已恢复备份"),
            Self::Theme => pecofence_core::i18n::text("已导入主题"),
        }
    }

    fn before(self) -> &'static str {
        match self {
            Self::Import | Self::Theme => pecofence_core::i18n::text("导入配置前"),
            Self::Backup => pecofence_core::i18n::text("恢复备份前"),
        }
    }

    /// The i18n key stored as the auto backup's `name`; display layers localize it on read.
    fn before_key(self) -> &'static str {
        match self {
            Self::Import | Self::Theme => "导入配置前",
            Self::Backup => "恢复备份前",
        }
    }
}

/// Room for one of the page's automatic "…前" snapshots. Like the CLI's `auto_snapshot_slot`,
/// it never evicts a snapshot the user saved: a full list loses its oldest automatic one (the
/// page's or the CLI's) instead, and `false` means none is left to drop, so none is taken.
/// `protect` (the snapshot about to be restored) always stays. Automatic names saved in another
/// UI language count as the user's.
pub(super) fn auto_snapshot_room(
    snapshots: &mut Vec<pecofence_core::Snapshot>,
    protect: Option<uuid::Uuid>,
) -> bool {
    let auto = [
        pecofence_core::i18n::text("恢复快照前"),
        ConfigSource::Import.before(),
        ConfigSource::Backup.before(),
    ];
    if snapshots.len() >= pecofence_core::MAX_SNAPSHOTS
        && let Some(i) = snapshots.iter().position(|s| {
            Some(s.id) != protect
                && (s.is_auto_backup == Some(true)
                    || s.name.starts_with(super::ipc::AUTO_SNAPSHOT_PREFIX)
                    || auto.contains(&s.name.as_str()))
        })
    {
        snapshots.remove(i);
    }
    snapshots.len() < pecofence_core::MAX_SNAPSHOTS
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
    pecofence_core::i18n::text(key).to_string()
}

fn snapshot_limit_text() -> String {
    pecofence_core::i18n::format(
        "快照已达上限（{0} 个），未保存新快照。请删除不需要的快照。",
        &[pecofence_core::MAX_SNAPSHOTS.to_string()],
    )
}

/// Remaps an imported `.peco` theme's layout monitor IDs to `work_areas` when the theme was
/// saved on a different monitor setup, preserving every fence's normalized coordinates and size.
fn remap_theme_monitors(
    cfg: &mut pecofence_core::Config,
    work_areas: &[pecofence_core::geometry::WorkArea],
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
        .map(pecofence_core::MonitorIdentity::of)
        .collect();
}

impl App {
    /// Replaces the configuration wholesale (import / backup) and rebuilds everything.
    pub(super) fn adopt_config(&mut self, cfg: pecofence_core::Config, source: ConfigSource) {
        self.end_peek_now();
        let old_settings = self.state.config.settings.clone();
        self.state.replace_config(cfg);
        if let Some(desk) = shell::user_desktop() {
            self.state.migrate_desktop_path(&desk);
        }
        // What the file (plus the desktop-path migration) wants the settings to be. The Store
        // edition's startup switch is Windows': a file does not change it (nor open its page).
        let mut new_settings = self.state.config.settings.clone();
        if pecofence_platform::process::is_packaged() {
            new_settings.autostart = old_settings.autostart;
        }
        self.sync_desktop_if_available("config adopted");
        // `.peco` manifests bind fence contents to this desktop's actual files (names that
        // match nothing are dropped); a theme's folder portal whose directory is missing
        // here degrades to a virtual fence rather than flashing "folder not found".
        let desktop_dirs: Vec<PathBuf> = [shell::user_desktop(), shell::public_desktop()]
            .into_iter()
            .flatten()
            .collect();
        self.state
            .bind_theme_members(&desktop_dirs, matches!(source, ConfigSource::Theme));
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
        self.state.save_if_dirty();
        self.push_settings_state();
        self.settings_toast(source.done());
    }

    pub(super) fn export_config(&mut self) {
        let owner = self.settings.as_ref().map(|h| h.hwnd());
        let name = format!(
            "pecofence-{}.json",
            pecofence_core::config_store::today_yyyy_mm_dd()
        );
        match pecofence_platform::filedialog::save_json(
            owner,
            pecofence_core::i18n::text("导出 PecoFence 配置"),
            &name,
        ) {
            Ok(Some(path)) => {
                self.state.save_if_dirty();
                match pecofence_core::ConfigStore::export_to(&self.state.config, &path) {
                    Ok(()) => self.settings_toast(&pecofence_core::i18n::format(
                        "已导出到 {0}",
                        &[format!("{}", path.display())],
                    )),
                    Err(e) => self.settings_error(&pecofence_core::i18n::format(
                        "导出失败：{0}",
                        &[e.to_string()],
                    )),
                }
            }
            Ok(None) => {}
            Err(e) => self.settings_error(&pecofence_core::i18n::format(
                "无法打开保存对话框：{0}",
                &[e.to_string()],
            )),
        }
    }

    pub(super) fn import_config(&mut self) {
        let owner = self.settings.as_ref().map(|h| h.hwnd());
        match pecofence_platform::filedialog::open_json(
            owner,
            pecofence_core::i18n::text("导入 PecoFence 配置"),
        ) {
            Ok(Some(path)) => self.restore_from_file(&path, ConfigSource::Import),
            Ok(None) => {}
            Err(e) => self.settings_error(&pecofence_core::i18n::format(
                "无法打开文件对话框：{0}",
                &[e.to_string()],
            )),
        }
    }

    pub(super) fn restore_from_file(&mut self, path: &std::path::Path, source: ConfigSource) {
        match self.state.parse_import_file(path) {
            Ok(cfg) => {
                // Keep a snapshot of what we are replacing so the step can be undone — the
                // fixed-id auto backup slot overwrites itself, so only a first-time slot
                // needs room. A list full of the user's own snapshots has none, so nothing
                // is replaced.
                let has_slot = self
                    .state
                    .config
                    .snapshots
                    .iter()
                    .any(|s| s.id == pecofence_core::AUTO_BACKUP_SNAPSHOT_ID);
                if !has_slot && !auto_snapshot_room(&mut self.state.config.snapshots, None) {
                    self.settings_error(&snapshot_limit_text());
                    return;
                }
                self.state.save_restore_backup(source.before_key());
                self.adopt_config(cfg, source);
            }
            Err(e) => self.settings_error(&pecofence_core::i18n::format(
                "无法使用此文件：{0}",
                std::slice::from_ref(&e),
            )),
        }
    }

    pub(super) fn open_settings(&mut self) {
        self.query_store_startup();
        if let Some(h) = &self.settings {
            window::show_normal(h.hwnd());
            window::bring_to_front(h.hwnd());
            return;
        }
        if self.web_env.is_none() {
            match WebEnvironment::create() {
                Ok(env) => self.web_env = Some(env),
                // WebView2's wait for its callback pumps messages and ends with an error when
                // it takes a WM_QUIT (the session ends): quit after all.
                Err(_) if window::quit_posted() => {
                    window::post_quit(0);
                    return;
                }
                Err(e) => {
                    tracing::error!(error = %e, "WebView2 environment failed");
                    if let Some(t) = &self.tray {
                        t.show_info(
                            "PecoFence",
                            pecofence_core::i18n::text("无法创建 WebView2 环境，请确认已安装 Microsoft Edge WebView2 运行时。"),
                            true,
                        );
                    }
                    return;
                }
            }
        }
        match SettingsHost::open(
            &self.settings_class,
            self.web_env.as_ref().unwrap(),
            self.theme_mode,
            self.ctx.theme.borrow().liquid_glass,
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
            }
            Err(_) if window::quit_posted() => window::post_quit(0),
            Err(e) => {
                tracing::error!(error = %e, "settings window failed");
                if let Some(t) = &self.tray {
                    t.show_info(
                        "PecoFence",
                        pecofence_core::i18n::text("无法打开设置窗口（WebView2 初始化失败），请检查 Microsoft Edge WebView2 运行时。"),
                        true,
                    );
                }
            }
        }
    }

    pub(super) fn settings_state_json(&self) -> String {
        let localization = pecofence_core::i18n::ui_payload();
        let fences: Vec<serde_json::Value> = self
            .state
            .fences()
            .iter()
            .map(|f| self.fence_options_json(f))
            .collect();
        let mem_mb = pecofence_platform::memstats::MemoryStats::current()
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
                    "date": pecofence_platform::fileinfo::format_local_datetime(s.ts),
                    "fenceCount": super::ipc::snapshot_dto(s, &self.state.work_areas).fence_count,
                    "wallpaperPath": s.wallpaper_path,
                    "isAutoBackup": s.is_auto_backup == Some(true),
                })
            })
            .collect();
        let backups: Vec<serde_json::Value> = self
            .state
            .backup_files()
            .iter()
            .map(|p| {
                serde_json::json!({
                    "path": p.to_string_lossy(),
                    "name": p.file_stem().map(|s| s.to_string_lossy().to_string()).unwrap_or_default(),
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
            "desktopIconsHidden": pecofence_platform::shell_icons::desktop_icons_hidden(),
            "rules": self.state.config.rules,
            "fences": fences,
            "snapshots": snapshots,
            "backups": backups,
            "themes": themes,
            "monitors": monitors,
            "tintPalette": fence_options::tint_palette_json(),
            "version": env!("CARGO_PKG_VERSION"),
            "storeEdition": pecofence_platform::process::is_packaged(),
            "configPath": self.state.config_path_on_disk().to_string_lossy(),
            "memoryMb": mem_mb,
            "itemCount": self.state.workspace_item_count(),
            "themeMode": if self.theme_mode == ThemeMode::Dark { "dark" } else { "light" },
            "accent": accent,
            "update": self.update_json(),
            "peekHotkey": self.peek_hotkey_json(),
        })
        .to_string()
    }

    pub(super) fn push_settings_state(&self) {
        if let Some(h) = &self.settings {
            h.post_json(&self.settings_state_json());
        }
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
        if !pecofence_platform::process::is_packaged() {
            return;
        }
        let result = self.store_startup.clone();
        let control = self.control.hwnd().0 as isize;
        let _ = std::thread::Builder::new()
            .name("pecofence-startup-task".into())
            .spawn(
                move || match pecofence_platform::store::startup_task_enabled() {
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
                match serde_json::from_value::<pecofence_core::Settings>(new_settings) {
                    Ok(new_settings) => self.apply_settings(new_settings),
                    Err(e) => tracing::warn!(error = %e, "bad settings from page"),
                }
            }
            Some("setRules") => {
                if let Some(rules) = v.get("rules").cloned()
                    && let Ok(rules) = serde_json::from_value::<pecofence_core::RuleSet>(rules)
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
                    self.settings_toast(&pecofence_core::i18n::format(
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
                                .any(|r| r.assigned_by == pecofence_core::AssignedBy::User)
                        });
                    if pinned {
                        self.auto_snapshot();
                    }
                    let entries = shell::enumerate_desktop();
                    let moved = self.state.apply_rules_force_all(&entries);
                    self.refresh_all();
                    self.schedule_save();
                    self.settings_toast(&pecofence_core::i18n::format(
                        "已按规则整理 {0} 个项目",
                        &[moved.to_string()],
                    ));
                }
                Some("addTemplate") => {
                    if let Some(template) = v
                        .get("template")
                        .and_then(|t| t.as_str())
                        .and_then(pecofence_core::rules::Template::parse)
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
                        self.apply_theme_from_library(Path::new(p));
                    }
                }
                Some("saveSetup") => {
                    let name = v.get("themeName").and_then(|n| n.as_str()).unwrap_or("");
                    let include_bg = v
                        .get("includeBackground")
                        .and_then(|f| f.as_bool())
                        .unwrap_or(false);
                    self.save_theme_to_library(name, include_bg);
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
                    if self.state.config.snapshots.len() >= pecofence_core::MAX_SNAPSHOTS {
                        self.settings_error(&snapshot_limit_text());
                    } else {
                        let name = v.get("snapshotName").and_then(|n| n.as_str()).unwrap_or("");
                        let include_wallpaper = v
                            .get("includeWallpaper")
                            .and_then(|f| f.as_bool())
                            .unwrap_or(false);
                        let wallpaper = include_wallpaper
                            .then(pecofence_platform::sysparams::desktop_wallpaper)
                            .flatten();
                        self.state.save_snapshot(name, wallpaper);
                        self.schedule_save();
                        self.push_settings_state();
                        self.settings_toast(pecofence_core::i18n::text("已保存快照"));
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
                            .any(|s| s.id == pecofence_core::AUTO_BACKUP_SNAPSHOT_ID);
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
                            if !pecofence_platform::sysparams::set_desktop_wallpaper(&path) {
                                tracing::warn!(path, "restoreSnapshot: wallpaper not applied");
                            }
                        }
                        self.schedule_save();
                        self.push_settings_state();
                        self.settings_toast(pecofence_core::i18n::text("已恢复快照"));
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
                Some("exportConfig") => self.export_config(),
                Some("importConfig") => self.import_config(),
                Some("restoreBackup") => {
                    if let Some(p) = v.get("path").and_then(|p| p.as_str()) {
                        let path = PathBuf::from(p);
                        // Only files from our own backups folder.
                        if self.state.backup_files().contains(&path) {
                            self.restore_from_file(&path, ConfigSource::Backup);
                        }
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
                Some("repairIcons") => self.set_desktop_icons_hidden(false),
                Some("hideDesktopIcons") => self.set_desktop_icons_hidden(true),
                Some("rateInStore") => self.open_store_rating(),
                Some("openManual") => self.open_manual(),
                Some("openRepository") => {
                    let url = pecofence_core::brand::REPOSITORY_URL;
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
    pub(super) fn set_rules(&mut self, mut rules: pecofence_core::RuleSet) {
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
    /// request must still repair Explorer's state if it changed outside PecoFence.
    pub(super) fn set_desktop_icons_hidden(&mut self, hidden: bool) {
        if self.apply_desktop_icons_hidden(hidden) {
            self.state.config.settings.hide_real_icons = hidden;
            self.settings_mutated();
            self.desktop_icons_setting_changed();
            self.settings_toast(if hidden {
                pecofence_core::i18n::text("Windows 桌面图标已重新隐藏。")
            } else {
                pecofence_core::i18n::text("桌面图标已显示，可点击“重新隐藏桌面图标”恢复隐藏。")
            });
        } else {
            self.push_settings_state();
            self.settings_error(pecofence_core::i18n::text(
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
        if pecofence_platform::shell_icons::desktop_icons_hidden()
            && let Some(h) = desktop::resolve_icon_host(a.generation)
        {
            let _ = pecofence_platform::shell_icons::set_desktop_icons_hidden(
                false, h.def_view, h.host,
            );
            // Clear our recovery marker only after Explorer actually restored the icons.
            a.restore_desktop_icons();
        }
        !pecofence_platform::shell_icons::desktop_icons_hidden()
    }

    /// Applies a full settings object from the page, reacting to what changed.
    pub(super) fn apply_settings(&mut self, mut new: pecofence_core::Settings) {
        let old = self.state.config.settings.clone();
        if new == old {
            return;
        }
        if new.autostart != old.autostart {
            if let Err(e) =
                pecofence_platform::autostart::set_product_enabled(new.autostart, self.portable)
            {
                tracing::warn!(error = %e, "autostart change failed");
                new.autostart = old.autostart;
                self.settings_error(pecofence_core::i18n::text(
                    "未能更改“随 Windows 启动”，请稍后重试。",
                ));
            } else if pecofence_platform::process::is_packaged() {
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
            self.settings_error(pecofence_core::i18n::text(
                "桌面图标状态未能更改，请稍后重试。",
            ));
        }
        let icons_toggled = new.hide_real_icons != old.hide_real_icons;
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
        self.sync_video_wallpaper();
        self.push_settings_state();
    }

    /// Starts or stops the WMF desktop video wallpaper player to match `settings.video_wallpaper_path`.
    pub(super) fn sync_video_wallpaper(&mut self) {
        let wanted = self
            .state
            .config
            .settings
            .video_wallpaper_path
            .clone()
            .filter(|s| !s.is_empty());
        let Some(path_str) = wanted else {
            self.wmf_player = None;
            return;
        };
        let path = PathBuf::from(&path_str);
        if self.wmf_player.as_ref().is_some_and(|p| p.path() == path) {
            return;
        }
        self.wmf_player = None;
        if !path.is_file() {
            return;
        }
        match pecofence_platform::wmf_wallpaper::WmfWallpaperPlayer::start(&path) {
            Ok(player) => {
                self.wmf_player = Some(player);
                self.capture_live_wallpapers(Duration::ZERO);
            }
            Err(error) => {
                tracing::error!(path = %path.display(), %error, "WMF video wallpaper failed to start");
                self.settings_error(&pecofence_core::i18n::format(
                    "无法使用此文件：{0}",
                    std::slice::from_ref(&error),
                ));
            }
        }
    }

    pub(super) fn pick_video_wallpaper(&mut self) {
        let owner = self.settings.as_ref().map(|h| h.hwnd());
        match pecofence_platform::filedialog::open_mp4(
            owner,
            pecofence_core::i18n::text("将视频设为桌面壁纸"),
        ) {
            Ok(Some(path)) => self.apply_video_wallpaper(&path),
            Ok(None) => {}
            Err(e) => self.settings_error(&pecofence_core::i18n::format(
                "无法打开文件对话框：{0}",
                &[e.to_string()],
            )),
        }
    }

    pub(super) fn apply_video_wallpaper(&mut self, path: &Path) {
        tracing::info!(path = %path.display(), "applying video wallpaper from settings");
        self.wmf_player = None;
        match pecofence_platform::wmf_wallpaper::WmfWallpaperPlayer::start(path) {
            Ok(player) => {
                self.wmf_player = Some(player);
                self.state.config.settings.video_wallpaper_path =
                    Some(path.to_string_lossy().into_owned());
                self.settings_mutated();
                self.capture_live_wallpapers(Duration::ZERO);
                self.settings_toast(pecofence_core::i18n::text("已将视频设为桌面壁纸"));
            }
            Err(err) => {
                tracing::error!(path = %path.display(), %err, "apply_video_wallpaper failed");
                self.settings_error(&pecofence_core::i18n::format(
                    "无法使用此文件：{0}",
                    std::slice::from_ref(&err),
                ));
            }
        }
    }

    pub(super) fn export_theme(&mut self) {
        let owner = self.settings.as_ref().map(|h| h.hwnd());
        let name = format!(
            "pecofence-theme-{}.peco",
            pecofence_core::config_store::today_yyyy_mm_dd()
        );
        match pecofence_platform::filedialog::save_peco(
            owner,
            pecofence_core::i18n::text("导出为 .peco 主题"),
            &name,
        ) {
            Ok(Some(path)) => self.export_theme_to(&path),
            Ok(None) => {}
            Err(e) => self.settings_error(&pecofence_core::i18n::format(
                "无法打开保存对话框：{0}",
                &[e.to_string()],
            )),
        }
    }

    /// The active desktop background media: the WMF video when one is set, else the desktop
    /// wallpaper image — only paths that still exist.
    fn current_background_source(&self) -> Option<PathBuf> {
        self.state
            .config
            .settings
            .video_wallpaper_path
            .as_deref()
            .map(PathBuf::from)
            .filter(|p| p.is_file())
            .or_else(|| {
                pecofence_platform::sysparams::desktop_wallpaper()
                    .map(PathBuf::from)
                    .filter(|p| p.is_file())
                    .or_else(|| {
                        pecofence_platform::wallpaper::query()
                            .ok()
                            .and_then(|q| q.monitors.into_iter().find_map(|m| m.path))
                            .filter(|p| p.is_file())
                    })
            })
    }

    /// Serializes `cfg` and builds a `.peco` payload: an optional `background.<ext>` entry
    /// copied from `bg_source` plus a 640x360 `preview.jpg` of the layout rendered over a
    /// frame of that background (`preview_frame` wins when the caller has live pixels).
    fn peco_payload(
        &self,
        cfg: &pecofence_core::Config,
        bg_source: Option<&Path>,
        preview_frame: Option<(u32, u32, Vec<u8>)>,
    ) -> std::result::Result<(String, Option<(String, Vec<u8>)>, Vec<u8>), String> {
        let mut cfg = cfg.clone();
        cfg.make_theme_portable();
        // Portal contents are runtime data, not config.items: their names become the
        // manifest so a machine without that folder restores a virtual fence holding
        // whatever desktop files match instead of showing "folder not found".
        for layout in &mut cfg.layouts {
            for fence in &mut layout.fences {
                if fence.kind == pecofence_core::FenceKind::FolderPortal {
                    fence.contained_files = self
                        .state
                        .unsorted_items_of(fence)
                        .filter_map(|it| it.manifest_name())
                        .collect();
                }
            }
        }
        let config_json = pecofence_core::ConfigStore::to_json(&cfg).map_err(|e| e.to_string())?;
        let mut bg_entry: Option<(String, Vec<u8>)> = None;
        let mut frame = preview_frame;
        if let Some(src) = bg_source.filter(|p| p.is_file()) {
            let ext = src
                .extension()
                .and_then(|e| e.to_str())
                .map(|e| e.to_ascii_lowercase())
                .filter(|e| {
                    matches!(
                        e.as_str(),
                        "mp4" | "jpg" | "jpeg" | "png" | "bmp" | "webp" | "gif" | "tif" | "tiff"
                    )
                })
                .unwrap_or_else(|| "jpg".into());
            if let Ok(bytes) = std::fs::read(src) {
                bg_entry = Some((format!("background.{ext}"), bytes));
            }
            if frame.is_none() {
                frame = if ext == "mp4" {
                    pecofence_platform::wmf_wallpaper::decode_video_first_frame(src)
                        .ok()
                        .map(|f| (f.width, f.height, f.bgra))
                } else {
                    pecofence_platform::wallpaper::decode_scaled(src, 640, 360)
                        .ok()
                        .map(|d| (d.width, d.height, d.bgra))
                };
            }
        }
        let bg_ref = frame.as_ref().map(|(w, h, px)| (*w, *h, px.as_slice()));
        let preview_bgra =
            pecofence_core::peco_theme::render_layout_preview_bgra(&cfg, bg_ref, 640, 360);
        let preview_jpg = pecofence_platform::wmf_wallpaper::encode_jpeg(640, 360, &preview_bgra)
            .map_err(|e| e.to_string())?;
        Ok((config_json, bg_entry, preview_jpg))
    }

    /// Writes `config_json`/`bg_entry`/`preview_jpg` to `dest` and reports through the page.
    fn write_theme_archive(
        &mut self,
        dest: &Path,
        config_json: &str,
        bg_entry: Option<(String, Vec<u8>)>,
        preview_jpg: &[u8],
    ) -> bool {
        let bg_slice = bg_entry.as_ref().map(|(n, b)| (n.as_str(), b.as_slice()));
        match pecofence_core::peco_theme::write_peco_file(dest, config_json, bg_slice, preview_jpg)
        {
            Ok(()) => {
                self.settings_toast(&pecofence_core::i18n::format(
                    "已导出到 {0}",
                    &[format!("{}", dest.display())],
                ));
                true
            }
            Err(e) => {
                self.settings_error(&pecofence_core::i18n::format(
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
        let live = self
            .wmf_player
            .as_ref()
            .and_then(|p| p.latest_frame())
            .map(|f| (f.width, f.height, f.bgra));
        match self.peco_payload(&self.state.config, bg.as_deref(), live) {
            Ok((config_json, bg_entry, preview_jpg)) => {
                self.write_theme_archive(dest, &config_json, bg_entry, &preview_jpg);
            }
            Err(e) => self.settings_error(&pecofence_core::i18n::format("导出失败：{0}", &[e])),
        }
    }

    /// File stem usable for a `.peco` filename: path-hostile characters become `-`, leading
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
                pecofence_platform::fileinfo::format_local_datetime(pecofence_core::now_unix())
                    .replace(['/', '\\', ':'], "-");
            format!("{} {}", pecofence_core::i18n::text("主题"), stamp)
        } else {
            clean
        }
    }

    /// A free `stem.peco` / `stem (n).peco` path inside the theme library.
    fn unique_theme_path(&self, stem: &str) -> PathBuf {
        let dir = self.themes_dir();
        let _ = std::fs::create_dir_all(&dir);
        let mut dest = dir.join(format!("{stem}.peco"));
        if dest.exists() {
            for i in 2..1000 {
                let candidate = dir.join(format!("{stem} ({i}).peco"));
                if !candidate.exists() {
                    dest = candidate;
                    break;
                }
            }
        }
        dest
    }

    /// Gallery "Save Current Setup": the live configuration becomes a `.peco` in the theme
    /// library — a card with a generated thumbnail. `include_bg` embeds the active video or
    /// wallpaper; without it the archived config clears `video_wallpaper_path` so applying
    /// the theme never resurrects a background the user excluded.
    fn save_theme_to_library(&mut self, name: &str, include_bg: bool) {
        let mut cfg = self.state.config.clone();
        let bg = if include_bg {
            self.current_background_source()
        } else {
            cfg.settings.video_wallpaper_path = None;
            None
        };
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
        let dest = self.unique_theme_path(&Self::sanitize_theme_stem(name));
        match self.peco_payload(&cfg, bg.as_deref(), live) {
            Ok((config_json, bg_entry, preview_jpg)) => {
                let bg_slice = bg_entry.as_ref().map(|(n, b)| (n.as_str(), b.as_slice()));
                match pecofence_core::peco_theme::write_peco_file(
                    &dest,
                    &config_json,
                    bg_slice,
                    &preview_jpg,
                ) {
                    Ok(()) => {
                        self.push_settings_state();
                        self.settings_toast(pecofence_core::i18n::text("已保存主题"));
                    }
                    Err(e) => self.settings_error(&pecofence_core::i18n::format(
                        "保存主题失败：{0}",
                        &[e.to_string()],
                    )),
                }
            }
            Err(e) => self.settings_error(&pecofence_core::i18n::format("保存主题失败：{0}", &[e])),
        }
    }

    /// `path` is a `.peco` file directly inside the theme library — the only paths a gallery
    /// card may ever hand to delete/copy operations.
    fn theme_in_library(&self, path: &Path) -> bool {
        let dir = self.themes_dir();
        let canon = |p: &Path| std::fs::canonicalize(p).unwrap_or_else(|_| p.to_path_buf());
        path.parent()
            .is_some_and(|parent| canon(parent) == canon(&dir))
            && path
                .extension()
                .is_some_and(|e| e.eq_ignore_ascii_case("peco"))
    }

    /// Card "Export .peco" on a library theme: the archive already is the `.peco` — offer a copy.
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
        match pecofence_platform::filedialog::save_peco(
            owner,
            pecofence_core::i18n::text("导出为 .peco 主题"),
            &format!("{stem}.peco"),
        ) {
            Ok(Some(dest)) => match std::fs::copy(path, &dest) {
                Ok(_) => self.settings_toast(&pecofence_core::i18n::format(
                    "已导出到 {0}",
                    &[format!("{}", dest.display())],
                )),
                Err(e) => self.settings_error(&pecofence_core::i18n::format(
                    "导出失败：{0}",
                    &[e.to_string()],
                )),
            },
            Ok(None) => {}
            Err(e) => self.settings_error(&pecofence_core::i18n::format(
                "无法打开保存对话框：{0}",
                &[e.to_string()],
            )),
        }
    }

    /// Card "Export .peco" on a layout snapshot: packs the snapshot's stored layouts (with the
    /// current visual settings) and its captured wallpaper into a fresh `.peco` archive.
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
        let suggested = format!("{}.peco", Self::sanitize_theme_stem(&name));
        let owner = self.settings.as_ref().map(|h| h.hwnd());
        let dest = match pecofence_platform::filedialog::save_peco(
            owner,
            pecofence_core::i18n::text("导出为 .peco 主题"),
            &suggested,
        ) {
            Ok(Some(d)) => d,
            Ok(None) => return,
            Err(e) => {
                self.settings_error(&pecofence_core::i18n::format(
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
        match self.peco_payload(&cfg, bg.as_deref(), None) {
            Ok((config_json, bg_entry, preview_jpg)) => {
                self.write_theme_archive(&dest, &config_json, bg_entry, &preview_jpg);
            }
            Err(e) => self.settings_error(&pecofence_core::i18n::format("导出失败：{0}", &[e])),
        }
    }

    pub(super) fn import_theme(&mut self) {
        let owner = self.settings.as_ref().map(|h| h.hwnd());
        match pecofence_platform::filedialog::open_peco(
            owner,
            pecofence_core::i18n::text("导入主题"),
        ) {
            Ok(Some(path)) => self.import_theme_from(&path),
            Ok(None) => {}
            Err(e) => self.settings_error(&pecofence_core::i18n::format(
                "无法打开文件对话框：{0}",
                &[e.to_string()],
            )),
        }
    }

    /// The `.peco` theme library folder next to `config.json` (portable and packaged stores
    /// get their own, like `imported_wallpapers/`).
    pub(super) fn themes_dir(&self) -> PathBuf {
        self.state
            .config_path()
            .parent()
            .unwrap_or(Path::new("."))
            .join("themes")
    }

    /// The theme gallery's cards: every `*.peco` in the library with its embedded
    /// `preview.jpg` as a data URI (empty string when the archive has none).
    fn themes_json(&self) -> Vec<serde_json::Value> {
        let mut files: Vec<PathBuf> = std::fs::read_dir(self.themes_dir())
            .map(|rd| {
                rd.filter_map(|e| e.ok().map(|e| e.path()))
                    .filter(|p| {
                        p.extension()
                            .is_some_and(|e| e.eq_ignore_ascii_case("peco"))
                    })
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
                let preview = pecofence_core::peco_theme::read_peco_preview(&p)
                    .map(|jpg| {
                        format!(
                            "data:image/jpeg;base64,{}",
                            pecofence_core::base64::encode(&jpg)
                        )
                    })
                    .unwrap_or_default();
                // The gallery sorts cards newest-first; a .peco's moment is its mtime.
                let (ts, date) = std::fs::metadata(&p)
                    .and_then(|m| m.modified())
                    .ok()
                    .and_then(|t| t.duration_since(std::time::UNIX_EPOCH).ok())
                    .map(|d| {
                        let ts = d.as_secs().min(i64::MAX as u64) as i64;
                        (ts, pecofence_platform::fileinfo::format_local_datetime(ts))
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

    /// Copies an imported `.peco` into the theme library so it appears as a card. A file
    /// already inside the library, or one whose bytes match an existing entry, is kept as is;
    /// name collisions get a " (n)" suffix.
    fn store_theme_copy(&self, src: &Path) {
        let dir = self.themes_dir();
        let canon = |p: &Path| std::fs::canonicalize(p).unwrap_or_else(|_| p.to_path_buf());
        if src
            .parent()
            .is_some_and(|parent| canon(parent) == canon(&dir))
        {
            return;
        }
        if std::fs::create_dir_all(&dir).is_err() {
            return;
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
        let mut dest = dir.join(format!("{stem}.peco"));
        if dest.exists() {
            if same_bytes(src, &dest) {
                return;
            }
            for i in 2..100 {
                let candidate = dir.join(format!("{stem} ({i}).peco"));
                if !candidate.exists() {
                    dest = candidate;
                    break;
                }
                if same_bytes(src, &candidate) {
                    return;
                }
            }
        }
        if let Err(e) = std::fs::copy(src, &dest) {
            tracing::warn!(error = %e, dest = %dest.display(), "theme library copy failed");
        }
    }

    /// Applies a `.peco` already living in the theme library (a gallery card click).
    pub(super) fn apply_theme_from_library(&mut self, path: &Path) {
        self.apply_theme_path(path, false);
    }

    /// Deletes a `.peco` from the theme library. Only files directly inside the library
    /// folder are removable — a page-supplied path can never reach outside it.
    pub(super) fn delete_library_theme(&mut self, path: &Path) {
        if self.theme_in_library(path) {
            let _ = std::fs::remove_file(path);
        }
        self.push_settings_state();
    }

    pub(super) fn import_theme_from(&mut self, path: &Path) {
        self.apply_theme_path(path, true);
    }

    /// Shared import/apply flow: `save_to_library` distinguishes a file the user picked
    /// (`importTheme`, kept as a gallery card) from a card that was clicked (`applyTheme`).
    fn apply_theme_path(&mut self, path: &Path, save_to_library: bool) {
        let extracted = match pecofence_core::peco_theme::extract_peco_to_temp(path) {
            Ok(ex) => ex,
            Err(e) => {
                self.settings_error(&pecofence_core::i18n::format(
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
                self.settings_error(&pecofence_core::i18n::format(
                    "无法使用此文件：{0}",
                    std::slice::from_ref(&e),
                ));
                return;
            }
        };

        if !auto_snapshot_room(&mut self.state.config.snapshots, None) {
            let _ = std::fs::remove_dir_all(&extracted.temp_dir);
            self.settings_error(&snapshot_limit_text());
            return;
        }

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
            let dest_bg = dest_dir.join(format!("theme-background.{ext}"));
            if std::fs::copy(bg_temp, &dest_bg).is_ok() {
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

        remap_theme_monitors(&mut cfg, &self.state.work_areas);
        // A picked file joins the library before the state push so its card appears with the
        // same update that applies it.
        if save_to_library {
            self.store_theme_copy(path);
        }
        // The fixed-id auto backup slot: every apply overwrites the same "…前" entry
        // instead of appending another card to the gallery.
        self.state
            .save_restore_backup(ConfigSource::Theme.before_key());
        self.adopt_config(cfg, ConfigSource::Theme);

        if let Some(bg) = persisted_bg {
            if is_video {
                self.sync_video_wallpaper();
            } else {
                self.wmf_player = None;
                let bg_str = bg.to_string_lossy();
                if !pecofence_platform::sysparams::set_desktop_wallpaper(&bg_str) {
                    tracing::warn!(path = %bg.display(), "importTheme: image wallpaper not applied");
                }
            }
        }
    }

    pub(super) fn refresh_language(&mut self) {
        let language = self
            .state
            .config
            .settings
            .language
            .resolve(pecofence_platform::locale::ui_language());
        if language == pecofence_core::i18n::language() {
            return;
        }
        pecofence_core::i18n::set_language(language);
        tracing::info!(language = language.tag(), "interface language changed");
        if let Some(host) = &self.settings {
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
        let mut cfg = pecofence_core::Config::default();
        let mut f = pecofence_core::Fence::new(
            "Shelf",
            pecofence_core::FenceKind::Virtual,
            pecofence_core::NormGeometry {
                monitor: "OLD_MON#1".into(),
                x: 120.0,
                y: 80.0,
                w: 400.0,
                h: 300.0,
                work_w: 1920.0,
                work_h: 1080.0,
                anchor: pecofence_core::Anchor::LeftTop,
                skew_left: Some(-12.0),
                skew_right: Some(12.0),
            },
        );
        f.view.shelf_3d = Some(true);
        cfg.layouts.push(pecofence_core::Layout {
            fingerprint: vec![pecofence_core::MonitorIdentity {
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
        let work = vec![pecofence_core::geometry::WorkArea {
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
