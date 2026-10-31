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
pub const NOTIFICATION_CENTER_TARGETS: &[(&str, &str)] = &[
    ("Grid#NotificationCenterGrid", "Background"),
    ("Grid#ControlCenterRegion", "Background"),
];

/// `Some(argb)` for `#RRGGBB` / `RRGGBB` palette values, `None` for
/// `""` (tint off) and malformed input — settings treats `None` on a
/// non-empty value as invalid and reverts.
pub fn argb(tint: &str) -> Option<String> {
    let hex = tint.strip_prefix('#').unwrap_or(tint);
    if hex.is_empty() {
        return Some(String::new());
    }
    if hex.len() == 6 && hex.chars().all(|c| c.is_ascii_hexdigit()) {
        return Some(format!("#{TINT_ALPHA:02X}{}", hex.to_uppercase()));
    }
    None
}

/// Appends the tint rules to a theme wire in place — strips the `E\n`
/// end marker, emits `T`/`S` lines after the theme's own rules (last
/// writer wins on the background property), restores `E\n`. No-op when
/// `tint` is `""`.
pub fn apply_to_wire(wire: &mut String, targets: &[(&str, &str)], tint: &str) {
    let Some(argb) = argb(tint) else {
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
    let argb = argb(tint)?;
    if argb.is_empty() {
        return None;
    }
    let mut wire = String::with_capacity(256);
    wire.push_str(magic);
    wire.push('\n');
    wire.push_str("E\n"); // apply_to_wire strips it again — keeps one code path
    apply_to_wire(&mut wire, targets, tint);
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
        assert!(tint_wire("PECO-NCS1", NOTIFICATION_CENTER_TARGETS, "").is_none());
    }

    #[test]
    fn tint_only_wire_is_wellformed() {
        let wire = tint_wire("PECO-NCS1", NOTIFICATION_CENTER_TARGETS, "#0078D4").unwrap();
        assert!(wire.starts_with("PECO-NCS1\n"));
        assert!(wire.ends_with("E\n"));
        assert!(wire.contains("T Grid#NotificationCenterGrid\n"));
        assert!(wire.contains("S Background:=<SolidColorBrush Color=\"#800078D4\"/>"));
    }
}
