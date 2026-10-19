import json, io, os

keys = {
    "Invisible Canvas · 隐形画布": {
        "en": "Invisible Canvas",
        "de": "Invisible Canvas",
        "es": "Lienzo invisible",
        "fr": "Canevas invisible",
        "ja": "インビジブルキャンバス",
        "ko": "투명 캔버스",
        "pt-BR": "Tela invisível",
        "ru": "Невидимый холст",
        "zh-TW": "隱形畫布",
    },
    "完全透明画布：栅栏背景完全隐藏，图标悬浮在壁纸上": {
        "en": "Fully transparent canvas: the fence background is hidden and icons float on the wallpaper",
        "de": "Vollständig transparente Fläche: Der Fence-Hintergrund ist ausgeblendet; Symbole schweben auf dem Hintergrundbild",
        "es": "Lienzo totalmente transparente: el fondo de la valla se oculta y los iconos flotan sobre el fondo de pantalla",
        "fr": "Canevas totalement transparent : l'arrière-plan de la barrière est masqué, les icônes flottent sur le fond d'écran",
        "ja": "完全に透明なキャンバス：フェンスの背景が隠れ、アイコンが壁紙上に浮かびます",
        "ko": "완전히 투명한 캔버스: 울타리 배경이 숨겨지고 아이콘이 배경 화면 위에 떠 있습니다",
        "pt-BR": "Tela totalmente transparente: o fundo da cerca fica oculto e os ícones flutuam sobre o papel de parede",
        "ru": "Полностью прозрачный холст: фон панели скрыт, значки парят на обоях",
        "zh-TW": "完全透明畫布：柵欄背景完全隱藏，圖示懸浮在桌布上",
    },
}

for f in sorted(os.listdir("locales")):
    if not f.endswith(".json"):
        continue
    lang = f[:-5]
    p = os.path.join("locales", f)
    d = json.load(io.open(p, encoding="utf-8"))
    for k, tr in keys.items():
        if k not in d:
            d[k] = tr.get(lang, k)
    with io.open(p, "w", encoding="utf-8", newline="\r\n") as fh:
        json.dump(d, fh, ensure_ascii=False, indent=4)
        fh.write("\n")
    print(p, "ok")
