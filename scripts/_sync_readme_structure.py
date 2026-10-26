"""Mirror the README.md structure fix into every translated README:
add the missing H1 title and swap the local poster-shelves image for the
same dangling alt + two user-attachments <img> lines the source uses."""
import re
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
README_DIR = ROOT / "docs/readme"

IMG_BLOCK = """<img width="3837" height="2160" alt="image" src="https://github.com/user-attachments/assets/1849b142-aa9e-4b5d-802e-4f3519d79c73" />

<img width="3837" height="2034" alt="image" src="https://github.com/user-attachments/assets/311962e3-984b-4a7a-bc2c-48ffdaa9379b" />"""

for path in sorted(README_DIR.glob("README.*.md")):
    text = path.read_text(encoding="utf-8")
    if text.startswith("# "):
        print(f"{path.name}: already has H1, skipping")
        continue
    # H1 mirrors README.md's literal title line
    if not text.startswith("# Wallpaper Engine"):
        text = "# Wallpaper Engine 自用版\n\n" + text
    # Mirror source: dangling ![alt] + two <img> user-attachments shots
    m = re.search(r"^(!\[[^\]]*\])\((\.\./assets/desktop-poster-shelves\.png)\)[ \t]*$", text, re.M)
    if m:
        text = text[: m.start()] + f"{m.group(1)}\n{IMG_BLOCK}" + text[m.end():]
        print(f"{path.name}: synced")
    else:
        print(f"{path.name}: poster-shelves image not found — CHECK MANUALLY")
    path.write_text(text, encoding="utf-8")
