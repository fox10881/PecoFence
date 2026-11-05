//! Config persistence: atomic writes, `.bak` rotation, load fallbacks (plan §7.5).
//! Historical daily backups under `backups/` are read as a last-resort fallback but are no
//! longer written — the .peco theme file is the single portable artifact.

use crate::geometry::MAX_DIP;
use crate::model::{Config, ItemSourceSpec, NormGeometry, SCHEMA_VERSION};
use std::collections::BTreeMap;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

/// Fences per layout. Enforced where fences are created (`AppState::new_fence`, the IPC create
/// paths) as well as here, so a save can never be refused for it.
pub const MAX_FENCES: usize = 64;
/// Unreadable primaries moved aside by [`ConfigStore::load_reporting`]; the oldest go first.
pub const UNREADABLE_KEPT: usize = 5;
const UNREADABLE_PREFIX: &str = "config.unreadable-";
/// Cap for embedded/extracted snapshot wallpapers — real wallpapers are a few MB; this
/// only stops absurd payloads from being read, decoded or written.
const MAX_WALLPAPER_BYTES: usize = 64 * 1024 * 1024;

#[derive(Debug)]
pub enum LoadOutcome {
    /// Loaded from the primary file.
    Primary(Config),
    /// Primary missing/corrupt; loaded from `.bak` or a daily backup (path given).
    Recovered(Config, PathBuf),
    /// Nothing usable: fresh default (first run or total loss).
    Fresh(Config),
}

impl LoadOutcome {
    pub fn into_config(self) -> Config {
        match self {
            LoadOutcome::Primary(c) | LoadOutcome::Recovered(c, _) | LoadOutcome::Fresh(c) => c,
        }
    }
}

/// A failed [`ConfigStore::save`]: the step that failed (a fixed name, safe to log) and why.
#[derive(Debug)]
pub struct SaveError {
    pub step: &'static str,
    pub error: io::Error,
}

impl std::fmt::Display for SaveError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.step, self.error)
    }
}

impl std::error::Error for SaveError {}

pub struct ConfigStore {
    dir: PathBuf,
}

impl ConfigStore {
    pub fn new(dir: impl Into<PathBuf>) -> Self {
        Self { dir: dir.into() }
    }

    /// Reuse an existing installation's data in place. No copying, rewriting or
    /// renaming of user files is needed just because the product name changed.
    pub fn with_legacy(preferred: impl Into<PathBuf>, legacy: impl Into<PathBuf>) -> Self {
        let preferred = Self::new(preferred);
        let legacy = Self::new(legacy);
        if !preferred.has_saved_data() && legacy.has_saved_data() {
            legacy
        } else {
            preferred
        }
    }

    fn has_saved_data(&self) -> bool {
        self.primary_path().exists() || self.bak_path().exists() || !self.list_backups().is_empty()
    }

    pub fn dir(&self) -> &Path {
        &self.dir
    }
    pub fn primary_path(&self) -> PathBuf {
        self.dir.join("config.json")
    }
    fn bak_path(&self) -> PathBuf {
        self.dir.join("config.bak")
    }
    pub fn tmp_path(&self) -> PathBuf {
        self.dir.join("config.json.tmp")
    }
    fn backups_dir(&self) -> PathBuf {
        self.dir.join("backups")
    }

    fn parse(path: &Path) -> Option<Config> {
        Self::parse_file(path).ok()
    }

    /// Reads and validates any config file (import / backup restore), with the reason on error.
    /// Sidecar files ([`WIDGET_CONFIG_FILE`], [`LIVE_EFFECTS_CONFIG_FILE`]) sitting next to
    /// it overlay their owned keys — covering both the live config directory and a `.peco`
    /// theme's extracted contents.
    pub fn parse_file(path: &Path) -> Result<Config, String> {
        let text = fs::read_to_string(path).map_err(|e| e.to_string())?;
        // Windows PowerShell 5.1 `Set-Content -Encoding utf8` and Notepad can add a BOM.
        let text = text.strip_prefix('\u{feff}').unwrap_or(&text);
        let mut doc: serde_json::Value = serde_json::from_str(text).map_err(|e| e.to_string())?;
        overlay_split_files(path, &mut doc);
        let cfg: Config = serde_json::from_value(doc).map_err(|e| e.to_string())?;
        validate(&cfg)?;
        Ok(migrate(cfg))
    }

    /// Writes the config as pretty JSON to an arbitrary path (export). Snapshots whose
    /// `wallpaperPath` resolves to a readable file additionally carry `wallpaperBase64`,
    /// a copy of the image, so the file is self-contained on another machine. The payload
    /// exists only in the exported document — it is never written to `config.json`
    /// ([`Self::save`] goes through [`Self::to_json`], not this function).
    pub fn export_to(cfg: &Config, path: &Path) -> io::Result<()> {
        fs::write(path, Self::embed_wallpapers(&Self::to_json(cfg)?))
    }

    /// Inserts a `wallpaperBase64` string into every snapshot whose `wallpaperPath` is a
    /// readable file. Anything that cannot be read is left as a plain path reference;
    /// missing or oversized images never fail the export itself.
    fn embed_wallpapers(text: &str) -> String {
        if !text.contains("\"wallpaperPath\"") {
            return text.into();
        }
        let Ok(mut doc) = serde_json::from_str::<serde_json::Value>(text) else {
            return text.into();
        };
        let mut touched = false;
        if let Some(snaps) = doc
            .get_mut("snapshots")
            .and_then(serde_json::Value::as_array_mut)
        {
            for snap in snaps.iter_mut() {
                let Some(obj) = snap.as_object_mut() else {
                    continue;
                };
                let Some(wp) = obj
                    .get("wallpaperPath")
                    .and_then(serde_json::Value::as_str)
                    .map(str::to_owned)
                else {
                    continue;
                };
                if fs::metadata(&wp).map(|m| m.len()).unwrap_or(u64::MAX)
                    > MAX_WALLPAPER_BYTES as u64
                {
                    continue;
                }
                let Ok(bytes) = fs::read(&wp) else { continue };
                obj.insert(
                    "wallpaperBase64".into(),
                    serde_json::Value::String(crate::base64::encode(&bytes)),
                );
                touched = true;
            }
        }
        if !touched {
            return text.into();
        }
        serde_json::to_string_pretty(&doc).unwrap_or_else(|_| text.into())
    }

    /// Like [`Self::parse_file`], but for files that may come from another machine:
    /// `wallpaperBase64` payloads (see [`Self::export_to`]) are written out under
    /// `imported_wallpapers/` next to `config.json` and `wallpaperPath` is repointed at
    /// that local copy. The payload key is always removed before the document becomes a
    /// [`Config`], so it can never reach the running configuration.
    pub fn import_file(&self, path: &Path) -> Result<Config, String> {
        let text = fs::read_to_string(path).map_err(|e| e.to_string())?;
        let text = text.strip_prefix('\u{feff}').unwrap_or(&text);
        let mut doc: serde_json::Value = serde_json::from_str(text).map_err(|e| e.to_string())?;
        self.materialize_wallpapers(&mut doc);
        overlay_split_files(path, &mut doc);
        let cfg: Config = serde_json::from_value(doc).map_err(|e| e.to_string())?;
        validate(&cfg)?;
        Ok(migrate(cfg))
    }

    /// Extracts every snapshot's `wallpaperBase64` into `imported_wallpapers/<id>.<ext>`
    /// and rewrites `wallpaperPath`. Undecodable or unwritable payloads are dropped and
    /// the original `wallpaperPath` is kept — a stale reference is no worse than today.
    fn materialize_wallpapers(&self, doc: &mut serde_json::Value) {
        let Some(snaps) = doc
            .get_mut("snapshots")
            .and_then(serde_json::Value::as_array_mut)
        else {
            return;
        };
        for snap in snaps.iter_mut() {
            let Some(obj) = snap.as_object_mut() else {
                continue;
            };
            let Some(payload) = obj.remove("wallpaperBase64") else {
                continue;
            };
            let Some(b64) = payload.as_str() else {
                continue;
            };
            // ~3/4 of the base64 length; refuse absurd payloads before decoding.
            if b64.len() > MAX_WALLPAPER_BYTES * 4 / 3 + 8 {
                continue;
            }
            let Some(bytes) = crate::base64::decode(b64) else {
                continue;
            };
            let id = obj
                .get("id")
                .and_then(serde_json::Value::as_str)
                .map(|s| {
                    s.chars()
                        .filter(|c| c.is_ascii_alphanumeric() || *c == '-')
                        .collect::<String>()
                })
                .filter(|s| !s.is_empty())
                .unwrap_or_else(|| uuid::Uuid::new_v4().to_string());
            let ext = obj
                .get("wallpaperPath")
                .and_then(serde_json::Value::as_str)
                .and_then(|p| Path::new(p).extension())
                .and_then(|e| e.to_str())
                .map(|e| e.to_lowercase())
                .filter(|e| {
                    ["jpg", "jpeg", "png", "bmp", "gif", "webp", "tif", "tiff"]
                        .contains(&e.as_str())
                })
                .unwrap_or_else(|| "jpg".into());
            let dir = self.dir.join("imported_wallpapers");
            let target = dir.join(format!("{id}.{ext}"));
            if fs::create_dir_all(&dir).is_err() || fs::write(&target, &bytes).is_err() {
                continue;
            }
            obj.insert(
                "wallpaperPath".into(),
                serde_json::Value::String(target.to_string_lossy().into_owned()),
            );
        }
    }

    /// The file text: pretty JSON with the `$schema` field pointing at the published schema
    /// (serde_json sorts keys, and `$` sorts first, so it is the first line after the brace).
    pub fn to_json(cfg: &Config) -> io::Result<String> {
        if cfg.schema.as_deref() == Some(crate::model::CONFIG_SCHEMA_URL) {
            // A loaded config already carries it: no copy of the whole config per save.
            return serde_json::to_string_pretty(cfg).map_err(io::Error::other);
        }
        let mut cfg = cfg.clone();
        cfg.schema = Some(crate::model::CONFIG_SCHEMA_URL.to_string());
        serde_json::to_string_pretty(&cfg).map_err(io::Error::other)
    }

    /// Loads with fallbacks: primary → .bak → newest legacy daily backup → default.
    pub fn load(&self) -> LoadOutcome {
        self.load_reporting().0
    }

    /// [`Self::load`], plus where an unreadable primary was moved. A `config.json` that exists
    /// but does not parse or validate (written by a newer version, or hand-edited) is renamed
    /// to `config.unreadable-<date>-<secs>.json` first: the `.bak` rotation in [`Self::save`]
    /// would otherwise turn it into `config.bak` and delete it on the next save.
    pub fn load_reporting(&self) -> (LoadOutcome, Option<PathBuf>) {
        let primary = self.primary_path();
        // Antivirus, backup and sync tools can hold the file open without read sharing for a
        // moment (ERROR_SHARING_VIOLATION, ERROR_LOCK_VIOLATION); wait for them before the
        // file counts as unreadable.
        for attempt in 1..=5 {
            match fs::File::open(&primary) {
                Err(e) if attempt < 5 && matches!(e.raw_os_error(), Some(32 | 33)) => {
                    std::thread::sleep(std::time::Duration::from_millis(300))
                }
                _ => break,
            }
        }
        let quarantined = match Self::parse_file(&primary) {
            Ok(c) => return (LoadOutcome::Primary(c), None),
            Err(_) if !primary.exists() => None,
            Err(_) => self.quarantine(&primary),
        };
        (self.load_fallbacks(), quarantined)
    }

    fn quarantine(&self, primary: &Path) -> Option<PathBuf> {
        let secs = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0);
        let to = self.dir.join(format!(
            "{UNREADABLE_PREFIX}{}-{secs}.json",
            today_yyyy_mm_dd()
        ));
        // A copy still protects it when the rename is refused (the save then rotates the
        // original into .bak as before, but this copy is never touched).
        if fs::rename(primary, &to).is_err() && fs::copy(primary, &to).is_err() {
            return None;
        }
        let mut kept: Vec<PathBuf> = fs::read_dir(&self.dir)
            .map(|rd| {
                rd.filter_map(|e| e.ok().map(|e| e.path()))
                    .filter(|p| {
                        p.file_name()
                            .and_then(|n| n.to_str())
                            .is_some_and(|n| n.starts_with(UNREADABLE_PREFIX))
                    })
                    .collect()
            })
            .unwrap_or_default();
        kept.sort();
        while kept.len() > UNREADABLE_KEPT {
            let _ = fs::remove_file(kept.remove(0));
        }
        Some(to)
    }

    fn load_fallbacks(&self) -> LoadOutcome {
        if let Some(c) = Self::parse(&self.bak_path()) {
            return LoadOutcome::Recovered(c, self.bak_path());
        }
        let mut backups = self.list_backups();
        backups.sort();
        for p in backups.into_iter().rev() {
            if let Some(c) = Self::parse(&p) {
                return LoadOutcome::Recovered(c, p);
            }
        }
        LoadOutcome::Fresh(Config::default())
    }

    /// Legacy daily backups (`backups/YYYY-MM-DD.json`), unsorted. No longer written;
    /// kept readable so older installs can still restore them via `backup restore`.
    pub fn list_backups(&self) -> Vec<PathBuf> {
        fs::read_dir(self.backups_dir())
            .map(|rd| {
                rd.filter_map(|e| e.ok().map(|e| e.path()))
                    .filter(|p| p.extension().is_some_and(|e| e == "json"))
                    .collect()
            })
            .unwrap_or_default()
    }

    /// Atomically writes the config: tmp → fsync → rotate old to .bak → rename tmp over primary.
    pub fn save(&self, cfg: &Config) -> Result<(), SaveError> {
        let at = |step: &'static str| move |error: io::Error| SaveError { step, error };
        validate(cfg)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
            .map_err(at("validate"))?;
        fs::create_dir_all(&self.dir).map_err(at("create folder"))?;
        let json = Self::to_json(cfg).map_err(at("serialize"))?;
        // `config.json` keeps the lean document; the modder-facing keys go
        // to `Widget_config.json` / `Live_Effects_config.json` beside it.
        let split = split_config_json(&json);
        {
            let mut f = fs::File::create(self.tmp_path()).map_err(at("create temp file"))?;
            io::Write::write_all(&mut f, split.main.as_bytes()).map_err(at("write temp file"))?;
            f.sync_all().map_err(at("flush temp file"))?;
        }
        let primary = self.primary_path();
        let mut rotated = false;
        if primary.exists() {
            // Best effort: keep the previous good file as .bak.
            let _ = fs::remove_file(self.bak_path());
            rotated = fs::rename(&primary, self.bak_path()).is_ok();
        }
        match fs::rename(self.tmp_path(), &primary) {
            Ok(()) => {}
            // The Store edition sees %APPDATA% merged with its package's copy, and Windows
            // refuses renames in it when %APPDATA% was moved to another drive (the .bak
            // rotation above fails too). Write the file in place, keeping the old one as .bak.
            // Without that copy a torn write would leave nothing readable, so it must succeed;
            // the new file is flushed like the temp file before the temp file goes.
            Err(e) if e.kind() == io::ErrorKind::CrossesDevices => {
                if primary.exists() {
                    fs::copy(&primary, self.bak_path()).map_err(at("keep config.bak"))?;
                }
                let mut f = fs::File::create(&primary).map_err(at("write config.json"))?;
                io::Write::write_all(&mut f, split.main.as_bytes())
                    .map_err(at("write config.json"))?;
                f.sync_all().map_err(at("flush config.json"))?;
                let _ = fs::remove_file(self.tmp_path());
            }
            Err(e) => {
                // Put the previous file back: a load never looks at the temp file.
                if rotated {
                    let _ = fs::rename(self.bak_path(), &primary);
                }
                return Err(at("replace config.json")(e));
            }
        }
        // The modder-facing sidecars are regenerated from `cfg` every save —
        // a write failure logs but doesn't fail the (already complete) save.
        // Authored deltas still on disk are merged back in first so a
        // hand-edit made while the app was running survives the save instead
        // of being stomped by the (stale) in-memory view.
        for (name, text, dict_key) in [
            (WIDGET_CONFIG_FILE, &split.widget, WIDGET_DICT_KEY),
            (LIVE_EFFECTS_CONFIG_FILE, &split.effects, EFFECTS_DICT_KEY),
        ] {
            let mut doc: serde_json::Value =
                serde_json::from_str(text).unwrap_or_else(|_| serde_json::json!({}));
            merge_disk_sidecar(&mut doc, &self.dir.join(name), dict_key);
            let text = serde_json::to_string_pretty(&doc).unwrap_or_else(|_| text.clone());
            if let Err(e) = fs::write(self.dir.join(name), text) {
                tracing::warn!(file = name, error = %e, "config sidecar write failed");
            }
        }
        Ok(())
    }
}

/// Modder-facing split files that live next to `config.json` (and inside
/// `.peco` theme archives): the clock-widget surface and the live-effects
/// tuning surface, in plain human-editable JSON.
pub const WIDGET_CONFIG_FILE: &str = "Widget_config.json";
pub const LIVE_EFFECTS_CONFIG_FILE: &str = "Live_Effects_config.json";

/// `settings.*` key each sidecar owns — the whole per-id dictionary.
const WIDGET_DICT_KEY: &str = "widgetParams";
const EFFECTS_DICT_KEY: &str = "effectParams";

/// Scalar keys the first split release wrote *inside* the sidecars before
/// they became pure dictionaries — read back into `settings` for migration,
/// so a file written by that build loses nothing.
const WIDGET_LEGACY_SCALARS: &[&str] = &["clockStyle", "clockBounds", "clockAdaptiveColor"];
const EFFECTS_LEGACY_SCALARS: &[&str] = &["desktopEffect", "desktopEffectIntensity"];

/// A serialized [`Config`] broken into the on-disk trio: `main` is the
/// `config.json` text (active selections included); `widget`/`effects`
/// are the per-id parameter dictionaries.
pub struct SplitConfigJson {
    pub main: String,
    pub widget: String,
    pub effects: String,
}

/// Fully-populated baseline for one `Live_Effects_config.json` entry —
/// the compiled defaults rendered as an editable template so a modder
/// sees every knob and its real starting value (e.g. snow's 320-particle
/// base), never an opaque `{}`. `primaryColor: null` = no tint.
fn effect_template(id: &str) -> serde_json::Map<String, serde_json::Value> {
    serde_json::json!({
        "name": crate::effect_display_name(id),
        "particleCount": crate::effect_base_particles(id),
        "speed": 1.0,
        "wind": 0.0,
        "gravity": 0.0,
        "primaryColor": null,
        "glowRadius": 1.0,
    })
    .as_object()
    .cloned()
    .unwrap_or_default()
}

/// Baseline for one `Widget_config.json` entry — `null` means "follow the
/// live/default state", so selecting an unconfigured design never
/// teleports or re-tints the widget. The `elements` block renders every
/// per-element knob (`time`/`day`/`date`) at its neutral baseline. `name`
/// is the design's English label — file-owned, never part of the model.
fn widget_template(id: &str) -> serde_json::Map<String, serde_json::Value> {
    let mut elements = serde_json::Map::new();
    for eid in crate::WIDGET_ELEMENT_IDS {
        elements.insert(
            eid.to_string(),
            serde_json::Value::Object(element_template()),
        );
    }
    serde_json::json!({
        "name": crate::widget_display_name(id),
        "bounds": null,
        "adaptiveColor": null,
        "use24Hour": true,
        "elements": serde_json::Value::Object(elements),
    })
    .as_object()
    .cloned()
    .unwrap_or_default()
}

/// The template a sidecar dict entry is measured against.
fn template_for(dict_key: &str, id: &str) -> serde_json::Map<String, serde_json::Value> {
    if dict_key == WIDGET_DICT_KEY {
        widget_template(id)
    } else {
        effect_template(id)
    }
}

/// Baseline for one `elements.<target>` block — every knob at its neutral
/// value (`scale 1`, no color override, `0` offsets).
fn element_template() -> serde_json::Map<String, serde_json::Value> {
    serde_json::json!({"scale": 1.0, "color": null, "offsetX": 0, "offsetY": 0})
        .as_object()
        .cloned()
        .unwrap_or_default()
}

/// Layers `entry`'s fields over its template — the file always shows the
/// complete schema with user overrides in place. `elements` merges one
/// level deeper: each target renders its full knob set.
fn merged_with_template(dict_key: &str, id: &str, v: &serde_json::Value) -> serde_json::Value {
    let mut merged = template_for(dict_key, id);
    if let Some(obj) = v.as_object() {
        for (k, fv) in obj {
            if dict_key == WIDGET_DICT_KEY && k == "elements" {
                if let (Some(t), Some(e)) = (
                    merged.get_mut("elements").and_then(|t| t.as_object_mut()),
                    fv.as_object(),
                ) {
                    for (eid, ev) in e {
                        let mut em = element_template();
                        if let Some(eo) = ev.as_object() {
                            em.extend(eo.iter().map(|(k2, v2)| (k2.clone(), v2.clone())));
                        }
                        t.insert(eid.clone(), serde_json::Value::Object(em));
                    }
                    continue;
                }
            }
            merged.insert(k.clone(), fv.clone());
        }
    }
    serde_json::Value::Object(merged)
}

/// Strips fields equal to the template — a pristine template entry (or a
/// field the user set back to its baseline) is semantically identical to
/// absence, so memory and `config export` stay delta-sparse.
fn delta_from_template(
    dict_key: &str,
    id: &str,
    v: &serde_json::Value,
) -> Option<serde_json::Value> {
    let obj = v.as_object()?;
    let tmpl = template_for(dict_key, id);
    let kept: serde_json::Map<String, serde_json::Value> = obj
        .iter()
        .filter_map(|(k, fv)| {
            // `name` is a file-layer label, not model data — a renamed or
            // malformed value can never reach memory (and a canonical one
            // falls out via the template comparison below anyway).
            if k == "name" {
                return None;
            }
            if tmpl.get(k) == Some(fv) {
                return None;
            }
            // `elements` must be an object of per-target objects —
            // a scalar there would fail the whole `WidgetParams` parse,
            // so non-object values (and entries) are dropped here instead.
            if dict_key == WIDGET_DICT_KEY && *k == "elements" {
                let els = fv
                    .as_object()?
                    .iter()
                    .filter(|(_, ev)| ev.is_object())
                    .map(|(ek, ev)| (ek.clone(), ev.clone()))
                    .collect::<serde_json::Map<_, _>>();
                return (!els.is_empty()).then(|| (k.clone(), serde_json::Value::Object(els)));
            }
            Some((k.clone(), fv.clone()))
        })
        .collect();
    (!kept.is_empty()).then_some(serde_json::Value::Object(kept))
}

/// Per-id authored deltas in one sidecar file (`id → delta Value`),
/// template-normalized. Missing or unparseable files contribute nothing.
fn sidecar_delta_values(path: &Path, dict_key: &str) -> BTreeMap<String, serde_json::Value> {
    let Ok(text) = fs::read_to_string(path) else {
        return BTreeMap::new();
    };
    let Ok(disk) = serde_json::from_str::<serde_json::Value>(&text) else {
        return BTreeMap::new();
    };
    let Some(obj) = disk.as_object() else {
        return BTreeMap::new();
    };
    obj.iter()
        .filter_map(|(id, v)| delta_from_template(dict_key, id, v).map(|d| (id.clone(), d)))
        .collect()
}

/// Authored `WidgetParams` deltas still on disk — read at apply/refresh
/// time so a hand-edit made while the app ran applies without a restart.
/// Callers merge via `WidgetParams::merge_disk` — authored knobs: disk
/// wins; live-mirror fields (bounds/adaptiveColor): memory wins.
pub fn read_widget_sidecar_deltas(path: &Path) -> BTreeMap<String, crate::WidgetParams> {
    sidecar_delta_values(path, WIDGET_DICT_KEY)
        .into_iter()
        .filter_map(|(id, v)| {
            serde_json::from_value::<crate::WidgetParams>(v)
                .ok()
                .map(|mut p| {
                    p.sanitize();
                    (id, p)
                })
        })
        .collect()
}

/// Same as [`read_widget_sidecar_deltas`] for `Live_Effects_config.json`.
pub fn read_effect_sidecar_deltas(path: &Path) -> BTreeMap<String, crate::EffectParams> {
    sidecar_delta_values(path, EFFECTS_DICT_KEY)
        .into_iter()
        .filter_map(|(id, v)| {
            serde_json::from_value::<crate::EffectParams>(v)
                .ok()
                .map(|mut p| {
                    p.sanitize();
                    (id, p)
                })
        })
        .collect()
}

/// Gap-fill `emitted` (the sidecar doc about to be written) with authored
/// deltas still on disk at `disk_path`. A hand-edit made while the app was
/// running isn't in memory — without this merge the save would stomp it
/// back to the template. Emitted (memory) fields win every conflict; disk
/// only supplies keys the emitted entry lacks, and `elements` merges one
/// level deeper per target. `name` never merges — it's file-owned.
fn merge_disk_sidecar(emitted: &mut serde_json::Value, disk_path: &Path, dict_key: &str) {
    let Ok(text) = fs::read_to_string(disk_path) else {
        return;
    };
    let Ok(disk) = serde_json::from_str::<serde_json::Value>(&text) else {
        return;
    };
    let (Some(disk_obj), Some(emitted_obj)) = (disk.as_object(), emitted.as_object_mut()) else {
        return;
    };
    for (id, dv) in disk_obj {
        let Some(delta) = delta_from_template(dict_key, id, dv) else {
            continue;
        };
        let Some(dobj) = delta.as_object() else {
            continue;
        };
        match emitted_obj.get_mut(id).and_then(|e| e.as_object_mut()) {
            Some(eobj) => {
                // "Lacks" = absent OR still sitting at its template
                // baseline — only a value that differs from the baseline
                // is something memory actually authored and must win.
                let tmpl = template_for(dict_key, id);
                for (k, v) in dobj {
                    if dict_key == WIDGET_DICT_KEY && k == "elements" {
                        if let (Some(te), Some(de)) = (
                            eobj.get_mut("elements").and_then(|x| x.as_object_mut()),
                            v.as_object(),
                        ) {
                            let etmpl = element_template();
                            for (tid, tv) in de {
                                match te.get_mut(tid).and_then(|x| x.as_object_mut()) {
                                    Some(to) => {
                                        if let Some(tvo) = tv.as_object() {
                                            for (fk, fv2) in tvo {
                                                let authored = to
                                                    .get(fk)
                                                    .is_some_and(|c| Some(c) != etmpl.get(fk));
                                                if !authored {
                                                    to.insert(fk.clone(), fv2.clone());
                                                }
                                            }
                                        }
                                    }
                                    None => {
                                        te.insert(tid.clone(), tv.clone());
                                    }
                                }
                            }
                            continue;
                        }
                    }
                    let authored = eobj.get(k).is_some_and(|c| Some(c) != tmpl.get(k));
                    if !authored {
                        eobj.insert(k.clone(), v.clone());
                    }
                }
            }
            None => {
                emitted_obj.insert(id.clone(), merged_with_template(dict_key, id, &delta));
            }
        }
    }
}

/// Splits a serialized config document into [`SplitConfigJson`]. The two
/// dictionary keys are *moved* out of `settings`; every catalog id is
/// emitted as a fully-populated template (compiled defaults, user
/// overrides layered on top) so the files are self-documenting, and the
/// active widget's entry mirrors the live `clockBounds`/
/// `clockAdaptiveColor` so its slot stays truthful.
pub fn split_config_json(text: &str) -> SplitConfigJson {
    let mut doc: serde_json::Value = match serde_json::from_str(text) {
        Ok(d) => d,
        Err(_) => {
            return SplitConfigJson {
                main: text.into(),
                widget: "{\n}\n".into(),
                effects: "{\n}\n".into(),
            };
        }
    };
    let mut widget_dict = serde_json::Map::new();
    let mut effects_dict = serde_json::Map::new();
    if let Some(settings) = doc
        .get_mut("settings")
        .and_then(serde_json::Value::as_object_mut)
    {
        if let Some(serde_json::Value::Object(m)) = settings.remove(WIDGET_DICT_KEY) {
            widget_dict = m;
        }
        if let Some(serde_json::Value::Object(m)) = settings.remove(EFFECTS_DICT_KEY) {
            effects_dict = m;
        }
        // Active design: write the live placement/flag into its dict slot.
        if let Some(style) = settings
            .get("clockStyle")
            .and_then(serde_json::Value::as_str)
            .map(str::to_owned)
            && crate::is_clock_widget_id(&style)
        {
            let entry = widget_dict
                .entry(style)
                .or_insert_with(|| serde_json::json!({}));
            if let Some(obj) = entry.as_object_mut() {
                if let Some(b) = settings.get("clockBounds") {
                    obj.insert("bounds".into(), b.clone());
                }
                if let Some(a) = settings.get("clockAdaptiveColor") {
                    obj.insert("adaptiveColor".into(), a.clone());
                }
            }
        }
    }
    // Every existing entry renders fully (template under the user's
    // fields); ids with no entry get the pristine template.
    for (id, v) in widget_dict.iter_mut() {
        *v = merged_with_template(WIDGET_DICT_KEY, id, v);
    }
    for (id, v) in effects_dict.iter_mut() {
        *v = merged_with_template(EFFECTS_DICT_KEY, id, v);
    }
    for id in crate::CLOCK_WIDGET_IDS {
        widget_dict
            .entry(id.to_string())
            .or_insert_with(|| serde_json::Value::Object(widget_template(id)));
    }
    for id in crate::EFFECT_IDS {
        effects_dict
            .entry(id.to_string())
            .or_insert_with(|| serde_json::Value::Object(effect_template(id)));
    }
    SplitConfigJson {
        main: serde_json::to_string_pretty(&doc).unwrap_or_else(|_| text.into()),
        widget: serde_json::to_string_pretty(&serde_json::Value::Object(widget_dict))
            .unwrap_or_else(|_| "{\n}\n".into()),
        effects: serde_json::to_string_pretty(&serde_json::Value::Object(effects_dict))
            .unwrap_or_else(|_| "{\n}\n".into()),
    }
}

/// Merges each sidecar's dictionary into a parsed config document. Files
/// are looked up *beside the config being read* — `%APPDATA%` on boot, an
/// imported export's directory, or a `.peco`'s temp extraction. Entries are
/// partitioned: legacy scalar keys (first-gen sidecar shape) go back to
/// `settings`, everything else accumulates into the dictionary key —
/// entries already in the doc are overridden by the sidecar. Malformed
/// sidecars are skipped, never fatal.
fn overlay_split_files(config_path: &Path, doc: &mut serde_json::Value) {
    let Some(dir) = config_path.parent() else {
        return;
    };
    for (file, dict_key, scalars) in [
        (WIDGET_CONFIG_FILE, WIDGET_DICT_KEY, WIDGET_LEGACY_SCALARS),
        (
            LIVE_EFFECTS_CONFIG_FILE,
            EFFECTS_DICT_KEY,
            EFFECTS_LEGACY_SCALARS,
        ),
    ] {
        let Ok(text) = fs::read_to_string(dir.join(file)) else {
            continue;
        };
        let text = text.strip_prefix('\u{feff}').unwrap_or(&text);
        let side: serde_json::Value = match serde_json::from_str(text) {
            Ok(v) => v,
            Err(e) => {
                tracing::warn!(file, error = %e, "ignoring unparseable config sidecar");
                continue;
            }
        };
        let Some(side_obj) = side.as_object() else {
            continue;
        };
        let Some(root) = doc.as_object_mut() else {
            return;
        };
        if !root.contains_key("settings") {
            root.insert(
                "settings".to_string(),
                serde_json::Value::Object(serde_json::Map::new()),
            );
        }
        let Some(target) = root
            .get_mut("settings")
            .and_then(serde_json::Value::as_object_mut)
        else {
            return;
        };
        let mut dict = serde_json::Map::new();
        for (k, v) in side_obj {
            if scalars.contains(&k.as_str()) {
                target.insert(k.clone(), v.clone());
            } else if k == dict_key {
                // First-gen nested wrapper: {"effectParams": {…}} — unwrap.
                if let Some(inner) = v.as_object() {
                    for (k2, v2) in inner {
                        if let Some(delta) = delta_from_template(dict_key, k2, v2) {
                            dict.insert(k2.clone(), delta);
                        }
                    }
                }
            } else if let Some(delta) = delta_from_template(dict_key, k, v) {
                // Delta-normalized merge: fields equal to the baseline (and
                // whole pristine template entries) are semantically absent,
                // so memory stays sparse; non-object entries can't parse as
                // a params struct and are ignored rather than failing the
                // whole config document.
                dict.insert(k.clone(), delta);
            }
        }
        if !dict.is_empty() {
            let dest = target
                .entry(dict_key.to_string())
                .or_insert_with(|| serde_json::json!({}));
            if let Some(d) = dest.as_object_mut() {
                d.extend(dict);
            }
        }
    }
}

/// Range checks from plan §7.5.
pub fn validate(cfg: &Config) -> Result<(), String> {
    if cfg.schema_version == 0 || cfg.schema_version > SCHEMA_VERSION {
        return Err(format!("unsupported schemaVersion {}", cfg.schema_version));
    }
    for layout in &cfg.layouts {
        if layout.fences.len() > MAX_FENCES {
            return Err(format!("too many fences: {}", layout.fences.len()));
        }
        let inboxes = layout
            .fences
            .iter()
            .filter(|f| f.kind == crate::model::FenceKind::Inbox)
            .count();
        if inboxes > 1 {
            return Err(format!("layout has {inboxes} inbox fences"));
        }
        for f in &layout.fences {
            let g = &f.geometry;
            if !(g.w.is_finite() && g.h.is_finite() && g.x.is_finite() && g.y.is_finite()) {
                return Err(format!("fence {} has non-finite geometry", f.title));
            }
            // The presets are 0.55 ("更透明"), 1.6 ("更厚实" = a veil) and 0 ("全透明").
            if let Some(a) = &f.appearance
                && let Some(o) = a.opacity
                && o != 0.0
                && !(0.2..=2.0).contains(&o)
            {
                return Err(format!("fence {} opacity {o} out of range", f.title));
            }
        }
    }
    Ok(())
}

/// `g`'s size clamped to 1 to 8192 DIP each way.
fn size_in_range(g: &NormGeometry) -> (f32, f32) {
    (g.w.clamp(1.0, MAX_DIP), g.h.clamp(1.0, MAX_DIP))
}

/// Brings fence sizes outside 1 to 8192 DIP (a hand-edited file) into that range, for the app
/// to load or import the file with just those sizes changed. True when one changed.
pub fn clamp_sizes(cfg: &mut Config) -> bool {
    let mut changed = false;
    for f in cfg.layouts.iter_mut().flat_map(|l| &mut l.fences) {
        let (w, h) = size_in_range(&f.geometry);
        if (w, h) != (f.geometry.w, f.geometry.h) {
            (f.geometry.w, f.geometry.h) = (w, h);
            changed = true;
        }
    }
    changed
}

/// 0.2.1 saved a portal on a network share or a mapped drive as the relative path
/// `UNC\server\share\…` (the canonical path with only its long-path prefix stripped), which
/// can neither be listed nor watched: such paths, in the layouts and the snapshots, become
/// `\\server\share\…`. True when one changed.
pub fn repair_unc_portals(cfg: &mut Config) -> bool {
    let mut changed = false;
    let layouts = cfg
        .layouts
        .iter_mut()
        .chain(cfg.snapshots.iter_mut().flat_map(|s| &mut s.layouts));
    for f in layouts.flat_map(|l| &mut l.fences) {
        if let ItemSourceSpec::Folder { path, .. } = &mut f.source
            && let Some(rest) = path.strip_prefix(r"UNC\")
        {
            *path = format!(r"\\{rest}");
            changed = true;
        }
    }
    changed
}

/// Severity of a [`lint`] finding.
#[derive(Clone, Copy, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum LintLevel {
    /// The app would refuse or silently drop this.
    Error,
    /// Loads, but something will not behave as the file suggests.
    Warning,
}

/// One finding of [`lint`].
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Lint {
    pub level: LintLevel,
    pub message: String,
    /// What the finding is about (`rule:<name>`, `fence:<title>`, `layout:<n>`, `file`).
    pub subject: String,
}

fn lint_at(out: &mut Vec<Lint>, level: LintLevel, subject: impl Into<String>, message: String) {
    out.push(Lint {
        level,
        message,
        subject: subject.into(),
    });
}

/// Cross-checks a configuration that already passed [`validate`] for things that load but will
/// not work as written: rules whose target fence exists in no layout, portals whose folder is
/// gone, tabs hosted by a missing fence, duplicate fence ids, fence sizes the app clamps,
/// memberships of unknown items, more snapshots than the app keeps, a missing or foreign
/// `$schema`. Pure apart from the folder existence checks; safe to run on an exported file
/// without the app.
pub fn lint(cfg: &Config) -> Vec<Lint> {
    use crate::model::FenceKind;
    use crate::rules::Target;
    let mut out = Vec::new();
    match cfg.schema.as_deref() {
        None => lint_at(
            &mut out,
            LintLevel::Warning,
            "file",
            format!(
                "no \"$schema\" field; add {:?} for editor validation",
                crate::model::CONFIG_SCHEMA_URL
            ),
        ),
        Some(url) if url != crate::model::CONFIG_SCHEMA_URL => lint_at(
            &mut out,
            LintLevel::Warning,
            "file",
            format!(
                "\"$schema\" is {url:?}; PecoFence publishes {:?}",
                crate::model::CONFIG_SCHEMA_URL
            ),
        ),
        Some(_) => {}
    }
    if cfg.layouts.is_empty() {
        lint_at(
            &mut out,
            LintLevel::Warning,
            "file",
            "no layouts: the app will create a default one on load".into(),
        );
    }
    let mut all_fences: Vec<(&crate::model::Fence, usize)> = Vec::new();
    for (li, layout) in cfg.layouts.iter().enumerate() {
        let mut seen = std::collections::HashSet::new();
        for f in &layout.fences {
            if !seen.insert(f.id) {
                lint_at(
                    &mut out,
                    LintLevel::Error,
                    format!("fence:{}", f.title),
                    format!("fence id {} appears twice in layout {li}", f.id),
                );
            }
            all_fences.push((f, li));
        }
        for f in &layout.fences {
            let subject = format!("fence:{}", f.title);
            if let Some(host) = f.tab_host {
                match layout.fences.iter().find(|h| h.id == host) {
                    None => lint_at(
                        &mut out,
                        LintLevel::Warning,
                        &subject,
                        format!(
                            "hosted by tab host {host}, which is not in layout {li}; shown as its own window"
                        ),
                    ),
                    Some(h) if h.tab_host.is_some() => lint_at(
                        &mut out,
                        LintLevel::Warning,
                        &subject,
                        format!("tab host {:?} is itself a tab", h.title),
                    ),
                    Some(_) => {}
                }
            }
            if let crate::model::ItemSourceSpec::Folder { path, .. } = &f.source
                && !Path::new(path).is_dir()
            {
                lint_at(
                    &mut out,
                    LintLevel::Warning,
                    &subject,
                    format!(
                        "portal folder {path:?} does not exist (or is not reachable from here)"
                    ),
                );
            }
            let g = &f.geometry;
            let (w, h) = size_in_range(g);
            if (w, h) != (g.w, g.h) {
                lint_at(
                    &mut out,
                    LintLevel::Warning,
                    &subject,
                    format!(
                        "size {}x{} is out of range; the app changes it to {w}x{h}",
                        g.w, g.h
                    ),
                );
            }
            if f.kind == FenceKind::FolderPortal
                && !matches!(f.source, crate::model::ItemSourceSpec::Folder { .. })
            {
                lint_at(
                    &mut out,
                    LintLevel::Error,
                    &subject,
                    "portal fence without a folder source".into(),
                );
            }
            let unknown = f
                .items
                .iter()
                .filter(|r| !cfg.items.contains_key(&r.item_id))
                .count();
            if unknown > 0 {
                lint_at(
                    &mut out,
                    LintLevel::Warning,
                    &subject,
                    format!(
                        "{unknown} membership entr{} refer to items missing from \"items\"; they are ignored",
                        if unknown == 1 { "y" } else { "ies" }
                    ),
                );
            }
        }
    }
    for rule in &cfg.rules.list {
        if let Target::Fence(id) = rule.target {
            match all_fences.iter().find(|(f, _)| f.id == id) {
                None => lint_at(
                    &mut out,
                    LintLevel::Warning,
                    format!("rule:{}", rule.name),
                    format!(
                        "target fence {id} exists in no layout; matches fall back to the inbox"
                    ),
                ),
                Some((f, _)) if f.kind == FenceKind::FolderPortal => lint_at(
                    &mut out,
                    LintLevel::Error,
                    format!("rule:{}", rule.name),
                    format!(
                        "target {:?} is a folder portal; rules can only target virtual fences or the inbox",
                        f.title
                    ),
                ),
                Some(_) => {}
            }
        }
        if rule.all_of.is_empty() {
            lint_at(
                &mut out,
                LintLevel::Warning,
                format!("rule:{}", rule.name),
                "no conditions: the rule never matches".into(),
            );
        }
    }
    if let Target::Fence(id) = cfg.rules.default_target
        && !all_fences.iter().any(|(f, _)| f.id == id)
    {
        lint_at(
            &mut out,
            LintLevel::Warning,
            "rules",
            format!(
                "defaultTarget fence {id} exists in no layout; unmatched items go to the inbox"
            ),
        );
    }
    if let Some(text) = &cfg.settings.peek.custom_hotkey
        && let Err(problem) = crate::hotkey::Hotkey::parse(text).and_then(|h| h.check())
    {
        lint_at(
            &mut out,
            LintLevel::Warning,
            "settings",
            format!("peek.customHotkey {text:?} is not used ({problem}); Peek uses peek.hotkey"),
        );
    }
    if cfg.snapshots.len() > crate::model::MAX_SNAPSHOTS {
        lint_at(
            &mut out,
            LintLevel::Warning,
            "snapshots",
            format!(
                "{} snapshots; the app keeps at most {}",
                cfg.snapshots.len(),
                crate::model::MAX_SNAPSHOTS
            ),
        );
    }
    out
}

/// Schema migration chain, and choices this version no longer offers.
pub fn migrate(mut cfg: Config) -> Config {
    if cfg.schema_version < SCHEMA_VERSION {
        cfg.schema_version = SCHEMA_VERSION;
    }
    // 0.2.0 offered Win+Space and Win+Shift+Space, which Windows keeps for the input switcher:
    // Peek never got them, and a file still choosing one showed the fallback notice at every
    // start. It takes the default (a usable `customHotkey` stays the choice).
    let peek = &mut cfg.settings.peek;
    if peek.chosen_hotkey() == peek.hotkey.combo() {
        peek.hotkey = crate::model::PeekHotkey::CtrlAltSpace;
    }
    // `stretchTolerancePct` is gone — a negative value meant strict Contain; any
    // non-negative value belonged to the distortion engine, which strict Cover
    // replaces. `skip_serializing` drops the legacy key on the next save.
    if cfg.settings.stretch_tolerance_pct < 0 {
        cfg.settings.contain_background = true;
    }
    cfg.settings.stretch_tolerance_pct = 0;
    dedupe_restore_backups(&mut cfg);
    // User-layer widget dictionary resolves onto the runtime fields at
    // parse — every load/import applies the active design's entry.
    cfg.settings.resolve_widget_params();
    cfg
}

/// The automatic "…前" backups used to append an entry (in whatever language the UI was in)
/// on every restore or import; they are now the single [`AUTO_BACKUP_SNAPSHOT_ID`] entry.
/// Fold the saved duplicates — any locale's label for the three auto names, or the raw
/// i18n keys newer files store — into the newest one and drop the rest.
pub fn dedupe_restore_backups(cfg: &mut Config) {
    use crate::model::AUTO_BACKUP_SNAPSHOT_ID;
    const KEYS: [&str; 3] = ["恢复快照前", "导入配置前", "恢复备份前"];
    let names: std::collections::HashSet<&str> = KEYS
        .iter()
        .copied()
        .chain(crate::i18n::SUPPORTED.iter().flat_map(|&l| {
            KEYS.iter()
                .filter_map(move |k| crate::i18n::catalog(l).get(*k).map(String::as_str))
        }))
        .collect();
    let is_backup = |s: &crate::model::Snapshot| {
        s.id == AUTO_BACKUP_SNAPSHOT_ID
            || s.is_auto_backup == Some(true)
            || names.contains(s.name.as_str())
    };
    let Some(keep_id) = cfg
        .snapshots
        .iter()
        .filter(|s| is_backup(s))
        .max_by_key(|s| s.ts)
        .map(|s| s.id)
    else {
        return;
    };
    cfg.snapshots.retain(|s| !is_backup(s) || s.id == keep_id);
    if let Some(s) = cfg.snapshots.iter_mut().find(|s| s.id == keep_id) {
        s.id = AUTO_BACKUP_SNAPSHOT_ID;
        s.is_auto_backup = Some(true);
    }
}

/// Local-date string without pulling in a date crate (civil-from-days algorithm, UTC-based;
/// backups are named for humans so the timezone offset is irrelevant).
pub fn today_yyyy_mm_dd() -> String {
    let secs = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs() as i64)
        .unwrap_or(0);
    yyyy_mm_dd_of(secs)
}

/// `yyyy_mm_dd` of a Unix timestamp (same civil-from-days math as [`today_yyyy_mm_dd`]) —
/// used to compare a file's mtime day against "today" without a date crate.
pub fn yyyy_mm_dd_of(secs: i64) -> String {
    let days = secs.div_euclid(86_400);
    let z = days + 719_468;
    let era = z.div_euclid(146_097);
    let doe = z - era * 146_097;
    let yoe = (doe - doe / 1460 + doe / 36_524 - doe / 146_096) / 365;
    let y = yoe + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = doy - (153 * mp + 2) / 5 + 1;
    let m = if mp < 10 { mp + 3 } else { mp - 9 };
    let y = if m <= 2 { y + 1 } else { y };
    format!("{y:04}-{m:02}-{d:02}")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::*;

    fn tmpdir(name: &str) -> PathBuf {
        let d = std::env::temp_dir().join(format!("pecofence-test-{name}-{}", std::process::id()));
        let _ = fs::remove_dir_all(&d);
        d
    }

    fn sample() -> Config {
        let mut c = Config::default();
        c.layouts.push(Layout {
            desktop_items: Default::default(),
            shown: 0,
            fingerprint: vec![],
            fences: vec![Fence::new(
                "A",
                FenceKind::Inbox,
                NormGeometry {
                    monitor: "m".into(),
                    x: 1.0,
                    y: 2.0,
                    w: 300.0,
                    h: 200.0,
                    work_w: 1920.0,
                    work_h: 1000.0,
                    anchor: Anchor::LeftTop,
                    skew_left: None,
                    skew_right: None,
                },
            )],
        });
        c
    }

    #[test]
    fn split_keeps_active_selection_and_moves_dicts() {
        let mut c = sample();
        c.settings.desktop_effect = "rain".into();
        c.settings.desktop_effect_intensity = 80;
        c.settings.clock_style = "d1".into();
        c.settings.clock_bounds = Some([10, 20, 400, 240]);
        c.settings.clock_adaptive_color = false;
        c.settings.effect_params.insert(
            "rain".into(),
            EffectParams {
                particle_count: Some(512),
                speed: Some(2.0),
                ..Default::default()
            },
        );

        let split = split_config_json(&ConfigStore::to_json(&c).unwrap());
        let main: serde_json::Value = serde_json::from_str(&split.main).unwrap();
        let s = &main["settings"];
        // Active selections and live widget state stay in config.json.
        for k in [
            "clockStyle",
            "clockBounds",
            "clockAdaptiveColor",
            "desktopEffect",
            "desktopEffectIntensity",
        ] {
            assert!(s.get(k).is_some(), "{k} must stay in config.json");
        }
        // The dictionaries move out to the sidecars.
        assert!(s.get("effectParams").is_none());
        assert!(s.get("widgetParams").is_none());

        // Sidecars are flat per-id dictionaries — every catalog id is
        // emitted as a fully-populated template so the file itself
        // documents every knob and its compiled baseline.
        let effects: serde_json::Value = serde_json::from_str(&split.effects).unwrap();
        for id in crate::EFFECT_IDS {
            assert!(effects.get(*id).is_some(), "missing effect entry {id}");
        }
        assert!(effects.get("desktopEffect").is_none());
        // User overrides layer on the full template — every field visible.
        assert_eq!(effects["rain"]["particleCount"], 512);
        assert_eq!(effects["rain"]["speed"], 2.0);
        assert_eq!(effects["rain"]["wind"], 0.0);
        assert_eq!(effects["rain"]["glowRadius"], 1.0);
        // Untouched ids render the compiled baseline (snow's real
        // 320-particle base, not "{}").
        assert_eq!(
            effects["snow"],
            serde_json::json!({
                "name": "Snowfall",
                "particleCount": 320,
                "speed": 1.0,
                "wind": 0.0,
                "gravity": 0.0,
                "primaryColor": null,
                "glowRadius": 1.0
            })
        );

        let widget: serde_json::Value = serde_json::from_str(&split.widget).unwrap();
        for id in crate::CLOCK_WIDGET_IDS {
            assert!(widget.get(*id).is_some(), "missing widget entry {id}");
        }
        assert!(widget.get("clockStyle").is_none());
        // The active design's slot mirrors the live runtime fields.
        assert_eq!(
            widget["d1"]["bounds"],
            serde_json::json!([10, 20, 400, 240])
        );
        assert_eq!(widget["d1"]["adaptiveColor"], false);
        // Untouched designs render the follow-live baseline with the full
        // sub-element schema discoverable in the file.
        assert_eq!(
            widget["g5"],
            serde_json::json!({
                "name": "Liquid Morphic Pill",
                "bounds": null,
                "adaptiveColor": null,
                "use24Hour": true,
                "elements": {
                    "time": {"scale": 1.0, "color": null, "offsetX": 0, "offsetY": 0},
                    "day": {"scale": 1.0, "color": null, "offsetX": 0, "offsetY": 0},
                    "date": {"scale": 1.0, "color": null, "offsetX": 0, "offsetY": 0}
                }
            })
        );
    }

    #[test]
    fn int_scale_parses_and_survives() {
        // Regression: `"scale": 3` (a JSON integer) must land as 3.0 —
        // serde deserializes ints into f32, and the delta/merge path must
        // not collapse it back to the 1.0 template.
        let mut v = crate::WidgetElementParams::default();
        let raw: serde_json::Value =
            serde_json::from_str(r#"{"scale":3,"offsetX":20,"offsetY":-15}"#).unwrap();
        let p: crate::WidgetElementParams = serde_json::from_value(raw).unwrap();
        v.merge_disk(&p);
        assert_eq!(v.scale, Some(3.0));
        assert_eq!(v.offset_x, Some(20.0));
        assert_eq!(v.offset_y, Some(-15.0));
    }

    #[test]
    fn save_gap_fills_disk_authored_deltas() {
        // The stomp bug: user hand-edits the sidecar while the app runs,
        // then any unrelated save used to rewrite the file from stale
        // memory. Now the save merges disk deltas back (memory wins
        // conflicts) so authored edits survive.
        let store = ConfigStore::new(tmpdir("disk-merge-save"));
        fs::create_dir_all(store.dir()).unwrap();
        let mut c = sample();
        c.settings.clock_style = "g10".into();
        // Seed the file with a hand-edit memory doesn't know about.
        fs::write(
            store.dir().join(WIDGET_CONFIG_FILE),
            r##"{"g10":{"elements":{"time":{"scale":2.5,"color":"#FF8800"}}},
                 "d3":{"elements":{"day":{"offsetX":40}}}}"##,
        )
        .unwrap();
        // Memory carries only the live mirror — no elements.
        c.settings.widget_params.insert(
            "g10".into(),
            crate::WidgetParams {
                bounds: Some([5, 5, 400, 200]),
                adaptive_color: Some(true),
                ..Default::default()
            },
        );
        store.save(&c).unwrap();
        let disk: serde_json::Value = serde_json::from_str(
            &fs::read_to_string(store.dir().join(WIDGET_CONFIG_FILE)).unwrap(),
        )
        .unwrap();
        // Disk-authored elements survived the save under the live mirror.
        assert_eq!(disk["g10"]["elements"]["time"]["scale"], 2.5);
        assert_eq!(disk["g10"]["elements"]["time"]["color"], "#FF8800");
        assert_eq!(disk["g10"]["bounds"], serde_json::json!([5, 5, 400, 200]));
        // Unvisited design's authored edit survived too.
        assert_eq!(disk["d3"]["elements"]["day"]["offsetX"], 40);
    }

    #[test]
    fn display_names_are_file_only_never_model() {
        // A renamed/malformed/deleted `name` must not reach memory or
        // break the parse — it's a label owned by the file layer.
        let store = ConfigStore::new(tmpdir("name-labels"));
        fs::create_dir_all(store.dir()).unwrap();
        fs::write(
            store.primary_path(),
            ConfigStore::to_json(&sample()).unwrap(),
        )
        .unwrap();
        fs::write(
            store.dir().join(WIDGET_CONFIG_FILE),
            r#"{"g10":{"name":"My Renamed Label","bounds":[1,2,300,200]},
                "d3":{"name":42},"g5":{}}"#,
        )
        .unwrap();
        fs::write(
            store.dir().join(LIVE_EFFECTS_CONFIG_FILE),
            r#"{"snow":{"name":{"nested":"junk"},"speed":2.5}}"#,
        )
        .unwrap();
        let loaded = match store.load() {
            LoadOutcome::Primary(c) => c,
            other => panic!("unexpected {other:?}"),
        };
        // Params landed; the labels did not.
        assert_eq!(
            loaded.settings.widget_params["g10"].bounds,
            Some([1, 2, 300, 200])
        );
        assert_eq!(loaded.settings.effect_params["snow"].speed, Some(2.5));
        // Re-emitting re-canonicalizes the labels from the table.
        let split = split_config_json(&ConfigStore::to_json(&loaded).unwrap());
        let widget: serde_json::Value = serde_json::from_str(&split.widget).unwrap();
        let effects: serde_json::Value = serde_json::from_str(&split.effects).unwrap();
        assert_eq!(widget["g10"]["name"], "Zen Minimal Ribbon");
        assert_eq!(widget["d3"]["name"], "Chrono Orbit");
        assert_eq!(effects["snow"]["name"], "Snowfall");
    }

    #[test]
    fn widget_elements_merge_over_template_and_normalize_back() {
        let mut c = sample();
        c.settings.clock_style = "g10".into();
        let mut p = crate::WidgetParams::default();
        p.elements.insert(
            "time".to_string(),
            crate::WidgetElementParams {
                scale: Some(1.5),
                color: Some("#FF8800".into()),
                offset_y: Some(-12.0),
                ..Default::default()
            },
        );
        c.settings.widget_params.insert("g10".into(), p);

        let split = split_config_json(&ConfigStore::to_json(&c).unwrap());
        let widget: serde_json::Value = serde_json::from_str(&split.widget).unwrap();
        // Authored fields override; the rest of the element schema fills in.
        assert_eq!(widget["g10"]["elements"]["time"]["scale"], 1.5);
        assert_eq!(widget["g10"]["elements"]["time"]["color"], "#FF8800");
        assert_eq!(widget["g10"]["elements"]["time"]["offsetX"], 0);
        assert_eq!(widget["g10"]["elements"]["day"]["scale"], 1.0);

        let store = ConfigStore::new(tmpdir("els-merge"));
        fs::create_dir_all(store.dir()).unwrap();
        fs::write(store.primary_path(), &split.main).unwrap();
        fs::write(store.dir().join(WIDGET_CONFIG_FILE), &split.widget).unwrap();
        fs::write(store.dir().join(LIVE_EFFECTS_CONFIG_FILE), &split.effects).unwrap();
        match store.load() {
            LoadOutcome::Primary(loaded) => {
                let e = &loaded.settings.widget_params["g10"].elements["time"];
                assert_eq!(e.scale, Some(1.5));
                assert_eq!(e.color.as_deref(), Some("#FF8800"));
                assert_eq!(e.offset_y, Some(-12.0));
            }
            other => panic!("unexpected {other:?}"),
        }

        // Non-object element values are dropped at the file boundary —
        // they can't poison the Config parse.
        fs::write(
            store.dir().join(WIDGET_CONFIG_FILE),
            r#"{"g10":{"elements":{"time":42,"day":{"offsetX":30}}}}"#,
        )
        .unwrap();
        match store.load() {
            LoadOutcome::Primary(loaded) => {
                let els = &loaded.settings.widget_params["g10"].elements;
                assert!(!els.contains_key("time"));
                assert_eq!(els["day"].offset_x, Some(30.0));
            }
            other => panic!("unexpected {other:?}"),
        }
    }

    #[test]
    fn save_writes_three_files_and_load_merges() {
        let store = ConfigStore::new(tmpdir("split3"));
        let mut c = sample();
        c.settings.desktop_effect = "snow".into();
        c.settings.clock_style = "g2".into();
        c.settings.clock_bounds = Some([1, 2, 300, 200]);
        c.settings.effect_params.insert(
            "snow".into(),
            EffectParams {
                particle_count: Some(64),
                ..Default::default()
            },
        );
        store.save(&c).unwrap();

        // config.json carries the active selection; sidecars carry the dicts.
        let main_text = fs::read_to_string(store.primary_path()).unwrap();
        assert!(!main_text.contains("effectParams"));
        assert!(!main_text.contains("widgetParams"));
        assert!(main_text.contains("desktopEffect"));
        assert!(main_text.contains("clockStyle"));
        assert!(store.dir().join(WIDGET_CONFIG_FILE).exists());
        assert!(store.dir().join(LIVE_EFFECTS_CONFIG_FILE).exists());
        let widget: serde_json::Value = serde_json::from_str(
            &fs::read_to_string(store.dir().join(WIDGET_CONFIG_FILE)).unwrap(),
        )
        .unwrap();
        assert_eq!(widget["g2"]["bounds"], serde_json::json!([1, 2, 300, 200]));

        match store.load() {
            LoadOutcome::Primary(loaded) => {
                assert_eq!(loaded.settings.desktop_effect, "snow");
                assert_eq!(loaded.settings.clock_style, "g2");
                assert_eq!(loaded.settings.clock_bounds, Some([1, 2, 300, 200]));
                assert_eq!(
                    loaded.settings.effect_params["snow"].particle_count,
                    Some(64)
                );
            }
            other => panic!("unexpected {other:?}"),
        }
    }

    #[test]
    fn hand_edited_sidecar_overrides_saved_values_on_load() {
        let store = ConfigStore::new(tmpdir("sidecar-edit"));
        let mut c = sample();
        c.settings.desktop_effect = "matrix".into();
        store.save(&c).unwrap();
        // A modder hand-edits the flat dict — boot picks it up (Path 3: no
        // watcher, parse happens at load/apply boundaries only). The active
        // selection stays owned by config.json.
        fs::write(
            store.dir().join(LIVE_EFFECTS_CONFIG_FILE),
            r#"{"matrix":{"particleCount":999},"snow":{"glowRadius":3.0}}"#,
        )
        .unwrap();
        match store.load() {
            LoadOutcome::Primary(c) => {
                assert_eq!(c.settings.desktop_effect, "matrix");
                assert_eq!(c.settings.effect_params["matrix"].particle_count, Some(999));
                assert_eq!(c.settings.effect_params["snow"].glow_radius, Some(3.0));
                // The resolved apply-time value is the sanitized dict entry.
                assert_eq!(
                    c.settings
                        .resolve_effect_params("matrix")
                        .unwrap()
                        .particle_count,
                    Some(999)
                );
            }
            other => panic!("unexpected {other:?}"),
        }
    }

    #[test]
    fn legacy_wrapper_sidecar_migrates() {
        // First-gen sidecars held scalars + a nested dict — they still load.
        let store = ConfigStore::new(tmpdir("legacy-wrap"));
        store.save(&sample()).unwrap();
        fs::write(
            store.dir().join(LIVE_EFFECTS_CONFIG_FILE),
            r#"{"desktopEffect":"aurora","desktopEffectIntensity":70,"effectParams":{"aurora":{"speed":1.5}}}"#,
        )
        .unwrap();
        fs::write(
            store.dir().join(WIDGET_CONFIG_FILE),
            r#"{"clockStyle":"d5","clockBounds":[9,9,500,300],"clockAdaptiveColor":false}"#,
        )
        .unwrap();
        match store.load() {
            LoadOutcome::Primary(c) => {
                assert_eq!(c.settings.desktop_effect, "aurora");
                assert_eq!(c.settings.desktop_effect_intensity, 70);
                assert_eq!(c.settings.effect_params["aurora"].speed, Some(1.5));
                assert_eq!(c.settings.clock_style, "d5");
            }
            other => panic!("unexpected {other:?}"),
        }
    }

    #[test]
    fn widget_dict_entry_resolves_onto_runtime_on_load() {
        let store = ConfigStore::new(tmpdir("widget-resolve"));
        let mut c = sample();
        c.settings.clock_style = "g2".into();
        c.settings.clock_bounds = Some([1, 2, 300, 200]);
        store.save(&c).unwrap();
        // Modder pre-configures g2 with its own spot + adaptive off.
        fs::write(
            store.dir().join(WIDGET_CONFIG_FILE),
            r#"{"g2":{"bounds":[50,60,640,400],"adaptiveColor":false}}"#,
        )
        .unwrap();
        match store.load() {
            LoadOutcome::Primary(c) => {
                assert_eq!(c.settings.clock_bounds, Some([50, 60, 640, 400]));
                assert!(!c.settings.clock_adaptive_color);
                assert_eq!(
                    c.settings.widget_params["g2"].bounds,
                    Some([50, 60, 640, 400])
                );
            }
            other => panic!("unexpected {other:?}"),
        }
    }

    #[test]
    fn corrupt_or_unknown_dict_entries_fall_back() {
        let store = ConfigStore::new(tmpdir("dict-fallback"));
        let mut c = sample();
        c.settings.desktop_effect = "rain".into();
        store.save(&c).unwrap();
        // Out-of-range values and a bogus id — load clamps/drops, no crash.
        fs::write(
            store.dir().join(LIVE_EFFECTS_CONFIG_FILE),
            r#"{"rain":{"particleCount":999999,"primaryColor":"not-a-color"},"bogus_fx":{"speed":9}}"#,
        )
        .unwrap();
        match store.load() {
            LoadOutcome::Primary(c) => {
                // Resolution (the apply boundary) clamps to the safe ranges.
                let p = c.settings.resolve_effect_params("rain").unwrap();
                assert_eq!(p.particle_count, Some(crate::EFFECT_MAX_PARTICLES));
                assert_eq!(p.primary_color, None);
                // sanitize_external (the adopt boundary) drops unknown ids.
                let mut adopted = c.clone();
                adopted.sanitize_external();
                assert!(!adopted.settings.effect_params.contains_key("bogus_fx"));
                // Missing entries resolve to None → compiled engine defaults.
                assert!(c.settings.resolve_effect_params("snow").is_none());
            }
            other => panic!("unexpected {other:?}"),
        }
    }

    #[test]
    fn template_entries_normalize_to_sparse_memory_on_load() {
        let store = ConfigStore::new(tmpdir("tmpl-delta"));
        let mut c = sample();
        c.settings.clock_style = "g10".into();
        store.save(&c).unwrap();
        // The on-disk file shows full templates…
        let effects: serde_json::Value = serde_json::from_str(
            &fs::read_to_string(store.dir().join(LIVE_EFFECTS_CONFIG_FILE)).unwrap(),
        )
        .unwrap();
        assert_eq!(effects["snow"]["particleCount"], 320);
        // …but loading strips baseline-equal fields back to deltas, so
        // memory (and any re-export) stays free of template noise.
        match store.load() {
            LoadOutcome::Primary(c) => {
                assert!(c.settings.effect_params.is_empty());
                // Only the active design's mirrored slot is a real entry.
                assert_eq!(c.settings.widget_params.len(), 1);
                assert!(c.settings.widget_params.contains_key("g10"));
            }
            other => panic!("unexpected {other:?}"),
        }
        // A user editing one field inside the template keeps just that
        // field as the delta — the baseline fields stay implicit.
        fs::write(
            store.dir().join(LIVE_EFFECTS_CONFIG_FILE),
            r#"{"snow":{"particleCount":320,"speed":2.5,"wind":0.0,"gravity":0.0,"primaryColor":null,"glowRadius":1.0}}"#,
        )
        .unwrap();
        match store.load() {
            LoadOutcome::Primary(c) => {
                assert_eq!(
                    serde_json::to_value(&c.settings.effect_params["snow"]).unwrap(),
                    serde_json::json!({"speed": 2.5})
                );
            }
            other => panic!("unexpected {other:?}"),
        }
    }

    #[test]
    fn legacy_monolithic_config_still_parses_fields() {
        // A legacy config.json with the keys embedded — no sidecars — keeps working.
        let store = ConfigStore::new(tmpdir("legacy-mono"));
        fs::create_dir_all(store.dir()).unwrap();
        let mut c = sample();
        c.settings.desktop_effect = "aurora".into();
        c.settings.clock_style = "d7".into();
        // Write the FULL serialized doc (as an old build would have).
        fs::write(store.primary_path(), ConfigStore::to_json(&c).unwrap()).unwrap();
        match store.load() {
            LoadOutcome::Primary(loaded) => {
                assert_eq!(loaded.settings.desktop_effect, "aurora");
                assert_eq!(loaded.settings.clock_style, "d7");
            }
            other => panic!("unexpected {other:?}"),
        }
    }

    #[test]
    fn malformed_sidecar_is_skipped_not_fatal() {
        let store = ConfigStore::new(tmpdir("bad-sidecar"));
        store.save(&sample()).unwrap();
        fs::write(store.dir().join(WIDGET_CONFIG_FILE), "{ not json").unwrap();
        assert!(matches!(store.load(), LoadOutcome::Primary(_)));
    }

    #[test]
    fn save_then_load_primary() {
        let store = ConfigStore::new(tmpdir("primary"));
        store.save(&sample()).unwrap();
        assert!(matches!(store.load(), LoadOutcome::Primary(_)));
        assert!(store.primary_path().exists());
        assert!(!store.tmp_path().exists());
        // One theme = one file: saves rotate config.bak only — no daily JSON backups.
        assert!(store.list_backups().is_empty());
    }

    #[test]
    fn corrupt_primary_falls_back_to_bak() {
        let store = ConfigStore::new(tmpdir("bak"));
        store.save(&sample()).unwrap();
        store.save(&sample()).unwrap(); // creates .bak
        fs::write(store.primary_path(), "{ not json").unwrap();
        match store.load() {
            LoadOutcome::Recovered(c, from) => {
                assert_eq!(c.layouts[0].fences[0].title, "A");
                assert_eq!(from, store.bak_path());
            }
            other => panic!("unexpected {other:?}"),
        }
    }

    #[test]
    fn opacity_presets_validate_and_stray_values_do_not() {
        for (opacity, ok) in [
            (0.0, true),
            (0.55, true),
            (1.6, true),
            (0.1, false),
            (2.5, false),
        ] {
            let mut c = sample();
            c.layouts[0].fences[0].appearance = Some(AppearanceOverride {
                opacity: Some(opacity),
                ..Default::default()
            });
            assert_eq!(validate(&c).is_ok(), ok, "opacity {opacity}");
        }
    }

    #[test]
    fn unreadable_primary_survives_later_saves() {
        // A config written by a newer version: this build cannot read it, falls back to the
        // defaults and saves twice. The original text must still be on disk afterwards.
        let store = ConfigStore::new(tmpdir("unreadable"));
        fs::create_dir_all(store.dir()).unwrap();
        let text = r#"{ "schemaVersion": 99, "marker": "users-layout" }"#;
        fs::write(store.primary_path(), text).unwrap();
        let (outcome, moved) = store.load_reporting();
        assert!(matches!(outcome, LoadOutcome::Fresh(_)));
        let moved = moved.expect("unreadable primary moved aside");
        store.save(&sample()).unwrap();
        store.save(&sample()).unwrap();
        assert_eq!(fs::read_to_string(&moved).unwrap(), text);
        assert!(matches!(store.load(), LoadOutcome::Primary(_)));
    }

    #[test]
    fn primary_with_a_utf8_bom_loads() {
        let store = ConfigStore::new(tmpdir("bom"));
        store.save(&sample()).unwrap();
        let mut bytes = vec![0xEF, 0xBB, 0xBF];
        bytes.extend(fs::read(store.primary_path()).unwrap());
        fs::write(store.primary_path(), bytes).unwrap();
        assert!(ConfigStore::parse_file(&store.primary_path()).is_ok());
        assert!(matches!(store.load(), LoadOutcome::Primary(_)));
    }

    #[test]
    fn export_embeds_wallpaper_and_import_restores_it() {
        let dir = tmpdir("portable-wp");
        fs::create_dir_all(&dir).unwrap();
        let image = dir.join("my-wallpaper.png");
        let bytes: Vec<u8> = (0..5000u32).map(|i| (i * 31) as u8).collect();
        fs::write(&image, &bytes).unwrap();

        let mut cfg = sample();
        cfg.snapshots.push(Snapshot {
            id: uuid::Uuid::new_v4(),
            name: "with wallpaper".into(),
            ts: 100,
            layouts: vec![],
            wallpaper_path: Some(image.to_string_lossy().into_owned()),
            is_auto_backup: None,
        });

        // Export: the image travels inside the JSON.
        let exported = dir.join("exported.json");
        ConfigStore::export_to(&cfg, &exported).unwrap();
        let text = fs::read_to_string(&exported).unwrap();
        assert!(text.contains("\"wallpaperBase64\""));

        // A normal save of the same config must not carry the payload.
        let home = ConfigStore::new(tmpdir("portable-wp-home"));
        home.save(&cfg).unwrap();
        assert!(
            !fs::read_to_string(home.primary_path())
                .unwrap()
                .contains("wallpaperBase64"),
            "save() must stay payload-free"
        );

        // Import on another machine (fresh store): the image lands under
        // imported_wallpapers/, wallpaperPath is repointed, the payload is gone.
        let elsewhere = ConfigStore::new(tmpdir("portable-wp-elsewhere"));
        let imported = elsewhere.import_file(&exported).unwrap();
        let snap = &imported.snapshots[0];
        let new_path = Path::new(snap.wallpaper_path.as_deref().unwrap());
        assert!(new_path.starts_with(elsewhere.dir().join("imported_wallpapers")));
        assert_eq!(new_path.extension().unwrap(), "png");
        assert_eq!(fs::read(new_path).unwrap(), bytes);
        // And what came back serializes cleanly — no base64 in the internal model.
        elsewhere.save(&imported).unwrap();
        assert!(
            !fs::read_to_string(elsewhere.primary_path())
                .unwrap()
                .contains("wallpaperBase64")
        );
    }

    #[test]
    fn import_drops_a_broken_wallpaper_payload() {
        let dir = tmpdir("broken-wp");
        fs::create_dir_all(&dir).unwrap();
        let mut cfg = sample();
        cfg.snapshots.push(Snapshot {
            id: uuid::Uuid::new_v4(),
            name: "corrupt".into(),
            ts: 100,
            layouts: vec![],
            wallpaper_path: Some("D:\\elsewhere\\old.jpg".into()),
            is_auto_backup: None,
        });
        let exported = dir.join("exported.json");
        ConfigStore::export_to(&cfg, &exported).unwrap();
        // No readable file → no payload was embedded; the plain path imports untouched.
        let store = ConfigStore::new(tmpdir("broken-wp-home"));
        let imported = store.import_file(&exported).unwrap();
        assert_eq!(
            imported.snapshots[0].wallpaper_path.as_deref(),
            Some("D:\\elsewhere\\old.jpg")
        );

        // Hand-injected garbage payload: dropped, original path survives.
        let mut doc: serde_json::Value =
            serde_json::from_str(&fs::read_to_string(&exported).unwrap()).unwrap();
        doc["snapshots"][0]["wallpaperBase64"] = "!!not-base64!!".into();
        let tampered = dir.join("tampered.json");
        fs::write(&tampered, serde_json::to_string_pretty(&doc).unwrap()).unwrap();
        let imported = store.import_file(&tampered).unwrap();
        assert_eq!(
            imported.snapshots[0].wallpaper_path.as_deref(),
            Some("D:\\elsewhere\\old.jpg")
        );
    }

    #[cfg(windows)]
    #[test]
    fn primary_locked_for_a_moment_still_loads() {
        use std::os::windows::fs::OpenOptionsExt;
        let store = ConfigStore::new(tmpdir("locked"));
        store.save(&sample()).unwrap();
        // Held without sharing, as a scanner does: opens fail with ERROR_SHARING_VIOLATION.
        let lock = fs::OpenOptions::new()
            .read(true)
            .share_mode(0)
            .open(store.primary_path())
            .unwrap();
        let unlock = std::thread::spawn(move || {
            std::thread::sleep(std::time::Duration::from_millis(400));
            drop(lock);
        });
        let (outcome, moved) = store.load_reporting();
        unlock.join().unwrap();
        assert!(matches!(outcome, LoadOutcome::Primary(_)), "{outcome:?}");
        assert_eq!(moved, None);
    }

    #[cfg(unix)]
    #[test]
    fn primary_without_read_access_is_not_waited_for() {
        use std::os::unix::fs::PermissionsExt;
        let store = ConfigStore::new(tmpdir("denied"));
        store.save(&sample()).unwrap();
        fs::set_permissions(store.primary_path(), fs::Permissions::from_mode(0o000)).unwrap();
        let started = std::time::Instant::now();
        let _ = store.load_reporting();
        assert!(started.elapsed() < std::time::Duration::from_millis(1000));
    }

    #[test]
    fn nothing_usable_gives_fresh() {
        let store = ConfigStore::new(tmpdir("fresh"));
        assert!(matches!(store.load(), LoadOutcome::Fresh(_)));
    }

    #[test]
    fn renamed_product_reuses_old_configuration_without_changing_it() {
        let parent = tmpdir("rebrand");
        let old = ConfigStore::new(parent.join(crate::brand::LEGACY_DATA_DIR));
        let mut config = sample();
        config.layouts[0].fences[0].title = "My OpenFence files {1}".into();
        config.settings.language = crate::i18n::Language::Japanese;
        old.save(&config).unwrap();
        let original = fs::read(old.primary_path()).unwrap();
        let preferred = parent.join(crate::brand::NAME);
        let selected = ConfigStore::with_legacy(&preferred, old.dir());
        assert_eq!(selected.dir(), old.dir());
        let mut loaded = selected.load().into_config();
        // Saving stamps the published schema URL; everything else is untouched.
        assert_eq!(
            loaded.schema.as_deref(),
            Some(crate::model::CONFIG_SCHEMA_URL)
        );
        loaded.schema = None;
        assert_eq!(
            serde_json::to_value(loaded).unwrap(),
            serde_json::to_value(config).unwrap()
        );
        assert_eq!(fs::read(old.primary_path()).unwrap(), original);
        assert!(!preferred.exists());
    }

    #[test]
    fn current_product_data_wins_and_old_backup_only_installs_still_load() {
        let parent = tmpdir("rebrand-precedence");
        let old = ConfigStore::new(parent.join(crate::brand::LEGACY_DATA_DIR));
        old.save(&sample()).unwrap();
        old.save(&sample()).unwrap();
        fs::remove_file(old.primary_path()).unwrap();
        let preferred = ConfigStore::new(parent.join(crate::brand::NAME));
        let selected = ConfigStore::with_legacy(preferred.dir(), old.dir());
        assert!(matches!(selected.load(), LoadOutcome::Recovered(..)));
        preferred.save(&sample()).unwrap();
        // Even a damaged new config belongs to the new installation. Its normal
        // backup recovery must not silently switch to a stale legacy layout.
        fs::write(preferred.primary_path(), "{ bad json").unwrap();
        assert_eq!(
            ConfigStore::with_legacy(preferred.dir(), old.dir()).dir(),
            preferred.dir()
        );
        assert_eq!(
            ConfigStore::with_legacy(parent.join("fresh"), parent.join("missing")).dir(),
            parent.join("fresh")
        );
    }

    #[test]
    fn validation_rejects_two_inboxes() {
        let mut c = sample();
        let f = c.layouts[0].fences[0].clone();
        c.layouts[0].fences.push(Fence {
            id: uuid::Uuid::new_v4(),
            ..f
        });
        assert!(validate(&c).is_err());
    }

    #[test]
    fn a_win_space_peek_shortcut_from_0_2_0_loads_as_the_default() {
        let mut c = sample();
        c.settings.peek.hotkey = PeekHotkey::WinSpace;
        assert_eq!(migrate(c.clone()).settings.peek, PeekSettings::default());
        c.settings.peek.hotkey = PeekHotkey::WinShiftSpace;
        assert_eq!(migrate(c.clone()).settings.peek, PeekSettings::default());
        // A usable custom shortcut is the choice; `hotkey` is only for older versions then.
        c.settings.peek.custom_hotkey = Some("Ctrl+Shift+F".into());
        let peek = migrate(c).settings.peek;
        assert_eq!(peek.hotkey, PeekHotkey::WinShiftSpace);
        assert_eq!(peek.chosen_hotkey().to_string(), "Ctrl+Shift+F");
    }

    #[test]
    fn restore_backup_duplicates_fold_into_the_fixed_id_entry() {
        let mk = |name: &str, ts: i64| Snapshot {
            id: uuid::Uuid::new_v4(),
            name: name.into(),
            ts,
            layouts: vec![],
            wallpaper_path: None,
            is_auto_backup: None,
        };
        let c = Config {
            snapshots: vec![
                mk("恢复快照前", 1),
                // The same label saved while the UI language was English.
                mk("Before snapshot restore", 5),
                mk("mine", 3),
            ],
            ..Default::default()
        };
        let c = migrate(c);
        let auto: Vec<_> = c
            .snapshots
            .iter()
            .filter(|s| s.is_auto_backup == Some(true))
            .collect();
        assert_eq!(auto.len(), 1);
        assert_eq!(auto[0].id, AUTO_BACKUP_SNAPSHOT_ID);
        assert_eq!(auto[0].ts, 5);
        assert_eq!(c.snapshots.len(), 2);
        assert!(c.snapshots.iter().any(|s| s.name == "mine"));
    }

    #[test]
    fn an_out_of_range_size_costs_only_that_size() {
        // A hand edit: one fence 1e9 wide and -500 tall, another 0 tall. The file still loads
        // as it is, `config check` warns, and the sizes come into range.
        let store = ConfigStore::new(tmpdir("sizes"));
        let mut c = sample();
        let mut b = Fence::new(
            "B",
            FenceKind::Virtual,
            c.layouts[0].fences[0].geometry.clone(),
        );
        b.geometry.h = 0.0;
        c.layouts[0].fences[0].geometry.w = 1e9;
        c.layouts[0].fences[0].geometry.h = -500.0;
        c.layouts[0].fences.push(b);
        fs::create_dir_all(store.dir()).unwrap();
        fs::write(store.primary_path(), serde_json::to_string(&c).unwrap()).unwrap();
        let (outcome, moved) = store.load_reporting();
        let LoadOutcome::Primary(mut loaded) = outcome else {
            panic!("{outcome:?}");
        };
        assert_eq!(moved, None);
        let lints = lint(&loaded);
        let sizes: Vec<&Lint> = lints
            .iter()
            .filter(|l| l.message.starts_with("size "))
            .collect();
        assert_eq!(sizes.len(), 2, "{lints:?}");
        assert_eq!(sizes[0].subject, "fence:A");
        assert_eq!(sizes[1].subject, "fence:B");
        assert!(sizes.iter().all(|l| l.level == LintLevel::Warning));
        assert!(clamp_sizes(&mut loaded));
        let g = &loaded.layouts[0].fences[0].geometry;
        assert_eq!((g.w, g.h), (8192.0, 1.0));
        assert_eq!(loaded.layouts[0].fences[1].geometry.h, 1.0);
        assert!(!clamp_sizes(&mut loaded));
        assert!(
            lint(&loaded)
                .iter()
                .all(|l| !l.message.starts_with("size "))
        );
    }

    /// A network portal saved by 0.2.1 (`UNC\server\share\…`) is repaired in the layouts and
    /// in the snapshots; drive paths are left alone.
    #[test]
    fn a_network_portal_saved_by_0_2_1_gets_its_path_back() {
        let folder = |path: &str| ItemSourceSpec::Folder {
            path: path.to_string(),
            recursive: false,
            filter: None,
        };
        let mut c = sample();
        let mut portal = c.layouts[0].fences[0].clone();
        portal.id = uuid::Uuid::new_v4();
        portal.kind = FenceKind::FolderPortal;
        portal.source = folder(r"UNC\nas\share\docs");
        let mut local = portal.clone();
        local.id = uuid::Uuid::new_v4();
        local.source = folder(r"C:\Users\Me\Documents");
        c.layouts[0].fences.push(portal.clone());
        c.layouts[0].fences.push(local);
        c.snapshots.push(Snapshot {
            id: uuid::Uuid::new_v4(),
            name: "before".into(),
            ts: 1,
            layouts: vec![Layout {
                shown: 0,
                fingerprint: vec![],
                fences: vec![portal],
                desktop_items: Default::default(),
            }],
            wallpaper_path: None,
            is_auto_backup: None,
        });
        assert!(repair_unc_portals(&mut c));
        let path_of = |f: &Fence| match &f.source {
            ItemSourceSpec::Folder { path, .. } => path.clone(),
            ItemSourceSpec::Desktop => String::new(),
        };
        assert_eq!(path_of(&c.layouts[0].fences[1]), r"\\nas\share\docs");
        assert_eq!(path_of(&c.layouts[0].fences[2]), r"C:\Users\Me\Documents");
        assert_eq!(
            path_of(&c.snapshots[0].layouts[0].fences[0]),
            r"\\nas\share\docs"
        );
        assert!(!repair_unc_portals(&mut c));
    }

    #[test]
    fn saved_file_starts_with_the_schema_field_and_loads_back() {
        let store = ConfigStore::new(tmpdir("schema"));
        store.save(&sample()).unwrap();
        let text = fs::read_to_string(store.primary_path()).unwrap();
        let second_line = text.lines().nth(1).unwrap_or_default();
        assert!(second_line.contains("\"$schema\""), "{second_line}");
        assert!(text.contains(crate::model::CONFIG_SCHEMA_URL));
        let cfg = ConfigStore::parse_file(&store.primary_path()).unwrap();
        assert_eq!(cfg.schema.as_deref(), Some(crate::model::CONFIG_SCHEMA_URL));
        // A file without the field (older versions) still loads.
        let stripped = text.replacen(
            &format!("\"$schema\": \"{}\",", crate::model::CONFIG_SCHEMA_URL),
            "",
            1,
        );
        assert!(!stripped.contains("$schema"));
        let cfg: Config = serde_json::from_str(&stripped).unwrap();
        assert_eq!(cfg.schema, None);
    }

    #[test]
    fn lint_finds_dangling_targets_and_folders() {
        use crate::rules::{Cond, Rule, Target, TypeCategory};
        let mut c = sample();
        let inbox = c.layouts[0].fences[0].id;
        // Valid rule to the inbox, a rule to a fence that exists nowhere, a rule without
        // conditions, a portal whose folder is gone, a tab hosted by a missing fence.
        c.rules.list.push(Rule::new(
            "ok",
            Target::Fence(inbox),
            vec![Cond::Type(vec![TypeCategory::Images])],
        ));
        let ghost = uuid::Uuid::new_v4();
        c.rules.list.push(Rule::new(
            "ghost",
            Target::Fence(ghost),
            vec![Cond::Type(vec![TypeCategory::Video])],
        ));
        c.rules.list.push(Rule::new("empty", Target::Inbox, vec![]));
        let geometry = c.layouts[0].fences[0].geometry.clone();
        let mut portal = Fence::new("Gone", FenceKind::FolderPortal, geometry.clone());
        portal.source = ItemSourceSpec::Folder {
            path: "Z:\\definitely\\missing\\folder".into(),
            recursive: false,
            filter: None,
        };
        let portal_id = portal.id;
        c.layouts[0].fences.push(portal);
        let mut tab = Fence::new("Tab", FenceKind::Virtual, geometry);
        tab.tab_host = Some(ghost);
        c.layouts[0].fences.push(tab);
        c.rules.list.push(Rule::new(
            "to-portal",
            Target::Fence(portal_id),
            vec![Cond::FoldersOnly],
        ));
        assert!(validate(&c).is_ok(), "lint covers what validate does not");

        let lints = lint(&c);
        let subjects: Vec<&str> = lints.iter().map(|l| l.subject.as_str()).collect();
        assert!(subjects.contains(&"file"), "$schema missing: {lints:?}");
        assert!(subjects.contains(&"rule:ghost"), "{lints:?}");
        assert!(subjects.contains(&"rule:empty"), "{lints:?}");
        assert!(subjects.contains(&"fence:Gone"), "{lints:?}");
        assert!(subjects.contains(&"fence:Tab"), "{lints:?}");
        assert!(!subjects.contains(&"rule:ok"), "{lints:?}");
        let errors: Vec<&Lint> = lints
            .iter()
            .filter(|l| l.level == LintLevel::Error)
            .collect();
        assert_eq!(errors.len(), 1, "{errors:?}");
        assert_eq!(errors[0].subject, "rule:to-portal");

        // A clean, saved-and-reloaded config lints without findings.
        let store = ConfigStore::new(tmpdir("lint-clean"));
        let mut clean = sample();
        let clean_inbox = clean.layouts[0].fences[0].id;
        clean.rules.list.push(Rule::new(
            "ok",
            Target::Fence(clean_inbox),
            vec![Cond::Type(vec![TypeCategory::Images])],
        ));
        store.save(&clean).unwrap();
        let reloaded = ConfigStore::parse_file(&store.primary_path()).unwrap();
        assert_eq!(lint(&reloaded), Vec::<Lint>::new());
    }

    #[test]
    fn date_formatting_is_sane() {
        let s = today_yyyy_mm_dd();
        assert_eq!(s.len(), 10);
        assert!(s.starts_with("20"));
    }

    /// Unicode paths must survive the config round-trip byte-exact — a theme
    /// named in Arabic (or any non-ASCII script) writes to NTFS as UTF-16 and
    /// to `config.json` as UTF-8, with no ANSI/codepage conversion anywhere.
    #[test]
    fn unicode_paths_round_trip_unchanged() {
        let store = ConfigStore::new(tmpdir("unicode"));
        let mut cfg = sample();
        let arabic = r"C:\Users\يحيى\AppData\Roaming\PecoFence\themes\سماء زرقاء (2).peco";
        let cjk = r"D:\壁紙\青い空.peco";
        cfg.settings.active_theme_path = Some(arabic.to_string());
        cfg.settings.video_wallpaper_path = Some(cjk.to_string());
        store.save(&cfg).unwrap();

        // The bytes on disk are valid UTF-8 and contain the names unescaped
        // (serde_json JSON-escapes backslashes, so match the Unicode stems).
        let raw = fs::read(&store.primary_path()).unwrap();
        let text = String::from_utf8(raw).expect("config.json must be UTF-8");
        assert!(
            text.contains("سماء زرقاء"),
            "Arabic path mangled in json: {text}"
        );
        assert!(
            text.contains("壁紙\\\\青い空.peco"),
            "CJK path mangled in json: {text}"
        );

        let back = ConfigStore::parse_file(&store.primary_path()).unwrap();
        assert_eq!(back.settings.active_theme_path.as_deref(), Some(arabic));
        assert_eq!(back.settings.video_wallpaper_path.as_deref(), Some(cjk));
    }
}
