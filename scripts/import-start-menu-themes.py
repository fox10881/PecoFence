#!/usr/bin/env python3
"""Convert the Windhawk `windows-11-start-menu-styler` theme structs into the
PecoFence backend catalog (crates/core/data/start_menu_themes.json).

The source is the mod's published C++ source: `const Theme g_theme<Name> = {{...},
{...}, {...}, {...}, DisableNewStartMenuLayout::X};` aggregates. The selectable
`theme` setting strings come from the mod's GetSelectedTheme() switch; that
mapping is reproduced in SELECTABLE below (id = exact `Wh_GetStringSetting`
value, struct = g_theme<struct> suffix, classic = optional _variant_ struct the
mod substitutes on the classic start-menu layout).
"""
import json
import re
import sys

SRC = sys.argv[1] if len(sys.argv) > 1 else "windows-11-start-menu-styler.txt"
DST = sys.argv[2] if len(sys.argv) > 2 else "crates/core/data/start_menu_themes.json"

# setting id -> (struct suffix, classic variant suffix or None, display label)
SELECTABLE = [
    ("TranslucentStartMenu", "TranslucentStartMenu", "TranslucentStartMenu_variant_ClassicStartMenu", "Translucent Start Menu"),
    ("NoRecommendedSection", "NoRecommendedSection", "NoRecommendedSection_variant_ClassicStartMenu", "No Recommended Section"),
    ("SideBySide", "SideBySide", "SideBySide_variant_ClassicStartMenu", "Side by Side"),
    ("SideBySide2", "SideBySide2", "SideBySide2_variant_ClassicStartMenu", "Side by Side 2"),
    ("SideBySideMinimal", "SideBySideMinimal", "SideBySideMinimal_variant_ClassicStartMenu", "Side by Side Minimal"),
    ("Down Aero", "Down_Aero", "Down_Aero_variant_ClassicStartMenu", "Down Aero"),
    ("Windows10", "Windows10", "Windows10_variant_ClassicStartMenu", "Windows 10"),
    ("Windows10_variant_Minimal", "Windows10_variant_Minimal", "Windows10_variant_Minimal_ClassicStartMenu", "Windows 10 — Minimal"),
    ("Windows11_Metro10", "Windows11_Metro10", "Windows11_Metro10_variant_ClassicStartMenu", "Windows 11 — Metro 10"),
    ("Fluent2Inspired", "Fluent2Inspired", "Fluent2Inspired_variant_ClassicStartMenu", "Fluent 2 Inspired"),
    ("RosePine", "RosePine", "RosePine_variant_ClassicStartMenu", "Rosé Pine"),
    ("Windows11_Metro10Minimal", "Windows11_Metro10Minimal", "Windows11_Metro10Minimal_variant_ClassicStartMenu", "Metro 10 Minimal"),
    ("Everblush", "Everblush", "Everblush_variant_ClassicStartMenu", "Everblush"),
    ("SunValley", "SunValley", None, "Sun Valley"),
    ("21996", "21996", "21996_variant_ClassicStartMenu", "Build 21996"),
    ("UniMenu", "UniMenu", "UniMenu_variant_ClassicStartMenu", "UniMenu"),
    ("LegacyFluent", "LegacyFluent", "LegacyFluent_variant_ClassicStartMenu", "Legacy Fluent"),
    ("OnlySearch", "OnlySearch", "OnlySearch_variant_ClassicStartMenu", "Only Search"),
    ("OnlySearch_variant_Minimal", "OnlySearch_variant_Minimal", None, "Only Search — Minimal"),
    ("WindowGlass", "WindowGlass", None, "Window Glass"),
    ("WindowGlass_variant_Minimal", "WindowGlass", None, "Window Glass — Minimal"),
    ("Fluid", "Fluid", None, "Fluid"),
    ("Oversimplified&Accentuated", "Oversimplified_Accentuated", None, "Oversimplified & Accentuated"),
    ("LiquidGlass2", "LiquidGlass2", None, "Liquid Glass 2"),
    ("LiquidGlass", "LiquidGlass", None, "Liquid Glass"),
    ("Windows10X", "Windows10X", "Windows10X_variant_ClassicStartMenu", "Windows 10X"),
    ("TintedGlass", "TintedGlass", "TintedGlass_variant_ClassicStartMenu", "Tinted Glass"),
    ("LayerMicaUI", "LayerMicaUI", None, "Layer Mica UI"),
    ("Borderless", "Borderless", None, "Borderless"),
    ("Command Center", "Command_Center", None, "Command Center"),
    ("FullScreen", "FullScreen", "FullScreen_variant_1", "Full Screen"),
    ("FrostyGlass", "FrostyGlass", None, "Frosty Glass"),
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
    # Split the aggregate on top-level commas: each slot is a braced list or
    # the trailing DisableNewStartMenuLayout enum.
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
        # Commas are structurally irrelevant inside the list - drop them so the
        # positional parse below sees a flat stream.
        s = [t for t in s if t != ("sym", ",")]
        out, i = [], 0
        while i < len(s):
            if s[i] == ("ident", "ThemeTargetStyles"):
                # expect { str { strings } }
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
                # k at styles close; next should be target close
                assert s[k + 1] == ("sym", "}"), f"expected target close, got {s[k + 1]}"
                out.append({"target": target, "styles": styles})
                i = k + 2
            else:
                i += 1
        return out

    # The optional DisableNewStartMenuLayout enum may sit in any trailing slot
    # when empty middle lists are written or omitted - find it first, then map
    # the remaining braced slots positionally onto the four vector fields.
    layout = "windowsDefault"
    list_slots = []
    for s in slots:
        enum = next((v for k, v in s if k == "ident" and v.startswith("DisableNewStartMenuLayout::")), None)
        if enum is not None:
            layout = enum.split("::", 1)[1]
        else:
            list_slots.append(s)
    target_styles = list_of_targets(list_slots[0]) if len(list_slots) > 0 else []
    constants = list_of_strings(list_slots[1]) if len(list_slots) > 1 else []
    resource_vars = list_of_strings(list_slots[2]) if len(list_slots) > 2 else []
    web_targets = list_of_targets(list_slots[3]) if len(list_slots) > 3 else []
    out = {"targetStyles": target_styles}
    if constants:
        out["styleConstants"] = constants
    if resource_vars:
        out["themeResourceVariables"] = resource_vars
    if web_targets:
        out["webViewTargetStyles"] = web_targets
    if layout != "windowsDefault":
        out["startMenuLayout"] = layout
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
    body = source[start + 1:i]  # inside the outer braces
    # The outer braces wrap the aggregate's elements directly.
    themes[name] = parse_theme(body)

catalog = {"source": "windows-11-start-menu-styler", "modVersion": "1.7", "options": [], "themes": {}}
catalog["options"].append({"id": "", "label": "Windows Default"})
for sid, struct, classic, label in SELECTABLE:
    if struct not in themes:
        raise SystemExit(f"missing struct g_theme{struct}")
    opt = {"id": sid, "label": label}
    payload = themes[struct]
    if classic:
        if classic not in themes:
            raise SystemExit(f"missing classic variant g_theme{classic}")
        payload = dict(payload)
        payload["classicVariant"] = themes[classic]
    catalog["themes"][sid] = payload
    catalog["options"].append(opt)

with open(DST, "w", encoding="utf-8", newline="\n") as f:
    json.dump(catalog, f, ensure_ascii=False, separators=(",", ":"))
    f.write("\n")
total_targets = sum(len(t["targetStyles"]) for t in catalog["themes"].values())
print(f"{len(catalog['options'])} options, {len(catalog['themes'])} themes, {total_targets} target rules -> {DST}")
