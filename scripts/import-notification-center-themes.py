#!/usr/bin/env python3
"""Convert the Windhawk `windows-11-notification-center-styler` theme structs
into the PecoFence backend catalog
(crates/core/data/notification_center_themes.json).

The source is the mod's published C++ source: `const Theme g_theme<Name> = {{...},
{...}, {...}}` aggregates — three lists (targetStyles, styleConstants,
themeResourceVariables). The selectable `theme` setting strings come from the
mod's ProcessAllStylesFromSettings() chain; that mapping is reproduced in
SELECTABLE below (id = exact `Wh_GetStringSetting` value, struct =
g_theme<struct> suffix). Two ids literally contain spaces
("OS26 Tahoe Glass", "Command Center") — keep them verbatim.
"""
import json
import re
import sys

SRC = sys.argv[1] if len(sys.argv) > 1 else "windows-11-notification-center-styler.txt"
DST = sys.argv[2] if len(sys.argv) > 2 else "crates/core/data/notification_center_themes.json"

# setting id -> (struct suffix, display label)
SELECTABLE = [
    ("TranslucentShell", "TranslucentShell", "Translucent Shell"),
    ("Matter", "Matter", "Matter"),
    ("Unified", "Unified", "Unified"),
    ("10JumpLists", "10JumpLists", "10 JumpLists"),
    ("WindowGlass", "WindowGlass", "Window Glass"),
    ("WindowGlass_variant_alternative", "WindowGlass_variant_alternative", "Window Glass — alternative"),
    ("Oversimplified&Accentuated", "Oversimplified_Accentuated", "Oversimplified & Accentuated"),
    ("TintedGlass", "TintedGlass", "Tinted Glass"),
    ("Fluid", "Fluid", "Fluid"),
    ("LiquidGlass", "LiquidGlass", "Liquid Glass"),
    ("BetterControl11", "BetterControl11", "Better Control 11"),
    ("LayerMicaUI", "LayerMicaUI", "Layer Mica UI"),
    ("Borderless", "Borderless", "Borderless"),
    ("Densy", "Densy", "Densy"),
    ("FrostyGlass", "FrostyGlass", "Frosty Glass"),
    ("OS26 Tahoe Glass", "OS26_Tahoe_Glass", "OS26 Tahoe Glass"),
    ("Command Center", "Command_Center", "Command Center"),
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

catalog = {"source": "windows-11-notification-center-styler", "modVersion": "1.8", "options": [], "themes": {}}
catalog["options"].append({"id": "", "label": "Windows Default"})
for sid, struct, label in SELECTABLE:
    if struct not in themes:
        raise SystemExit(f"missing struct g_theme{struct}")
    catalog["themes"][sid] = themes[struct]
    catalog["options"].append({"id": sid, "label": label})

with open(DST, "w", encoding="utf-8", newline="\n") as f:
    json.dump(catalog, f, ensure_ascii=False, separators=(",", ":"))
    f.write("\n")
total_targets = sum(len(t["targetStyles"]) for t in catalog["themes"].values())
print(f"{len(catalog['options'])} options, {len(catalog['themes'])} themes, {total_targets} target rules -> {DST}")
