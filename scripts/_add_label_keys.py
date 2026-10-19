"""Insert the 显示图标标签 toggle strings into every locale file (en already has them)."""

T = {
    "de": (
        "Symbolbeschriftungen anzeigen",
        "Blendet die Namen unter den Symbolen aus; Cover liegen dann auf den Regalbrettern auf",
    ),
    "es": (
        "Mostrar etiquetas de icono",
        "Oculta los nombres bajo los iconos; las carátulas quedan apoyadas en las baldas",
    ),
    "fr": (
        "Afficher les étiquettes d'icônes",
        "Masque les noms sous les icônes ; les jaquettes reposent sur les planches",
    ),
    "ja": (
        "アイコンのラベルを表示",
        "オフにするとアイコン下の名前を表示しません。本棚テーマではカバーが棚板にぴったり乗ります",
    ),
    "ko": (
        "아이콘 레이블 표시",
        "끄면 아이콘 아래 이름을 숨깁니다. 책장 테마에서 커버가 선반에 딱 맞게 놓입니다",
    ),
    "pt-BR": (
        "Mostrar rótulos dos ícones",
        "Oculta os nomes sob os ícones; as capas ficam apoiadas nas tábuas da estante",
    ),
    "ru": (
        "Показывать подписи значков",
        "Скрывает имена под значками; обложки сидят на досках книжной полки",
    ),
    "zh-TW": (
        "顯示圖示標籤",
        "關閉後圖示下方不顯示名稱，書架主題下封面正好坐在木板上",
    ),
}

ANCHOR = '"栅栏标题和标签页文字的大小"'
K1, K2 = "显示图标标签", "关闭后图标下方不显示名称，书架主题下封面正好坐在木板上"

for loc, (v1, v2) in T.items():
    path = f"locales/{loc}.json"
    text = open(path, encoding="utf-8").read()
    if K1 in text:
        print(loc, "already has key, skipped")
        continue
    i = text.find(ANCHOR)
    if i < 0:
        print(loc, "ANCHOR NOT FOUND")
        continue
    end = text.find("\n", i)
    add = f'  "{K1}": "{v1}",\n  "{K2}": "{v2}",\n'
    text = text[:end + 1] + add + text[end + 1:]
    open(path, "w", encoding="utf-8").write(text)
    print(loc, "done")
