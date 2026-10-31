//! Windhawk `windows-11-taskbar-styler` theme catalog.
//!
//! `scripts/import-taskbar-themes.py` converts the mod's published theme
//! structs into `data/taskbar_themes.json`, embedded here. The settings UI
//! receives only the `{id, label}` option list; the full targeting rules,
//! style constants and theme-resource variables stay behind this module
//! (`theme()`) so the frontend never carries the raw payload. `id` is the
//! exact string the mod's `theme` setting expects; `""` is Windows Default
//! (the stock taskbar restored).
//!
//! `Squircle` is auto-resolved by the mod against OS feature `variantFeature`
//! (ExtendedModeAEPForTaskbar): when enabled, the `weatherVariant` payload
//! applies instead of the base. `resolve_variant(id, feature_enabled)` picks
//! the concrete payload the host serializes.

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
        serde_json::from_str(include_str!("../data/taskbar_themes.json"))
            .expect("taskbar_themes.json is generated and parse-checked")
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

/// The theme's full converted payload — target styles, style constants,
/// theme-resource variables, and `Squircle`'s feature-gated variant.
pub fn theme(id: &str) -> Option<&'static serde_json::Value> {
    catalog().themes.get(id)
}

/// The OS feature id gating a theme's auto-variant (`Squircle` -> its
/// WeatherOnTheRight payload when `RtlQueryFeatureConfiguration` reports it
/// enabled). `None` for themes without a variant.
pub fn variant_feature(id: &str) -> Option<u32> {
    theme(id)?["variantFeature"].as_u64().map(|v| v as u32)
}

/// Serializes the theme into the line wire-format the injected
/// `pecofence_tap.dll` parses inside `explorer.exe` (the host transports it
/// with `WM_COPYDATA`). `use_variant` picks the `weatherVariant` payload when
/// present (the host resolves `variant_feature` via
/// `RtlQueryFeatureConfiguration`):
///
/// ```text
/// PECO-TBS1
/// T <target>
/// S <style>
/// ...
/// R <key>\t<value>       (theme resource variables)
/// E
/// ```
///
/// `$constant` substitution is done here (longest-name-first, the mod's own
/// ordering), so the DLL only ever sees expanded style strings.
pub fn theme_wire(id: &str, use_variant: bool) -> Option<String> {
    let theme = theme(id)?;
    let theme = if use_variant && theme["weatherVariant"].is_object() {
        &theme["weatherVariant"]
    } else {
        theme
    };
    let mut out = String::with_capacity(64 * 1024);
    out.push_str("PECO-TBS1\n");

    // Constants: `name=value` pairs applied to every style/target string with
    // longest-name-first replacement order (LoadStyleConstants keeps them
    // sorted descending so longer names win; later duplicates override).
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
    // One expansion pass inside each constant value so a constant may
    // reference the ones defined before it (ParseStyleConstant applies the
    // constants loaded so far).
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
            // `//` comments a target in the mod's settings format.
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
                    let style = expand_constants(style, &constants);
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
            let var = expand_constants(var, &constants);
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
        assert_eq!(opts.len(), 55);
        assert_eq!(opts[0].id, "");
        assert_eq!(opts[0].label, "Windows Default");
        for o in &opts[1..] {
            assert!(!o.id.is_empty(), "non-default styles carry the Windhawk id");
            assert!(is_style_id(o.id), "{} must validate", o.id);
        }
        assert!(is_style_id("TranslucentTaskbar"));
        assert!(is_style_id("Oversimplified&Accentuated"));
        assert!(is_style_id("Command_Center"));
        assert!(is_style_id("21996Taskbar"));
        assert!(is_style_id(""));
        assert!(!is_style_id("NoSuchTheme"));
        // The weather variant is internal — resolved from `Squircle`, not
        // offered as its own option.
        assert!(!opts.iter().any(|o| o.id.contains("WeatherOnTheRight")));
    }

    #[test]
    fn theme_wire_serializes_rules_and_resources() {
        let wire = theme_wire("TranslucentTaskbar", false).expect("wire for known theme");
        assert!(wire.starts_with("PECO-TBS1\n"));
        assert!(wire.ends_with("E\n"));
        // No layout override or classic variant on the taskbar wire.
        assert!(!wire.contains("\nL "));
        assert!(!wire.contains("\nC\n"));
        assert!(wire.contains("T Taskbar.TaskbarFrame > Grid#RootGrid > Taskbar.TaskbarBackground > Grid > Rectangle#BackgroundFill\n"));
        // `$CommonBgBrush` must be expanded before transport.
        assert!(!wire.contains("$CommonBgBrush"));
        assert!(wire.contains("S Fill:=<WindhawkBlur"));

        // Squircle's feature-gated variant swaps the whole payload.
        assert_eq!(variant_feature("Squircle"), Some(48660958));
        let base = theme_wire("Squircle", false).expect("base wire");
        let variant = theme_wire("Squircle", true).expect("variant wire");
        assert_ne!(base, variant);
        assert!(variant.starts_with("PECO-TBS1\n"));
        assert_eq!(variant_feature("TranslucentTaskbar"), None);

        // Windows Default and unknown ids produce no wire.
        assert!(theme_wire("", false).is_none());
        assert!(theme_wire("NoSuchTheme", false).is_none());
    }

    #[test]
    fn themes_carry_the_full_converted_payload() {
        let t = theme("TranslucentTaskbar").expect("theme exists");
        let targets = t["targetStyles"].as_array().expect("target rules");
        assert_eq!(targets.len(), 8);
        assert_eq!(
            targets[0]["target"],
            "Taskbar.TaskbarFrame > Grid#RootGrid > Taskbar.TaskbarBackground > Grid > Rectangle#BackgroundFill"
        );
        let frosted = theme("FrostedAcrylic").expect("theme exists");
        let styles = frosted["targetStyles"][1]["styles"]
            .as_array()
            .expect("styles");
        assert!(
            styles
                .iter()
                .any(|s| s.as_str() == Some("Tag=>taskbarDock"))
        );
    }
}
