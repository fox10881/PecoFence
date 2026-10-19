"""Adds the per-fence layout override strings (图标形状 / 自定义图标大小 / 水平间距 /
垂直间距) to every locale file, next to the existing 图标间距 entries."""
import json
import os

ROOT = os.path.join(os.path.dirname(__file__), "..", "locales")

# key -> per-locale translation (zh source strings are the keys; zh files carry none).
T = {
    "图标形状": {
        "en": "Icon shape",
        "de": "Symbolform",
        "es": "Forma de icono",
        "fr": "Forme d'icône",
        "ja": "アイコンの形",
        "ko": "아이콘 모양",
        "pt-BR": "Formato do ícone",
        "ru": "Форма значка",
        "zh-TW": "圖示形狀",
    },
    "海报形状把图标裁成 2:3 封面，铺满无空隙，适合游戏封面": {
        "en": "Poster crops icons into flush 2:3 covers — great for game art",
        "de": "Poster schneidet Symbole als lückenlose 2:3-Cover zu — ideal für Spiel-Cover",
        "es": "Póster recorta los iconos en portadas 2:3 sin huecos — ideal para juegos",
        "fr": "Poster recadre les icônes en jaquettes 2:3 jointives — idéal pour les jeux",
        "ja": "ポスターはアイコンを隙間なく 2:3 のカバーに切り抜きます — ゲームカバー向け",
        "ko": "포스터는 아이콘을 빈틈없는 2:3 커버로 잘라냅니다 — 게임 커버에 적합",
        "pt-BR": "Pôster recorta os ícones em capas 2:3 justapostas — ideal para jogos",
        "ru": "«Постер» обрезает значки в обложки 2:3 без зазоров — удобно для игр",
        "zh-TW": "海報形狀把圖示裁成 2:3 封面，鋪滿無空隙，適合遊戲封面",
    },
    "海报": {
        "en": "Poster",
        "de": "Poster",
        "es": "Póster",
        "fr": "Poster",
        "ja": "ポスター",
        "ko": "포스터",
        "pt-BR": "Pôster",
        "ru": "Постер",
        "zh-TW": "海報",
    },
    "自定义图标大小": {
        "en": "Custom icon size",
        "de": "Benutzerdefinierte Symbolgröße",
        "es": "Tamaño de icono personalizado",
        "fr": "Taille d'icône personnalisée",
        "ja": "アイコンサイズのカスタム",
        "ko": "사용자 지정 아이콘 크기",
        "pt-BR": "Tamanho de ícone personalizado",
        "ru": "Свой размер значков",
        "zh-TW": "自訂圖示大小",
    },
    "覆盖“图标大小”预设，可在 16–256 间自由调整": {
        "en": "Overrides the icon size preset; freely adjustable from 16 to 256",
        "de": "Überschreibt die Symbolgrößen-Voreinstellung; frei wählbar von 16 bis 256",
        "es": "Anula el tamaño de icono predefinido; ajustable entre 16 y 256",
        "fr": "Remplace la taille d'icône prédéfinie ; réglable de 16 à 256",
        "ja": "アイコンサイズのプリセットを上書きし、16～256 で自由に調整",
        "ko": "아이콘 크기 프리셋을 재정의하고 16–256 사이에서 자유롭게 조정",
        "pt-BR": "Substitui o tamanho de ícone predefinido; ajustável de 16 a 256",
        "ru": "Заменяет предустановленный размер значков; от 16 до 256",
        "zh-TW": "覆蓋「圖示大小」預設，可在 16–256 間自由調整",
    },
    "水平间距": {
        "en": "Column spacing",
        "de": "Spaltenabstand",
        "es": "Espacio entre columnas",
        "fr": "Espacement des colonnes",
        "ja": "列の間隔",
        "ko": "열 간격",
        "pt-BR": "Espaçamento entre colunas",
        "ru": "Интервал между столбцами",
        "zh-TW": "水平間距",
    },
    "图标列之间的间距；拖到 0 让图标完全贴合，关闭则跟随“图标间距”": {
        "en": "Gap between icon columns; 0 packs them flush, off follows Icon spacing",
        "de": "Abstand zwischen Symbolspalten; 0 = bündig, aus = folgt „Symbolabstand“",
        "es": "Espacio entre columnas de iconos; 0 los une sin hueco, apagado sigue a Espacio",
        "fr": "Espace entre les colonnes ; 0 les accole, désactivé suit Espacement",
        "ja": "アイコン列の間隔。0 で隙間なし、オフで「アイコンの間隔」に従う",
        "ko": "아이콘 열 사이 간격. 0이면 빈틈없이 붙고, 끄면 아이콘 간격을 따름",
        "pt-BR": "Espaço entre colunas de ícones; 0 os encosta, desligado segue Espaçamento",
        "ru": "Интервал между столбцами значков; 0 — вплотную, выкл. — как «Интервал»",
        "zh-TW": "圖示列之間的間距；拖到 0 讓圖示完全貼合，關閉則跟隨「圖示間距」",
    },
    "垂直间距": {
        "en": "Row spacing",
        "de": "Zeilenabstand",
        "es": "Espacio entre filas",
        "fr": "Espacement des lignes",
        "ja": "行の間隔",
        "ko": "행 간격",
        "pt-BR": "Espaçamento entre linhas",
        "ru": "Интервал между строками",
        "zh-TW": "垂直間距",
    },
    "图标行之间的间距；拖到 0 让图标完全贴合，关闭则跟随“图标间距”": {
        "en": "Gap between icon rows; 0 packs them flush, off follows Icon spacing",
        "de": "Abstand zwischen Symbolzeilen; 0 = bündig, aus = folgt „Symbolabstand“",
        "es": "Espacio entre filas de iconos; 0 las une sin hueco, apagado sigue a Espacio",
        "fr": "Espace entre les lignes ; 0 les accole, désactivé suit Espacement",
        "ja": "アイコン行の間隔。0 で隙間なし、オフで「アイコンの間隔」に従う",
        "ko": "아이콘 행 사이 간격. 0이면 빈틈없이 붙고, 끄면 아이콘 간격을 따름",
        "pt-BR": "Espaço entre linhas de ícones; 0 as encosta, desligado segue Espaçamento",
        "ru": "Интервал между строками значков; 0 — вплотную, выкл. — как «Интервал»",
        "zh-TW": "圖示行之間的間距；拖到 0 讓圖示完全貼合，關閉則跟隨「圖示間距」",
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
