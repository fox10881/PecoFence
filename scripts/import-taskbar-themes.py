#!/usr/bin/env python3
"""Convert the Windhawk `windows-11-taskbar-styler` theme structs into the
PecoFence backend catalog (crates/core/data/taskbar_themes.json).

The source is the mod's published C++ source: `const Theme g_theme<Name> = {{...},
{...}, {...}}` aggregates — three lists (targetStyles, styleConstants,
themeResourceVariables). The selectable `theme` setting strings come from the
mod's GetSelectedTheme() switch; that mapping is reproduced in SELECTABLE below
(id = exact `Wh_GetStringSetting` value, struct = g_theme<struct> suffix).

`Squircle` is special: the mod auto-picks g_themeSquircle_variant_WeatherOnTheRight
when OS feature 48660958 (ExtendedModeAEPForTaskbar) is enabled. The catalog
keeps the variant as a `weatherVariant` sub-payload + `variantFeature` metadata;
the host resolves which wire to build at apply time.
"""
import json
import re
import sys

SRC = sys.argv[1] if len(sys.argv) > 1 else "windows-11-taskbar-styler.txt"
DST = sys.argv[2] if len(sys.argv) > 2 else "crates/core/data/taskbar_themes.json"

# setting id -> (struct suffix, display label)
SELECTABLE = [
    ("TranslucentTaskbar", "TranslucentTaskbar", "Translucent Taskbar"),
    ("DockLike", "DockLike", "Dock Like"),
    ("SimplyTransparent", "SimplyTransparent", "Simply Transparent"),
    ("Squircle", "Squircle", "Squircle"),
    ("Matter", "Matter", "Matter"),
    ("WinXP", "WinXP", "Windows XP"),
    ("WinXP_variant_Zune", "WinXP_variant_Zune", "Windows XP — Zune"),
    ("Bubbles", "Bubbles", "Bubbles"),
    ("RosePine", "RosePine", "Rosé Pine"),
    ("WinVista", "WinVista", "Windows Vista"),
    ("CleanSlate", "CleanSlate", "Clean Slate"),
    ("Lucent", "Lucent", "Lucent"),
    ("Lucent_variant_Light", "Lucent_variant_Light", "Lucent — Light"),
    ("SunValley", "SunValley", "Sun Valley"),
    ("21996Taskbar", "21996Taskbar", "Build 21996 Taskbar"),
    ("BottomDensy", "BottomDensy", "BottomDensy"),
    ("BottomDensy_variant_NoInd", "BottomDensy_variant_NoInd", "BottomDensy — No Indicators"),
    ("TaskbarXII", "TaskbarXII", "TaskbarXII"),
    ("xdark", "xdark", "xdark"),
    ("Windows7", "Windows7", "Windows 7"),
    ("Aeris", "Aeris", "Aeris"),
    ("Plasma", "Plasma", "Plasma"),
    ("WindowGlass", "WindowGlass", "Window Glass"),
    ("WindowGlass_variant_Split", "WindowGlass_variant_Split", "Window Glass — Split"),
    ("WindowGlass_variant_FullLength", "WindowGlass_variant_FullLength", "Window Glass — Full Length"),
    ("Surface", "Surface", "Surface"),
    ("Oversimplified&Accentuated", "Oversimplified_Accentuated", "Oversimplified & Accentuated"),
    ("Luminosity_variant_Dock", "Luminosity_variant_Dock", "Luminosity — Dock"),
    ("Luminosity_variant_Classic", "Luminosity_variant_Classic", "Luminosity — Classic"),
    ("Luminosity_variant_Compact", "Luminosity_variant_Compact", "Luminosity — Compact"),
    ("LayerMicaUI", "LayerMicaUI", "Layer Mica UI"),
    ("Fluid", "Fluid", "Fluid"),
    ("TintedGlass", "TintedGlass", "Tinted Glass"),
    ("TaskbarToStatusbar", "TaskbarToStatusbar", "Taskbar to Statusbar"),
    ("UltraWideFriendly", "UltraWideFriendly", "Ultra-Wide Friendly"),
    ("LiquidGlass2", "LiquidGlass2", "Liquid Glass 2"),
    ("LiquidGlass", "LiquidGlass", "Liquid Glass"),
    ("LiquidGlass_variant_Alternate", "LiquidGlass_variant_Alternate", "Liquid Glass — Alternate"),
    ("Borderless", "Borderless", "Borderless"),
    ("Command_Center", "Command_Center", "Command Center"),
    ("FluentGlass", "FluentGlass", "Fluent Glass"),
    ("OS26_Liquid_Glass_variant_ClearMacDock", "OS26_Liquid_Glass_variant_ClearMacDock", "OS26 Liquid Glass — Clear Mac Dock"),
    ("OS26_Liquid_Glass_variant_ClearMacDockCompact", "OS26_Liquid_Glass_variant_ClearMacDockCompact", "OS26 Liquid Glass — Clear Mac Dock Compact"),
    ("OS26_Liquid_Glass_variant_ClearTaskbar", "OS26_Liquid_Glass_variant_ClearTaskbar", "OS26 Liquid Glass — Clear Taskbar"),
    ("OS26_Liquid_Glass_variant_DarkMacDock", "OS26_Liquid_Glass_variant_DarkMacDock", "OS26 Liquid Glass — Dark Mac Dock"),
    ("OS26_Liquid_Glass_variant_DarkMacDockCompact", "OS26_Liquid_Glass_variant_DarkMacDockCompact", "OS26 Liquid Glass — Dark Mac Dock Compact"),
    ("OS26_Liquid_Glass_variant_DarkTaskbar", "OS26_Liquid_Glass_variant_DarkTaskbar", "OS26 Liquid Glass — Dark Taskbar"),
    ("FrostyGlass", "FrostyGlass", "Frosty Glass"),
    ("FrostedAcrylic", "FrostedAcrylic", "Frosted Acrylic"),
    ("Pills", "Pills", "Pills"),
    ("Blob", "Blob", "Blob"),
    ("Minecraft_Hotbar", "Minecraft_Hotbar", "Minecraft Hotbar"),
    ("One_UI_8_5_variant_Dock", "One_UI_8_5_variant_Dock", "One UI 8.5 — Dock"),
    ("One_UI_8_5_variant_Taskbar", "One_UI_8_5_variant_Taskbar", "One UI 8.5 — Taskbar"),
]

source = open(SRC, encoding="utf-8").read()


def unescape(raw: str) -> str:
    out, i = [], 0
    while i < len(raw):
        c = raw[i]
        if c == "\\" and i + 1 < len(raw):
            n = raw[i + 1]
            out.append({"n": "\n", "t": "\t", "r": "\r", '"': '"', "\\": "\\"}.get(n, n))
            i += 2
        else:
            out.append(c)
            i += 1
    return "".join(out)


def tokens(text: str):
    """Yield ('str', value) / ('sym', ch) / ('ident', name) over a theme body."""
    i = 0
    while i < len(text):
        c = text[i]
        if c == 'L' and i + 1 < len(text) and text[i + 1] == '"':
            j = i + 2
            buf = []
            while j < len(text) and text[j] != '"':
                if text[j] == "\\":
                    buf.append(text[j:j + 2]); j += 2
                else:
                    buf.append(text[j]); j += 1
            yield ("str", unescape("".join(buf)))
            i = j + 1
        elif c in "{},":
            yield ("sym", c)
            i += 1
        elif c == ";":
            yield ("sym", ";")
            i += 1
        elif c.isalpha() or c == "_":
            j = i
            while j < len(text) and (text[j].isalnum() or text[j] in "_:"):
                j += 1
            yield ("ident", text[i:j])
            i = j
        else:
            i += 1


def parse_theme(body: str):
    tk = list(tokens(body))
    # Split the aggregate on top-level commas: each slot is one braced list.
    depth = 0
    slot, slots = [], []
    for kind, val in tk:
        if kind == "sym" and val == "{":
            depth += 1
            slot.append((kind, val))
        elif kind == "sym" and val == "}":
            depth -= 1
            slot.append((kind, val))
        elif kind == "sym" and val == "," and depth == 0:
            slots.append(slot)
            slot = []
        else:
            slot.append((kind, val))
    if slot:
        slots.append(slot)

    def list_of_strings(s):
        return [v for k, v in s if k == "str"]

    def list_of_targets(s):
        """ThemeTargetStyles{L"t", {L"a", L"b"}} -> [(t, [a,b])]"""
        s = [t for t in s if t != ("sym", ",")]
        out, i = [], 0
        while i < len(s):
            if s[i] == ("ident", "ThemeTargetStyles"):
                j = i + 1
                assert s[j] == ("sym", "{"), f"expected '{{' after ThemeTargetStyles, got {s[j]}"
                assert s[j + 1][0] == "str"
                target = s[j + 1][1]
                assert s[j + 2] == ("sym", "{"), f"expected styles '{{', got {s[j + 2]}"
                styles = []
                k = j + 3
                while s[k] != ("sym", "}"):
                    if s[k][0] == "str":
                        styles.append(s[k][1])
                    k += 1
                assert s[k + 1] == ("sym", "}"), f"expected target close, got {s[k + 1]}"
                out.append({"target": target, "styles": styles})
                i = k + 2
            else:
                i += 1
        return out

    # The Theme aggregate is exactly three braced lists; empty tail lists may
    # be written or omitted entirely, so pad to three slots.
    while len(slots) < 3:
        slots.append([])
    target_styles = list_of_targets(slots[0])
    constants = list_of_strings(slots[1])
    resource_vars = list_of_strings(slots[2])
    out = {"targetStyles": target_styles}
    if constants:
        out["styleConstants"] = constants
    if resource_vars:
        out["themeResourceVariables"] = resource_vars
    return out


# Extract each `const Theme g_theme<Name> = { ... };` block with brace counting
# that skips string literals (style strings carry `{`/`}` in {{expr}} markup).
themes = {}
for m in re.finditer(r"const Theme g_theme(\w+) = \{", source):
    name, start = m.group(1), m.start() + len(m.group(0)) - 1
    depth, i, instr = 0, start, False
    while i < len(source):
        c = source[i]
        if instr:
            if c == "\\":
                i += 2
                continue
            if c == '"':
                instr = False
        elif c == '"':
            instr = True
        elif c == "{":
            depth += 1
        elif c == "}":
            depth -= 1
            if depth == 0:
                break
        i += 1
    body = source[start + 1:i]
    themes[name] = parse_theme(body)

catalog = {"source": "windows-11-taskbar-styler", "modVersion": "1.4", "options": [], "themes": {}}
catalog["options"].append({"id": "", "label": "Windows Default"})
for sid, struct, label in SELECTABLE:
    if struct not in themes:
        raise SystemExit(f"missing struct g_theme{struct}")
    payload = themes[struct]
    if sid == "Squircle":
        # The mod substitutes the WeatherOnTheRight variant when OS feature
        # 48660958 (ExtendedModeAEPForTaskbar) is enabled.
        variant = "Squircle_variant_WeatherOnTheRight"
        if variant not in themes:
            raise SystemExit(f"missing struct g_theme{variant}")
        payload = dict(payload)
        payload["weatherVariant"] = themes[variant]
        payload["variantFeature"] = 48660958
    catalog["themes"][sid] = payload
    catalog["options"].append({"id": sid, "label": label})

with open(DST, "w", encoding="utf-8", newline="\n") as f:
    json.dump(catalog, f, ensure_ascii=False, separators=(",", ":"))
    f.write("\n")
total_targets = sum(len(t["targetStyles"]) for t in catalog["themes"].values())
print(f"{len(catalog['options'])} options, {len(catalog['themes'])} themes, {total_targets} target rules -> {DST}")
