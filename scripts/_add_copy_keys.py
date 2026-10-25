"""Adds the「从其他栅栏复制设置」row strings to every locale file."""
import json
import os

ROOT = os.path.join(os.path.dirname(__file__), "..", "locales")

# key -> per-locale translation (zh source strings are the keys; zh files carry none).
T = {
    "从其他栅栏复制设置": {
        "en": "Copy settings from another fence",
        "de": "Einstellungen aus einem anderen Bereich kopieren",
        "es": "Copiar la configuración de otro grupo",
        "fr": "Copier les paramètres d'un autre groupe",
        "it": "Copia le impostazioni da un altro riquadro",
        "ja": "他のフェンスから設定をコピー",
        "ko": "다른 펜스에서 설정 복사",
        "pt-BR": "Copiar configurações de outro grupo",
        "ru": "Копировать настройки из другой области",
        "ar": "نسخ الإعدادات من سياج آخر",
        "zh-TW": "從其他圍欄複製設定",
    },
    "复制外观和布局设置；名称、内容和排序规则保持不变": {
        "en": "Copies appearance and layout settings; the name, contents and sorting rules stay unchanged",
        "de": "Kopiert Darstellung und Layout; Name, Inhalt und Sortierregeln bleiben unverändert",
        "es": "Copia la apariencia y el diseño; el nombre, el contenido y las reglas de ordenación no cambian",
        "fr": "Copie l'apparence et la disposition ; le nom, le contenu et les règles de tri restent inchangés",
        "it": "Copia aspetto e layout; nome, contenuto e regole di ordinamento restano invariati",
        "ja": "外観とレイアウトの設定をコピーします。名前、内容、並べ替えルールはそのままです",
        "ko": "모양과 레이아웃 설정을 복사합니다. 이름, 내용, 정렬 규칙은 그대로 유지됩니다",
        "pt-BR": "Copia as configurações de aparência e layout; o nome, o conteúdo e as regras de classificação permanecem inalterados",
        "ru": "Копирует оформление и макет; имя, содержимое и правила сортировки не меняются",
        "ar": "ينسخ إعدادات المظهر والتخطيط؛ يبقى الاسم والمحتوى وقواعد الفرز دون تغيير",
        "zh-TW": "複製外觀和版面配置設定；名稱、內容和排序規則保持不變",
    },
    "复制设置的来源栅栏": {
        "en": "Fence to copy settings from",
        "de": "Bereich, aus dem Einstellungen kopiert werden",
        "es": "Grupo del que copiar la configuración",
        "fr": "Groupe dont copier les paramètres",
        "it": "Riquadro da cui copiare le impostazioni",
        "ja": "設定のコピー元フェンス",
        "ko": "설정을 복사할 원본 펜스",
        "pt-BR": "Grupo do qual copiar as configurações",
        "ru": "Область, из которой копируются настройки",
        "ar": "السياج الذي تُنسخ منه الإعدادات",
        "zh-TW": "複製設定的來源圍欄",
    },
    "确认": {
        "en": "Confirm",
        "de": "Bestätigen",
        "es": "Confirmar",
        "fr": "Confirmer",
        "it": "Conferma",
        "ja": "確認",
        "ko": "확인",
        "pt-BR": "Confirmar",
        "ru": "Подтвердить",
        "ar": "تأكيد",
        "zh-TW": "確認",
    },
    "已复制“{0}”的设置": {
        "en": "Copied settings from “{0}”",
        "de": "Einstellungen von „{0}“ kopiert",
        "es": "Configuración de «{0}» copiada",
        "fr": "Paramètres de « {0} » copiés",
        "it": "Impostazioni di «{0}» copiate",
        "ja": "「{0}」の設定をコピーしました",
        "ko": "“{0}”의 설정을 복사했습니다",
        "pt-BR": "Configurações de “{0}” copiadas",
        "ru": "Настройки области «{0}» скопированы",
        "ar": "تم نسخ إعدادات «{0}»",
        "zh-TW": "已複製「{0}」的設定",
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
