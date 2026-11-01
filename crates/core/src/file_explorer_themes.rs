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
/// mod), applied to `CabinetWClass` windows via
/// `DWMWA_SYSTEMBACKDROP_TYPE` on the platform side. Every non-default
/// catalog theme drives the system-acrylic path: on Windows 11 23H2 the
/// `kDefault`/empty entries still need `DWMSBT_TRANSIENTWINDOW` or the
/// frame-extension glass never engages — only `kMica` keeps its own
/// system backdrop. `None` only for `""`/unknown ids.
pub fn backdrop(id: &str) -> Option<&'static str> {
    let theme = theme(id)?;
    Some(match theme["backgroundTranslucentEffect"].as_str() {
        Some("kMica") => "mica",
        _ => "acrylic",
    })
}

/// `WindhawkBlur` is not a real XAML type — the mod injects it. Re-express
/// it as a translucent `SolidColorBrush` so `XamlReader.Load` resolves a
/// plain brush inside the WinUI3 session: WinUI3's `AcrylicBrush` is
/// in-app acrylic and has no usable backdrop inside a `CabinetWClass`
/// island (it degrades to a ~95%-opaque block), while the DWM accent
/// policy already blurs the desktop behind the whole window. Alpha comes
/// from `TintOpacity` (else the tint color's own alpha), clamped to
/// `0x10..=0x45` — translucent, never opaque, never fully clear.
fn normalize_blur(value: &str) -> String {
    const TAGS: [&str; 2] = ["<WindhawkBlur", "<AcrylicBrush"];
    let mut out = String::with_capacity(value.len());
    let mut rest = value;
    // Earliest occurrence of either blur-brush tag wins each rewrite.
    while let Some((pos, tag)) = TAGS
        .iter()
        .filter_map(|t| rest.find(t).map(|p| (p, *t)))
        .min_by_key(|(p, _)| *p)
    {
        out.push_str(&rest[..pos]);
        let inner = &rest[pos + tag.len()..];
        let end = inner.find('>').map(|e| e + 1).unwrap_or(inner.len());
        let attrs = &inner[..end.min(inner.len())];
        let attr = |name: &str| -> Option<String> {
            let pat = format!("{name}=\"");
            let s = attrs.find(&pat)? + pat.len();
            let e = attrs[s..].find('"')? + s;
            Some(attrs[s..e].to_string())
        };
        let opacity = attr("TintOpacity")
            .or_else(|| attr("Opacity"))
            .and_then(|o| o.parse::<f32>().ok())
            .map(|o| (o.clamp(0.0, 1.0) * 255.0).round() as u32);
        let tint = attr("TintColor").or_else(|| attr("FallbackColor"));
        let (r, g, b, a) = match tint.as_deref().map(str::trim) {
            // `Transparent` (hover/rest states): near-clear white veil so
            // DWM glass stays visible; alpha rides the brush's opacity.
            Some(c) if c.eq_ignore_ascii_case("transparent") => {
                (0xFF, 0xFF, 0xFF, opacity.unwrap_or(0x14).clamp(0x08, 0x24))
            }
            // Literal #AARRGGBB / #RRGGBB: keep the author's RGB; alpha
            // from the color (TintOpacity wins), clamped translucent.
            Some(c) if c.starts_with('#') => {
                let n = u32::from_str_radix(&c[1..], 16).unwrap_or(0);
                match c.len() - 1 {
                    8 => (
                        (n >> 16) & 0xFF,
                        (n >> 8) & 0xFF,
                        n & 0xFF,
                        opacity.unwrap_or((n >> 24) & 0xFF).clamp(0x10, 0x28),
                    ),
                    6 => (
                        (n >> 16) & 0xFF,
                        (n >> 8) & 0xFF,
                        n & 0xFF,
                        opacity.unwrap_or(0x80).clamp(0x10, 0x28),
                    ),
                    _ => (0x20, 0x20, 0x20, opacity.unwrap_or(0x1A).clamp(0x12, 0x24)),
                }
            }
            // `{ThemeResource x}`, missing, or anything unparseable:
            // neutral frosted-gray — never the old opaque #45101010 mud.
            _ => (0x20, 0x20, 0x20, opacity.unwrap_or(0x1A).clamp(0x12, 0x24)),
        };
        out.push_str(&format!(
            "<SolidColorBrush Color=\"#{a:02X}{r:02X}{g:02X}{b:02X}\"/>"
        ));
        rest = &inner[end.min(inner.len())..];
    }
    out.push_str(rest);
    out
}

/// Top-bar (WinUI 3 island) transparency guaranteed for every theme: the
/// island's root containers default to opaque theme resources, so these
/// rules are appended after each theme's own styles — last write wins —
/// to keep the DWM `DWMSBT_TRANSIENTWINDOW` blur visible end to end.
const TOP_BAR_RULES: &[(&str, &[&str])] = &[
    (
        "FileExplorerExtensions.FileExplorerTabControl",
        &["Background=Transparent"],
    ),
    (
        "FileExplorerExtensions.FileExplorerTabControl > Grid",
        &["Background=Transparent"],
    ),
    (
        "Microsoft.UI.Xaml.Controls.TabView",
        &["Background=Transparent"],
    ),
    (
        "Microsoft.UI.Xaml.Controls.Grid#TabContainerGrid",
        &["Background=Transparent"],
    ),
    (
        "FileExplorerExtensions.NavigationBarControl",
        &["Background=Transparent"],
    ),
    (
        "Microsoft.UI.Xaml.Controls.Grid#NavigationBarControlGrid",
        &["Background=Transparent", "BorderBrush=Transparent"],
    ),
    (
        "FileExplorerExtensions.CommandBarControl_Wave1",
        &["Background=Transparent"],
    ),
    (
        "FileExplorerExtensions.CommandBarControl_Wave1 > Grid, Grid#CommandBarControlRootGrid",
        &["Background=Transparent", "BorderBrush=Transparent"],
    ),
    (
        "Microsoft.UI.Xaml.Controls.CommandBar#FileExplorerCommandBar",
        &["Background=Transparent"],
    ),
    (
        "Microsoft.UI.Xaml.Controls.CommandBar#FileExplorerCommandBar > Grid#LayoutRoot > Grid#ContentRoot",
        &["Background=Transparent", "BorderBrush=Transparent"],
    ),
    (
        "Microsoft.UI.Xaml.Controls.Grid#PART_LayoutRoot",
        &["Background=Transparent"],
    ),
    (
        "Microsoft.UI.Xaml.Controls.Grid#ContentRoot",
        &["Background=Transparent"],
    ),
    (
        "Microsoft.UI.Xaml.Controls.Grid#RootContainer",
        &["Background=Transparent"],
    ),
    // Content grids hosting DirectUIHWND — the Win32 punch already lays
    // the veil/tint beneath them; an XAML background here would
    // double-stack the overlay.
    (
        "Microsoft.UI.Xaml.Controls.Grid#DetailsViewControlRootGrid",
        &["Background=Transparent"],
    ),
    (
        "Microsoft.UI.Xaml.Controls.Grid#HomeViewRootGrid",
        &["Background=Transparent"],
    ),
    (
        "FileExplorerExtensions.GalleryViewControl#GalleryViewControl > Grid",
        &["Background=Transparent"],
    ),
];

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
    // Top-bar transparency floor — appended last so it overrides any
    // opaque island background a theme left behind.
    for (target, styles) in TOP_BAR_RULES {
        out.push_str("T ");
        out.push_str(target);
        out.push('\n');
        for style in *styles {
            out.push_str("S ");
            out.push_str(style);
            out.push('\n');
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
    fn windhawk_blur_rewrites_to_translucent_solid() {
        // OS26 themes carry `<WindhawkBlur>` — the wire must emit a
        // translucent SolidColorBrush (WinUI3 AcrylicBrush is in-app
        // acrylic and reads opaque inside a CabinetWClass island).
        let wire = theme_wire("OS26_Liquid_Glass").expect("wire for known theme");
        assert!(
            !wire.contains("WindhawkBlur"),
            "mod-only type must not ship"
        );
        assert!(!wire.contains("<AcrylicBrush"));
        assert!(wire.contains("<SolidColorBrush"));
    }

    #[test]
    fn backdrop_defaults_every_theme_to_acrylic() {
        assert_eq!(backdrop("Translucent_Explorer11"), Some("acrylic"));
        // `kDefault` themes must still select the system-acrylic path —
        // without it the frame-extension glass never engages on 23H2.
        assert_eq!(backdrop("LiquidGlass"), Some("acrylic"));
        assert_eq!(backdrop("Compact_Explorer11"), Some("acrylic"));
        assert_eq!(backdrop(""), None);
        assert_eq!(backdrop("NoSuchTheme"), None);
    }
}
