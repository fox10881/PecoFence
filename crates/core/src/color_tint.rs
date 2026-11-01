//! Accent-color tint for the stable shell surfaces.
//!
//! Instead of fighting the shell for transparency, a user-picked color is
//! applied as a semi-transparent `SolidColorBrush` over each surface's
//! canonical background element — the same `controlStyles` grammar every
//! theme wire already speaks. The tint lines are appended *after* the
//! theme's own rules so they always win on the background property, and
//! they ride the normal restore path: a later wire without the lines (or
//! a reset) puts the theme's originals back, so rapid switching can't
//! stack stale visuals.
//!
//! Palette + swatch UI live in `ui/settings.html`; this module owns the
//! engine-facing bits: validation, `#RRGGBB` → `#AARRGGBB` expansion and
//! the per-surface target tables.

/// Overlay opacity baked into every tint — strong enough to read as a
/// real accent, translucent enough that the surface's own material still
/// shows through (mirrors Windows' "show accent color" strength).
const TINT_ALPHA: u8 = 0x80;

/// `(target, property)` pairs the tint writes per surface — the canonical
/// background elements every surface's XAML tree always exposes (the same
/// elements the theme catalogs restyle most often).
pub const TASKBAR_TARGETS: &[(&str, &str)] = &[
    ("Rectangle#BackgroundFill", "Fill"),
    ("Rectangle#BackgroundStroke", "Fill"),
];
pub const START_MENU_TARGETS: &[(&str, &str)] = &[
    ("Border#AcrylicBorder", "Background"),
    ("Border#AcrylicOverlay", "Background"),
];
/// File Explorer's WinUI3 chrome — the three NON-overlapping top-bar
/// root containers only. Nested targets (`TabViewItem > Grid#LayoutRoot`)
/// stacked the veil 2-3 layers deep, and content grids must stay clear:
/// the Win32 side already lays the identical 0x24-alpha BGRA tint on
/// `SysTreeView32`/`DirectUIHWND` via `fe_glass::punch_black`.
pub const FILE_EXPLORER_TARGETS: &[(&str, &str)] = &[
    (
        "FileExplorerExtensions.FileExplorerTabControl > Grid",
        "Background",
    ),
    (
        "Microsoft.UI.Xaml.Controls.Grid#NavigationBarControlGrid",
        "Background",
    ),
    (
        "FileExplorerExtensions.CommandBarControl_Wave1 > Grid",
        "Background",
    ),
];
/// File Explorer tint alpha — ~28%; the Win32 punch
/// (`fe_glass::punch_black` BGRA stretch) and this XAML veil carry the
/// identical opacity. Taskbar/Start Menu keep `TINT_ALPHA`.
pub const FE_TINT_ALPHA: u8 = 0x48;
/// The veil painted when NO swatch is picked: ~31%-opaque dark smoked
/// charcoal. Bare transparent (`#00000000`) let bright windows behind
/// Explorer shine straight through white file names — this supplies the
/// contrast the glyphs need while staying glassy.
pub const FE_DEFAULT_VEIL_ARGB: &str = "#50101216";

/// `fe_veil_argb` — the File Explorer overlay color in `#AARRGGBB`:
/// `""` → `FE_DEFAULT_VEIL_ARGB`; `#RRGGBB` → `FE_TINT_ALPHA` alpha over
/// the swatch darkened to 65% (bright swatches would wash the glass out).
/// `None` only for malformed input. Identical RGB/alpha to the resident's
/// `tint_abgr` BGRA value — one color across XAML and Win32.
pub fn fe_veil_argb(tint: &str) -> Option<String> {
    let hex = tint.strip_prefix('#').unwrap_or(tint);
    if hex.is_empty() {
        return Some(FE_DEFAULT_VEIL_ARGB.to_string());
    }
    if hex.len() == 6 && hex.chars().all(|c| c.is_ascii_hexdigit()) {
        let n = u32::from_str_radix(hex, 16).ok()?;
        let (r, g, b) = ((n >> 16) & 0xFF, (n >> 8) & 0xFF, n & 0xFF);
        return Some(format!(
            "#{FE_TINT_ALPHA:02X}{:02X}{:02X}{:02X}",
            r * 65 / 100,
            g * 65 / 100,
            b * 65 / 100
        ));
    }
    None
}

/// Always-on File Explorer veil — appends the three top-bar-root rules
/// even when `tint` is `""` (default smoked glass). `fe_glass`'s Win32
/// punch carries the same veil, so XAML only covers the island.
pub fn apply_fe_tint(wire: &mut String, tint: &str) {
    let Some(argb) = fe_veil_argb(tint) else {
        return;
    };
    if wire.ends_with("E\n") {
        wire.truncate(wire.len() - 2);
    }
    for (target, prop) in FILE_EXPLORER_TARGETS {
        wire.push_str("T ");
        wire.push_str(target);
        wire.push('\n');
        wire.push_str("S ");
        wire.push_str(prop);
        wire.push_str(":=<SolidColorBrush Color=\"");
        wire.push_str(&argb);
        wire.push_str("\"/>\n");
    }
    wire.push_str("E\n");
}

/// A veil-only FE wire (`""` theme): magic + the smoked/darkened tint
/// rules — File Explorer with no theme still gets readable glass.
pub fn fe_tint_wire(magic: &str, tint: &str) -> Option<String> {
    fe_veil_argb(tint)?;
    let mut wire = String::with_capacity(256);
    wire.push_str(magic);
    wire.push('\n');
    wire.push_str("E\n");
    apply_fe_tint(&mut wire, tint);
    Some(wire)
}

/// `Some(argb)` for `#RRGGBB` / `RRGGBB` palette values, `None` for
/// `""` (tint off) and malformed input — settings treats `None` on a
/// non-empty value as invalid and reverts.
pub fn argb(tint: &str) -> Option<String> {
    argb_with_alpha(tint, TINT_ALPHA)
}

/// `argb` with a caller-chosen alpha — File Explorer uses the lighter
/// `FE_TINT_ALPHA` so its XAML veil matches the Win32 punch.
pub fn argb_with_alpha(tint: &str, alpha: u8) -> Option<String> {
    let hex = tint.strip_prefix('#').unwrap_or(tint);
    if hex.is_empty() {
        return Some(String::new());
    }
    if hex.len() == 6 && hex.chars().all(|c| c.is_ascii_hexdigit()) {
        return Some(format!("#{alpha:02X}{}", hex.to_uppercase()));
    }
    None
}

/// Appends the tint rules to a theme wire in place — strips the `E\n`
/// end marker, emits `T`/`S` lines after the theme's own rules (last
/// writer wins on the background property), restores `E\n`. No-op when
/// `tint` is `""`.
pub fn apply_to_wire(wire: &mut String, targets: &[(&str, &str)], tint: &str) {
    apply_to_wire_alpha(wire, targets, tint, TINT_ALPHA);
}

/// `apply_to_wire` with a caller-chosen alpha (File Explorer's `FE_TINT_ALPHA`).
pub fn apply_to_wire_alpha(wire: &mut String, targets: &[(&str, &str)], tint: &str, alpha: u8) {
    let Some(argb) = argb_with_alpha(tint, alpha) else {
        return;
    };
    if argb.is_empty() {
        return;
    }
    if wire.ends_with("E\n") {
        wire.truncate(wire.len() - 2);
    }
    for (target, prop) in targets {
        wire.push_str("T ");
        wire.push_str(target);
        wire.push('\n');
        wire.push_str("S ");
        wire.push_str(prop);
        wire.push_str(":=<SolidColorBrush Color=\"");
        wire.push_str(&argb);
        wire.push_str("\"/>\n");
    }
    wire.push_str("E\n");
}

/// A tint-only wire (`""` theme) — just the magic header + tint rules,
/// so a surface with no selected style still gets the color overlay.
pub fn tint_wire(magic: &str, targets: &[(&str, &str)], tint: &str) -> Option<String> {
    tint_wire_alpha(magic, targets, tint, TINT_ALPHA)
}

/// `tint_wire` with a caller-chosen alpha (File Explorer's `FE_TINT_ALPHA`).
pub fn tint_wire_alpha(
    magic: &str,
    targets: &[(&str, &str)],
    tint: &str,
    alpha: u8,
) -> Option<String> {
    let argb = argb_with_alpha(tint, alpha)?;
    if argb.is_empty() {
        return None;
    }
    let mut wire = String::with_capacity(256);
    wire.push_str(magic);
    wire.push('\n');
    wire.push_str("E\n"); // apply_to_wire strips it again — keeps one code path
    apply_to_wire_alpha(&mut wire, targets, tint, alpha);
    Some(wire)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hex_values_expand_to_argb() {
        assert_eq!(argb("#E3008C").as_deref(), Some("#80E3008C"));
        assert_eq!(argb("e3008c").as_deref(), Some("#80E3008C"));
        assert_eq!(argb("").as_deref(), Some(""));
        assert!(argb("#FF00").is_none());
        assert!(argb("not-a-color").is_none());
    }

    #[test]
    fn tint_lines_append_after_the_theme_and_keep_the_end_marker() {
        let mut wire = String::from("PECO-TBS1\nT Grid\nS Fill=Red\nE\n");
        apply_to_wire(&mut wire, TASKBAR_TARGETS, "#107C10");
        assert!(wire.ends_with("E\n"));
        let fill = wire
            .find("S Fill:=<SolidColorBrush Color=\"#80107C10\"/>")
            .unwrap();
        let theme = wire.find("S Fill=Red").unwrap();
        assert!(fill > theme, "tint must land after the theme's rules");
        assert_eq!(wire.matches("T Rectangle#").count(), 2);
    }

    #[test]
    fn empty_tint_is_a_noop() {
        let mut wire = String::from("PECO-SMS1\nT Grid\nE\n");
        let before = wire.clone();
        apply_to_wire(&mut wire, START_MENU_TARGETS, "");
        assert_eq!(wire, before);
        assert!(tint_wire("PECO-FES1", FILE_EXPLORER_TARGETS, "").is_none());
    }

    #[test]
    fn tint_only_wire_is_wellformed() {
        let wire = fe_tint_wire("PECO-FES1", "#0078D4").unwrap();
        assert!(wire.starts_with("PECO-FES1\n"));
        assert!(wire.ends_with("E\n"));
        assert!(wire.contains("T FileExplorerExtensions.FileExplorerTabControl > Grid\n"));
        // #0078D4 darkened to 65% → #004E89 under the 0x48 veil alpha.
        assert!(wire.contains("S Background:=<SolidColorBrush Color=\"#48004E89\"/>"));
        // One veil per surface — no overlapping child targets.
        assert_eq!(wire.matches("S Background:=").count(), 3);
    }
}
