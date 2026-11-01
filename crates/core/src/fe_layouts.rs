//! File Explorer toolbar/icon layout presets (`feLayout` setting).
//!
//! Ported from the mod's `fe_layouts_only` preset table — pure positioning
//! rules only: **no `Background`, `BorderBrush`, or `Fill` styles may ever
//! appear here** (the EBM material pipeline owns every visual surface). Each
//! preset serializes to a `PECO-FES1` wire consumed by the tap resident's
//! mux (WinUI3) styler — same grammar `taskbar_themes::theme_wire` emits.

/// One `(target, styles)` pair from the preset table.
pub struct LayoutRule {
    pub target: &'static str,
    pub styles: &'static [&'static str],
}

/// A selectable toolbar/icon arrangement for File Explorer.
pub struct LayoutPreset {
    pub id: &'static str,
    pub label: &'static str,
    pub rules: &'static [LayoutRule],
    /// `name=value` pairs; expanded into style strings before emission.
    pub constants: &'static [&'static str],
}

/// Longest-name-first `$name` substitution, the mod's own ordering.
fn expand(s: &str, constants: &[(String, String)]) -> String {
    let mut out = s.to_string();
    for (k, v) in constants {
        let needle = format!("${k}");
        out = out.replace(&needle, v);
    }
    out
}

/// Every preset, in dropdown order (`""` = Windows Default, not listed).
pub const LAYOUTS: &[LayoutPreset] = &[
    LayoutPreset {
        id: "NoCommandBar",
        label: "No Command Bar",
        rules: &[
            LayoutRule {
                target: "FileExplorerExtensions.CommandBarControl_Wave1, FileExplorerExtensions.CommandBarControl",
                styles: &["Visibility=Collapsed"],
            },
            LayoutRule {
                target: "FileExplorerExtensions.NavigationBarControl",
                styles: &["Grid.RowSpan=2", "Margin=0,0,0,1"],
            },
        ],
        constants: &[],
    },
    LayoutPreset {
        id: "Minimal_Explorer11",
        label: "Minimal Explorer 11",
        rules: &[
            LayoutRule {
                target: "AppBarButton#refreshButton",
                styles: &["Visibility=Collapsed"],
            },
            LayoutRule {
                target: "AppBarButton#upButton",
                styles: &["Visibility=Collapsed"],
            },
            LayoutRule {
                target: "Border#BottomBorderLine",
                styles: &["Visibility=Collapsed"],
            },
            LayoutRule {
                target: "FileExplorerExtensions.CommandBarControl_Wave1, FileExplorerExtensions.CommandBarControl",
                styles: &["Visibility=Collapsed"],
            },
            LayoutRule {
                target: "FileExplorerExtensions.AddressBarControl > Grid#PART_LayoutRoot > Grid#NormalModeGrid",
                styles: &["BorderThickness=0,0,0,1"],
            },
            LayoutRule {
                target: "Grid#TabContainerGrid > Border#LeftBottomBorderLine",
                styles: &["Visibility=Collapsed"],
            },
            LayoutRule {
                target: "Grid#TabContainerGrid > Border#RightBottomBorderLine",
                styles: &["Visibility=Collapsed"],
            },
            LayoutRule {
                target: "TabViewItem",
                styles: &["Margin=0,0,3,0"],
            },
            LayoutRule {
                target: "TabViewItem > Grid#LayoutRoot",
                styles: &["CornerRadius=4", "Margin=0,-3,0,3", "Height=28"],
            },
            LayoutRule {
                target: "TabViewItem > Grid#LayoutRoot > Canvas",
                styles: &["Visibility=Collapsed"],
            },
            LayoutRule {
                target: "Grid#FileExplorerAddressBarGrid",
                styles: &[
                    "Grid.ColumnSpan=2",
                    "Margin=0,0,10,0",
                    "Visibility=Collapsed",
                ],
            },
            LayoutRule {
                target: "AutoSuggestBox#FileExplorerSearchBox",
                styles: &["Visibility=Collapsed"],
            },
            LayoutRule {
                target: "AppBarButton#backButton > Grid#Root",
                styles: &["Padding=2"],
            },
            LayoutRule {
                target: "AppBarButton#forwardButton > Grid#Root",
                styles: &["Padding=2"],
            },
            LayoutRule {
                target: "AppBarButton#forwardButton > Grid#Root > Grid#ContentRoot > Viewbox#ContentViewbox",
                styles: &["Margin=9"],
            },
            LayoutRule {
                target: "AppBarButton#backButton > Grid#Root > Grid#ContentRoot > Viewbox#ContentViewbox",
                styles: &["Margin=9"],
            },
            LayoutRule {
                target: "Grid#PART_LayoutRoot",
                styles: &["MinHeight=28", "Height=28"],
            },
            LayoutRule {
                target: "Grid#TabContainerGrid > Border > Button#AddButton",
                styles: &["Margin=0,0,20,4"],
            },
            LayoutRule {
                target: "Border#ScrollIncreaseButtonContainer",
                styles: &["Margin=0,0,0,4"],
            },
            LayoutRule {
                target: "Border#ScrollDecreaseButtonContainer",
                styles: &["Margin=0,0,0,4"],
            },
            LayoutRule {
                target: "FileExplorerExtensions.NavigationBarControl#NavigationBarControl",
                styles: &["Grid.Row=0", "Grid.RowSpan=2"],
            },
            LayoutRule {
                target: "FileExplorerExtensions.FileExplorerTabControl",
                styles: &["Margin=100,0,0,-15", "Grid.RowSpan=2"],
            },
            LayoutRule {
                target: "FileExplorerExtensions.NavigationBarControl > Grid#NavigationBarControlGrid",
                styles: &["Margin=0,0,0,-18", "Width=100", "HorizontalAlignment=0"],
            },
        ],
        constants: &[],
    },
    LayoutPreset {
        id: "Tabless",
        label: "Tabless (Swapped Bars)",
        rules: &[
            LayoutRule {
                target: "FileExplorerExtensions.NavigationBarControl",
                styles: &["Grid.Row=$NavigationBarGrid"],
            },
            LayoutRule {
                target: "FileExplorerExtensions.CommandBarControl_Wave1, FileExplorerExtensions.CommandBarControl",
                styles: &["Grid.Row=$CommandBarGrid", "BorderThickness=0,0,0,1"],
            },
            LayoutRule {
                target: "Microsoft.UI.Xaml.Controls.Grid#TabContainerGrid > Border",
                styles: &["Visibility=Collapsed"],
            },
            LayoutRule {
                target: "Microsoft.UI.Xaml.Controls.Grid#TabContainer > Microsoft.UI.Xaml.Controls.Button#CloseButton",
                styles: &["Visibility=Collapsed"],
            },
            LayoutRule {
                target: "Microsoft.UI.Xaml.Controls.TabViewItem > Microsoft.UI.Xaml.Controls.Grid#LayoutRoot > Microsoft.UI.Xaml.Controls.Canvas",
                styles: &["Opacity=0"],
            },
            LayoutRule {
                target: "Microsoft.UI.Xaml.Controls.Grid#TabContainer",
                styles: &["BorderThickness=0", "Padding=1,0,0,1"],
            },
            LayoutRule {
                target: "Microsoft.UI.Xaml.Controls.ContentPresenter > Microsoft.UI.Xaml.Controls.StackPanel > Microsoft.UI.Xaml.Controls.TextBlock",
                styles: &[
                    "FontFamily=Segoe UI, Segoe Fluent Icons",
                    "FontWeight=Normal",
                ],
            },
            LayoutRule {
                target: "FileExplorerExtensions.FileExplorerTabControl",
                styles: &["Height=36"],
            },
            LayoutRule {
                target: "Microsoft.UI.Xaml.Controls.Viewbox#IconBox",
                styles: &["Margin=0,0,4,0"],
            },
            LayoutRule {
                target: "Microsoft.UI.Xaml.Controls.TabViewItem",
                styles: &["Margin=0,-8,0,0"],
            },
        ],
        constants: &["NavigationBarGrid=2", "CommandBarGrid=1"],
    },
    LayoutPreset {
        id: "Matter",
        label: "Matter (Clean Command Bar)",
        rules: &[
            LayoutRule {
                target: "CommandBar#FileExplorerCommandBar",
                styles: &["HorizontalAlignment=1"],
            },
            LayoutRule {
                target: "CommandBar#FileExplorerSecondaryCommandBar",
                styles: &["Visibility=1"],
            },
            LayoutRule {
                target: "Grid#TabContainerGrid > Border#LeftBottomBorderLine",
                styles: &["Visibility=Collapsed"],
            },
            LayoutRule {
                target: "Grid#TabContainerGrid > Border#RightBottomBorderLine",
                styles: &["Visibility=Collapsed"],
            },
            LayoutRule {
                target: "TabViewItem",
                styles: &["Margin=0,0,4,0"],
            },
            LayoutRule {
                target: "TabViewItem > Grid#LayoutRoot",
                styles: &["CornerRadius=5", "Margin=2,4,0,4", "Height=29"],
            },
            LayoutRule {
                target: "TabViewItem > Grid#LayoutRoot > Canvas",
                styles: &["Visibility=Collapsed"],
            },
            LayoutRule {
                target: "TabViewItem > Grid#LayoutRoot > Grid#TabContainer",
                styles: &["BorderThickness=0"],
            },
            LayoutRule {
                target: "Grid#TabContainerGrid > Border > Button#AddButton",
                styles: &["Visibility=0", "Margin=0,0,0,3"],
            },
            LayoutRule {
                target: "FileExplorerExtensions.CommandBarControl_Wave1 > Grid, Grid#CommandBarControlRootGrid",
                styles: &["BorderThickness=0"],
            },
            LayoutRule {
                target: "Grid#PART_LayoutRoot",
                styles: &["CornerRadius=6", "BorderThickness=0"],
            },
            LayoutRule {
                target: "FileExplorerExtensions.CommandBarControl_Wave1, FileExplorerExtensions.CommandBarControl",
                styles: &["Margin=0,-5,0,0"],
            },
            LayoutRule {
                target: "AutoSuggestBox#FileExplorerSearchBox > Grid#LayoutRoot > TextBox > Grid@CommonStates > Border#BorderElement",
                styles: &["CornerRadius=6", "BorderThickness=0"],
            },
            LayoutRule {
                target: "Microsoft.UI.Xaml.Controls.AppBarButton[ToolTipService.ToolTip = Cut]",
                styles: &["Visibility=1"],
            },
            LayoutRule {
                target: "Microsoft.UI.Xaml.Controls.AppBarButton[ToolTipService.ToolTip = Copy]",
                styles: &["Visibility=1"],
            },
            LayoutRule {
                target: "Microsoft.UI.Xaml.Controls.AppBarButton[ToolTipService.ToolTip = Paste]",
                styles: &["Visibility=1"],
            },
            LayoutRule {
                target: "Microsoft.UI.Xaml.Controls.AppBarButton[ToolTipService.ToolTip = Rename]",
                styles: &["Visibility=1"],
            },
            LayoutRule {
                target: "Microsoft.UI.Xaml.Controls.AppBarButton[ToolTipService.ToolTip = Share]",
                styles: &["Visibility=1"],
            },
            LayoutRule {
                target: "Microsoft.UI.Xaml.Controls.AppBarSeparator",
                styles: &["Visibility=1"],
            },
            LayoutRule {
                target: "Microsoft.UI.Xaml.Controls.Border#ScrollDecreaseButtonContainer",
                styles: &["Margin=0,0,0,3"],
            },
            LayoutRule {
                target: "Microsoft.UI.Xaml.Controls.Border#ScrollIncreaseButtonContainer",
                styles: &["Margin=0,0,0,3"],
            },
            LayoutRule {
                target: "Microsoft.UI.Xaml.Controls.AppBarButton#refreshButton",
                styles: &["Visibility=1"],
            },
            LayoutRule {
                target: "Microsoft.UI.Xaml.Controls.AppBarButton#upButton",
                styles: &["Visibility=1"],
            },
            LayoutRule {
                target: "Microsoft.UI.Xaml.Controls.AppBarButton#forwardButton",
                styles: &["Visibility=1"],
            },
            LayoutRule {
                target: "Microsoft.UI.Xaml.Controls.AppBarButton#backButton",
                styles: &["Visibility=1"],
            },
            LayoutRule {
                target: "Microsoft.UI.Xaml.Controls.AppBarButton[ToolTipService.ToolTip = Create a new item in the current location.]",
                styles: &["Visibility=1"],
            },
        ],
        constants: &[],
    },
    LayoutPreset {
        id: "WindowGlass",
        label: "Centered Icon Bar",
        rules: &[
            LayoutRule {
                target: "Microsoft.UI.Xaml.Controls.Grid#PART_LayoutRoot",
                styles: &["RenderTransform:=<TranslateTransform X=\\\"0\\\"/>"],
            },
            LayoutRule {
                target: "FileExplorerExtensions.FirstCrumbStackPanelControl#FirstCrumbStackPanel",
                styles: &["Visibility=Collapsed"],
            },
            LayoutRule {
                target: "Windows.UI.Xaml.Controls.Grid#RootCommandSearchGrid > Windows.UI.Xaml.Controls.Border#BorderElement",
                styles: &["Visibility=Collapsed"],
            },
            LayoutRule {
                target: "Microsoft.UI.Xaml.Controls.Primitives.NavigationViewItemPresenter#NavigationViewItemPresenter > Microsoft.UI.Xaml.Controls.Grid#LayoutRoot",
                styles: &["BorderThickness=$BorderThickness"],
            },
            LayoutRule {
                target: "Microsoft.UI.Xaml.Controls.CommandBar#FileExplorerCommandBar",
                styles: &[
                    "RenderTransform:=<TranslateTransform X=\\\"0\\\" Y=\\\"0\\\" />",
                    "HorizontalAlignment=Center",
                    "Margin=-4",
                    "Padding=10",
                ],
            },
            LayoutRule {
                target: "Microsoft.UI.Xaml.Controls.CommandBar#FileExplorerSecondaryCommandBar",
                styles: &[
                    "RenderTransform:=<TranslateTransform X=\\\"Auto\\\" />",
                    "HorizontalAlignment=Center",
                    "Margin=-4",
                    "Padding=10",
                    "Visibility=Collapsed",
                ],
            },
            LayoutRule {
                target: "Microsoft.UI.Xaml.Controls.CommandBar#FileExplorerCommandBar > Microsoft.UI.Xaml.Controls.Grid#LayoutRoot > Microsoft.UI.Xaml.Controls.Grid#ContentRoot",
                styles: &[
                    "CornerRadius=$CornerRadius",
                    "BorderThickness=$BorderThickness",
                ],
            },
            LayoutRule {
                target: "Microsoft.UI.Xaml.Controls.CommandBar#FileExplorerSecondaryCommandBar > Microsoft.UI.Xaml.Controls.Grid#LayoutRoot > Microsoft.UI.Xaml.Controls.Grid#ContentRoot",
                styles: &[
                    "CornerRadius=$CornerRadius",
                    "BorderThickness=$BorderThickness",
                    "Visibility=Collapsed",
                ],
            },
            LayoutRule {
                target: "Microsoft.UI.Xaml.Controls.Grid#NavigationBarControlGrid",
                styles: &[
                    "ColumnDefinitions:=<ColumnDefinitionCollection><ColumnDefinition Width=\\\"Auto\\\"/><ColumnDefinition Width=\\\"*\\\"/><ColumnDefinition Width=\\\"430\\\"/></ColumnDefinitionCollection>",
                ],
            },
            LayoutRule {
                target: "Microsoft.UI.Xaml.Controls.Grid#HomeViewRootGrid",
                styles: &["CornerRadius=8", "BorderThickness=0", "Margin=0,0,8,8"],
            },
            LayoutRule {
                target: "FileExplorerExtensions.GalleryViewControl#GalleryViewControl > Grid",
                styles: &["CornerRadius=8", "BorderThickness=0", "Margin=0,0,8,8"],
            },
            LayoutRule {
                target: "Grid#TabContainerGrid > Border#LeftBottomBorderLine",
                styles: &["Visibility=Collapsed"],
            },
            LayoutRule {
                target: "Grid#TabContainerGrid > Border#RightBottomBorderLine",
                styles: &["Visibility=Collapsed"],
            },
            LayoutRule {
                target: "TabViewItem > Grid#LayoutRoot",
                styles: &["CornerRadius=8", "Margin=5", "Height=35"],
            },
            LayoutRule {
                target: "TabViewItem > Grid#LayoutRoot > Canvas",
                styles: &["Visibility=Collapsed"],
            },
            LayoutRule {
                target: "Microsoft.UI.Xaml.Controls.Border#BottomBorderLine",
                styles: &["Visibility=Collapsed"],
            },
            LayoutRule {
                target: "Microsoft.UI.Xaml.Shapes.Path#LeftRadiusRenderArc",
                styles: &["Visibility=Collapsed"],
            },
            LayoutRule {
                target: "Microsoft.UI.Xaml.Shapes.Path#RightRadiusRenderArc",
                styles: &["Visibility=Collapsed"],
            },
            LayoutRule {
                target: "Microsoft.UI.Xaml.Controls.Grid#TabContainer",
                styles: &["Visibility=Visible"],
            },
            LayoutRule {
                target: "Microsoft.UI.Xaml.Controls.Viewbox#IconBox",
                styles: &["Visibility=Collapsed"],
            },
            LayoutRule {
                target: "Microsoft.UI.Xaml.Controls.Primitives.CommandBarFlyoutCommandBar > Grid#LayoutRoot > Grid#OuterContentRoot > Grid#ContentRoot > Grid#PrimaryItemsRoot",
                styles: &[
                    "BorderThickness=$BorderThickness",
                    "CornerRadius=$CornerRadius",
                ],
            },
            LayoutRule {
                target: "Microsoft.UI.Xaml.Controls.Primitives.CommandBarFlyoutCommandBar > Grid#LayoutRoot@ExpansionStates > Grid#OuterContentRoot > Grid#ContentRoot > Microsoft.UI.Xaml.Controls.Primitives.Popup#OverflowPopup",
                styles: &[
                    "VerticalOffset@ExpandedUp=-5",
                    "VerticalOffset@ExpandedDown=5",
                ],
            },
            LayoutRule {
                target: "Grid#OuterOverflowContentRootV2",
                styles: &["CornerRadius=$CornerRadius"],
            },
            LayoutRule {
                target: "Grid#OuterOverflowContentRootV2 > Grid#OverflowContentRoot > CommandBarOverflowPresenter#SecondaryItemsControl > Grid#LayoutRoot",
                styles: &[
                    "BorderThickness=$BorderThickness",
                    "CornerRadius=$CornerRadius",
                ],
            },
            LayoutRule {
                target: "MenuFlyoutPresenter > Border",
                styles: &[
                    "BorderThickness=$BorderThickness",
                    "CornerRadius=$CornerRadius",
                ],
            },
            LayoutRule {
                target: "Grid#SecondaryItemsControlShadowWrapper > CommandBarOverflowPresenter#SecondaryItemsControl > Grid#LayoutRoot",
                styles: &[
                    "BorderThickness=$BorderThickness",
                    "CornerRadius=$CornerRadius",
                ],
            },
            LayoutRule {
                target: "Microsoft.UI.Xaml.Controls.AutoSuggestBox#FileExplorerSearchBox > Microsoft.UI.Xaml.Controls.Grid#LayoutRoot > Microsoft.UI.Xaml.Controls.TextBox#TextBox",
                styles: &["CornerRadius=$CornerRadius", "Margin=0,0,180,0"],
            },
            LayoutRule {
                target: "Microsoft.UI.Xaml.Controls.Grid#FileExplorerAddressBarGrid",
                styles: &["MaxWidth=750", "CornerRadius=$CornerRadius"],
            },
            LayoutRule {
                target: "Microsoft.UI.Xaml.Controls.AutoSuggestBox#PART_AutoSuggestBox > Microsoft.UI.Xaml.Controls.Grid#LayoutRoot > Microsoft.UI.Xaml.Controls.TextBox#TextBox",
                styles: &["CornerRadius=$CornerRadius"],
            },
            LayoutRule {
                target: "Microsoft.UI.Xaml.Controls.CommandBar#NavigationCommands",
                styles: &["Margin=180,0,0,0"],
            },
            LayoutRule {
                target: "Microsoft.UI.Xaml.Controls.Border > Microsoft.UI.Xaml.Controls.Button#AddButton",
                styles: &["RenderTransform:=<TranslateTransform Y=\\\"-6\\\" />"],
            },
            LayoutRule {
                target: "Microsoft.UI.Xaml.Controls.CommandBar > * > Microsoft.UI.Xaml.Controls.TextBlock#TextLabel",
                styles: &["Visibility=Collapsed"],
            },
            LayoutRule {
                target: "Microsoft.UI.Xaml.Controls.Grid#SubItemChevronPanel > Microsoft.UI.Xaml.Controls.FontIcon#SubItemChevron",
                styles: &["RenderTransform:=<TranslateTransform X=\\\"-5\\\" Y=\\\"12\\\" />"],
            },
        ],
        constants: &["BorderThickness=0.3,1,0.3,0.3", "CornerRadius=8"],
    },
    LayoutPreset {
        id: "AddressSearchOnly",
        label: "Address & Search Only",
        rules: &[
            LayoutRule {
                target: "FileExplorerExtensions.NavigationBarControl",
                styles: &["Grid.Row=0", "MinHeight=48", "Margin=0,26,0,1"],
            },
            LayoutRule {
                target: "FileExplorerExtensions.FileExplorerTabControl",
                styles: &["Visibility=Collapsed"],
            },
            LayoutRule {
                target: "AppBarButton#refreshButton",
                styles: &["Visibility=Collapsed"],
            },
            LayoutRule {
                target: "AppBarButton#upButton",
                styles: &["Visibility=Collapsed"],
            },
            LayoutRule {
                target: "AppBarButton#backButton",
                styles: &["Visibility=Collapsed"],
            },
            LayoutRule {
                target: "AppBarButton#forwardButton",
                styles: &["Visibility=Collapsed"],
            },
        ],
        constants: &[],
    },
    LayoutPreset {
        id: "LiquidGlass",
        label: "Liquid Compact (No Command Bar)",
        rules: &[
            LayoutRule {
                target: "Microsoft.UI.Xaml.Controls.Grid#PART_LayoutRoot",
                styles: &["HorizontalAlignment=Stretch"],
            },
            LayoutRule {
                target: "FileExplorerExtensions.FirstCrumbStackPanelControl#FirstCrumbStackPanel",
                styles: &["Visibility=1"],
            },
            LayoutRule {
                target: "Windows.UI.Xaml.Controls.Grid#RootCommandSearchGrid > Windows.UI.Xaml.Controls.Border#BorderElement",
                styles: &["Visibility=1"],
            },
            LayoutRule {
                target: "Microsoft.UI.Xaml.Controls.Primitives.NavigationViewItemPresenter#NavigationViewItemPresenter > Microsoft.UI.Xaml.Controls.Grid#LayoutRoot",
                styles: &[
                    "BorderThickness=$ElementBorderThickness",
                    "CornerRadius=$ElementCornerRadius",
                ],
            },
            LayoutRule {
                target: "Microsoft.UI.Xaml.Controls.Grid#HomeViewRootGrid",
                styles: &[
                    "CornerRadius=$ElementCornerRadius",
                    "BorderThickness=$ElementBorderThickness",
                    "Margin=4,0",
                ],
            },
            LayoutRule {
                target: "FileExplorerExtensions.GalleryViewControl#GalleryViewControl > Grid",
                styles: &[
                    "CornerRadius=$ElementCornerRadius",
                    "BorderThickness=$ElementBorderThickness",
                    "Margin=4,0",
                ],
            },
            LayoutRule {
                target: "ToolTip",
                styles: &[
                    "BorderThickness=$ElementBorderThickness",
                    "CornerRadius=$ElementCornerRadius",
                ],
            },
            LayoutRule {
                target: "Grid#TabContainerGrid > Border#LeftBottomBorderLine",
                styles: &["Visibility=1"],
            },
            LayoutRule {
                target: "Grid#TabContainerGrid > Border#RightBottomBorderLine",
                styles: &["Visibility=1"],
            },
            LayoutRule {
                target: "TabViewItem > Grid#LayoutRoot",
                styles: &[
                    "Margin=5",
                    "Height=28",
                    "BorderThickness=$ElementBorderThickness",
                    "CornerRadius=$ElementCornerRadius",
                ],
            },
            LayoutRule {
                target: "TabViewItem > Grid#LayoutRoot > Canvas",
                styles: &["Visibility=1"],
            },
            LayoutRule {
                target: "Microsoft.UI.Xaml.Controls.Border#BottomBorderLine",
                styles: &["Visibility=1"],
            },
            LayoutRule {
                target: "Microsoft.UI.Xaml.Shapes.Path#LeftRadiusRenderArc",
                styles: &["Visibility=1"],
            },
            LayoutRule {
                target: "Microsoft.UI.Xaml.Shapes.Path#RightRadiusRenderArc",
                styles: &["Visibility=1"],
            },
            LayoutRule {
                target: "Microsoft.UI.Xaml.Controls.Grid#TabContainer",
                styles: &["Visibility=0"],
            },
            LayoutRule {
                target: "Microsoft.UI.Xaml.Controls.Viewbox#IconBox",
                styles: &["Visibility=1"],
            },
            LayoutRule {
                target: "CommandBarOverflowPresenter#SecondaryItemsControl > Grid#LayoutRoot",
                styles: &[
                    "BorderThickness=$ElementBorderThickness",
                    "CornerRadius=$ElementCornerRadius",
                ],
            },
            LayoutRule {
                target: "Microsoft.UI.Xaml.Controls.AutoSuggestBox#FileExplorerSearchBox > Microsoft.UI.Xaml.Controls.Grid#LayoutRoot > Microsoft.UI.Xaml.Controls.TextBox#TextBox",
                styles: &[
                    "CornerRadius=$ElementCornerRadius",
                    "BorderThickness=$ElementBorderThickness",
                ],
            },
            LayoutRule {
                target: "Microsoft.UI.Xaml.Controls.Grid#FileExplorerAddressBarGrid",
                styles: &[
                    "CornerRadius=$ElementCornerRadius",
                    "BorderThickness=$ElementBorderThickness",
                ],
            },
            LayoutRule {
                target: "Microsoft.UI.Xaml.Controls.AutoSuggestBox#PART_AutoSuggestBox > Microsoft.UI.Xaml.Controls.Grid#LayoutRoot > Microsoft.UI.Xaml.Controls.TextBox#TextBox",
                styles: &["CornerRadius=$ElementCornerRadius"],
            },
            LayoutRule {
                target: "Microsoft.UI.Xaml.Controls.Border > Microsoft.UI.Xaml.Controls.Button#AddButton",
                styles: &["RenderTransform:=<TranslateTransform Y=\\\"-8\\\" />"],
            },
            LayoutRule {
                target: "Microsoft.UI.Xaml.Controls.Grid#SubItemChevronPanel > Microsoft.UI.Xaml.Controls.FontIcon#SubItemChevron",
                styles: &["RenderTransform:=<TranslateTransform X=\\\"-5\\\" Y=\\\"12\\\" />"],
            },
            LayoutRule {
                target: "FileExplorerExtensions.CommandBarControl_Wave1, FileExplorerExtensions.CommandBarControl",
                styles: &["Visibility=1"],
            },
            LayoutRule {
                target: "FileExplorerExtensions.NavigationBarControl",
                styles: &["Grid.RowSpan=2", "Margin=0,0,0,1"],
            },
        ],
        constants: &[
            "BorderThickness=0.3,1,0.3,0.3",
            "ElementBorderThickness=0.3,0.3,0.3,1",
            "CornerRadius=12",
            "ElementCornerRadius=8",
        ],
    },
    LayoutPreset {
        id: "OS26_Liquid_Glass",
        label: "OS26 Floating Layout",
        rules: &[
            LayoutRule {
                target: "Grid#DetailsViewControlRootGrid",
                styles: &["Margin=20,20,20,1", "CornerRadius=15"],
            },
            LayoutRule {
                target: "Grid#HomeViewRootGrid",
                styles: &["Margin=20,20,20,0", "CornerRadius=15"],
            },
            LayoutRule {
                target: "FileExplorerExtensions.GalleryViewControl#GalleryViewControl > Grid",
                styles: &["Margin=20,20,20,0", "CornerRadius=15"],
            },
            LayoutRule {
                target: "Microsoft.UI.Xaml.Controls.Grid#GalleryRootGrid",
                styles: &["Margin=10", "CornerRadius=15"],
            },
            LayoutRule {
                target: "Microsoft.UI.Xaml.Controls.AppBarButton > Grid@CommonStates",
                styles: &[
                    "CornerRadius@Disabled=12",
                    "BorderThickness@Disabled=1",
                    "Margin@Disabled=2,6,2,6",
                    "Padding@Disabled=0,-7",
                ],
            },
            LayoutRule {
                target: "Microsoft.UI.Xaml.Controls.AppBarButton#backButton > Grid@CommonStates",
                styles: &[
                    "CornerRadius@Disabled=11",
                    "BorderThickness@Disabled=1",
                    "Margin@Disabled=0,0,0,0",
                    "Height@Disabled=32",
                    "Width@Disabled=20",
                    "Padding@Disabled=0,-2,0,2",
                ],
            },
            LayoutRule {
                target: "Microsoft.UI.Xaml.Controls.AppBarButton#forwardButton > Grid@CommonStates",
                styles: &[
                    "CornerRadius@Disabled=11",
                    "BorderThickness@Disabled=1",
                    "Margin@Disabled=0,0,0,0",
                    "Height@Disabled=32",
                    "Width@Disabled=20",
                    "Padding@Disabled=0,-2,0,2",
                ],
            },
            LayoutRule {
                target: "Microsoft.UI.Xaml.Controls.AppBarButton#upButton > Grid@CommonStates",
                styles: &[
                    "CornerRadius@Disabled=11",
                    "BorderThickness@Disabled=1",
                    "Margin@Disabled=0,0,0,0",
                    "Height@Disabled=32",
                    "Width@Disabled=20",
                    "Padding@Disabled=0,-2,0,2",
                ],
            },
            LayoutRule {
                target: "CommandBar#FileExplorerCommandBar",
                styles: &["HorizontalAlignment=1"],
            },
            LayoutRule {
                target: "CommandBar#FileExplorerSecondaryCommandBar",
                styles: &["MinHeight=0"],
            },
            LayoutRule {
                target: "Grid#TabContainerGrid > Border#LeftBottomBorderLine",
                styles: &["Visibility=Collapsed"],
            },
            LayoutRule {
                target: "Grid#TabContainerGrid > Border#RightBottomBorderLine",
                styles: &["Visibility=Collapsed"],
            },
            LayoutRule {
                target: "TabViewItem",
                styles: &["Margin=0,0,8,0"],
            },
            LayoutRule {
                target: "TabViewItem > Grid#LayoutRoot",
                styles: &[
                    "CornerRadius=12",
                    "Margin=2,4,0,4",
                    "Height=27",
                    "BorderThickness=1",
                ],
            },
            LayoutRule {
                target: "TabViewItem > Grid#LayoutRoot > Canvas",
                styles: &["Visibility=Collapsed"],
            },
            LayoutRule {
                target: "TabViewItem > Grid#LayoutRoot > Grid#TabContainer",
                styles: &["BorderThickness=0"],
            },
            LayoutRule {
                target: "Grid#TabContainerGrid > Border > Button#AddButton",
                styles: &[
                    "Visibility=Visible",
                    "Margin=0,0,0,2",
                    "CornerRadius=10",
                    "BorderThickness=1",
                    "Width=24",
                    "Height=24",
                ],
            },
            LayoutRule {
                target: "Microsoft.UI.Xaml.Controls.Grid#FileExplorerAddressBarGrid",
                styles: &["Margin=-6,0,0,0"],
            },
            LayoutRule {
                target: "Grid#PART_LayoutRoot",
                styles: &["CornerRadius=14", "BorderThickness=1", "Margin=2"],
            },
            LayoutRule {
                target: "FileExplorerExtensions.CommandBarControl_Wave1, FileExplorerExtensions.CommandBarControl",
                styles: &["Margin=0,0,0,0"],
            },
            LayoutRule {
                target: "AutoSuggestBox#FileExplorerSearchBox > Grid#LayoutRoot > TextBox > Grid@CommonStates > Border#BorderElement",
                styles: &["CornerRadius=14", "BorderThickness=1"],
            },
            LayoutRule {
                target: "Microsoft.UI.Xaml.Controls.AppBarButton",
                styles: &["CornerRadius=12", "BorderThickness=1", "Margin=3,0,3,1"],
            },
            LayoutRule {
                target: "Microsoft.UI.Xaml.Controls.AppBarToggleButton",
                styles: &["CornerRadius=12", "BorderThickness=1", "Margin=3,0,3,1"],
            },
            LayoutRule {
                target: "Microsoft.UI.Xaml.Controls.Grid#OuterOverflowContentRootV2",
                styles: &["CornerRadius=20"],
            },
            LayoutRule {
                target: "Button#MoreButton",
                styles: &["Visibility=Collapsed"],
            },
            LayoutRule {
                target: "Microsoft.UI.Xaml.Controls.AppBarSeparator",
                styles: &["Visibility=Collapsed"],
            },
            LayoutRule {
                target: "Microsoft.UI.Xaml.Controls.AppBarButton#backButton",
                styles: &["Margin=0,9,9,0"],
            },
            LayoutRule {
                target: "Microsoft.UI.Xaml.Controls.AppBarButton#forwardButton",
                styles: &["Margin=0,9,9,0"],
            },
            LayoutRule {
                target: "Microsoft.UI.Xaml.Controls.AppBarButton#upButton",
                styles: &["Margin=0,9,9,0"],
            },
            LayoutRule {
                target: "Microsoft.UI.Xaml.Controls.AppBarButton#refreshButton",
                styles: &["Margin=0,9,9,0"],
            },
        ],
        constants: &[],
    },
    LayoutPreset {
        id: "OS26_Liquid_Glass_variant_Compact",
        label: "OS26 Merged Top Bar",
        rules: &[
            LayoutRule {
                target: "Microsoft.UI.Xaml.Controls.Primitives.SuggestionsPopup",
                styles: &["Margin=0,0,0,900"],
            },
            LayoutRule {
                target: "Microsoft.UI.Xaml.Controls.AppBarButton > Grid@CommonStates",
                styles: &[
                    "CornerRadius@Disabled=12",
                    "BorderThickness@Disabled=1",
                    "Margin@Disabled=2,6,2,6",
                    "Padding@Disabled=0,-7",
                ],
            },
            LayoutRule {
                target: "CommandBar#FileExplorerCommandBar > Grid#LayoutRoot > Grid#ContentRoot > Button#MoreButton",
                styles: &[
                    "CornerRadius=12",
                    "BorderThickness=1",
                    "Margin=3,2,3,2",
                    "Width=45",
                    "Height=32",
                ],
            },
            LayoutRule {
                target: "Microsoft.UI.Xaml.Controls.AppBarButton#backButton > Grid@CommonStates",
                styles: &[
                    "CornerRadius@Disabled=11",
                    "BorderThickness@Disabled=1",
                    "Margin@Disabled=0,0,0,0",
                    "Height@Disabled=32",
                    "Width@Disabled=20",
                    "Padding@Disabled=0,-2,0,2",
                ],
            },
            LayoutRule {
                target: "Microsoft.UI.Xaml.Controls.AppBarButton#forwardButton > Grid@CommonStates",
                styles: &[
                    "CornerRadius@Disabled=11",
                    "BorderThickness@Disabled=1",
                    "Margin@Disabled=0,0,0,0",
                    "Height@Disabled=32",
                    "Width@Disabled=20",
                    "Padding@Disabled=0,-2,0,2",
                ],
            },
            LayoutRule {
                target: "Microsoft.UI.Xaml.Controls.AppBarButton#refreshButton > Grid@CommonStates",
                styles: &[
                    "CornerRadius@Disabled=11",
                    "BorderThickness@Disabled=1",
                    "Margin@Disabled=0,0,0,0",
                    "Height@Disabled=32",
                    "Width@Disabled=20",
                    "Padding@Disabled=0,-2,0,2",
                ],
            },
            LayoutRule {
                target: "Grid#DetailsViewControlRootGrid",
                styles: &["Margin=20,20,20,1", "CornerRadius=15"],
            },
            LayoutRule {
                target: "Microsoft.UI.Xaml.Controls.Grid > OuterOverflowContentRootV2",
                styles: &["CornerRadius=250"],
            },
            LayoutRule {
                target: "AppBarButton[7]",
                styles: &[
                    "Visibility=Collapsed",
                    "Width=0",
                    "MinWidth=0",
                    "Margin=0,0,0,0",
                    "Padding=0,0,0,0",
                ],
            },
            LayoutRule {
                target: "Microsoft.UI.Xaml.Controls.Viewbox > ContentViewB",
                styles: &["Visibility=Collapsed"],
            },
            LayoutRule {
                target: "Grid#HomeViewRootGrid",
                styles: &["Margin=20,20,20,0", "CornerRadius=15"],
            },
            LayoutRule {
                target: "FileExplorerExtensions.GalleryViewControl#GalleryViewControl > Grid",
                styles: &["Margin=20,20,20,0", "CornerRadius=15"],
            },
            LayoutRule {
                target: "Microsoft.UI.Xaml.Controls.Grid#GalleryRootGrid",
                styles: &["Margin=10", "CornerRadius=15"],
            },
            LayoutRule {
                target: "CommandBar#FileExplorerCommandBar",
                styles: &[
                    "Grid.Row=0",
                    "Grid.RowSpan=1",
                    "CornerRadius:=15",
                    "Width=400",
                    "HorizontalAlignment=Left",
                    "Padding=0,0,0,0",
                ],
            },
            LayoutRule {
                target: "CommandBar#FileExplorerCommandBar > Grid#LayoutRoot > Grid#ContentRoot > Grid#OverflowSeparator",
                styles: &["Visibility=Collapsed", "Width=0", "MinWidth=0"],
            },
            LayoutRule {
                target: "CommandBar#FileExplorerCommandBar > Grid#LayoutRoot > Grid#ContentRoot",
                styles: &["HorizontalAlignment=Left"],
            },
            LayoutRule {
                target: "CommandBar#FileExplorerCommandBar > Grid#LayoutRoot > Grid#ContentRoot > ItemsControl#PrimaryItemsControl",
                styles: &["HorizontalAlignment=Left"],
            },
            LayoutRule {
                target: "CommandBar#FileExplorerSecondaryCommandBar",
                styles: &["Visibility=Visible", "Margin=0,40,0,-20"],
            },
            LayoutRule {
                target: "Grid#TabContainerGrid",
                styles: &["Margin=370,1,0,1"],
            },
            LayoutRule {
                target: "Grid#TabContainerGrid > Border#LeftBottomBorderLine",
                styles: &["Visibility=Collapsed"],
            },
            LayoutRule {
                target: "Grid#TabContainerGrid > Border#RightBottomBorderLine",
                styles: &["Visibility=Collapsed"],
            },
            LayoutRule {
                target: "TabViewItem",
                styles: &["Width=150", "Height=40", "Margin=0,0,8,0"],
            },
            LayoutRule {
                target: "TabViewItem > Grid#LayoutRoot",
                styles: &[
                    "CornerRadius=10",
                    "BorderThickness=1",
                    "Margin=2,2,0,2",
                    "Height=35",
                ],
            },
            LayoutRule {
                target: "TabViewItem > Grid#LayoutRoot > Canvas",
                styles: &["Visibility=Collapsed"],
            },
            LayoutRule {
                target: "TabViewItem > Grid#LayoutRoot > Grid#TabContainer",
                styles: &["BorderThickness=0"],
            },
            LayoutRule {
                target: "Grid#TabContainerGrid > Border > Button#AddButton",
                styles: &[
                    "Visibility=Visible",
                    "Margin=0,0,0,4",
                    "CornerRadius=8",
                    "BorderThickness=1",
                    "Width=24",
                    "Height=24",
                ],
            },
            LayoutRule {
                target: "Grid#PART_LayoutRoot",
                styles: &["CornerRadius=10", "BorderThickness=1", "Margin=2"],
            },
            LayoutRule {
                target: "FileExplorerExtensions.CommandBarControl_Wave1, FileExplorerExtensions.CommandBarControl",
                styles: &["Grid.Row=0", "Grid.RowSpan=2", "Margin=0,0,0,0"],
            },
            LayoutRule {
                target: "AutoSuggestBox#FileExplorerSearchBox > Grid#LayoutRoot > TextBox > Grid@CommonStates",
                styles: &[
                    "BorderThickness=1",
                    "CornerRadius=10",
                    "Margin=-90,0,90,0",
                    "Height=32",
                ],
            },
            LayoutRule {
                target: "Microsoft.UI.Xaml.Controls.Grid#FileExplorerAddressBarGrid",
                styles: &["Margin=-8,0,90,0"],
            },
            LayoutRule {
                target: "CommandBarOverflowPresenter Microsoft.UI.Xaml.Controls.AppBarButton",
                styles: &["CornerRadius=8", "Margin=2,1,2,1"],
            },
            LayoutRule {
                target: "Microsoft.UI.Xaml.Controls.AppBarButton",
                styles: &["CornerRadius=12", "BorderThickness=1", "Margin=3,2,3,2"],
            },
            LayoutRule {
                target: "Microsoft.UI.Xaml.Controls.Grid#OuterOverflowContentRootV2",
                styles: &["CornerRadius=20"],
            },
            LayoutRule {
                target: "Microsoft.UI.Xaml.Controls.AppBarToggleButton",
                styles: &["CornerRadius=8", "BorderThickness=1", "Margin=3,0,3,1"],
            },
            LayoutRule {
                target: "Microsoft.UI.Xaml.Controls.AppBarSeparator",
                styles: &["Visibility=Collapsed"],
            },
            LayoutRule {
                target: "Microsoft.UI.Xaml.Controls.AppBarButton#backButton",
                styles: &["Margin=0,9,9,0", "Visibility=Visible"],
            },
            LayoutRule {
                target: "Microsoft.UI.Xaml.Controls.AppBarButton#forwardButton",
                styles: &["Margin=0,9,9,0", "Visibility=Visible"],
            },
            LayoutRule {
                target: "Microsoft.UI.Xaml.Controls.AppBarButton#upButton",
                styles: &["Margin=0,9,9,0", "Visibility=Collapsed"],
            },
            LayoutRule {
                target: "Microsoft.UI.Xaml.Controls.AppBarButton#refreshButton",
                styles: &["Visibility=Visible", "Margin=0,9,9,0"],
            },
            LayoutRule {
                target: "Microsoft.UI.Xaml.Controls.AppBarButton#stopButton",
                styles: &["Visibility=Collapsed", "Margin=0,9,9,0"],
            },
            LayoutRule {
                target: "FileExplorerExtensions.NavigationBarControl",
                styles: &["Grid.RowSpan=2"],
            },
        ],
        constants: &[],
    },
    LayoutPreset {
        id: "ZEUSosX_044",
        label: "ZEUS macOS Single Bar",
        rules: &[
            LayoutRule {
                target: "FileExplorerExtensions.CommandBarControl_Wave1 > Grid, Grid#CommandBarControlRootGrid",
                styles: &[
                    "BorderThickness=0",
                    "Grid.Row=0",
                    "Grid.RowSpan=2",
                    "HorizontalAlignment=Left",
                    "VerticalAlignment=Top",
                    "Width=155",
                    "Margin=197,-30,0,0",
                ],
            },
            LayoutRule {
                target: "Microsoft.UI.Xaml.Controls.CommandBar#FileExplorerCommandBar",
                styles: &["HorizontalAlignment=Left", "VerticalAlignment=Top"],
            },
            LayoutRule {
                target: "Microsoft.UI.Xaml.Controls.Border#BottomBorderLine",
                styles: &["Visibility=Collapsed"],
            },
            LayoutRule {
                target: "Microsoft.UI.Xaml.Controls.Grid#NavigationBarControlGrid",
                styles: &[
                    "ColumnDefinitions:=<ColumnDefinitionCollection><ColumnDefinition Width=\\\"Auto\\\"/><ColumnDefinition Width=\\\"*\\\"/><ColumnDefinition Width=\\\"380\\\"/></ColumnDefinitionCollection>",
                    "Margin=0,-16,0,-21",
                ],
            },
            LayoutRule {
                target: "Grid#TabContainerGrid",
                styles: &[
                    "Grid.Row=0",
                    "HorizontalAlignment=Left",
                    "Margin=100,0,0,0",
                    "Width=1",
                    "MaxWidth=1",
                ],
            },
            LayoutRule {
                target: "FileExplorerExtensions.FileExplorerTabControl",
                styles: &[
                    "HorizontalAlignment=Left",
                    "Margin=100,0,0,0",
                    "Width=1",
                    "MaxWidth=1",
                ],
            },
            LayoutRule {
                target: "TabViewItem",
                styles: &["Width=0", "Visibility=Collapsed"],
            },
            LayoutRule {
                target: "Grid#TabContainerGrid > Border > Button#AddButton",
                styles: &["Visibility=Collapsed"],
            },
            LayoutRule {
                target: "AutoSuggestBox#FileExplorerSearchBox > Grid#LayoutRoot > TextBox > Grid@CommonStates > Border#BorderElement",
                styles: &["BorderThickness=0"],
            },
            LayoutRule {
                target: "Microsoft.UI.Xaml.Controls.Grid#FileExplorerAddressBarGrid > Grid#LayoutRoot > TextBox > Grid@CommonStates > Border#BorderElement",
                styles: &["BorderThickness=0"],
            },
            LayoutRule {
                target: "Microsoft.UI.Xaml.Controls.AutoSuggestBox#FileExplorerSearchBox > Microsoft.UI.Xaml.Controls.Grid#LayoutRoot > Microsoft.UI.Xaml.Controls.TextBox#TextBox",
                styles: &[
                    "Margin=0,0,140,0",
                    "TextAlignment=Center",
                    "HorizontalContentAlignment=Center",
                ],
            },
            LayoutRule {
                target: "Microsoft.UI.Xaml.Controls.Grid#FileExplorerAddressBarGrid",
                styles: &[
                    "HorizontalAlignment=Stretch",
                    "Height=28",
                    "Margin=155,0,0,0",
                ],
            },
            LayoutRule {
                target: "AutoSuggestBox#FileExplorerSearchBox",
                styles: &[
                    "HorizontalAlignment=Stretch",
                    "Height=28",
                    "Margin=-7,-1,7,0",
                ],
            },
            LayoutRule {
                target: "Microsoft.UI.Xaml.Controls.CommandBar#FileExplorerCommandBar Button",
                styles: &["FontSize=14"],
            },
        ],
        constants: &[],
    },
    LayoutPreset {
        id: "Compact_Explorer11",
        label: "Compact Explorer 11",
        rules: &[
            LayoutRule {
                target: "Microsoft.UI.Xaml.Controls.Primitives.SuggestionsPopup",
                styles: &["Margin=0,0,0,900"],
            },
            LayoutRule {
                target: "Microsoft.UI.Xaml.Controls.AppBarButton > Grid@CommonStates",
                styles: &[
                    "CornerRadius@Disabled=10",
                    "BorderThickness@Disabled=1",
                    "Margin@Disabled=2,6,2,6",
                    "Padding@Disabled=0,-7",
                ],
            },
            LayoutRule {
                target: "CommandBar#FileExplorerCommandBar > Grid#LayoutRoot > Grid#ContentRoot > Button#MoreButton",
                styles: &[
                    "CornerRadius=10",
                    "BorderThickness=1",
                    "Margin=3,2,3,2",
                    "Width=45",
                    "Height=32",
                ],
            },
            LayoutRule {
                target: "Microsoft.UI.Xaml.Controls.AppBarButton#backButton > Grid@CommonStates",
                styles: &[
                    "CornerRadius@Disabled=11",
                    "BorderThickness@Disabled=1",
                    "Margin@Disabled=0,0,0,0",
                    "Height@Disabled=32",
                    "Width@Disabled=20",
                    "Padding@Disabled=0,-2,0,2",
                ],
            },
            LayoutRule {
                target: "Microsoft.UI.Xaml.Controls.AppBarButton#forwardButton > Grid@CommonStates",
                styles: &[
                    "CornerRadius@Disabled=11",
                    "BorderThickness@Disabled=1",
                    "Margin@Disabled=0,0,0,0",
                    "Height@Disabled=32",
                    "Width@Disabled=20",
                    "Padding@Disabled=0,-2,0,2",
                ],
            },
            LayoutRule {
                target: "Microsoft.UI.Xaml.Controls.AppBarButton#refreshButton > Grid@CommonStates",
                styles: &[
                    "CornerRadius@Disabled=11",
                    "BorderThickness@Disabled=1",
                    "Margin@Disabled=0,0,0,0",
                    "Height@Disabled=32",
                    "Width@Disabled=20",
                    "Padding@Disabled=0,-2,0,2",
                ],
            },
            LayoutRule {
                target: "Grid#DetailsViewControlRootGrid",
                styles: &["Margin=20,20,20,1", "CornerRadius=15"],
            },
            LayoutRule {
                target: "Microsoft.UI.Xaml.Controls.Grid > OuterOverflowContentRootV2",
                styles: &["CornerRadius=250"],
            },
            LayoutRule {
                target: "AppBarButton[7]",
                styles: &[
                    "Visibility=Collapsed",
                    "Width=0",
                    "MinWidth=0",
                    "Margin=0,0,0,0",
                    "Padding=0,0,0,0",
                ],
            },
            LayoutRule {
                target: "Microsoft.UI.Xaml.Controls.Viewbox > ContentViewB",
                styles: &["Visibility=Collapsed"],
            },
            LayoutRule {
                target: "Grid#HomeViewRootGrid",
                styles: &["Margin=20,20,20,0", "CornerRadius=15"],
            },
            LayoutRule {
                target: "FileExplorerExtensions.GalleryViewControl#GalleryViewControl > Grid",
                styles: &["Margin=20,20,20,0", "CornerRadius=15"],
            },
            LayoutRule {
                target: "Microsoft.UI.Xaml.Controls.Grid#GalleryRootGrid",
                styles: &["Margin=10", "CornerRadius=15"],
            },
            LayoutRule {
                target: "CommandBar#FileExplorerCommandBar",
                styles: &[
                    "Grid.Row=0",
                    "Grid.RowSpan=1",
                    "CornerRadius:=15",
                    "Width=400",
                    "HorizontalAlignment=Left",
                    "Padding=0,0,0,0",
                ],
            },
            LayoutRule {
                target: "CommandBar#FileExplorerCommandBar > Grid#LayoutRoot > Grid#ContentRoot > Grid#OverflowSeparator",
                styles: &["Visibility=Collapsed", "Width=0", "MinWidth=0"],
            },
            LayoutRule {
                target: "CommandBar#FileExplorerCommandBar > Grid#LayoutRoot > Grid#ContentRoot",
                styles: &["HorizontalAlignment=Left"],
            },
            LayoutRule {
                target: "CommandBar#FileExplorerCommandBar > Grid#LayoutRoot > Grid#ContentRoot > ItemsControl#PrimaryItemsControl",
                styles: &["HorizontalAlignment=Left"],
            },
            LayoutRule {
                target: "CommandBar#FileExplorerSecondaryCommandBar",
                styles: &["Visibility=Visible", "Margin=0,40,0,-20"],
            },
            LayoutRule {
                target: "Grid#TabContainerGrid",
                styles: &["Margin=370,1,0,1"],
            },
            LayoutRule {
                target: "Grid#TabContainerGrid > Border#LeftBottomBorderLine",
                styles: &["Visibility=Collapsed"],
            },
            LayoutRule {
                target: "Grid#TabContainerGrid > Border#RightBottomBorderLine",
                styles: &["Visibility=Collapsed"],
            },
            LayoutRule {
                target: "TabViewItem",
                styles: &["Width=150", "Height=40", "Margin=0,0,8,0"],
            },
            LayoutRule {
                target: "TabViewItem > Grid#LayoutRoot",
                styles: &[
                    "CornerRadius=10",
                    "BorderThickness=1",
                    "Margin=2,2,0,2",
                    "Height=35",
                ],
            },
            LayoutRule {
                target: "TabViewItem > Grid#LayoutRoot > Canvas",
                styles: &["Visibility=Collapsed"],
            },
            LayoutRule {
                target: "TabViewItem > Grid#LayoutRoot > Grid#TabContainer",
                styles: &["BorderThickness=0"],
            },
            LayoutRule {
                target: "Grid#TabContainerGrid > Border > Button#AddButton",
                styles: &[
                    "Visibility=Visible",
                    "Margin=0,0,0,4",
                    "CornerRadius=10",
                    "BorderThickness=0",
                    "Width=24",
                    "Height=24",
                ],
            },
            LayoutRule {
                target: "Grid#PART_LayoutRoot",
                styles: &["CornerRadius=10", "BorderThickness=1", "Margin=1"],
            },
            LayoutRule {
                target: "FileExplorerExtensions.CommandBarControl_Wave1, FileExplorerExtensions.CommandBarControl",
                styles: &["Grid.Row=0", "Grid.RowSpan=2", "Margin=0,0,0,0"],
            },
            LayoutRule {
                target: "AutoSuggestBox#FileExplorerSearchBox > Grid#LayoutRoot > TextBox > Grid@CommonStates",
                styles: &["CornerRadius=10", "Margin=-90,0,90,0", "Height=30"],
            },
            LayoutRule {
                target: "Microsoft.UI.Xaml.Controls.Grid#FileExplorerAddressBarGrid",
                styles: &["Margin=-8,0,90,0"],
            },
            LayoutRule {
                target: "CommandBarOverflowPresenter#SecondaryItemsControl > Microsoft.UI.Xaml.Controls.AppBarButton",
                styles: &["CornerRadius=4", "BorderThickness=0", "Margin=0,0,0,0"],
            },
            LayoutRule {
                target: "Microsoft.UI.Xaml.Controls.AppBarButton",
                styles: &["CornerRadius=10", "BorderThickness=1", "Margin=3,2,3,2"],
            },
            LayoutRule {
                target: "Microsoft.UI.Xaml.Controls.AppBarToggleButton",
                styles: &["CornerRadius=8", "BorderThickness=1", "Margin=3,0,3,1"],
            },
            LayoutRule {
                target: "Microsoft.UI.Xaml.Controls.AppBarSeparator",
                styles: &["Visibility=Collapsed"],
            },
            LayoutRule {
                target: "Microsoft.UI.Xaml.Controls.AppBarButton#backButton",
                styles: &["Margin=0,9,9,0", "Visibility=Visible"],
            },
            LayoutRule {
                target: "Microsoft.UI.Xaml.Controls.AppBarButton#forwardButton",
                styles: &["Margin=0,9,9,0", "Visibility=Visible"],
            },
            LayoutRule {
                target: "Microsoft.UI.Xaml.Controls.AppBarButton#upButton",
                styles: &["Margin=0,9,9,0", "Visibility=Collapsed"],
            },
            LayoutRule {
                target: "Microsoft.UI.Xaml.Controls.AppBarButton#refreshButton",
                styles: &["Visibility=Visible", "Margin=0,9,9,0"],
            },
            LayoutRule {
                target: "Microsoft.UI.Xaml.Controls.AppBarButton#stopButton",
                styles: &["Visibility=Collapsed", "Margin=0,9,9,0"],
            },
            LayoutRule {
                target: "FileExplorerExtensions.NavigationBarControl",
                styles: &["Grid.RowSpan=2"],
            },
        ],
        constants: &[],
    },
    LayoutPreset {
        id: "Float",
        label: "Floating Spaced Bars",
        rules: &[
            LayoutRule {
                target: "TabViewItem > Grid#LayoutRoot@CommonStates",
                styles: &["CornerRadius=6"],
            },
            LayoutRule {
                target: "TabViewItem > Grid#LayoutRoot > Grid#TabContainer",
                styles: &["BorderThickness=0"],
            },
            LayoutRule {
                target: "TabViewItem > Grid#LayoutRoot > Canvas",
                styles: &["Visibility=Collapsed"],
            },
            LayoutRule {
                target: "TabViewItem > Grid#LayoutRoot",
                styles: &["BorderThickness=1", "Margin=2,0,0,0", "Height=35"],
            },
            LayoutRule {
                target: "Microsoft.UI.Xaml.Controls.Border#BottomBorderLine",
                styles: &["Visibility=Collapsed"],
            },
            LayoutRule {
                target: "Grid#TabContainerGrid > Border#LeftBottomBorderLine",
                styles: &["Visibility=Collapsed"],
            },
            LayoutRule {
                target: "Grid#TabContainerGrid > Border#RightBottomBorderLine",
                styles: &["Visibility=Collapsed"],
            },
            LayoutRule {
                target: "TabViewItem",
                styles: &["CornerRadius=4"],
            },
            LayoutRule {
                target: "Grid#TabContainerGrid > Border > Button#AddButton",
                styles: &[
                    "Visibility=Visible",
                    "Margin=0,0,0,3",
                    "CornerRadius=10",
                    "BorderThickness=0",
                    "Width=24",
                    "Height=24",
                ],
            },
            LayoutRule {
                target: "Microsoft.UI.Xaml.Controls.Grid#TabContainerGrid",
                styles: &["Height=44"],
            },
            LayoutRule {
                target: "Microsoft.UI.Xaml.Controls.Border#RightBottomBorderLine",
                styles: &["Visibility=Collapsed"],
            },
            LayoutRule {
                target: "Microsoft.UI.Xaml.Controls.Border#LeftBottomBorderLine",
                styles: &["Visibility=Collapsed"],
            },
            LayoutRule {
                target: "Microsoft.UI.Xaml.Controls.Grid#NavigationBarControlGrid",
                styles: &["CornerRadius=6", "Margin=8,4,8,0", "Height=54"],
            },
            LayoutRule {
                target: "FileExplorerExtensions.CommandBarControl_Wave1 > Grid, Grid#CommandBarControlRootGrid",
                styles: &["Margin=0,8,0,0", "BorderThickness=0,1,0,1"],
            },
            LayoutRule {
                target: "Microsoft.UI.Xaml.Controls.Primitives.TabViewListView#TabListView",
                styles: &["Margin=-3,0,0,0"],
            },
            LayoutRule {
                target: "Microsoft.UI.Xaml.Shapes.Path#RightRadiusRenderArc",
                styles: &["Visibility=Collapsed"],
            },
            LayoutRule {
                target: "Microsoft.UI.Xaml.Shapes.Path#LeftRadiusRenderArc",
                styles: &["Visibility=Collapsed"],
            },
        ],
        constants: &[],
    },
];

/// The preset's dropdown options: `("", "Windows Default")` first.
pub fn options() -> impl Iterator<Item = (&'static str, &'static str)> {
    std::iter::once(("", "Windows Default")).chain(LAYOUTS.iter().map(|p| (p.id, p.label)))
}

/// `PECO-FES1` wire for the preset — `T target` then its `S prop=value`
/// rules with `$constants` expanded. `None` for unknown ids; `""` needs no
/// wire (the reset path restores originals instead).
pub fn layout_wire(id: &str) -> Option<String> {
    let p = LAYOUTS.iter().find(|p| p.id == id)?;
    let mut constants: Vec<(String, String)> = p
        .constants
        .iter()
        .filter_map(|c| c.split_once('='))
        .map(|(k, v)| (k.trim().to_string(), v.trim().to_string()))
        .collect();
    for i in 0..constants.len() {
        let prior = constants[..i].to_vec();
        constants[i].1 = expand(&constants[i].1, &prior);
    }
    constants.sort_by_key(|c| std::cmp::Reverse(c.0.len()));

    let mut out = String::with_capacity(8 * 1024);
    out.push_str("PECO-FES1\n");
    for rule in p.rules {
        let target = expand(rule.target, &constants);
        out.push_str("T ");
        out.push_str(&target.replace(['\t', '\n'], " "));
        out.push('\n');
        for style in rule.styles {
            let style = expand(style, &constants);
            out.push_str("S ");
            out.push_str(&style);
            out.push('\n');
        }
    }
    out.push_str("E\n");
    Some(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_preset_serializes_without_visual_rules() {
        // The EBM material pipeline owns every surface — a layout wire that
        // emitted a brush would silently fight it. Hard-guard both levels.
        for p in LAYOUTS {
            for rule in p.rules {
                for s in rule.styles {
                    let head = s.split(['=', ':']).next().unwrap_or("");
                    assert!(
                        !matches!(head, "Background" | "BorderBrush" | "Fill"),
                        "{} emits forbidden style {s}",
                        p.id,
                    );
                }
            }
            let wire = layout_wire(p.id).expect("wire");
            assert!(
                wire.starts_with(
                    "PECO-FES1
"
                ),
                "{}",
                p.id
            );
            for line in wire.lines().filter(|l| l.starts_with("S ")) {
                let head = line[2..].split(['=', ':']).next().unwrap_or("");
                assert!(
                    !matches!(head, "Background" | "BorderBrush" | "Fill"),
                    "{} wire emits {line}",
                    p.id,
                );
            }
        }
    }

    #[test]
    fn wire_expands_constants_and_splits_targets() {
        let w = layout_wire("Tabless").unwrap();
        assert!(!w.contains('$'), "constants left unexpanded: {w}");
        assert!(w.contains("Grid.Row=2"));
        assert!(w.contains("Grid.Row=1"));
        assert_eq!(options().count(), 1 + LAYOUTS.len());
        assert!(layout_wire("").is_none());
        assert!(layout_wire("nope").is_none());
    }
}
