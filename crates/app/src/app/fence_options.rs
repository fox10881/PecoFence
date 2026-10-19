//! Per-fence options shared by the fence context menu and the settings page's 「栅栏」 tab:
//! one setter per property (state + window + anchor + save) and the page protocol
//! (`fences[]` details in the state JSON, `setFence` messages, `showFence` navigation).
//!
//! Window-level properties (appearance, title on hover, lock, quick-hide exclusion, auto
//! height, dock) act on the fence's host window when the fence is a tab; content properties
//! (icon size, spacing, portal flags) act on the fence itself, exactly like the menu did.

use super::*;

/// Fences-style colour choices for the per-fence tint (name, rgb); the settings page builds
/// its 色调 options from this list (`tintPalette` in the state JSON).
const TINT_PALETTE: &[(&str, [u8; 3])] = &[
    ("红", [0xE7, 0x48, 0x56]),
    ("橙", [0xF7, 0x63, 0x0C]),
    ("黄", [0xFF, 0xB9, 0x00]),
    ("绿", [0x10, 0x89, 0x3E]),
    ("青", [0x00, 0xB7, 0xC3]),
    ("蓝", [0x00, 0x78, 0xD4]),
    ("紫", [0x87, 0x64, 0xB8]),
    ("粉", [0xE3, 0x00, 0x8C]),
    ("灰", [0x7A, 0x75, 0x74]),
];

pub(super) fn tint_palette_json() -> serde_json::Value {
    TINT_PALETTE
        .iter()
        .map(|(name, rgb)| serde_json::json!({ "name": pecofence_core::i18n::text(name), "hex": hex(*rgb) }))
        .collect()
}

/// "更透明" / "更厚实" opacity presets (the menu's former values) and "全透明": no plate until
/// the pointer is over the fence.
const OPACITY_CLEAR: f32 = 0.55;
const OPACITY_SOLID: f32 = 1.6;
const OPACITY_NONE: f32 = 0.0;

fn hex(rgb: [u8; 3]) -> String {
    format!("{:02X}{:02X}{:02X}", rgb[0], rgb[1], rgb[2])
}

fn parse_hex(s: &str) -> Option<[u8; 3]> {
    let s = s.trim_start_matches('#');
    if s.len() != 6 {
        return None;
    }
    let v = u32::from_str_radix(s, 16).ok()?;
    Some([(v >> 16) as u8, (v >> 8) as u8, v as u8])
}

/// Every property `parse_fence_prop` accepts (the order the error message lists them in).
pub(super) const FENCE_PROPS: &[&str] = &[
    "title",
    "iconSize",
    "spacing",
    "autoHeight",
    "locked",
    "excludeFromQuickHide",
    "opacity",
    "tint",
    "tintTitleOnly",
    "tabColor",
    "titleColor",
    "titleSize",
    "titleOnHover",
    "layout",
    "sort",
    "reverse",
    "groupByDate",
    "labelLines",
    "showLabels",
    "iconShape",
    "customIconSize",
    "spacingX",
    "spacingY",
    "hideScrollbar",
    "lockScroll",
    "portalNavigate",
    "portalTitleIcon",
    "hideWhenEmpty",
];

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum TitleColorChoice {
    Theme,
    /// Follow the fence tint.
    Tint,
    Rgb([u8; 3]),
}

/// Parsed「显示标题栏」 choice: `title_on_hover` covers default/hover/always; Never maps to
/// `appearance.title_hidden`, which folds the title row out entirely.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum TitleBarChoice {
    /// Follow the global `rollUp.titleOnHover`.
    Default,
    /// Always show the title row.
    Always,
    /// Show it only while the pointer is over the fence.
    Hover,
    /// Never show it — the row is gone (no draw, no hit) and content reclaims its height.
    Never,
}

/// One per-fence option, parsed and range-checked (shared by the settings page's `setFence`
/// and the CLI's `fences.setOption`).
#[derive(Clone, Debug, PartialEq)]
pub(super) enum FenceProp {
    Title(String),
    IconSize(u32),
    Spacing(Spacing),
    AutoHeight(bool),
    Locked(bool),
    ExcludeFromQuickHide(bool),
    /// None = the global default; otherwise a preset (`OPACITY_CLEAR` / `_SOLID` / `_NONE`).
    Opacity(Option<f32>),
    Tint(Option<[u8; 3]>),
    TintTitleOnly(bool),
    /// None = the tab's tint.
    TabColor(Option<[u8; 3]>),
    TitleColor(TitleColorChoice),
    /// None = normal.
    TitleSize(Option<TitleSize>),
    /// 「显示标题栏」: global / always / hover / never (`appearance.title_hidden`).
    TitleOnHover(TitleBarChoice),
    PortalNavigate(bool),
    PortalTitleIcon(bool),
    /// Inbox only: 为空时自动隐藏.
    HideWhenEmpty(bool),
    Layout(ViewLayout),
    Sort(SortMode),
    Reverse(bool),
    GroupByDate(bool),
    LabelLines(u8),
    /// 显示图标标签 (FenceView.show_labels): false parks the label rows without losing
    /// `label_lines`, so covers can sit on the bookshelf boards.
    ShowLabels(bool),
    /// 图标形状 (FenceView.icon_shape): Poster draws a 2:3 centre-cropped cover.
    IconShape(IconShape),
    /// 自定义图标大小 (FenceView.custom_icon_size): None follows `iconSize`.
    CustomIconSize(Option<u32>),
    /// 水平间距 / 垂直间距 (FenceView.spacing_x / spacing_y): None follows `spacing`;
    /// 0.0 packs the cells flush.
    GapX(Option<f32>),
    GapY(Option<f32>),
    /// 隐藏滚动条 (FenceView.hide_scrollbar): no track/arrows/thumb; wheel still scrolls.
    HideScrollbar(bool),
    /// 锁定滚动 (FenceView.lock_scroll): no scrolling at all; the offset pins to the top.
    LockScroll(bool),
}

/// Parses `prop` / `value` as the settings page and `fence get` spell them. Unknown property
/// → `invalid_value` listing every property; a bad value → `invalid_value` listing what the
/// property accepts.
pub(super) fn parse_fence_prop(
    prop: &str,
    value: &serde_json::Value,
) -> std::result::Result<FenceProp, pecofence_ipc::IpcError> {
    use pecofence_ipc::IpcError;
    let bad = |allowed: &[&str]| {
        IpcError::invalid_value(
            format!("{prop} cannot be {value}; allowed: {}", allowed.join(", ")),
            allowed,
        )
    };
    let as_bool = || value.as_bool().ok_or_else(|| bad(&["true", "false"]));
    let as_str = |allowed: &[&str]| value.as_str().ok_or_else(|| bad(allowed));
    Ok(match prop {
        "title" => {
            let title = as_str(&["<non-empty string>"])?;
            FenceProp::Title(super::ipc::checked_name("title", title)?)
        }
        "iconSize" => {
            const ALLOWED: &[&str] = &["32", "48", "64", "96"];
            match value.as_u64() {
                Some(size @ (32 | 48 | 64 | 96)) => FenceProp::IconSize(size as u32),
                _ => return Err(bad(ALLOWED)),
            }
        }
        "spacing" => {
            const ALLOWED: &[&str] = &["compact", "normal", "loose"];
            FenceProp::Spacing(match as_str(ALLOWED)? {
                "compact" => Spacing::Compact,
                "normal" => Spacing::Normal,
                "loose" => Spacing::Loose,
                _ => return Err(bad(ALLOWED)),
            })
        }
        "autoHeight" => FenceProp::AutoHeight(as_bool()?),
        "locked" => FenceProp::Locked(as_bool()?),
        "excludeFromQuickHide" => FenceProp::ExcludeFromQuickHide(as_bool()?),
        "opacity" => {
            const ALLOWED: &[&str] = &["default", "clear", "solid", "transparent"];
            FenceProp::Opacity(match as_str(ALLOWED)? {
                "default" => None,
                "clear" => Some(OPACITY_CLEAR),
                "solid" => Some(OPACITY_SOLID),
                "transparent" => Some(OPACITY_NONE),
                _ => return Err(bad(ALLOWED)),
            })
        }
        "tint" | "tabColor" => {
            const ALLOWED: &[&str] = &["#RRGGBB", "null"];
            let rgb = match value {
                serde_json::Value::Null => None,
                serde_json::Value::String(s) if s.is_empty() => None,
                serde_json::Value::String(s) => Some(parse_hex(s).ok_or_else(|| bad(ALLOWED))?),
                _ => return Err(bad(ALLOWED)),
            };
            if prop == "tint" {
                FenceProp::Tint(rgb)
            } else {
                FenceProp::TabColor(rgb)
            }
        }
        "tintTitleOnly" => FenceProp::TintTitleOnly(as_bool()?),
        "titleColor" => {
            const ALLOWED: &[&str] = &["theme", "tint", "white", "black", "#RRGGBB"];
            FenceProp::TitleColor(match as_str(ALLOWED)? {
                "theme" => TitleColorChoice::Theme,
                "tint" => TitleColorChoice::Tint,
                "white" => TitleColorChoice::Rgb([0xFF, 0xFF, 0xFF]),
                "black" => TitleColorChoice::Rgb([0x00, 0x00, 0x00]),
                other => TitleColorChoice::Rgb(parse_hex(other).ok_or_else(|| bad(ALLOWED))?),
            })
        }
        "titleSize" => {
            const ALLOWED: &[&str] = &["small", "normal", "large", "extraLarge"];
            FenceProp::TitleSize(match as_str(ALLOWED)? {
                "small" => Some(TitleSize::Small),
                "normal" => None,
                "large" => Some(TitleSize::Large),
                "extraLarge" => Some(TitleSize::ExtraLarge),
                _ => return Err(bad(ALLOWED)),
            })
        }
        "titleOnHover" => {
            const ALLOWED: &[&str] = &["default", "hover", "always", "never"];
            FenceProp::TitleOnHover(match as_str(ALLOWED)? {
                "default" => TitleBarChoice::Default,
                "hover" => TitleBarChoice::Hover,
                "always" => TitleBarChoice::Always,
                "never" => TitleBarChoice::Never,
                _ => return Err(bad(ALLOWED)),
            })
        }
        "layout" => {
            const ALLOWED: &[&str] = &["icons", "list", "details"];
            FenceProp::Layout(match as_str(ALLOWED)? {
                "icons" => ViewLayout::Icons,
                "list" => ViewLayout::List,
                "details" => ViewLayout::Details,
                _ => return Err(bad(ALLOWED)),
            })
        }
        "sort" => {
            const ALLOWED: &[&str] = &["manual", "name", "type", "date", "size", "openCount"];
            FenceProp::Sort(match as_str(ALLOWED)? {
                "manual" => SortMode::Manual,
                "name" => SortMode::Name,
                "type" => SortMode::Type,
                "date" => SortMode::Date,
                "size" => SortMode::Size,
                "openCount" => SortMode::OpenCount,
                _ => return Err(bad(ALLOWED)),
            })
        }
        "reverse" => FenceProp::Reverse(as_bool()?),
        "groupByDate" => FenceProp::GroupByDate(as_bool()?),
        "labelLines" => {
            const ALLOWED: &[&str] = &["1", "2", "3"];
            match value.as_u64() {
                Some(n @ 1..=3) => FenceProp::LabelLines(n as u8),
                _ => return Err(bad(ALLOWED)),
            }
        }
        "showLabels" => FenceProp::ShowLabels(as_bool()?),
        "iconShape" => {
            const ALLOWED: &[&str] = &["standard", "poster"];
            FenceProp::IconShape(match as_str(ALLOWED)? {
                "standard" => IconShape::Standard,
                "poster" => IconShape::Poster,
                _ => return Err(bad(ALLOWED)),
            })
        }
        "customIconSize" => {
            const ALLOWED: &[&str] = &["16..=256", "null"];
            match value {
                serde_json::Value::Null => FenceProp::CustomIconSize(None),
                _ => match value.as_u64() {
                    Some(n @ 16..=256) => FenceProp::CustomIconSize(Some(n as u32)),
                    _ => return Err(bad(ALLOWED)),
                },
            }
        }
        "spacingX" | "spacingY" => {
            const ALLOWED: &[&str] = &["0..=64", "null"];
            let gap = match value {
                serde_json::Value::Null => None,
                _ => Some(
                    value
                        .as_f64()
                        .filter(|n| (0.0..=64.0).contains(n))
                        .ok_or_else(|| bad(ALLOWED))? as f32,
                ),
            };
            if prop == "spacingX" {
                FenceProp::GapX(gap)
            } else {
                FenceProp::GapY(gap)
            }
        }
        "hideScrollbar" => FenceProp::HideScrollbar(as_bool()?),
        "lockScroll" => FenceProp::LockScroll(as_bool()?),
        "portalNavigate" => FenceProp::PortalNavigate(as_bool()?),
        "portalTitleIcon" => FenceProp::PortalTitleIcon(as_bool()?),
        "hideWhenEmpty" => FenceProp::HideWhenEmpty(as_bool()?),
        _ => {
            return Err(IpcError::invalid_value(
                format!("unknown fence property {prop:?}"),
                FENCE_PROPS,
            ));
        }
    })
}

/// String forms of `FenceView.layout` / `.sort` as the page, `fence get` and `fence set` use.
pub(super) fn layout_name(layout: ViewLayout) -> &'static str {
    match layout {
        ViewLayout::Icons => "icons",
        ViewLayout::List => "list",
        ViewLayout::Details => "details",
    }
}

pub(super) fn sort_name(sort: SortMode) -> &'static str {
    match sort {
        SortMode::Manual => "manual",
        SortMode::Name => "name",
        SortMode::Type => "type",
        SortMode::Date => "date",
        SortMode::Size => "size",
        SortMode::OpenCount => "openCount",
    }
}

impl App {
    pub(super) fn set_fence_auto_height(&mut self, fence: FenceId, on: bool) {
        let host = self.state.host_of(fence);
        self.state.set_auto_height(host, on);
        if let Some(w) = self.fences.get(&host) {
            w.set_auto_height(on);
        }
        self.apply_auto_height(host);
        self.schedule_save();
    }

    pub(super) fn set_fence_locked(&mut self, fence: FenceId, on: bool) {
        let host = self.state.host_of(fence);
        self.state.set_locked(host, on);
        if let Some(w) = self.fences.get(&host) {
            w.set_locked(on);
        }
        self.schedule_save();
    }

    pub(super) fn set_fence_quick_hide_excluded(&mut self, fence: FenceId, on: bool) {
        let host = self.state.host_of(fence);
        self.state.set_exclude_from_quick_hide(host, on);
        if let Some(w) = self.fences.get(&host)
            && let Some(a) = self.anchor.borrow_mut().as_mut()
        {
            a.set_quick_hide_excluded(w.hwnd(), on);
        }
        self.schedule_save();
    }

    /// The inbox's 为空时自动隐藏 (other fences keep the flag but never hide).
    pub(super) fn set_fence_hide_when_empty(&mut self, fence: FenceId, on: bool) {
        self.state.set_hide_when_empty(fence, on);
        self.sync_inbox_window();
        self.schedule_save();
    }

    /// `None` = default opacity; otherwise one of the presets (or any imported value).
    pub(super) fn set_fence_opacity(&mut self, fence: FenceId, opacity: Option<f32>) {
        let host = self.state.host_of(fence);
        self.state.set_appearance(host, None, opacity);
        self.apply_fence_appearance(host);
        self.schedule_save();
    }

    /// Tint / title colour / title size in one go (`None` fields = theme defaults). A title
    /// that followed the tint keeps following it; clearing the tint clears such a title.
    pub(super) fn set_fence_style(
        &mut self,
        fence: FenceId,
        tint: Option<[u8; 3]>,
        title_rgb: Option<[u8; 3]>,
        title_size: Option<TitleSize>,
    ) {
        let host = self.state.host_of(fence);
        self.state.set_style(host, tint, title_rgb, title_size);
        self.apply_fence_appearance(host);
        // Tab colour bars follow the tint.
        self.refresh_fence(host);
        self.schedule_save();
    }

    pub(super) fn set_fence_tint(&mut self, fence: FenceId, tint: Option<[u8; 3]>) {
        let host = self.state.host_of(fence);
        let (old_tint, mut title_rgb, title_size) = self.style_of(host);
        let follow = title_rgb.is_some() && title_rgb == old_tint;
        if follow {
            title_rgb = tint;
        }
        self.set_fence_style(fence, tint, title_rgb, title_size);
    }

    pub(super) fn set_fence_tint_title_only(&mut self, fence: FenceId, on: bool) {
        let host = self.state.host_of(fence);
        self.state.set_tint_title_only(host, on);
        self.apply_fence_appearance(host);
        self.schedule_save();
    }

    /// A tab's own bar colour; it stays with the tab, whichever window it is in.
    pub(super) fn set_fence_tab_color(&mut self, fence: FenceId, rgb: Option<[u8; 3]>) {
        self.state.set_tab_color(fence, rgb);
        self.refresh_fence(fence);
        self.schedule_save();
    }

    /// A tab keeps its own title appearance when it joins or leaves a stack.
    fn set_fence_title_style(
        &mut self,
        fence: FenceId,
        color: Option<[u8; 3]>,
        size: Option<TitleSize>,
    ) {
        let (tint, _, _) = self.style_of(fence);
        self.state.set_style(fence, tint, color, size);
        if self.state.host_of(fence) == fence {
            self.apply_fence_appearance(fence);
        }
        self.refresh_fence(fence);
        self.schedule_save();
    }

    /// 「显示标题栏」 choice (window-level, like the rest of `appearance`). `Never` sets
    /// `title_hidden` — the row folds out of the layout — so the fence re-snaps to its
    /// cells afterward.
    pub(super) fn set_fence_title_bar(&mut self, fence: FenceId, choice: TitleBarChoice) {
        let host = self.state.host_of(fence);
        match choice {
            TitleBarChoice::Default => {
                self.state.set_title_hidden(host, false);
                self.state.set_title_on_hover(host, None);
            }
            TitleBarChoice::Hover => {
                self.state.set_title_hidden(host, false);
                self.state.set_title_on_hover(host, Some(true));
            }
            TitleBarChoice::Always => {
                self.state.set_title_hidden(host, false);
                self.state.set_title_on_hover(host, Some(false));
            }
            TitleBarChoice::Never => self.state.set_title_hidden(host, true),
        }
        self.apply_fence_appearance(host);
        self.apply_cell_snap(fence);
        self.apply_auto_height(fence);
        self.schedule_save();
    }

    pub(super) fn set_fence_spacing(&mut self, fence: FenceId, spacing: Spacing) {
        self.state.set_spacing(fence, spacing);
        if let Some(w) = self.window_for(fence)
            && w.active_fence() == fence
        {
            w.set_spacing(spacing);
        }
        self.apply_cell_snap(fence);
        self.apply_auto_height(fence);
        self.schedule_save();
    }

    pub(super) fn set_fence_portal_navigate(&mut self, fence: FenceId, on: bool) {
        self.state.set_portal_navigate(fence, on);
        self.apply_portal_deco(self.state.host_of(fence));
        self.schedule_save();
    }

    pub(super) fn set_fence_title_icon(&mut self, fence: FenceId, show: bool) {
        self.state.set_hide_title_icon(fence, !show);
        let host = self.state.host_of(fence);
        // The tab pills carry the glyph too: push the strip so they follow at once.
        if let Some(w) = self.fences.get(&host) {
            w.set_tabs(self.tab_views(host), self.state.active_tab_of(host));
        }
        self.apply_portal_deco(host);
        self.schedule_save();
    }

    /// (tint, title colour, title size) of the host window's appearance override.
    fn style_of(&self, host: FenceId) -> (Option<[u8; 3]>, Option<[u8; 3]>, Option<TitleSize>) {
        self.state
            .fence(host)
            .and_then(|f| f.appearance.as_ref())
            .map(|a| (a.tint_rgb, a.title_rgb, a.title_size))
            .unwrap_or((None, None, None))
    }

    /// Opens the settings window on the 「栅栏」 page with `fence` selected.
    pub(super) fn open_fence_options(&mut self, fence: FenceId) {
        self.open_settings();
        let Some(host) = &self.settings else {
            return;
        };
        if host.ready {
            self.post_show_fence(fence);
        } else {
            // The page is still loading; `ready` delivers the state and then the latest
            // request: this fence, or the feedback form.
            self.settings_focus_fence = Some(fence);
            self.feedback.focus = None;
        }
    }

    pub(super) fn post_show_fence(&self, fence: FenceId) {
        if let Some(h) = &self.settings {
            h.post_json(&serde_json::json!({ "type": "showFence", "id": fence }).to_string());
        }
    }

    /// The per-fence block of the state JSON. Window-level values come from the host so a
    /// tab shows (and edits) what its window actually uses.
    pub(super) fn fence_options_json(&self, f: &pecofence_core::Fence) -> serde_json::Value {
        let host_id = self.state.host_of(f.id);
        let h = self.state.fence(host_id).unwrap_or(f);
        let (tint, _, _) = self.style_of(host_id);
        let (_, title_rgb, title_size) = self.style_of(f.id);
        let opacity = h.appearance.as_ref().and_then(|a| a.opacity);
        let opacity = match opacity {
            None => "default",
            Some(o) if o <= 0.0 => "transparent",
            Some(o) if o < 1.0 => "clear",
            Some(_) => "solid",
        };
        let title_color = match title_rgb {
            None => "theme".to_string(),
            Some(rgb) if tint == Some(rgb) => "tint".to_string(),
            Some([0xFF, 0xFF, 0xFF]) => "white".to_string(),
            Some([0x00, 0x00, 0x00]) => "black".to_string(),
            Some(rgb) => hex(rgb),
        };
        let title_on_hover = if h.appearance.as_ref().and_then(|a| a.title_hidden) == Some(true)
        {
            "never"
        } else {
            match h.appearance.as_ref().and_then(|a| a.title_on_hover) {
                None => "default",
                Some(true) => "hover",
                Some(false) => "always",
            }
        };
        let title_size = match title_size.unwrap_or_default() {
            TitleSize::Small => "small",
            TitleSize::Normal => "normal",
            TitleSize::Large => "large",
            TitleSize::ExtraLarge => "extraLarge",
        };
        let spacing = match f.view.spacing {
            Spacing::Compact => "compact",
            Spacing::Normal => "normal",
            Spacing::Loose => "loose",
        };
        let portal = (f.kind == FenceKind::FolderPortal).then(|| {
            serde_json::json!({
                "navigate": f.portal_navigate,
                "titleIcon": !f.hide_title_icon,
            })
        });
        serde_json::json!({
            "id": f.id,
            "title": f.title,
            "kind": match f.kind {
                FenceKind::Inbox => "inbox",
                FenceKind::Virtual => "virtual",
                FenceKind::FolderPortal => "portal",
            },
            "host": (host_id != f.id).then(|| self.state.fence(host_id).map(|h| h.title.clone())),
            "tabbed": self.state.tabs_of(host_id).len() > 1,
            "iconSize": f.view.icon_size,
            "spacing": spacing,
            "showLabels": f.view.show_labels.unwrap_or(true),
            "iconShape": match f.view.effective_icon_shape() {
                IconShape::Standard => "standard",
                IconShape::Poster => "poster",
            },
            "customIconSize": f.view.custom_icon_size,
            "spacingX": f.view.spacing_x,
            "spacingY": f.view.spacing_y,
            "hideScrollbar": f.view.hide_scrollbar.unwrap_or(false),
            "lockScroll": f.view.lock_scroll.unwrap_or(false),
            "autoHeight": h.view.auto_height,
            "locked": h.locked,
            "excludeFromQuickHide": h.exclude_from_quick_hide,
            "opacity": opacity,
            "tint": tint.map(hex),
            "tintTitleOnly": h.appearance.as_ref().is_some_and(|a| a.tint_title_only),
            "tabColor": f.appearance.as_ref().and_then(|a| a.tab_rgb).map(hex),
            "titleColor": title_color,
            "titleSize": title_size,
            "titleOnHover": title_on_hover,
            "hideWhenEmpty": f.hide_when_empty,
            "portal": portal,
        })
    }

    /// `{"type":"setFence","id":…,"prop":…,"value":…}` from the page.
    pub(super) fn on_set_fence(&mut self, v: &serde_json::Value) {
        let Some(fence) = v
            .get("id")
            .and_then(|i| i.as_str())
            .and_then(|i| uuid::Uuid::parse_str(i).ok())
        else {
            return;
        };
        if self.state.fence(fence).is_none() {
            return;
        }
        let Some(prop) = v.get("prop").and_then(|p| p.as_str()) else {
            return;
        };
        let value = v.get("value").cloned().unwrap_or(serde_json::Value::Null);
        if prop == "dockTop" {
            // An action, not a property (the CLI has `fences.dockTop`); the page still sends
            // it through `setFence`.
            self.dock_to_top(self.state.host_of(fence));
        } else {
            match parse_fence_prop(prop, &value) {
                Ok(p) => {
                    self.apply_fence_prop(fence, p);
                }
                Err(error) => {
                    tracing::warn!(prop, %error, "bad fence property from settings page");
                    return;
                }
            }
        }
        self.push_settings_state();
    }

    /// Pushes the shown fence's effective label rows to its window: `labelLines`, or 0 while
    /// `showLabels` is off (the saved line count survives either way).
    fn push_label_lines(&mut self, fence: FenceId) {
        let Some(lines) = self
            .state
            .fence(fence)
            .map(|f| f.view.effective_label_lines())
        else {
            return;
        };
        if let Some(w) = self.window_for(fence)
            && w.active_fence() == fence
        {
            w.set_label_lines(lines);
        }
    }

    /// Pushes the shown fence's grid overrides to its window: 「图标形状」 plus the per-axis
    /// gaps (`spacing_x` / `spacing_y`), each `None` falling back to the named `spacing`.
    fn push_grid_extras(&mut self, fence: FenceId) {
        let Some(view) = self.state.fence(fence).map(|f| f.view.clone()) else {
            return;
        };
        if let Some(w) = self.window_for(fence)
            && w.active_fence() == fence
        {
            w.set_grid_extras(view.effective_icon_shape(), view.spacing_x, view.spacing_y);
        }
    }

    /// Pushes `effective_icon_size` (custom size while set, else the preset) to the window —
    /// `set_icon_size` reloads the icons when the edge actually changes.
    fn push_effective_icon_size(&mut self, fence: FenceId) {
        let Some(size) = self.state.fence(fence).map(|f| f.view.effective_icon_size())
        else {
            return;
        };
        if let Some(w) = self.window_for(fence)
            && w.active_fence() == fence
        {
            w.set_icon_size(size);
        }
    }

    /// Applies one parsed property to `fence` (window-level ones act on its host, see the
    /// module docs). Returns whether the persisted state actually changed.
    pub(super) fn apply_fence_prop(&mut self, fence: FenceId, prop: FenceProp) -> bool {
        let host = self.state.host_of(fence);
        let before = (
            self.state.fence(fence).cloned(),
            self.state.fence(host).cloned(),
        );
        match prop {
            FenceProp::Title(title) => {
                self.state.rename_fence(fence, &title);
                self.refresh_fence(fence);
                if let Some(w) = self.window_for(fence) {
                    w.redraw();
                }
                self.schedule_save();
            }
            FenceProp::IconSize(size) => self.apply_icon_size(fence, size),
            FenceProp::Spacing(spacing) => self.set_fence_spacing(fence, spacing),
            FenceProp::AutoHeight(on) => self.set_fence_auto_height(fence, on),
            FenceProp::Locked(on) => self.set_fence_locked(fence, on),
            FenceProp::ExcludeFromQuickHide(on) => self.set_fence_quick_hide_excluded(fence, on),
            FenceProp::Opacity(op) => self.set_fence_opacity(fence, op),
            FenceProp::Tint(tint) => self.set_fence_tint(fence, tint),
            FenceProp::TintTitleOnly(on) => self.set_fence_tint_title_only(fence, on),
            FenceProp::TabColor(rgb) => self.set_fence_tab_color(fence, rgb),
            FenceProp::TitleColor(choice) => {
                let (tint, _, _) = self.style_of(host);
                let (_, _, title_size) = self.style_of(fence);
                let title_rgb = match choice {
                    TitleColorChoice::Theme => None,
                    TitleColorChoice::Tint => tint,
                    TitleColorChoice::Rgb(rgb) => Some(rgb),
                };
                self.set_fence_title_style(fence, title_rgb, title_size);
            }
            FenceProp::TitleSize(size) => {
                let (_, title_rgb, _) = self.style_of(fence);
                self.set_fence_title_style(fence, title_rgb, size);
            }
            FenceProp::TitleOnHover(choice) => self.set_fence_title_bar(fence, choice),
            FenceProp::PortalNavigate(on) => self.set_fence_portal_navigate(fence, on),
            FenceProp::PortalTitleIcon(on) => self.set_fence_title_icon(fence, on),
            FenceProp::HideWhenEmpty(on) => self.set_fence_hide_when_empty(fence, on),
            FenceProp::Layout(layout) => {
                // Same steps as the fence menu's 视图 items.
                self.state.set_layout(fence, layout);
                if let Some(w) = self.window_for(fence)
                    && w.active_fence() == fence
                {
                    w.set_layout(layout);
                }
                self.apply_cell_snap(fence);
                self.apply_auto_height(fence);
                self.schedule_save();
            }
            FenceProp::Sort(sort) => {
                self.state.set_sort(fence, sort);
                self.refresh_fence(fence);
                self.schedule_save();
            }
            FenceProp::Reverse(on) => {
                self.state.set_reverse(fence, on);
                self.refresh_fence(fence);
                self.schedule_save();
            }
            FenceProp::GroupByDate(on) => {
                self.state.set_group_by_date(fence, on);
                self.refresh_fence(fence);
                self.schedule_save();
            }
            FenceProp::LabelLines(lines) => {
                if let Some(f) = self.state.fence_mut(fence)
                    && f.view.label_lines != lines
                {
                    f.view.label_lines = lines;
                    self.state.mark_dirty();
                }
                self.push_label_lines(fence);
                self.apply_cell_snap(fence);
                self.apply_auto_height(fence);
                self.schedule_save();
            }
            FenceProp::ShowLabels(on) => {
                // `None` while on, so the field leaves the file until labels are turned off.
                let stored = (!on).then_some(false);
                if let Some(f) = self.state.fence_mut(fence)
                    && f.view.show_labels != stored
                {
                    f.view.show_labels = stored;
                    self.state.mark_dirty();
                }
                self.push_label_lines(fence);
                self.apply_cell_snap(fence);
                self.apply_auto_height(fence);
                self.schedule_save();
            }
            FenceProp::IconShape(shape) => {
                let stored = (shape != IconShape::Standard).then_some(shape);
                if let Some(f) = self.state.fence_mut(fence)
                    && f.view.icon_shape != stored
                {
                    f.view.icon_shape = stored;
                    self.state.mark_dirty();
                }
                self.push_grid_extras(fence);
                self.apply_cell_snap(fence);
                self.apply_auto_height(fence);
                self.schedule_save();
            }
            FenceProp::CustomIconSize(size) => {
                if let Some(f) = self.state.fence_mut(fence)
                    && f.view.custom_icon_size != size
                {
                    f.view.custom_icon_size = size;
                    self.state.mark_dirty();
                }
                // The effective edge changed: push it like `apply_icon_size` does.
                self.push_effective_icon_size(fence);
                self.drop_unneeded_icon_requests();
                self.push_grid_extras(fence);
                self.apply_cell_snap(fence);
                self.apply_auto_height(fence);
                self.schedule_save();
            }
            FenceProp::GapX(gap) | FenceProp::GapY(gap) => {
                let x = matches!(prop, FenceProp::GapX(_));
                if let Some(f) = self.state.fence_mut(fence) {
                    let slot = if x {
                        &mut f.view.spacing_x
                    } else {
                        &mut f.view.spacing_y
                    };
                    if *slot != gap {
                        *slot = gap;
                        self.state.mark_dirty();
                    }
                }
                self.push_grid_extras(fence);
                self.apply_cell_snap(fence);
                self.apply_auto_height(fence);
                self.schedule_save();
            }
            FenceProp::HideScrollbar(on) => {
                self.state.set_hide_scrollbar(fence, on);
                if let Some(w) = self.window_for(fence)
                    && w.active_fence() == fence
                {
                    w.set_hide_scrollbar(on);
                }
                self.schedule_save();
            }
            FenceProp::LockScroll(on) => {
                self.state.set_lock_scroll(fence, on);
                if let Some(w) = self.window_for(fence)
                    && w.active_fence() == fence
                {
                    w.set_lock_scroll(on);
                }
                self.schedule_save();
            }
        }
        let after = (
            self.state.fence(fence).cloned(),
            self.state.fence(host).cloned(),
        );
        before != after
    }
}
