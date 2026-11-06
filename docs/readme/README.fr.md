<p align="center">
  <a href="../../README.md">English</a>
  &nbsp;·&nbsp;   <a href="README.zh-CN.md">简体中文</a>
  &nbsp;·&nbsp;   <a href="README.zh-TW.md">繁體中文</a>
  &nbsp;·&nbsp;   <a href="README.ja.md">日本語</a>
  &nbsp;·&nbsp;   <a href="README.ko.md">한국어</a>
  &nbsp;·&nbsp;   <a href="README.de.md">Deutsch</a>
  &nbsp;·&nbsp;   <strong>Français</strong>
  &nbsp;·&nbsp;   <a href="README.es.md">Español</a>
  &nbsp;·&nbsp;   <a href="README.pt-BR.md">Português (Brasil)</a>
  &nbsp;·&nbsp;   <a href="README.it.md">Italiano</a>
  &nbsp;·&nbsp;   <a href="README.ru.md">Русский</a>
  &nbsp;·&nbsp;   <a href="README.ar.md">العربية</a>
</p>

---

<div align="center">

# 💠 CoreDesk Engine — Suite ultime de personnalisation et d'organisation du bureau

**Transforme Windows 11 en un espace de travail de verre vivant et ultra-organisé.**

CoreDesk Engine est une suite d'organisation de bureau et de personnalisation du shell Windows 11 de nouvelle génération, écrite en Rust. Bien au-delà de simples clôtures d'icônes, elle réunit l'organisation magnétique, des étagères en perspective 3D, des fonds vidéo H.264 en direct, des widgets d'horloge adaptatifs, une personnalisation profonde du shell (barre des tâches, menu Démarrer, centre de notifications et Explorateur Mica/Acrylic) et des packs de thèmes `.cde` complets en un clic.

<!-- TODO: ADD IMAGE 1 (Hero Desktop Showcase) -->
![CoreDesk Engine Hero Showcase](assets/hero-showcase.png)
*> **[IMAGE PLACEHOLDER 1]:** Ajoute une capture complète du bureau montrant les clôtures Invisible/Bookshelf, le widget d'horloge et la barre des tâches translucide.*

</div>

---

## ✨ Qu'est-ce qui rend cette édition extraordinaire ?

Chaque couche de CoreDesk Engine a été réarchitecturée pour te donner un contrôle créatif total sur tes icônes et le shell Windows 11 lui-même — avec une consommation CPU/GPU quasi nulle.

---

## 🎨 1. Styles de clôture nouvelle génération et fusion visuelle

Va au-delà des conteneurs classiques avec quatre paradigmes visuels :
- **Fluent (Classique) :** Cartes Fluent nettes aux bordures précises.
- **Liquid Glass :** Verre réfringent en temps réel échantillonnant fonds statiques ou vidéo.
- **Bookshelf (`رف الكتب`) [NOUVEAU] :** Transforme la clôture en étagère dimensionnelle où les icônes s'alignent en rangées.
- **Invisible Canvas (`لوحة غير مرئية`) [NOUVEAU] :** Masque entièrement fond et bordures — tu gardes l'aimantation, les règles et la grille pendant que les icônes semblent flotter sur ton fond.

### 🦎 Fusion caméléon des icônes et nettoyage des raccourcis
- **Mode Caméléon (`دمج الأيقونات مع الخلفية`) [NOUVEAU] :** Désature et fusionne les icônes avec le fond via un **curseur d'intensité (0 %–100 %)** pour une esthétique apaisée.
- **Interrupteur de flèches de raccourci (`إظهار/إخفاء أسهم الاختصارات`) [NOUVEAU] :** Retire ou restaure les flèches de raccourci directement dans les paramètres.
- **Localisation arabe complète et interface RTL [NOUVEAU] :** Interface arabe native (`العربية`) avec mise en page droite-à-gauche et changement de langue instantané.

<!-- TODO: ADD IMAGE 2 (Fence Styles & Chameleon Mode) -->
![Fence Styles Showcase](assets/fence-styles.png)
*> **[IMAGE PLACEHOLDER 2]:** Ajoute une capture de l'onglet Général montrant les 4 styles de thème (Fluent, Liquid Glass, Bookshelf, Invisible Canvas) et la fusion Caméléon.*

---

## 🎬 2. Fonds vidéo H.264 natifs et widgets d'horloge adaptatifs

### 🎞️ Moteur de fond vidéo accéléré matériellement [NOUVEAU]
- Définis n'importe quelle vidéo **MP4 (H.264)** comme fond animé ultra-fluide directement depuis CoreDeskEngine.
- Des liens rapides vers *CloudConvert* et *FreeConvert* facilitent la conversion en fond léger et GPU-friendly.

### 🕰️ Widget d'horloge sans cadre (`ويدجت الساعة المكتبية`) [NOUVEAU]
- Choisis parmi **20 designs artisanaux et responsifs d'horloge/date/jour** (10 signatures Gemini + 10 signatures Devin), ou désactive-le avec (`بدون`).
- **Vraiment sans cadre :** Aucune barre de titre. Glisse depuis **n'importe quel point intérieur** pour le déplacer, saisis **un bord ou un coin** pour le redimensionner en douceur avec mise à l'échelle typographique automatique.
- **Contraste adaptatif du fond (`تنسيق ألوان الويدجت تلقائياً لتناسب الخلفية`) [NOUVEAU] :** CoreDesk Engine échantillonne en continu les pixels exacts derrière le widget. Fond blanc éclatant ou noir profond, le widget inverse automatiquement le contraste du texte, du verre et des accents pour une lisibilité parfaite.

<!-- TODO: ADD IMAGE 3 (Desktop Clock Widget & Video Wallpaper) -->
![Desktop Clock Widget Showcase](assets/clock-widgets.png)
*> **[IMAGE PLACEHOLDER 3]:** Ajoute une capture du widget d'horloge sur le bureau avec ses réglages et l'interrupteur de contraste adaptatif.*

---

## 🪟 3. Personnalisation profonde du shell Windows 11 et verre de l'Explorateur

Pourquoi installer cinq outils quand CoreDesk Engine unifie toute l'esthétique de Windows 11 ?

### 🛠️ Styles de shell instantanés (presets Windhawk) [NOUVEAU]
Change de thème à la volée — appliqué immédiatement, sans redémarrage :
- **Styles de barre des tâches (`نمط شريط المهام`) :** *Translucent Taskbar, Window Glass, OS26 Liquid Glass (variantes Clear/Dark Mac Dock & Taskbar), Plasma, Surface, Luminosity, Layer Mica UI, Fluid, Tinted Glass, Frosted Acrylic, Pills, Blob, Minecraft Hotbar, One UI 8.5 Dock* et plus.
- **Styles du menu Démarrer (`نمط قائمة ابدأ`) :** *Down Aero, Translucent Start Menu, No Recommended Section, Side by Side, Windows 10, Fluent 2 Inspired, Rose Pine, Metro 10 Minimal, Everblush, Sun Valley, Liquid Glass, Borderless, Command Center, Full Screen, Frosty Glass*.
- **Styles du centre de notifications (`نمط مركز الإشعارات`) :** *Layer Mica UI, Translucent Shell, Matte, Unified, JumpLists 10, Better Control 11, Fluid, Liquid Glass, Borderless, Dense, Frosty Glass, OS26 Tahoe Glass, Command Center*.

<!-- TODO: ADD IMAGE 4 (Taskbar, Start Menu & Notification Center Themes) -->
![Windows 11 Shell Customization](assets/windows-shell-customization.png)
*> **[IMAGE PLACEHOLDER 4]:** Ajoute une capture des listes déroulantes de thèmes du menu Démarrer, du centre de notifications et de la barre des tâches.*

### 🔍 Studio de verre de l'Explorateur et transparence unifiée de l'app [NOUVEAU]
- **Éditeur dédié de verre de l'Explorateur (`تأثير خلفية مستكشف الملفات`) :** Ouvre le studio **Paramètres avancés (`الإعدادات المتقدمة`)** avec **aperçu interactif de l'Explorateur (`معاينة مباشرة`)** et fonds sélectionnables !
- Personnalise **Blur, Acrylic, Mica, Mica Alt et Clear Blur**, bascule la transparence de la barre d'adresse/scrollbar/toolbar WinUI et affine des teintes `#AARRGGBB` claires et sombres sur mesure.
- **Vraie réplication DComp :** Les fenêtres de CoreDesk Engine (y compris la barre de titre sans cadre et la zone cliente) reflètent ton matériau de verre de l'Explorateur en temps réel avec un décalage d'opacité intelligent de `-70` — du vrai verre matériel sans déferlement de teinte opaque.
- **Rafraîchir l'apparence en un clic (`تحديث المظهر`) [NOUVEAU] :** Redémarre `explorer.exe` instantanément et raccroche proprement toutes les clôtures et ton fond vidéo actif.

<!-- TODO: ADD IMAGE 5 (File Explorer Glass Studio & Live Preview) -->
![File Explorer Glass Editor](assets/explorer-glass-editor.png)
*> **[IMAGE PLACEHOLDER 5]:** Ajoute une capture de la fenêtre Paramètres avancés du verre de l'Explorateur avec l'aperçu interactif et les contrôles RGBA.*

---

## 📐 4. Personnalisation granulaire par clôture et étagères 3D en perspective

Chaque clôture se règle au pixel près dans l'onglet **Clôtures (`الأسوار`)** :

- **Cloner les réglages d'une clôture (`نسخ الإعدادات من سياج آخر`) [NOUVEAU] :** La clôture parfaite ? Copie son apparence et son layout complet vers une autre en un clic — sans écraser le nom, le contenu ni les règles de la cible.
- **Visibilité de la barre de titre (`إظهار شريط العنوان`) :** Nouvelle option **« Jamais afficher » (`عدم الإظهار أبداً`) [NOUVEAU]** aux côtés de *Toujours* et *Au survol* pour un rendu sans en-tête.
- **Cadres d'icônes et palettes personnalisées (`إطارات الأيقونات`) [NOUVEAU] :** Enveloppe les icônes dans des cartes arrondies **transparentes (`شفاف`)** ou **entièrement colorées (`ملون بالكامل`)** avec des sélecteurs 11 couleurs pour la **couleur du texte (`لون النص`)** et du **cadre (`لون الإطار`)**.
- **Mode icône jointive (`شكل الأيقونة: ملتصق`) [NOUVEAU] :** Recadre les icônes en affiches `2:3` bord à bord — parfait pour les clôtures de jeux et de médias !
- **Curseurs de pixels de précision [NOUVEAU] :**
  - **Taille d'icône personnalisée (`حجم أيقونة مخصص`) :** Curseur libre de `16px` à `512px`.
  - **Écart colonnes (`تباعد الأعمدة`) et lignes (`تباعد الصفوف`) :** Descends à `0px` pour des icônes jointives, ou élargis à `48px+`.
- **Contrôle de la scrollbar [NOUVEAU] :** **Masquer la scrollbar (`إخفاء شريط التمرير`)** en gardant la molette active, ou **Verrouiller le défilement (`قفل التمرير`)** pour figer le contenu en haut.
- **Étagère inclinée 3D et échelle de perspective (`وضع الرف المائل` + `معامل التكبير`) [NOUVEAU] :** Incline la clôture en parallélogramme 3D aligné avec les fonds de pièces en angle ! Règle le **facteur d'échelle de perspective (`معامل التكبير`)** pour que les icônes rétrécissent vers le bord lointain de l'étagère.

<!-- TODO: ADD IMAGE 6 (Per-Fence Customization, Icon Frames & Tilted Shelf) -->
![Advanced Fence Layout & 3D Shelf](assets/fence-advanced-layout.png)
*> **[IMAGE PLACEHOLDER 6]:** Ajoute une capture de l'onglet Clôtures montrant le clonage de réglages, les cadres d'icônes et sélecteurs de couleur, le mode jointif 2:3, les curseurs d'écart et l'étagère inclinée.*

---

## ⚡ 5. Moteur de tri à code couleur et application forcée

Le moteur de **règles de tri (`قواعد الفرز`)** a été entièrement reconstruit :
- **Badges de clôture à code couleur [NOUVEAU] :** Chaque clôture reçoit un badge coloré distinct dans la liste — tu identifies d'un coup quelle règle va où.
- **Appliquer maintenant vs. Application forcée (`تطبيق إجباري على كل العناصر`) [NOUVEAU] :**
  - **Appliquer maintenant (`تطبيق الآن`) :** Achemine les éléments non triés en respectant tes placements manuels.
  - **Application forcée (`تطبيق إجباري`) :** Ignore tout placement manuel et re-trie chaque élément strictement selon tes règles actives.
- **Presets rapides en un clic (`إضافة سريعة`) :** Génère instantanément des clôtures et règles pour *Images, Musique, Vidéos, Archives, Installateurs* ou *Nettoyage du bureau* (qui ne balaie que les installateurs et archives intacts depuis 30+ jours sans supprimer un seul fichier).

<!-- TODO: ADD IMAGE 7 (Color-Coded Sorting Rules & Force Apply) -->
![Sorting Rules Engine](assets/sorting-rules-colored.png)
*> **[IMAGE PLACEHOLDER 7]:** Ajoute une capture de l'onglet Règles de tri avec les badges colorés et le bouton orange d'application forcée.*

---

## 📦 6. Packs de thème `.cde` complets et ajustement intelligent du fond

Oublie les sauvegardes `.json` — l'onglet **Disposition et sauvegardes (`التخطيط والنسخ الاحتياطية`)** est désormais une **galerie de thèmes visuelle (`معرض الثيمات`)** complète :
- **Archives `.cde` tout-en-un [NOUVEAU] :** Enregistrer un thème capture **tout** dans un bundle `.cde` portable — coordonnées exactes, styles par clôture, tailles d'icônes, règles, réglages de verre ET ton fond statique ou vidéo, avec miniature de bureau automatique !
- **Appliquer, exporter et importer en un clic :** Bascule entre des bureaux entiers en quelques secondes, ou exporte des `.cde` à partager.
- **Ajustement intelligent du ratio (`ضبط الصورة لتظهر بالكامل مهما كانت أبعاد الشاشة`) [NOUVEAU] :** Un fond au mauvais ratio ? Active pour afficher l'image complète non rognée, centrée avec un flou d'ambiance sur les bords — ou désactive pour le recadrage classique.
- **Sauvegarde quotidienne auto (`حفظ تحديث الثيم تلقائياً كل يوم`) [NOUVEAU] :** Met à jour ton `.cde` actif une fois par jour sur place — tes derniers réglages sont toujours préservés sans doublons.

<!-- TODO: ADD IMAGE 8 (Theme Gallery & .cde Packages) -->
![Theme Gallery Showcase](assets/themes-gallery.png)
*> **[IMAGE PLACEHOLDER 8]:** Ajoute une capture de la galerie de thèmes avec cartes visuelles, boutons Appliquer/Exporter .cde/Supprimer et l'interrupteur d'ajustement intelligent.*

---

## 🚀 Pour commencer

1. Télécharge le dernier `setup.exe` ou `portable.zip` depuis la page **Releases**.
2. Lance **CoreDesk Engine** depuis la zone de notification et ouvre les **Paramètres (`إعدادات CoreDesk Engine`)**.
3. Choisis un **style de thème** (*Fluent, Liquid Glass, Bookshelf ou Invisible Canvas*), active un **widget d'horloge** ou importe un thème `.cde` depuis la **galerie de thèmes** — et regarde ton bureau se transformer !
