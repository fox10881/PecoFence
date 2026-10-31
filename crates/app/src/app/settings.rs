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
    /// The last theme re-fitted on a contain/cover switch — same machinery, its own toast.
    Retune,
}

impl ConfigSource {
    fn done(self) -> &'static str {
        match self {
            Self::Import => pecofence_core::i18n::text("已导入配置"),
            Self::Backup => pecofence_core::i18n::text("已恢复备份"),
            Self::Theme => pecofence_core::i18n::text("已导入主题"),
            Self::Retune => pecofence_core::i18n::text("已应用背景显示方式"),
        }
    }

    fn before(self) -> &'static str {
        match self {
            Self::Import | Self::Theme | Self::Retune => pecofence_core::i18n::text("导入配置前"),
            Self::Backup => pecofence_core::i18n::text("恢复备份前"),
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
        if !matches!(source, ConfigSource::Theme | ConfigSource::Retune) {
            // A plain config import/backup restore severs the link to whatever theme
            // described the old setup — never trust a path a foreign file carried.
            new_settings.active_theme_path = None;
        }
        self.sync_desktop_if_available("config adopted");
        // `.peco` manifests bind fence contents to this desktop's actual files (names that
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
        self.state.save_if_dirty();
        self.push_settings_state();
        self.settings_toast(source.done());
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
            "themeDirty": self.theme_dirty.get(),
            // Start Menu Style dropdown: only {id, label} reach the page — the
            // converted rule payload stays behind start_menu_themes::theme().
            "startMenuStyles": pecofence_core::start_menu_themes::style_options_json(),
            // Taskbar Style dropdown — same isolation for the taskbar catalog.
            "taskbarStyles": pecofence_core::taskbar_themes::style_options_json(),
            // Notification Center Style dropdown — same isolation again.
            "notificationCenterStyles": pecofence_core::notification_center_themes::style_options_json(),
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
                            if !self.apply_static_wallpaper(&path) {
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
                Some("restartExplorer") => {
                    // Failsafe for a stuck taskbar/desktop: kill the shell and
                    // let Windows' own respawn (or our detached spawn) bring it
                    // back — the anchor tick then re-injects and re-styles it.
                    self.settings_toast(pecofence_core::i18n::text("正在重启资源管理器…"));
                    std::thread::spawn(pecofence_platform::tap::restart_explorer);
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
        if new.start_menu_style != old.start_menu_style {
            // The dropdown only offers catalog ids; an unknown id here means a
            // hand-edited patchSettings — same treatment as a failed write.
            let id = new.start_menu_style.as_str();
            let applied = pecofence_core::start_menu_themes::is_style_id(id)
                && pecofence_platform::start_menu::apply_start_menu_theme(id);
            if !applied {
                new.start_menu_style.clone_from(&old.start_menu_style);
                self.settings_error(pecofence_core::i18n::text(
                    "开始菜单样式未能更改，请稍后重试。",
                ));
            }
        }
        if new.taskbar_style != old.taskbar_style {
            let id = new.taskbar_style.as_str();
            let applied = pecofence_core::taskbar_themes::is_style_id(id)
                && pecofence_platform::taskbar_styler::apply(id);
            if !applied {
                new.taskbar_style.clone_from(&old.taskbar_style);
                self.settings_error(pecofence_core::i18n::text(
                    "任务栏样式未能更改，请稍后重试。",
                ));
            }
        }
        if new.notification_center_style != old.notification_center_style {
            let id = new.notification_center_style.as_str();
            let applied = pecofence_core::notification_center_themes::is_style_id(id)
                && pecofence_platform::notification_center::apply(id);
            if !applied {
                new.notification_center_style
                    .clone_from(&old.notification_center_style);
                self.settings_error(pecofence_core::i18n::text(
                    "通知中心样式未能更改，请稍后重试。",
                ));
            }
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
            self.static_under_video = None;
            return;
        };
        let path = PathBuf::from(&path_str);
        let letterbox = self.contain_background();
        if self
            .wmf_player
            .as_ref()
            .is_some_and(|p| p.path() == path && p.is_alive() && p.letterbox() == letterbox)
        {
            return;
        }
        self.wmf_player = None;
        self.static_under_video = None;
        if !path.is_file() {
            return;
        }
        match pecofence_platform::wmf_wallpaper::WmfWallpaperPlayer::start(&path, letterbox) {
            Ok(player) => {
                self.static_under_video = Self::static_wallpaper_now();
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

    /// Self-heal for the desktop video wallpaper: an Explorer restart destroys the
    /// WorkerW/Progman layer the video window is parented to, so the player thread
    /// exits while `video_wallpaper_path` still names the file. When the shell is
    /// back up, `WmfWallpaperPlayer::start` re-runs the `0x052C` WorkerW spawn and
    /// re-attaches — driven from the periodic `check_wallpaper` so any cause of
    /// Explorer loss (our Restart Explorer failsafe, a crash, a user taskkill)
    /// recovers without a settings round-trip. Failures stay quiet: a shell that
    /// is mid-boot retries on the next poll.
    pub(super) fn ensure_video_wallpaper(&mut self) {
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
        if self.wmf_player.as_ref().is_some_and(|p| p.is_alive()) {
            return;
        }
        // Shell down (GetShellWindow is a live query) — attach would just fail;
        // the next poll retries once Progman exists again.
        if pecofence_platform::desktop::shell_window().0.is_null() {
            return;
        }
        self.wmf_player = None;
        self.static_under_video = None;
        let path = PathBuf::from(&path_str);
        if !path.is_file() {
            return;
        }
        let letterbox = self.contain_background();
        match pecofence_platform::wmf_wallpaper::WmfWallpaperPlayer::start(&path, letterbox) {
            Ok(player) => {
                self.static_under_video = Self::static_wallpaper_now();
                self.wmf_player = Some(player);
                self.capture_live_wallpapers(Duration::ZERO);
                tracing::info!("wmf video wallpaper re-attached after desktop restart");
            }
            Err(error) => {
                tracing::debug!(%error, "video wallpaper re-attach deferred; shell not ready");
            }
        }
    }

    /// Applies a static image as the desktop wallpaper and retires the video engine:
    /// choosing a static background must stop a still-playing video and clear its stored
    /// path so exports and restarts don't resurrect it. Returns `set_desktop_wallpaper`'s
    /// result.
    pub(super) fn apply_static_wallpaper(&mut self, path: &str) -> bool {
        self.wmf_player = None;
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
        pecofence_platform::sysparams::set_desktop_wallpaper(path)
    }

    /// `WM_SETTINGCHANGE`/`SPI_SETDESKWALLPAPER`: a static wallpaper was applied — by us
    /// (`apply_static_wallpaper` already cleared state, so the guard makes it a no-op) or
    /// outside the app (Settings > Personalization, Explorer's "Set as desktop background").
    /// An outside change means wallpaper intent moved to the image, so a running or
    /// configured video yields — otherwise it keeps covering the image and exports keep
    /// bundling it.
    pub(super) fn on_wallpaper_changed(&mut self) {
        if self.wmf_player.is_none() && self.state.config.settings.video_wallpaper_path.is_none() {
            return;
        }
        tracing::info!(
            player_alive = self.wmf_player.as_ref().is_some_and(|p| p.is_alive()),
            configured = ?self.state.config.settings.video_wallpaper_path,
            "external wallpaper change; retiring video wallpaper"
        );
        self.wmf_player = None;
        self.static_under_video = None;
        self.state.config.settings.video_wallpaper_path = None;
        // `settings_mutated` — not bare `schedule_save` — or the cleared path never
        // reaches disk (`save_if_dirty` writes only flagged state) and the next
        // launch resurrects the video.
        self.settings_mutated();
    }

    /// The static wallpaper path as `IDesktopWallpaper` resolves it — the same provider
    /// `follow_snapshot` compares `static_under_video` against, so a SPI↔IDW divergence
    /// (`TranscodedWallpaper` vs the source path) cannot look like a user apply.
    fn static_wallpaper_now() -> Option<PathBuf> {
        pecofence_platform::wallpaper::query()
            .ok()
            .and_then(|q| q.monitors.into_iter().find_map(|m| m.path))
            .filter(|p| p.is_file())
            .or_else(|| {
                pecofence_platform::sysparams::desktop_wallpaper()
                    .map(PathBuf::from)
                    .filter(|p| p.is_file())
            })
    }

    /// A static image applied over a running video retires it — for applies the
    /// `SPI_SETDESKWALLPAPER` broadcast misses (Explorer's "Set as desktop background"
    /// goes through `IDesktopWallpaper`). Detected by the reported static path differing
    /// from `static_under_video`, recorded when the player started. `desktops_differ`
    /// guards virtual-desktop switches, which legitimately report another image.
    pub(super) fn retire_video_for_static(&mut self, static_path: Option<&Path>) {
        let player_alive = self.wmf_player.as_ref().is_some_and(|p| p.is_alive());
        let (Some(baseline), Some(now)) = (self.static_under_video.as_ref(), static_path) else {
            return;
        };
        if !player_alive || now == baseline {
            return;
        }
        if pecofence_platform::wallpaper::desktops_differ() {
            tracing::debug!(
                applied = %now.display(),
                "static wallpaper changed but virtual desktops differ; keeping video"
            );
            return;
        }
        tracing::info!(
            applied = %now.display(),
            was = %baseline.display(),
            video = %self.wmf_player.as_ref().map(|p| p.path().display().to_string()).unwrap_or_default(),
            "static image applied over video wallpaper; retiring player"
        );
        self.wmf_player = None;
        self.static_under_video = None;
        self.state.config.settings.video_wallpaper_path = None;
        self.settings_mutated();
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
        self.static_under_video = None;
        let letterbox = self.contain_background();
        match pecofence_platform::wmf_wallpaper::WmfWallpaperPlayer::start(path, letterbox) {
            Ok(player) => {
                self.static_under_video = Self::static_wallpaper_now();
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

    /// The active desktop background media: the playing WMF video, else the desktop
    /// wallpaper image — only paths that still exist. `video_wallpaper_path` keeps the
    /// last-used path even after playback stopped or a static wallpaper took over, so the
    /// live player — not the config string — decides.
    fn current_background_source(&mut self) -> Option<PathBuf> {
        let spi = pecofence_platform::sysparams::desktop_wallpaper()
            .map(PathBuf::from)
            .filter(|p| p.is_file());
        let idw = spi.is_none().then(|| {
            pecofence_platform::wallpaper::query()
                .ok()
                .and_then(|q| q.monitors.into_iter().find_map(|m| m.path))
                .filter(|p| p.is_file())
        });
        let static_src = spi.clone().or_else(|| idw.clone().flatten());
        // A static image applied after the video started overrides it — including
        // Explorer applies that broadcast nothing. The comparison uses the same
        // IDW-resolved provider as the recorded baseline (`static_wallpaper_now`).
        self.retire_video_for_static(Self::static_wallpaper_now().as_deref());
        let player = self.wmf_player.as_ref();
        let player_alive = player.is_some_and(|p| p.is_alive());
        let player_path = player.map(|p| p.path().to_path_buf());
        let player_file = player_path.as_ref().is_some_and(|p| p.is_file());
        let chosen = if player_alive && player_file {
            player_path.clone()
        } else {
            static_src.clone()
        };
        tracing::info!(
            player_present = player.is_some(),
            player_alive,
            player_path = ?player_path,
            player_file,
            spi_wallpaper = ?spi,
            idw_wallpaper = ?idw.as_ref().map(|o| o.as_ref().map(|p| p.display().to_string())),
            static_under_video = ?self.static_under_video,
            configured_video = ?self.state.config.settings.video_wallpaper_path,
            chosen = ?chosen,
            "background source evaluated"
        );
        chosen
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
                if fence.kind == pecofence_core::FenceKind::FolderPortal {
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
            bg_entry = pecofence_core::peco_theme::background_entry(&mut cfg, src);
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
                    pecofence_platform::wmf_wallpaper::decode_video_first_frame(src)
                        .ok()
                        .map(|f| (f.width, f.height, f.bgra))
                } else {
                    pecofence_platform::wallpaper::decode_scaled(src, 640, 360)
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
        let config_json = pecofence_core::ConfigStore::to_json(&cfg).map_err(|e| e.to_string())?;
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
        meta_json: Option<&str>,
        bg_entry: Option<(String, Vec<u8>)>,
        preview_jpg: &[u8],
    ) -> bool {
        let bg_slice = bg_entry.as_ref().map(|(n, b)| (n.as_str(), b.as_slice()));
        tracing::info!(
            dest = %dest.display(),
            background = ?bg_slice.map(|(n, b)| (n, b.len())),
            config_bytes = config_json.len(),
            preview_bytes = preview_jpg.len(),
            "writing .peco archive"
        );
        match pecofence_core::peco_theme::write_peco_file(
            dest,
            config_json,
            meta_json,
            bg_slice,
            preview_jpg,
        ) {
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
        match self.peco_payload(&self.state.config, bg.as_deref(), live) {
            Ok((config_json, bg_entry, preview_jpg)) => {
                self.write_theme_archive(
                    dest,
                    &config_json,
                    meta_json.as_deref(),
                    bg_entry,
                    &preview_jpg,
                );
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

    /// Adaptive-engine metadata for a `.peco` export: the monitor the theme was captured
    /// on, where its background asset lands there under the capped fit, and every fence's
    /// rect normalized onto that asset — so fences re-attach to the same visual feature
    /// (shelves, frame graphics) wherever the theme is applied. Empty when no bundled
    /// background can be measured.
    fn theme_meta(&self, bg_source: Option<&Path>) -> pecofence_core::adaptive::ThemeMeta {
        use pecofence_core::adaptive as ad;
        use pecofence_core::geometry;
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
                        pecofence_platform::wmf_wallpaper::decode_video_first_frame(p)
                            .ok()
                            .map(|f| (f.width, f.height))
                    })
            } else {
                pecofence_platform::wallpaper::image_dimensions(p).ok()
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
        cfg: &mut pecofence_core::Config,
        meta: &pecofence_core::adaptive::ThemeMeta,
        bg: Option<&Path>,
        is_video: bool,
    ) -> Option<(u32, u32)> {
        use pecofence_core::adaptive as ad;
        let source = meta.source.as_ref()?;
        let media_anchored = meta.canvas_space == ad::CanvasSpace::Media;
        // Media-anchored coords need the asset's own dims — probe the bundled file,
        // then the dims saved in the metadata. A screen-anchored video needs none (the
        // monitor itself is the canvas); images always need them for the capped fit.
        let meta_asset = source.asset.map(|[w, h]| (w, h));
        let asset = if is_video {
            bg.and_then(|p| {
                pecofence_platform::wmf_wallpaper::decode_video_first_frame(p)
                    .ok()
                    .map(|f| (f.width, f.height))
            })
            .or(meta_asset)
        } else {
            bg.and_then(|p| pecofence_platform::wallpaper::image_dimensions(p).ok())
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
                let Some(work) =
                    pecofence_core::geometry::resolve_work_area(&fence.geometry, areas)
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
                let rect = pecofence_core::geometry::PxRect {
                    left: px[0] + work.mon_left,
                    top: px[1] + work.mon_top,
                    right: px[2] + work.mon_left,
                    bottom: px[3] + work.mon_top,
                };
                let mut g = pecofence_core::geometry::normalize(rect, work);
                // Canvas coordinates are anchored to the wallpaper, which is drawn on the
                // monitor — a work-area change (a taskbar style swap, auto-hide toggling)
                // moves nothing under it. The picked edge-gap anchor would re-anchor the
                // fence to the new work area and slide it off the art; holding the absolute
                // position is what media-anchored mapping means.
                g.anchor = pecofence_core::model::Anchor::LeftTop;
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
        use pecofence_core::adaptive as ad;
        // Same retirement the plain route performs — the video engine yields to images.
        self.wmf_player = None;
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
        let snap = pecofence_platform::wallpaper::query().ok();
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
                        pecofence_platform::wallpaper::adaptive_wallpaper(bg, mw, mh, rect)
                            .and_then(|img| {
                                pecofence_platform::wmf_wallpaper::encode_jpeg(
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
                    if let Err(e) = pecofence_platform::wallpaper::set_monitor_wallpaper(&id, &path)
                    {
                        tracing::warn!(error = %e, "adaptive wallpaper: set failed");
                    } else {
                        applied += 1;
                    }
                }
                None => {
                    let _ = pecofence_platform::sysparams::set_desktop_wallpaper(
                        &path.to_string_lossy(),
                    );
                    applied += 1;
                }
            }
        }
        // Fill is mandatory under the canvas math: both Cover (raw asset, OS crops)
        // and the Contain composite are drawn for the exact monitor rect, and a
        // remembered Center/Tile position would misplace them.
        let _ = pecofence_platform::wallpaper::set_position_fill();
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
        if !is_video && let Ok((aw, ah)) = pecofence_platform::wallpaper::image_dimensions(&bg) {
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
        let (config_json, bg_entry, preview_jpg) =
            match self.peco_payload(&cfg, bg.as_deref(), live) {
                Ok(p) => p,
                Err(e) => {
                    self.settings_error(&pecofence_core::i18n::format("保存主题失败：{0}", &[e]));
                    return false;
                }
            };
        let bg_slice = bg_entry.as_ref().map(|(n, b)| (n.as_str(), b.as_slice()));
        let meta_json = meta
            .source
            .is_some()
            .then(|| serde_json::to_string(&meta).ok())
            .flatten();
        match pecofence_core::peco_theme::write_peco_file(
            dest,
            &config_json,
            meta_json.as_deref(),
            bg_slice,
            &preview_jpg,
        ) {
            Ok(()) => true,
            Err(e) => {
                self.settings_error(&pecofence_core::i18n::format(
                    "保存主题失败：{0}",
                    &[e.to_string()],
                ));
                false
            }
        }
    }

    /// Gallery "Save Current Setup": the live configuration becomes a `.peco` in the theme
    /// library — a card with a generated thumbnail. The active video or wallpaper is always
    /// embedded; there is no way to save a theme without its background. The saved file
    /// becomes the active theme: it is the current setup's own archive now.
    fn save_theme_to_library(&mut self, name: &str) {
        tracing::info!(theme = name, "theme library save requested");
        // Meta first: `My Setup` becomes `My Setup_3840x2160_100%.peco` — the source
        // canvas rides in the filename so the card says what the theme was authored on.
        let bg = self.current_background_source();
        let meta = self.theme_meta(bg.as_deref());
        let mut stem = Self::sanitize_theme_stem(name);
        if let Some(s) = &meta.source {
            stem.push('_');
            stem.push_str(&pecofence_core::adaptive::source_suffix(s));
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
        self.settings_toast(pecofence_core::i18n::text("已保存主题"));
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
        tracing::info!(
            snapshot = %snap.name,
            snapshot_wallpaper = ?snap.wallpaper_path,
            background = ?bg,
            "snapshot export: background source selected"
        );
        match self.peco_payload(&cfg, bg.as_deref(), None) {
            Ok((config_json, bg_entry, preview_jpg)) => {
                self.write_theme_archive(&dest, &config_json, None, bg_entry, &preview_jpg);
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
        let mut dest = dir.join(format!("{stem}.peco"));
        if dest.exists() {
            if same_bytes(src, &dest) {
                return dest;
            }
            for i in 2..100 {
                let candidate = dir.join(format!("{stem} ({i}).peco"));
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

    /// Applies a `.peco` already living in the theme library (a gallery card click).
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
        let today = pecofence_core::config_store::today_yyyy_mm_dd();
        let file_day = std::fs::metadata(&path)
            .and_then(|m| m.modified())
            .ok()
            .and_then(|t| t.duration_since(std::time::UNIX_EPOCH).ok())
            .map(|d| pecofence_core::config_store::yyyy_mm_dd_of(d.as_secs() as i64));
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

    /// Deletes a `.peco` from the theme library. Only files directly inside the library
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
