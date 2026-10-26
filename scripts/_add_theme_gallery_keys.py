"""Adds the saved-themes gallery (.peco card grid) strings to every locale file."""
import json
import os

ROOT = os.path.join(os.path.dirname(__file__), "..", "locales")

T = {
    "已保存的主题": {
        "en": "Saved Themes",
        "ar": "الثيمات المحفوظة",
        "de": "Gespeicherte Designs",
        "es": "Temas guardados",
        "fr": "Thèmes enregistrés",
        "it": "Temi salvati",
        "ja": "保存したテーマ",
        "ko": "저장된 테마",
        "pt-BR": "Temas salvos",
        "ru": "Сохранённые темы",
        "zh-TW": "已儲存的主題",
    },
    "选择一个 .peco 主题文件并应用到桌面": {
        "en": "Choose a .peco theme file to apply to the desktop",
        "ar": "اختر ملف ثيم .peco لتطبيقه على سطح المكتب",
        "de": "Wählen Sie eine .peco-Designdatei aus, um sie auf den Desktop anzuwenden",
        "es": "Elige un archivo de tema .peco y aplícalo al escritorio",
        "fr": "Choisissez un fichier de thème .peco à appliquer au bureau",
        "it": "Scegli un file tema .peco da applicare al desktop",
        "ja": ".peco テーマファイルを選んでデスクトップに適用",
        "ko": ".peco 테마 파일을 선택해 바탕 화면에 적용",
        "pt-BR": "Escolha um arquivo de tema .peco para aplicar à área de trabalho",
        "ru": "Выберите файл темы .peco и примените его к рабочему столу",
        "zh-TW": "選擇一個 .peco 主題檔案並套用到桌面",
    },
    "尚无已保存的主题": {
        "en": "No saved themes yet",
        "ar": "لا توجد ثيمات محفوظة بعد",
        "de": "Noch keine gespeicherten Designs",
        "es": "Aún no hay temas guardados",
        "fr": "Aucun thème enregistré pour l'instant",
        "it": "Nessun tema salvato",
        "ja": "保存したテーマはまだありません",
        "ko": "아직 저장된 테마가 없습니다",
        "pt-BR": "Ainda não há temas salvos",
        "ru": "Пока нет сохранённых тем",
        "zh-TW": "尚無已儲存的主題",
    },
    "应用主题：": {
        "en": "Apply theme: ",
        "ar": "تطبيق الثيم: ",
        "de": "Design anwenden: ",
        "es": "Aplicar tema: ",
        "fr": "Appliquer le thème : ",
        "it": "Applica il tema: ",
        "ja": "テーマを適用：",
        "ko": "테마 적용: ",
        "pt-BR": "Aplicar tema: ",
        "ru": "Применить тему: ",
        "zh-TW": "套用主題：",
    },
    "删除主题：": {
        "en": "Delete theme: ",
        "ar": "حذف الثيم: ",
        "de": "Design löschen: ",
        "es": "Eliminar tema: ",
        "fr": "Supprimer le thème : ",
        "it": "Elimina il tema: ",
        "ja": "テーマを削除：",
        "ko": "테마 삭제: ",
        "pt-BR": "Excluir tema: ",
        "ru": "Удалить тему: ",
        "zh-TW": "刪除主題：",
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
        data[k] = tr.get(locale, tr["en"])
    with open(path, "w", encoding="utf-8", newline="\n") as fh:
        json.dump(data, fh, ensure_ascii=False, indent=4)
        fh.write("\n")
    print(f"{name} ok")
