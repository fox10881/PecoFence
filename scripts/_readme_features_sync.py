# Sync the new-feature README blocks (portable snapshots, 3D shelf mode + known issue,
# 2D wallpapers pack + pro tip) into every localized README, and append the
# 3D-shelf row to each features table. Run from repo root; deletes nothing.
import io, re, sys

ANCHOR = '<a id="ask-your-ai-to-organize-it"></a>'

T = {}

T['zh-CN'] = dict(sub='''
### 📸 便携布局快照

布局快照现在是可以完全带走的备份:直接在快照列表里重命名存档文件,还可以把当前桌面壁纸一并打进存档。这样在任何一台电脑上还原布局都分毫不差——哪怕那台电脑从未见过你的壁纸。

### 🧊 3D 倾斜层架(实验性)

把任意栅栏剪切成一个完美的平行四边形,让每一行都能贴合斜视角壁纸里的层板:拖动侧边上缘的竖向手柄倾斜栅栏的一侧,图标会朝更低、更近的一侧逐级缩放——可在栅栏设置中通过**纵深缩放**滑杆调节。

> **⚠️ 已知问题——求助。** 平行四边形内的鼠标移动、对齐和命中测试都完全正常。但使用 3D 透视壁纸时,第一行能与层板完美对齐,后续各行却因为壁纸的灭点透视而对不上。如果哪位开发者知道如何完美解决这种多行透视数学问题,我们非常欢迎 PR!

### 🖼️ 现成的 2D 壁纸

仓库自带一组正面平视的书架壁纸,位于 [`assets/wallpapers/`](../../assets/wallpapers/),与未倾斜的平直栅栏完美对齐——选一张,摆好层架,完成。

> **💡 小贴士:** 我们提供了一套与平直栅栏完美对齐的现成 2D 壁纸。想要惊艳的效果,强烈建议把这些 2D 图片交给 AI 视频生成器做成动画,再用 [Lively Wallpaper](https://github.com/rocksdanister/lively) 应用到桌面。
''', row='| **3D 倾斜层架(实验性)** | 把栅栏剪切成平行四边形,让各行贴合斜视角壁纸的层板;图标朝更近的一侧逐级缩放。 |')

T['zh-TW'] = dict(sub='''
### 📸 可攜式版面快照

版面快照現在是可以完全帶著走的備份:直接在快照清單中重新命名存檔,還可以選擇把目前的桌面桌布一併打包進存檔。這樣在任何一台電腦上還原版面都分毫不差——即使那台電腦從未看過你的桌布。

### 🧊 3D 傾斜層架(實驗性)

把任意圍欄剪切成完美的平行四邊形,讓每一列都能貼合斜視桌布裡的層板:拖曳側邊上緣的垂直把手傾斜圍欄的一側,圖示會朝較低、較近的一側逐級縮放——可在圍欄設定中透過**縱深縮放**滑桿調整。

> **⚠️ 已知問題——求助。** 平行四邊形內的滑鼠移動、對齊與命中測試都完全正常。但使用 3D 透視桌布時,第一列能與層板完美對齊,後續各列卻因為桌布的滅點透視而對不齊。如果哪位開發者知道如何完美解決這種多列透視的數學問題,我們非常歡迎 PR!

### 🖼️ 現成的 2D 桌布

儲存庫內附一組正面平視的層架桌布,位於 [`assets/wallpapers/`](../../assets/wallpapers/),與未傾斜的平直圍欄完美對齊——挑一張、排好層架,完成。

> **💡 小提示:** 我們提供了一套與平直圍欄完美對齊的現成 2D 桌布。想要驚豔的效果,強烈建議把這些 2D 圖片交給 AI 影片產生器做成動畫,再用 [Lively Wallpaper](https://github.com/rocksdanister/lively) 套用到桌面。
''', row='| **3D 傾斜層架(實驗性)** | 將圍欄剪切成平行四邊形,讓各列貼合斜視桌布的層板;圖示朝較近的一側逐級縮放。 |')

T['ja'] = dict(sub='''
### 📸 ポータブルレイアウトスナップショット

レイアウトスナップショットは完全に持ち運べるバックアップになりました。スナップショット一覧で保存ファイルの名前を直接変更でき、現在のデスクトップ壁紙をファイルに同梱することもできます。これにより、壁紙を持っていないPCでもレイアウトをピクセル単位で正確に復元できます。

### 🧊 3D 傾斜シェルフ(実験的)

フェンスを完璧な平行四辺形にせん断変形し、各段を斜め視点の壁紙の棚板に合わせられます。側面上部の縦方向ハンドルをドラッグしてフェンスの片側を傾けると、アイコンは低い(近い)側ほど大きく表示されます。倍率はフェンス設定の**奥行きスケール**スライダーで調整できます。

> **⚠️ 既知の問題——協力者募集。** 平行四辺形内でのマウス操作・配置・ヒットテストは完璧に動作しています。ただし 3D パースペクティブの壁紙を使うと、最初の一段は棚板にぴったり合いますが、壁紙の消失点の遠近法により 2 段目以降がずれてしまいます。この複数段パースペクティブの数学的解法をご存じの開発者の方、プルリクエストを大歓迎です!

### 🖼️ すぐに使える 2D 壁紙

正面・平行視点で描かれた厳選の棚壁紙パックを [`assets/wallpapers/`](../../assets/wallpapers/) に同梱しています。せん断していない平らなフェンスと完璧に揃います——選んで棚を並べるだけです。

> **💡 プロのヒント:** 平らなフェンスと完璧に揃う 2D 壁紙を用意しました。さらに素晴らしいセットアップのために、この 2D 画像を AI 動画ジェネレーターでアニメーション化し、[Lively Wallpaper](https://github.com/rocksdanister/lively) でデスクトップに適用することを強くおすすめします。
''', row='| **3D 傾斜シェルフ(実験的)** | フェンスを平行四辺形にせん断し、斜め視点の壁紙の棚板に各段を合わせます。近い側ほど大きくなるアイコンの奥行きスケーリング付き。 |')

T['ko'] = dict(sub='''
### 📸 포터블 레이아웃 스냅샷

레이아웃 스냅샷이 이제 완전히 휴대 가능한 백업이 되었습니다. 스냅샷 목록에서 저장 파일의 이름을 바로 바꿀 수 있고, 현재 바탕 화면 배경 무늬를 파일 안에 함께 담을 수도 있습니다. 그러면 배경 무늬가 없던 PC에서도 레이아웃이 픽셀 단위로 정확히 복원됩니다.

### 🧊 3D 기울어진 선반(실험적)

펜스를 완벽한 평행사변형으로 전단 변형해 각 줄이 기울어진 배경 무늬의 선반을 따라가게 할 수 있습니다. 측면 상단의 세로 핸들을 드래그해 펜스 한쪽을 기울이면, 아이콘이 더 낮고 가까운 쪽을 향해 점진적으로 커집니다. 펜스 설정의 **깊이 배율** 슬라이더로 조절하세요.

> **⚠️ 알려진 문제 — 도움 환영.** 기울어진 평행사변형 안에서의 마우스 이동, 정렬, 히트 테스트는 완벽하게 동작합니다. 다만 3D 원근감 배경 무늬를 사용하면 첫 번째 줄은 선반에 정확히 맞지만, 배경 무늬의 소실점 원근 때문에 이후 줄들은 어긋납니다. 이 다중 행 원근 수학을 완벽하게 해결할 방법을 아는 개발자가 계시다면 풀 리퀘스트를 환영합니다!

### 🖼️ 바로 쓸 수 있는 2D 배경 무늬

정면 평면 시점으로 그려진 엄선된 선반 배경 무늬 팩을 [`assets/wallpapers/`](../../assets/wallpapers/)에 담았습니다. 기울이지 않은 평평한 펜스와 완벽하게 정렬됩니다 — 하나 골라 선반을 배치하면 끝입니다.

> **💡 프로 팁:** 평평한 펜스와 완벽하게 어울리는 2D 배경 무늬를 제공합니다. 더 멋진 설정을 위해 이 2D 이미지를 AI 영상 생성기로 애니메이션화한 뒤 [Lively Wallpaper](https://github.com/rocksdanister/lively)로 바탕 화면에 적용해 보세요.
''', row='| **3D 기울어진 선반(실험적)** | 펜스를 평행사변형으로 전단해 각 줄이 기울어진 배경 무늬의 선반을 따라가게 하고, 가까운 쪽으로 갈수록 아이콘이 커지는 깊이 스케일링을 지원합니다. |')

T['de'] = dict(sub='''
### 📸 Portable Layout-Snapshots

Layout-Snapshots sind jetzt vollständig portable Backups: Benenne eine Sicherungsdatei direkt in der Snapshot-Liste um und packe optional das aktuelle Desktop-Hintergrundbild mit in die Datei. Das Layout wird dann auf jedem PC pixelgenau wiederhergestellt – selbst auf einem, der dein Hintergrundbild nie gesehen hat.

### 🧊 3D-Schrägregal (experimentell)

Schere einen Bereich zu einem perfekten Parallelogramm, damit seine Reihen den Brettern eines schrägen Hintergrundbilds folgen: Ziehe die vertikalen Griffe am oberen Rand einer Seite, und die Symbole skalieren progressiv zur näheren, tieferen Kante – einstellbar über den **Tiefenfaktor**-Schieberegler in den Bereichs-Einstellungen.

> **⚠️ Bekanntes Problem – Hilfe gesucht.** Mausbewegung, Ausrichtung und Treffererkennung innerhalb des schrägen Parallelogramms funktionieren einwandfrei. Bei einem 3D-Perspektiv-Hintergrundbild sitzt jedoch nur die erste Reihe perfekt auf dem Brett – die folgenden Reihen verziehen sich wegen der Fluchtpunkt-Perspektive des Bildes. Wenn du weißt, wie man diese mehrzeilige Perspektiv-Mathematik perfekt löst, sind Pull Requests herzlich willkommen!

### 🖼️ Fertige 2D-Hintergrundbilder

Das Repository enthält ein kuratiertes Paket flacher Frontansicht-Regal-Hintergrundbilder in [`assets/wallpapers/`](../../assets/wallpapers/), die perfekt zu den ungescherten, geraden Bereichen passen – eines auswählen, Regale anordnen, fertig.

> **💡 Profi-Tipp:** Wir stellen einsatzbereite 2D-Hintergrundbilder bereit, die perfekt zu den geraden Bereichen passen. Für ein beeindruckendes Setup empfehlen wir, diese 2D-Bilder durch einen KI-Videogenerator zu animieren und mit [Lively Wallpaper](https://github.com/rocksdanister/lively) als Desktop-Hintergrund zu nutzen.
''', row='| **3D-Schrägregal (experimentell)** | Schere einen Bereich zu einem Parallelogramm, damit seine Reihen den Brettern eines schrägen Hintergrundbilds folgen – mit Tiefenskalierung der Symbole zur näheren Kante. |')

T['fr'] = dict(sub='''
### 📸 Instantanés de disposition portables

Les instantanés de disposition sont désormais des sauvegardes entièrement portables : renommez un fichier de sauvegarde directement dans la liste des instantanés et, si vous le souhaitez, embarquez le fond d'écran actuel dans le fichier lui-même. La disposition se restaure alors au pixel près sur n'importe quel PC — même s'il n'a jamais vu votre fond d'écran.

### 🧊 Étagère 3D inclinée (expérimental)

Ciselez n'importe quel groupe en un parfait parallélogramme afin que ses rangées suivent les planches d'un fond d'écran en perspective : faites glisser les poignées verticales en haut d'un côté pour incliner le groupe, et les icônes se mettent à l'échelle progressivement vers le bord le plus proche et le plus bas — réglable via le curseur **Facteur de profondeur** dans les réglages du groupe.

> **⚠️ Problème connu — aide bienvenue.** Le déplacement de la souris, l'alignement et les tests de collision dans le parallélogramme incliné fonctionnent parfaitement. En revanche, avec un fond d'écran en perspective 3D, la première rangée s'aligne parfaitement sur sa planche, mais les suivantes non, à cause du point de fuite du fond d'écran. Si un développeur sait résoudre parfaitement ce calcul de perspective sur plusieurs rangées, les pull requests sont les bienvenues !

### 🖼️ Fonds d'écran 2D prêts à l'emploi

Le dépôt embarque une sélection de fonds d'écran d'étagères en vue frontale plane dans [`assets/wallpapers/`](../../assets/wallpapers/), parfaitement alignés avec les groupes plats non inclinés — choisissez-en un, disposez vos étagères, c'est prêt.

> **💡 Astuce de pro :** Nous fournissons des fonds d'écran 2D prêts à l'emploi qui s'alignent parfaitement avec les groupes plats. Pour une configuration spectaculaire, nous vous recommandons vivement de faire passer ces images 2D dans un générateur vidéo IA pour les animer, puis de les appliquer à votre bureau avec [Lively Wallpaper](https://github.com/rocksdanister/lively).
''', row='| **Étagère 3D inclinée (expérimental)** | Ciselez un groupe en parallélogramme pour que ses rangées suivent les planches d’un fond d’écran en perspective, avec mise à l’échelle de profondeur des icônes vers le bord proche. |')

T['es'] = dict(sub='''
### 📸 Instantáneas de distribución portátiles

Las instantáneas de distribución ahora son copias de seguridad totalmente portátiles: renombra un archivo guardado directamente en la lista de instantáneas y, si quieres, empaqueta el fondo de pantalla actual dentro del propio archivo. La distribución se restaura entonces al píxel en cualquier PC — incluso en uno que nunca haya visto tu fondo.

### 🧊 Estantería 3D inclinada (experimental)

Cizalla cualquier grupo hasta convertirlo en un paralelogramo perfecto para que sus filas sigan las baldas de un fondo de pantalla en ángulo: arrastra los tiradores verticales del borde superior de un lado para inclinar el grupo, y los iconos se escalan progresivamente hacia el borde más cercano y bajo — ajustable con el deslizador **Factor de profundidad** en la configuración del grupo.

> **⚠️ Problema conocido — se agradece ayuda.** El movimiento del ratón, la alineación y la detección de clics dentro del paralelogramo inclinado funcionan perfectamente. Sin embargo, con un fondo de pantalla de perspectiva 3D, la primera fila se alinea perfectamente con la balda, pero las siguientes no, debido a la perspectiva del punto de fuga del fondo. Si algún desarrollador sabe resolver perfectamente esta matemática de perspectiva en varias filas, ¡los pull requests son más que bienvenidos!

### 🖼️ Fondos de pantalla 2D listos para usar

El repositorio incluye un paquete seleccionado de fondos de estanterías en vista frontal plana en [`assets/wallpapers/`](../../assets/wallpapers/), que se alinean a la perfección con los grupos planos sin inclinar: elige uno, coloca tus baldas y listo.

> **💡 Consejo profesional:** Hemos incluido fondos 2D listos para usar que se alinean perfectamente con los grupos planos. Para una configuración espectacular, te recomendamos pasar estas imágenes 2D por un generador de vídeo con IA para animarlas y aplicarlas a tu escritorio con [Lively Wallpaper](https://github.com/rocksdanister/lively).
''', row='| **Estantería 3D inclinada (experimental)** | Cizalla un grupo hasta formar un paralelogramo para que sus filas sigan las baldas de un fondo en ángulo, con escalado de profundidad de los iconos hacia el borde cercano. |')

T['pt-BR'] = dict(sub='''
### 📸 Snapshots de layout portáteis

Os snapshots de layout agora são backups totalmente portáteis: renomeie um arquivo salvo diretamente na lista de snapshots e, se quiser, embuta o papel de parede atual no próprio arquivo. O layout é então restaurado pixel a pixel em qualquer PC — mesmo um que nunca tenha visto seu papel de parede.

### 🧊 Prateleira 3D inclinada (experimental)

Deforme qualquer grupo em um paralelogramo perfeito para que suas linhas acompanhem as prateleiras de um papel de parede em ângulo: arraste as alças verticais no topo de uma lateral para inclinar o grupo, e os ícones escalam progressivamente em direção à borda mais próxima e mais baixa — ajustável pelo controle **Fator de profundidade** nas configurações do grupo.

> **⚠️ Problema conhecido — ajuda bem-vinda.** O movimento do mouse, o alinhamento e a detecção de cliques dentro do paralelogramo inclinado funcionam perfeitamente. Porém, com um papel de parede em perspectiva 3D, a primeira linha se alinha perfeitamente à prateleira, mas as seguintes não, devido à perspectiva do ponto de fuga do papel de parede. Se algum desenvolvedor souber resolver perfeitamente essa matemática de perspectiva em várias linhas, pull requests são muito bem-vindos!

### 🖼️ Papéis de parede 2D prontos para usar

O repositório traz um pacote selecionado de papéis de parede de prateleiras em vista frontal plana em [`assets/wallpapers/`](../../assets/wallpapers/), que se alinham perfeitamente com os grupos planos e sem inclinação — escolha um, arrume suas prateleiras e pronto.

> **💡 Dica de profissional:** Fornecemos papéis de parede 2D prontos que se alinham perfeitamente com os grupos planos. Para uma configuração incrível, recomendamos fortemente passar essas imagens 2D por um gerador de vídeo com IA para animá-las e aplicá-las à área de trabalho com o [Lively Wallpaper](https://github.com/rocksdanister/lively).
''', row='| **Prateleira 3D inclinada (experimental)** | Deforme um grupo em paralelogramo para que suas linhas sigam as prateleiras de um papel de parede em ângulo, com escala de profundidade dos ícones em direção à borda próxima. |')

T['ru'] = dict(sub='''
### 📸 Портативные снимки расположения

Снимки расположения теперь — полностью переносимые резервные копии: переименуйте файл прямо в списке снимков и при желании вложите текущие обои рабочего стола в сам файл. Расположение восстановится с точностью до пикселя на любом ПК — даже там, где ваши обои никогда не были.

### 🧊 3D наклонная полка (экспериментально)

Скосите любую область в идеальный параллелограмм, чтобы её ряды повторяли полки обоев под углом: потяните вертикальные ручки у верхнего края одной стороны, и значки будут постепенно увеличиваться к более низкому, ближнему краю — интенсивность задаётся ползунком **Коэффициент глубины** в настройках области.

> **⚠️ Известная проблема — нужна помощь.** Перемещение мыши, выравнивание и определение попадания внутри скошенного параллелограмма работают идеально. Однако с обоями с 3D-перспективой первый ряд точно совпадает с полкой, а последующие — нет, из-за перспективы точки схода обоев. Если какой-то разработчик знает, как идеально решить эту математику многорядной перспективы, pull request'ы очень приветствуются!

### 🖼️ Готовые 2D-обои

В репозитории есть подборка плоских фронтальных обоев с полками в [`assets/wallpapers/`](../../assets/wallpapers/), которые идеально совпадают с плоскими, нескошенными областями — выберите одни, расставьте полки, готово.

> **💡 Совет:** Мы подготовили готовые 2D-обои, идеально совпадающие с плоскими областями. Для впечатляющего результата настоятельно рекомендуем прогнать эти 2D-изображения через ИИ-генератор видео и применить их к рабочему столу через [Lively Wallpaper](https://github.com/rocksdanister/lively).
''', row='| **3D наклонная полка (экспериментально)** | Скосите область в параллелограмм, чтобы её ряды повторяли полки обоев под углом; значки масштабируются по глубине к ближнему краю. |')

T['ar'] = dict(sub='''
### 📸 لقطات تخطيط محمولة

أصبحت لقطات التخطيط نُسخًا احتياطية محمولة بالكامل: أعد تسمية ملف الحفظ مباشرةً من قائمة اللقطات، ويمكنك اختياريًا تضمين خلفية سطح المكتب الحالية داخل ملف الحفظ نفسه. عندها تُستعاد اللقطة بدقة البكسل على أي جهاز — حتى لو لم يسبق لذلك الجهاز أن رأى خلفيتك.

### 🧊 وضع الرف المائل (تجريبي)

حوِّل أي سياج إلى متوازي أضلاع مثالي بحيث تتبع صفوفه أرفف خلفية مائلة: اسحب المقبضين الرأسيين عند أعلى أحد الجانبين لإمالة السياج، وتتدرج الأيقونات في الحجم نحو الحافة الأقرب والأخفض — ويُضبط ذلك عبر شريط **معامل التكبير** في إعدادات السياج.

> **⚠️ مشكلة معروفة — نرحب بالمساعدة.** تعمل حركة المؤشر والمحاذاة واختبار الإصابة داخل متوازي الأضلاع المائل بشكل مثالي. لكن عند استخدام خلفية بمنظور ثلاثي الأبعاد، يصطف الصف الأول مع الرف بإتقان بينما لا تفعل الصفوف التالية، بسبب منظور نقطة التلاشي في الخلفية. إن كان لدى أي مطوّر حلٌّ رياضي مثالي لهذه المشكلة متعددة الصفوف، فطلبات الدمج مرحّب بها جدًا!

### 🖼️ خلفيات ثنائية الأبعاد جاهزة

يتضمّن المستودع مجموعة مختارة من خلفيات الأرفف المسطّحة بالإسقاط الأمامي في [`assets/wallpapers/`](../../assets/wallpapers/)، وهي تتطابق تمامًا مع السياجات المستوية غير المائلة — اختر واحدة، ورتّب أرففك، وانتهى الأمر.

> **💡 نصيحة احترافية:** وفرنا خلفيات ثنائية الأبعاد جاهزة تتطابق تمامًا مع السياجات المستوية. ولإعدادٍ مبهر، ننصح بشدة بتمرير هذه الصور ثنائية الأبعاد عبر مولّد فيديو بالذكاء الاصطناعي لتحريكها، ثم تطبيقها على سطح المكتب باستخدام [Lively Wallpaper](https://github.com/rocksdanister/lively).
''', row='| **وضع الرف المائل (تجريبي)** | يحوّل السياج إلى متوازي أضلاع لتتبع صفوفه أرفف خلفية مائلة، مع تدرّج عمقي لحجم الأيقونات نحو الحافة الأقرب. |')

LANGS = ['zh-CN','zh-TW','ja','ko','de','fr','es','pt-BR','ru','ar']

for lang in LANGS:
    path = f'docs/readme/README.{lang}.md'
    text = io.open(path, encoding='utf-8').read()
    if '3D' in text and 'wallpapers' in text:
        print(f'{lang}: already updated, skipping'); continue
    m = re.search(r'(?m)^<a id="[^"]*"></a>\s*$', text)
    if not m:
        print(f'{lang}: ANCHOR NOT FOUND'); sys.exit(1)
    # 1. subsections before the AI-section anchor
    text = text[:m.start()] + T[lang]['sub'].strip() + '\n\n' + text[m.start():]
    # 2. append the 3D-shelf row after the last table row
    lines = text.split('\n')
    last_tbl = max(i for i, l in enumerate(lines) if l.startswith('|'))
    lines.insert(last_tbl, T[lang]['row'])
    text = '\n'.join(lines)
    io.open(path, 'w', encoding='utf-8', newline='\n').write(text)
    print(f'{lang}: updated')
