"""Adds the 锁定滚动 toggle strings to every locale file."""
import json
import os

ROOT = os.path.join(os.path.dirname(__file__), "..", "locales")

# key -> per-locale translation (zh source strings are the keys; zh files carry none).
T = {
    "锁定滚动": {
        "en": "Lock scrolling",
        "de": "Scrollen sperren",
        "es": "Bloquear desplazamiento",
        "fr": "Verrouiller le défilement",
        "ja": "スクロールをロック",
        "ko": "스크롤 잠금",
        "pt-BR": "Bloquear rolagem",
        "ru": "Заблокировать прокрутку",
        "zh-TW": "鎖定捲動",
    },
    "完全禁止滚动；滚轮、拖动和自动滚动都不起作用，内容固定在顶部，适合静态橱窗布局": {
        "en": "Disables all scrolling; the wheel, dragging and auto-scroll do nothing and content stays pinned to the top — for static display layouts",
        "de": "Deaktiviert das Scrollen vollständig; Mausrad, Ziehen und Auto-Scrollen bleiben wirkungslos, der Inhalt bleibt oben fixiert — für statische Schaufenster-Layouts",
        "es": "Desactiva todo el desplazamiento; la rueda, el arrastre y el desplazamiento automático no hacen nada y el contenido queda fijo arriba — para diseños estáticos",
        "fr": "Désactive tout défilement ; la molette, le glisser et le défilement automatique sont sans effet et le contenu reste épinglé en haut — pour les mises en page statiques",
        "ja": "すべてのスクロールを無効にします。ホイール、ドラッグ、自動スクロールは動作せず、コンテンツは上端に固定されます — 静的なレイアウト用",
        "ko": "모든 스크롤을 비활성화합니다. 휠, 드래그, 자동 스크롤이 작동하지 않으며 콘텐츠가 맨 위에 고정됩니다 — 정적 레이아웃용",
        "pt-BR": "Desativa toda a rolagem; a roda, o arrastar e a rolagem automática não fazem nada e o conteúdo fica fixado no topo — para layouts estáticos",
        "ru": "Полностью отключает прокрутку: колёсико, перетаскивание и автопрокрутка не действуют, содержимое закреплено сверху — для статичных макетов",
        "zh-TW": "完全禁止捲動；滾輪、拖曳和自動捲動都不起作用，內容固定在頂部，適合靜態櫥窗佈局",
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
