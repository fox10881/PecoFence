"""Insert the snapshot-wallpaper locale keys after the existing snapshot-name key.

Adds, to every locales/*.json catalog:
  "包含当前壁纸"  (include current wallpaper, the save-checkbox label)
  "已随壁纸保存"  (saved with wallpaper, the list indicator)

Keys are the zh source strings, matching catalog convention; translations are
per-locale. Plain text insertion keeps each file's indentation and order.
"""
import json
import os
import sys

LOCALES = os.path.join(os.path.dirname(__file__), "..", "locales")

ANCHOR_KEY = "快照名称（可选）"
KEY_INCLUDE = "包含当前壁纸"
KEY_SAVED = "已随壁纸保存"

TRANSLATIONS = {
    "en.json": ("Include current wallpaper", "Saved with wallpaper"),
    "ar.json": ("تضمين الخلفية الحالية", "تم الحفظ مع الخلفية"),
    "de.json": ("Aktuelles Hintergrundbild einschließen", "Mit Hintergrundbild gespeichert"),
    "es.json": ("Incluir el fondo actual", "Guardado con el fondo"),
    "fr.json": ("Inclure le fond d'écran actuel", "Enregistré avec le fond d'écran"),
    "ja.json": ("現在の壁紙を含める", "壁紙と共に保存済み"),
    "ko.json": ("현재 배경 화면 포함", "배경 화면과 함께 저장됨"),
    "pt-BR.json": ("Incluir o papel de parede atual", "Salvo com o papel de parede"),
    "ru.json": ("Включить текущие обои", "Сохранено с обоями"),
    "zh-TW.json": ("包含目前桌布", "已隨桌布儲存"),
}

for fname, (inc, saved) in TRANSLATIONS.items():
    path = os.path.join(LOCALES, fname)
    with open(path, encoding="utf-8") as f:
        text = f.read()
    if KEY_INCLUDE in text:
        print(f"{fname}: already present, skipping")
        continue
    anchor = None
    for line in text.splitlines(keepends=True):
        if f'"{ANCHOR_KEY}"' in line:
            anchor = line
            break
    if anchor is None:
        print(f"{fname}: ANCHOR NOT FOUND", file=sys.stderr)
        sys.exit(1)
    indent = anchor[: len(anchor) - len(anchor.lstrip())]
    newline = "\r\n" if "\r\n" in anchor else "\n"
    insert = (
        f'{indent}"{KEY_INCLUDE}": {json.dumps(inc, ensure_ascii=False)},{newline}'
        f'{indent}"{KEY_SAVED}": {json.dumps(saved, ensure_ascii=False)},{newline}'
    )
    text = text.replace(anchor, anchor + insert, 1)
    json.loads(text)  # validate before writing
    with open(path, "w", encoding="utf-8", newline="") as f:
        f.write(text)
    n = len(json.loads(text))
    print(f"{fname}: +2 keys -> {n}")
