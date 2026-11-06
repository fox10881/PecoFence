"""Render the Store/tile PNGs for the MSIX package from packaging/assets/badge.png.

The CoreDesk Engine badge is the circular fox emblem; every asset is that artwork
composited (Lanczos) onto a transparent canvas at the size the Store expects. Wide
tiles and the splash screen keep the transparent ring so the manifest BackgroundColor
(the plate indigo) shows behind the badge.

    python scripts/make-msix-assets.py <output directory> [--preview <png>]
    python scripts/make-msix-assets.py --preview .cache/icon-sheet.png

`scripts/make-app-icon.py` reuses `render_icon` for the executable's .ico.
"""

import argparse
from pathlib import Path

from PIL import Image, ImageDraw

PLATE = (0x47, 0x68, 0xDE)
TILE_MARGIN = 0.18  # badge inset on tiles: badge = 64% of the side
ICON_MARGIN = 0.12  # taskbar / Explorer icons: the badge fills a bit more

# Logical name -> (base width, base height) at 100% scale.
TILES = {
    "Square44x44Logo": (44, 44),
    "Square150x150Logo": (150, 150),
    "SmallTile": (71, 71),
    "LargeTile": (310, 310),
    "Wide310x150Logo": (310, 150),
    "StoreLogo": (50, 50),
    "SplashScreen": (620, 300),
}
# Mark only, no plate; the manifest BackgroundColor fills the rest of the tile.
MARK_ONLY = {"Wide310x150Logo", "SplashScreen"}
SCALES = (100, 125, 150, 200, 400)
# Taskbar / Start list icons: the square 44 asset also ships in target sizes.
TARGET_SIZES = (16, 20, 24, 30, 32, 36, 40, 48, 60, 64, 72, 80, 96, 256)


BADGE_SRC = Path(__file__).resolve().parent.parent / "packaging" / "assets" / "badge.png"


def badge() -> Image.Image:
    # The CoreDesk Engine fox badge (circular, transparent margin) is the whole icon now.
    return Image.open(BADGE_SRC).convert("RGBA")


def render_icon(width: int, height: int, *, plate: bool = True, margin: float = TILE_MARGIN) -> Image.Image:
    """The fox badge centred on a transparent canvas at the given size.

    `plate`/`margin` are kept for the tile call sites; a badge tile is the badge at the
    fill the Store expects (the manifest BackgroundColor shows through the transparent ring).
    """
    b = badge()
    side = min(width, height)
    fill = side if plate else round(side * (1 - 2 * margin))
    img = Image.new("RGBA", (width, height), (0, 0, 0, 0))
    img.paste(b.resize((fill, fill), Image.LANCZOS), ((width - fill) // 2, (height - fill) // 2), b.resize((fill, fill), Image.LANCZOS))
    return img


def render_tile(name: str, width: int, height: int) -> Image.Image:
    return render_icon(width, height, plate=name not in MARK_ONLY)


def render_target(size: int) -> Image.Image:
    # The badge carries its own circular plate — plated and unplated forms are identical.
    return render_icon(size, size, margin=ICON_MARGIN)


def write_assets(out: Path) -> int:
    out.mkdir(parents=True, exist_ok=True)
    count = 0
    for name, (w, h) in TILES.items():
        for scale in SCALES:
            f = scale / 100
            render_tile(name, round(w * f), round(h * f)).save(out / f"{name}.scale-{scale}.png")
            count += 1
    for size in TARGET_SIZES:
        icon = render_target(size)
        icon.save(out / f"Square44x44Logo.targetsize-{size}.png")
        icon.save(out / f"Square44x44Logo.targetsize-{size}_altform-unplated.png")
        count += 2
    return count


def write_preview(path: Path) -> None:
    """Contact sheet: target-size icons on light and dark, tiles on the manifest background."""
    pad = 16
    light, dark = (0xF3, 0xF3, 0xF3, 255), (0x20, 0x20, 0x20, 255)
    rows: list[tuple[str, tuple, list[tuple[str, Image.Image]]]] = []
    icons = [(f"{n}", render_target(n)) for n in TARGET_SIZES]
    rows.append(("targetsize (plated + unplated) on light", light, icons))
    rows.append(("targetsize on dark", dark, icons))
    tiles = [(f"{name} {w}x{h}", render_tile(name, w, h)) for name, (w, h) in TILES.items() if name != "SplashScreen"]
    rows.append(("tiles at scale-100 on BackgroundColor", PLATE + (255,), tiles))
    rows.append(("SplashScreen 620x300 on BackgroundColor", PLATE + (255,), [("SplashScreen", render_tile("SplashScreen", 620, 300))]))
    rows.append(("Square150x150Logo scale-200 / LargeTile scale-100", (0xFF, 0xFF, 0xFF, 255), [("150 @200", render_tile("Square150x150Logo", 300, 300)), ("310", render_tile("LargeTile", 310, 310))]))

    label_h = 18
    measure = ImageDraw.Draw(Image.new("RGBA", (1, 1)))

    def cell_width(label, img):
        return max(img.width, int(measure.textlength(label)) + 4)

    row_widths = [sum(cell_width(l, img) + pad for l, img in items) + pad for _, _, items in rows]
    row_heights = [max(img.height for _, img in items) + label_h + 2 * pad for _, _, items in rows]
    sheet = Image.new("RGBA", (max(row_widths), sum(row_heights)), (0xFF, 0xFF, 0xFF, 255))
    y = 0
    for (title, bg, items), h in zip(rows, row_heights):
        band = Image.new("RGBA", (sheet.width, h), bg)
        d = ImageDraw.Draw(band)
        text = (0, 0, 0, 255) if sum(bg[:3]) > 384 else (255, 255, 255, 255)
        d.text((pad, 4), title, fill=text)
        x = pad
        for label, img in items:
            band.alpha_composite(img, (x, label_h + pad))
            d.text((x, h - pad + 2), label, fill=text)
            x += cell_width(label, img) + pad
        sheet.alpha_composite(band, (0, y))
        y += h
    path.parent.mkdir(parents=True, exist_ok=True)
    sheet.save(path)


def main() -> int:
    ap = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    ap.add_argument("out", nargs="?", type=Path, help="directory for the MSIX Assets PNGs")
    ap.add_argument("--preview", type=Path, metavar="PNG", help="also write a contact sheet of all sizes")
    args = ap.parse_args()
    if args.out is None and args.preview is None:
        ap.error("give an output directory and/or --preview")
    if args.out is not None:
        count = write_assets(args.out)
        print(f"wrote {count} PNGs to {args.out}")
    if args.preview is not None:
        write_preview(args.preview)
        print(f"wrote preview {args.preview}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
