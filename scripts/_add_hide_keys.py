"""Adds the「从不显示」title option and 隐藏滚动条 toggle strings to every locale file."""
import json
import os

ROOT = os.path.join(os.path.dirname(__file__), "..", "locales")

# key -> per-locale translation (zh source strings are the keys; zh files carry none).
T = {
    "从不显示": {
        "en": "Never",
        "de": "Nie",
        "es": "Nunca",
        "fr": "Jamais",
        "ja": "表示しない",
        "ko": "표시 안 함",
        "pt-BR": "Nunca",
        "ru": "Никогда",
        "zh-TW": "永不顯示",
    },
    "隐藏滚动条": {
        "en": "Hide scrollbar",
        "de": "Scrollleiste ausblenden",
        "es": "Ocultar barra de desplazamiento",
        "fr": "Masquer la barre de défilement",
        "ja": "スクロールバーを非表示",
        "ko": "스크롤 바 숨기기",
        "pt-BR": "Ocultar barra de rolagem",
        "ru": "Скрыть полосу прокрутки",
        "zh-TW": "隱藏捲軸",
    },
    "完全不绘制滚动条；鼠标滚轮仍可滚动": {
        "en": "The scrollbar is never drawn; the mouse wheel still scrolls",
        "de": "Die Scrollleiste wird nie gezeichnet; das Mausrad scrollt weiter",
        "es": "La barra nunca se dibuja; la rueda del ratón sigue desplazando",
        "fr": "La barre n'est jamais dessinée ; la molette défile toujours",
        "ja": "スクロールバーは描画されません。マウスホイールは引き続き使えます",
        "ko": "스크롤 바를 그리지 않습니다. 마우스 휠 스크롤은 계속 작동합니다",
        "pt-BR": "A barra nunca é desenhada; a roda do mouse continua rolando",
        "ru": "Полоса прокрутки не рисуется; колёсико мыши по-прежнему прокручивает",
        "zh-TW": "完全不繪製捲軸；滑鼠滾輪仍可捲動",
    },
}

for name in sorted(os.listdir(ROOT)):
    if not name.endswith(".json"):
        continue
    locale = name[:-5]
    path = os.path.join(ROOT, name)
    with open(path, encoding="utf-8") as fh:
        data = json.load(fh)
    for k, tr in T.items():
        if k not in data:
            data[k] = tr.get(locale, tr["en"])
    with open(path, "w", encoding="utf-8", newline="\r\n") as fh:
        json.dump(data, fh, ensure_ascii=False, indent=4)
        fh.write("\n")
    print(f"{name} ok")
