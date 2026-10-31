//! Windhawk `windows-11-file-explorer-styler` theme catalog.
//!
//! `scripts/import-file-explorer-themes.py` converts the mod's published
//! theme structs into `data/file_explorer_themes.json`, embedded here.
//! The settings UI receives only the `{id, label}` option list; the full
//! targeting rules, style constants, theme-resource variables and the DWM
//! backdrop choice stay behind this module so the frontend never carries
//! the raw payload. `id` is the exact string the mod's `theme` setting
//! expects; `""` is Windows Default (stock File Explorer restored).
//!
//! The themes target the WinUI3 islands inside `CabinetWClass` windows —
//! the same surface the mod's `Microsoft.Internal.FrameworkUdk`
//! `InitializeXamlDiagnosticsEx` session watches.

use serde::Deserialize;
use std::collections::BTreeMap;
use std::sync::OnceLock;

#[derive(Deserialize)]
struct OptionEntry {
    id: String,
    label: String,
}

#[derive(Deserialize)]
struct Catalog {
    options: Vec<OptionEntry>,
    #[serde(default)]
    themes: BTreeMap<String, serde_json::Value>,
}

static CATALOG: OnceLock<Catalog> = OnceLock::new();

fn catalog() -> &'static Catalog {
    CATALOG.get_or_init(|| {
        serde_json::from_str(include_str!("../data/file_explorer_themes.json"))
            .expect("file_explorer_themes.json is generated and parse-checked")
    })
}

/// A dropdown entry: Windhawk `theme` setting id + display label.
pub struct StyleOption {
    pub id: &'static str,
    pub label: &'static str,
}

/// Every selectable style, Windows Default first.
pub fn style_options() -> &'static [StyleOption] {
    static OPTIONS: OnceLock<Vec<StyleOption>> = OnceLock::new();
    OPTIONS.get_or_init(|| {
        catalog()
            .options
            .iter()
            .map(|o| StyleOption {
                id: o.id.as_str(),
                label: o.label.as_str(),
            })
            .collect()
    })
}

/// Whether `id` is selectable: `""` (Windows Default) or a known theme.
pub fn is_style_id(id: &str) -> bool {
    id.is_empty() || catalog().themes.contains_key(id)
}

/// The theme's full converted payload — target styles, style constants and
/// theme-resource variables.
pub fn theme(id: &str) -> Option<&'static serde_json::Value> {
    catalog().themes.get(id)
}

/// The theme's DWM backdrop choice (`BackgroundTranslucentEffect` in the
/// mod): `""` for kDefault / kNone, or `blur` / `acrylic` / `mica` /
/// `micaAlt` — applied to `CabinetWClass` windows via
/// `DWMWA_SYSTEMBACKDROP_TYPE` on the platform side.
pub fn backdrop(id: &str) -> Option<&'static str> {
    let effect = theme(id)?["backgroundTranslucentEffect"].as_str()?;
    Some(match effect {
        "kBlur" => "blur",
        "kAcrylic" => "acrylic",
        "kMica" => "mica",
        "kMicaAlt" => "micaAlt",
        _ => "",
    })
}

/// `WindhawkBlur` is not a real XAML type — the mod injects it. Re-express
/// it as the stock `AcrylicBrush` so `XamlReader.Load` resolves inside the
/// WinUI3 session: `TintColor`/`TintOpacity`/`Opacity` carry over, the
/// blur-specific `BlurAmount`/`TintLuminosityOpacity` have no Acrylic
/// equivalent and are dropped.
fn normalize_blur(value: &str) -> String {
    let mut out = String::with_capacity(value.len());
    let mut rest = value;
    while let Some(pos) = rest.find("<WindhawkBlur") {
        out.push_str(&rest[..pos]);
        let tag_start = pos;
        let inner = &rest[tag_start + "<WindhawkBlur".len()..];
        let end = inner.find('>').map(|e| e + 1).unwrap_or(inner.len());
        let attrs = &inner[..end.min(inner.len())];
        let attr = |name: &str| -> Option<String> {
            let pat = format!("{name}=\"");
            let s = attrs.find(&pat)? + pat.len();
            let e = attrs[s..].find('"')? + s;
            Some(attrs[s..e].to_string())
        };
        out.push_str("<AcrylicBrush");
        let mut wrote_opacity = false;
        for (src_name, dst_name) in [
            ("TintColor", "TintColor"),
            ("TintOpacity", "TintOpacity"),
            ("Opacity", "Opacity"),
        ] {
            if let Some(v) = attr(src_name) {
                out.push_str(&format!(" {dst_name}=\"{v}\""));
                wrote_opacity |= src_name != "TintColor";
            }
        }
        if !wrote_opacity {
            // WindhawkBlur defaults to a strong blur; Acrylic with no
            // opacity arg reads opaque — keep the translucent intent.
            out.push_str(" TintOpacity=\"0.5\"");
        }
        out.push_str("/>");
        rest = &inner[end..];
    }
    out.push_str(rest);
    out
}

/// Serializes the theme into the line wire-format the injected
/// `pecofence_tap.dll` parses inside `explorer.exe`'s WinUI3 diagnostics
/// session (the host transports it with `WM_COPYDATA`):
///
/// ```text
/// PECO-FES1
/// T <target>
/// S <style>
/// ...
/// R <key>\t<value>       (theme resource variables)
/// E
/// ```
///
/// `$constant` substitution is done here (longest-name-first, the mod's own
/// ordering), so the DLL only ever sees expanded style strings.
pub fn theme_wire(id: &str) -> Option<String> {
    let theme = theme(id)?;
    let mut out = String::with_capacity(64 * 1024);
    out.push_str("PECO-FES1\n");

    // Constants: `name=value` pairs applied to every style/target string
    // with longest-name-first replacement order.
    let mut constants: Vec<(String, String)> = theme["styleConstants"]
        .as_array()
        .map(|a| {
            a.iter()
                .filter_map(|v| v.as_str())
                .filter(|s| !s.starts_with("//"))
                .filter_map(|s| {
                    let (k, v) = s.split_once('=')?;
                    Some((k.trim().to_string(), v.trim().to_string()))
                })
                .collect()
        })
        .unwrap_or_default();
    for i in 0..constants.len() {
        let prior = constants[..i].to_vec();
        constants[i].1 = expand_constants(&constants[i].1, &prior);
    }
    constants.sort_by_key(|c| std::cmp::Reverse(c.0.len()));

    if let Some(styles) = theme["targetStyles"].as_array() {
        for entry in styles {
            let Some(target) = entry["target"].as_str() else {
                continue;
            };
            if target.starts_with("//") {
                continue;
            }
            let target = expand_constants(target, &constants);
            let mut emitted_target = false;
            if let Some(arr) = entry["styles"].as_array() {
                for style in arr.iter().filter_map(|s| s.as_str()) {
                    let style = style.trim();
                    if style.is_empty() || style.starts_with("//") {
                        continue;
                    }
                    if !emitted_target {
                        out.push_str("T ");
                        out.push_str(&target.replace(['\t', '\n'], " "));
                        out.push('\n');
                        emitted_target = true;
                    }
                    let style = normalize_blur(&expand_constants(style, &constants));
                    out.push_str("S ");
                    out.push_str(&style.replace(['\t', '\n'], " "));
                    out.push('\n');
                }
            }
        }
    }
    if let Some(vars) = theme["themeResourceVariables"].as_array() {
        for var in vars.iter().filter_map(|v| v.as_str()) {
            if var.starts_with("//") {
                continue;
            }
            let var = normalize_blur(&expand_constants(var, &constants));
            out.push_str("R ");
            out.push_str(&var.replace(['\t', '\n'], " "));
            out.push('\n');
        }
    }
    out.push_str("E\n");
    Some(out)
}

/// `ApplyStyleConstants` from the mod: replace each `$name` token with its
/// constant's value; unknown `$x` survives literally.
fn expand_constants(input: &str, constants: &[(String, String)]) -> String {
    let mut out = String::with_capacity(input.len());
    let bytes = input.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] == b'$' {
            let rest = &input[i + 1..];
            if let Some((name, value)) =
                constants.iter().find(|(n, _)| rest.starts_with(n.as_str()))
            {
                out.push_str(value);
                i += 1 + name.len();
                continue;
            }
        }
        let ch_len = input[i..].chars().next().map(char::len_utf8).unwrap_or(1);
        out.push_str(&input[i..i + ch_len]);
        i += ch_len;
    }
    out
}

/// The option list as a JSON value for the settings state payload.
pub fn style_options_json() -> serde_json::Value {
    serde_json::Value::Array(
        catalog()
            .options
            .iter()
            .map(|o| {
                serde_json::json!({
                    "id": o.id,
                    "label": o.label,
                })
            })
            .collect(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_catalog_lists_every_selectable_style() {
        let opts = style_options();
        assert_eq!(opts.len(), 17);
        assert_eq!(opts[0].id, "");
        assert_eq!(opts[0].label, "Windows Default");
        for o in &opts[1..] {
            assert!(!o.id.is_empty(), "non-default styles carry the Windhawk id");
            assert!(is_style_id(o.id), "{} must validate", o.id);
        }
        assert!(is_style_id("Translucent_Explorer11"));
        assert!(is_style_id("MicaBar"));
        assert!(is_style_id("WindowGlass"));
        assert!(is_style_id(""));
        assert!(!is_style_id("NoSuchTheme"));
    }

    #[test]
    fn theme_wire_serializes_rules_and_resources() {
        let wire = theme_wire("Translucent_Explorer11").expect("wire for known theme");
        assert!(wire.starts_with("PECO-FES1\n"));
        assert!(wire.ends_with("E\n"));
        assert!(wire.contains(
            "T FileExplorerExtensions.CommandBarControl_Wave1 > Grid, Grid#CommandBarControlRootGrid\n"
        ));
        assert!(wire.contains("S Background=Transparent\n"));

        // Windows Default and unknown ids produce no wire.
        assert!(theme_wire("").is_none());
        assert!(theme_wire("NoSuchTheme").is_none());
    }

    #[test]
    fn windhawk_blur_rewrites_to_acrylic() {
        let wire = theme_wire("WindowGlass").expect("wire for known theme");
        assert!(
            !wire.contains("WindhawkBlur"),
            "mod-only type must not ship"
        );
        assert!(wire.contains("<AcrylicBrush"));
    }

    #[test]
    fn backdrop_follows_the_theme_declared_effect() {
        assert_eq!(backdrop("Translucent_Explorer11"), Some("acrylic"));
        assert_eq!(backdrop(""), None);
        assert_eq!(backdrop("NoSuchTheme"), None);
    }
}
