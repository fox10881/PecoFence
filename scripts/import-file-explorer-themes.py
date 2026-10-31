#!/usr/bin/env python3
"""Convert the Windhawk `windows-11-file-explorer-styler` theme structs into
`crates/core/data/file_explorer_themes.json`.

The .cpp theme blocks are regular enough to walk with a brace-matcher:

    const Theme g_themeName = {{
        ThemeTargetStyles{L"target", { L"style", ... }},
        ...
    }, { L"constant", ... }, { L"resvar", ... }, <height>, BackgroundTranslucentEffect::kX};

Labels come from the mod's `theme` options table (`Theme` dropdown): the id is
the `g_theme` suffix, the label the option's display text.
"""

import json
import re
import sys

SRC = sys.argv[1] if len(sys.argv) > 1 else "festyler.cpp"
OUT = sys.argv[2] if len(sys.argv) > 2 else "../crates/core/data/file_explorer_themes.json"

src = open(SRC, encoding="utf-8", errors="replace").read()


def unescape(s: str) -> str:
    return (
        s.replace('\\"', '"')
        .replace("\\\\", "\\")
        .replace("\\'", "'")
        .replace("\\n", "\n")
        .replace("\\t", "\t")
    )


def wstrings(block: str):
    """All L"..." literals in a block, in order."""
    return [unescape(m.group(1)) for m in re.finditer(r'L"((?:[^"\\]|\\.)*)"', block)]


def top_level_commas(block: str):
    """Split at commas with brace depth 0 (inside the outer `{{ ... }}` list)."""
    parts, depth, cur = [], 0, []
    i = 0
    while i < len(block):
        c = block[i]
        if c == "{":
            depth += 1
        elif c == "}":
            depth -= 1
        if c == "," and depth == 0:
            parts.append("".join(cur))
            cur = []
        else:
            cur.append(c)
        i += 1
    if "".join(cur).strip():
        parts.append("".join(cur))
    return parts


def parse_target_styles(block: str):
    """`ThemeTargetStyles{L"target", { L"s", ... }}` entries."""
    # The field still carries its own vector braces — strip the wrap.
    block = block.strip()
    if block.startswith("{") and block.rstrip().endswith("}"):
        block = block[1:block.rfind("}")]
    out = []
    for part in top_level_commas(block):
        m = re.match(r'\s*ThemeTargetStyles\s*\{\s*L"((?:[^"\\]|\\.)*)"', part)
        if not m:
            continue
        target = unescape(m.group(1))
        # The styles list is the inner {...} after the target.
        rest = part[m.end():]
        lb, rb = rest.find("{"), rest.rfind("}")
        styles = wstrings(rest[lb:rb + 1]) if lb != -1 and rb > lb else []
        out.append({"target": target, "styles": styles})
    return out


themes = {}
for m in re.finditer(r"const Theme g_theme(\w+)\s*=\s*\{", src):
    tid = m.group(1)
    # Brace-match the whole initializer from the first '{'.
    start = m.end() - 1
    depth, end = 0, start
    while end < len(src):
        if src[end] == "{":
            depth += 1
        elif src[end] == "}":
            depth -= 1
            if depth == 0:
                break
        end += 1
    body = src[start + 1:end]
    # Top level of `{{...}, {...}, {...}, height, Effect::kX}` — first three
    # items are the vectors, then optional trailing fields.
    fields = top_level_commas(body)
    target_styles = parse_target_styles(fields[0]) if fields else []
    constants = wstrings(fields[1]) if len(fields) > 1 else []
    resvars = wstrings(fields[2]) if len(fields) > 2 else []
    height = 0
    effect = "kDefault"
    for tail in fields[3:]:
        t = tail.strip()
        hm = re.match(r"^(\d+)$", t)
        em = re.search(r"BackgroundTranslucentEffect::(\w+)", t)
        if em:
            effect = em.group(1)
        elif hm:
            height = int(hm.group(1))
    themes[tid] = {
        "targetStyles": target_styles,
        "styleConstants": constants,
        "themeResourceVariables": resvars,
        "explorerFrameContainerHeight": height,
        "backgroundTranslucentEffect": effect,
    }

# Option labels: themes are selectable by their g_theme suffix. Humanize the
# id for the dropdown (the guide's own titles where they differ).
labels = {
    "Translucent_Explorer11": "Translucent Explorer11",
    "WindowGlass": "WindowGlass",
    "MicaBar": "MicaBar",
    "MicaTabless": "MicaTabless",
    "LiquidGlass": "LiquidGlass",
    "Compact_Explorer11": "Compact Explorer11",
    "Minimal_Explorer11": "Minimal Explorer11",
    "NoCommandBar": "NoCommandBar",
    "Tabless": "Tabless",
    "TintedGlass": "TintedGlass",
    "Float": "Float",
    "Matter": "Matter",
    "ZEUSosX_044": "ZEUSosX_044",
    "SimplyTransparent": "SimplyTransparent",
    "explorer32": "explorer32",
    "RosePine": "Rose Pine",
    "UniMenu": "UniMenu",
    "Native": "Native",
    "Fluentify": "Fluentify",
    "Bubbles": "Bubbles",
}
options = [{"id": "", "label": "Windows Default"}]
for tid in themes:
    options.append({"id": tid, "label": labels.get(tid, tid.replace("_", " "))})

json.dump({"options": options, "themes": themes}, open(OUT, "w"), indent=1, ensure_ascii=False)
print(f"{len(themes)} themes: {', '.join(themes)}")
