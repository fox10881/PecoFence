<p align="center">
  <a href="../../README.md">English</a>
  &nbsp;·&nbsp;   <a href="README.zh-CN.md">简体中文</a>
  &nbsp;·&nbsp;   <strong>繁體中文</strong>
  &nbsp;·&nbsp;   <a href="README.ja.md">日本語</a>
  &nbsp;·&nbsp;   <a href="README.ko.md">한국어</a>
  &nbsp;·&nbsp;   <a href="README.de.md">Deutsch</a>
  &nbsp;·&nbsp;   <a href="README.fr.md">Français</a>
  &nbsp;·&nbsp;   <a href="README.es.md">Español</a>
  &nbsp;·&nbsp;   <a href="README.pt-BR.md">Português (Brasil)</a>
  &nbsp;·&nbsp;   <a href="README.it.md">Italiano</a>
  &nbsp;·&nbsp;   <a href="README.ru.md">Русский</a>
  &nbsp;·&nbsp;   <a href="README.ar.md">العربية</a>
</p>

---

<div align="center">

# 💠 CoreDesk Engine — 終極桌面客製化與整理套件

**將 Windows 11 變身為生動鮮活、極度有序的玻璃工作空間。**

CoreDesk Engine 是一款用 Rust 打造的超強次世代桌面整理與 Windows 11 外殼客製套件。遠超基礎圖示圍欄，CoreDesk Engine 融合了磁性桌面整理、3D 透視書架、即時 H.264 影片桌布、自適應桌面時鐘小工具、Windows 11 外殼深度主題化（工作列、開始功能表、通知中心和檔案總管 Mica/Acrylic），以及一鍵式 `.cde` 完整主題套件。

<!-- TODO: ADD IMAGE 1 (Hero Desktop Showcase) -->
![CoreDesk Engine Hero Showcase](assets/hero-showcase.png)
*> **[IMAGE PLACEHOLDER 1]:** 新增一張完整桌面截圖，展示 Invisible/Bookshelf 圍欄、桌面時鐘小工具和半透明工作列。*

</div>

---

## ✨ 本版本的非凡之處

CoreDesk Engine 的每一層都經過重新設計，讓你對桌面圖示和 Windows 11 外殼本身擁有完全的創意掌控——同時 CPU 和 GPU 佔用近乎為零。

---

## 🎨 1. 次世代圍欄樣式與視覺融合

以四種獨特視覺範式超越標準容器：
- **Fluent（經典）：** 乾淨的 Windows 11 Fluent 卡片，邊框銳利。
- **Liquid Glass：** 即時折射玻璃，可對靜態或動態影片桌布進行取樣。
- **Bookshelf（`رف الكتب`）[新]：** 將圍欄變成有觸感的立體書架，圖示整齊排列在書架上。
- **Invisible Canvas（`لوحة غير مرئية`）[新]：** 完全隱藏圍欄背景和邊框——保留磁性吸附、排序規則和網格對齊的全部能力，圖示彷彿自然懸浮在原始桌布上。

### 🦎 變色龍圖示融合與捷徑清理
- **變色龍模式（`دمج الأيقونات مع الخلفية`）[新]：** 透過可自訂的**融合強度滑桿（0%–100%）**動態去飽和並將圖示融入桌布，營造平靜、無干擾的美感。
- **捷徑箭頭開關（`إظهار/إخفاء أسهم الاختصارات`）[新]：** 在設定中即時移除或恢復雜亂的 Windows 捷徑箭頭。
- **完整阿拉伯語本地化與 RTL 介面 [新]：** 原生阿拉伯語介面（`العربية`），無縫從右到左配置與即時語言切換。

<!-- TODO: ADD IMAGE 2 (Fence Styles & Chameleon Mode) -->
![Fence Styles Showcase](assets/fence-styles.png)
*> **[IMAGE PLACEHOLDER 2]:** 新增「一般」分頁截圖，展示 4 種主題樣式（Fluent、Liquid Glass、Bookshelf、Invisible Canvas）和變色龍融合。*

---

## 🎬 2. 原生 H.264 影片桌布與自適應時鐘小工具

### 🎞️ 硬體加速影片桌布引擎 [新]
- 直接從 CoreDesk Engine 將任意 **MP4 (H.264)** 影片設為絲般順滑的動態桌面桌布。
- 內建 *CloudConvert* 和 *FreeConvert* 快速連結，輕鬆將任何影片轉換為超輕量、GPU 友善的桌布。

### 🕰️ 無邊框桌面時鐘小工具（`ويدجت الساعة المكتبية`）[新]
- 從 **20 款手工打造的回應式時鐘/日期/星期設計**（10 款 Gemini 簽名 + 10 款 Devin 簽名）中選擇，或選 `بدون` 完全關閉。
- **真正的無邊框互動：** 無標題列、無雜物。在小工具**內部任意位置**點擊拖曳即可移動，抓住**任意邊緣/角落**即可平滑縮放並自動縮放字體。
- **自適應桌布對比度（`تنسيق ألوان الويدجت تلقائياً لتناسب الخلفية`）[新]：** CoreDesk Engine 持續取樣時鐘小工具正後方的精確桌布像素。無論桌布是刺眼的純白還是純黑，小工具都會自動翻轉文字、玻璃和強調的對比度，使時間和日期始終 100% 清晰可讀。

<!-- TODO: ADD IMAGE 3 (Desktop Clock Widget & Video Wallpaper) -->
![Desktop Clock Widget Showcase](assets/clock-widgets.png)
*> **[IMAGE PLACEHOLDER 3]:** 新增一張桌面時鐘小工具懸浮截圖及其設定與自適應對比度開關。*

---

## 🪟 3. Windows 11 外殼與檔案總管玻璃深度客製

何必安裝五個獨立調校工具？CoreDesk Engine 一站式統一你的整個 Windows 11 美學。

### 🛠️ 即時即時外殼樣式（Windhawk 驅動預設）[新]
即時切換主題——選擇即套用，零重啟：
- **工作列樣式（`نمط شريط المهام`）：***Translucent Taskbar、Window Glass、OS26 Liquid Glass（Clear/Dark Mac Dock & Taskbar 變體）、Plasma、Surface、Luminosity、Layer Mica UI、Fluid、Tinted Glass、Frosted Acrylic、Pills、Blob、Minecraft Hotbar、One UI 8.5 Dock* 等。
- **開始功能表樣式（`نمط قائمة ابدأ`）：***Down Aero、Translucent Start Menu、No Recommended Section、Side by Side、Windows 10、Fluent 2 Inspired、Rose Pine、Metro 10 Minimal、Everblush、Sun Valley、Liquid Glass、Borderless、Command Center、Full Screen、Frosty Glass*。
- **通知中心樣式（`نمط مركز الإشعارات`）：***Layer Mica UI、Translucent Shell、Matte、Unified、JumpLists 10、Better Control 11、Fluid、Liquid Glass、Borderless、Dense、Frosty Glass、OS26 Tahoe Glass、Command Center*。

<!-- TODO: ADD IMAGE 4 (Taskbar, Start Menu & Notification Center Themes) -->
![Windows 11 Shell Customization](assets/windows-shell-customization.png)
*> **[IMAGE PLACEHOLDER 4]:** 新增自訂開始功能表、通知中心和工作列主題下拉選單實際運行的截圖。*

### 🔍 檔案總管玻璃工作室與統一應用透明化 [新]
- **專屬檔案總管玻璃編輯器（`تأثير خلفية مستكشف الملفات`）：** 開啟帶有互動式**檔案總管即時預覽（`معاينة مباشرة`）**和可選預覽背景的**進階設定（`الإعدادات المتقدمة`）**工作室！
- 自訂 **Blur、Acrylic、Mica、Mica Alt、Clear Blur**，切換網址列/捲軸/WinUI 工具列透明度，並微調自訂 `#AARRGGBB` 淺色與深色混合色調及不透明度。
- **真正的 DComp 應用鏡像：** CoreDesk Engine 自身視窗（含無邊框標題列和客戶區）以智慧 `-70` 不透明度偏移即時鏡像你的總管玻璃材質——整個應用獲得真正的透視硬體玻璃，無雜色覆蓋。
- **一鍵外觀重新整理（`تحديث المظهر`）[新]：** 即時重啟 `explorer.exe` 並乾淨地重新掛鉤所有桌面圍欄和活動影片桌布。

<!-- TODO: ADD IMAGE 5 (File Explorer Glass Studio & Live Preview) -->
![File Explorer Glass Editor](assets/explorer-glass-editor.png)
*> **[IMAGE PLACEHOLDER 5]:** 新增檔案總管玻璃進階設定視窗的截圖，展示互動式即時預覽和自訂 RGBA 控制項。*

---

## 📐 4. 圍欄級精細客製與 3D 透視書架

在**圍欄（`الأسوار`）**分頁中，每個圍欄都能精確到像素調整：

- **複製圍欄設定（`نسخ الإعدادات من سياج آخر`）[新]：** 設計好完美圍欄？一鍵將其整體視覺外觀和版面設定複製到任意其他圍欄——不覆蓋目標圍欄的名稱、內容或排序規則。
- **標題列可見性（`إظهار شريط العنوان`）：** 在*總是*和*懸停時*之外新增**「永不顯示」（`عدم الإظهار أبداً`）[新]**，實現完全無標題外觀。
- **自訂圖示框架與調色盤（`إطارات الأيقونات`）[新]：** 用**透明（`شفاف`）**或**全彩（`ملون بالكامل`）**圓角卡片包裹單個圖示，**文字顏色（`لون النص`）**和**框架顏色（`لون الإطار`）**各配專用 11 色選擇器。
- **貼邊圖示模式（`شكل الأيقونة: ملتصق`）[新]：** 將圖示裁剪為 `2:3` 邊到邊海報——遊戲庫和媒體圍欄的完美之選！
- **精密像素滑桿 [新]：**
  - **自訂圖示大小（`حجم أيقونة مخصص`）：** `16px` 到 `512px` 自由滑桿。
  - **欄間距（`تباعد الأعمدة`）與列間距（`تباعد الصفوف`）：** 可縮至 `0px` 讓圖示無縫貼合，或拉大到 `48px+`。
- **捲軸控制 [新]：** 獨立切換**隱藏捲軸（`إخفاء شريط التمرير`）**（滑鼠滾輪捲動保持有效），或啟用**鎖定捲動（`قفل التمرير`）**將內容固定在頂部用於展示版面。
- **3D 傾斜書架模式與透視縮放（`وضع الرف المائل` + `معامل التكبير`）[新]：** 將圍欄傾斜成 3D 透視平行四邊形，與帶角度的 3D 房間桌布對齊！調整**透視縮放係數（`معامل التكبير`）**讓圖示向書架遠端平滑縮小，營造真實縱深感。

<!-- TODO: ADD IMAGE 6 (Per-Fence Customization, Icon Frames & Tilted Shelf) -->
![Advanced Fence Layout & 3D Shelf](assets/fence-advanced-layout.png)
*> **[IMAGE PLACEHOLDER 6]:** 新增「圍欄」分頁截圖，展示複製設定、圖示框架與顏色選擇器、貼邊 2:3 模式、間距滑桿和傾斜書架模式。*

---

## ⚡ 5. 顏色編碼排序引擎與強制套用

**排序規則（`قواعد الفرز`）**引擎已為可靠性和視覺清晰度完全重建：
- **顏色編碼圍欄徽章 [新]：** 每個圍欄在規則清單中獲得獨特的顏色徽章，一眼即可識別哪條規則通往哪個圍欄。
- **立即套用 vs 強制套用（`تطبيق إجباري على كل العناصر`）[新]：**
  - **立即套用（`تطبيق الآن`）：** 溫和地引導未整理的桌面項目，同時尊重你手動放置的條目。
  - **強制套用（`تطبيق إجباري`）：** 覆蓋所有手動放置，嚴格按照活動規則對桌面每一項進行無情重排。
- **一鍵快速預設（`إضافة سريعة`）：** 立即為*圖片、音樂、影片、壓縮檔、安裝程式*或*桌面清理*（僅掃走 30 天以上未動的安裝程式和壓縮檔，不刪除任何檔案）產生預建圍欄和規則。

<!-- TODO: ADD IMAGE 7 (Color-Coded Sorting Rules & Force Apply) -->
![Sorting Rules Engine](assets/sorting-rules-colored.png)
*> **[IMAGE PLACEHOLDER 7]:** 新增排序規則分頁截圖，突出顏色編碼圍欄徽章和橙色強制套用按鈕。*

---

## 📦 6. `.cde` 完整主題套件與智慧桌布適配

忘掉基礎的 `.json` 版面備份——**版面與備份（`التخطيط والنسخ الاحتياطية`）**分頁現在是完整的**視覺化主題庫（`معرض الثيمات`）**：
- **一體化 `.cde` 主題存檔 [新]：** 儲存主題將**一切**捕獲進可攜式 `.cde` 套件——圍欄精確座標、圍欄級樣式、自訂圖示大小、排序規則、玻璃設定，以及你的靜態或動態影片桌布，並附帶自動桌面預覽縮圖！
- **一鍵套用、匯出與匯入：** 數秒內在整套桌面配置間切換，或匯出 `.cde` 與他人分享你的桌面主題。
- **智慧寬高比桌布適配（`ضبط الصورة لتظهر بالكامل مهما كانت أبعاد الشاشة`）[新]：** 桌布與顯示器寬高比不符？開啟後可將完整未裁剪圖像置中顯示，邊緣以氛圍感模糊背景填充——關閉則為標準邊到邊裁剪填充。
- **每日自動儲存（`حفظ تحديث الثيم تلقائياً كل يوم`）[新]：** 每天自動就地更新你的活動 `.cde` 主題檔案——最新桌面調整始終儲存且無重複。

<!-- TODO: ADD IMAGE 8 (Theme Gallery & .cde Packages) -->
![Theme Gallery Showcase](assets/themes-gallery.png)
*> **[IMAGE PLACEHOLDER 8]:** 新增主題庫截圖，展示帶套用/匯出 .cde/刪除按鈕的視覺化主題卡片和智慧桌布適配開關。*

---

## 🚀 開始使用

1. 從 **Releases** 頁面下載最新的 `setup.exe` 或 `portable.zip`。
2. 從系統匣啟動 **CoreDesk Engine** 並開啟**設定（`إعدادات CoreDesk Engine`）**。
3. 選擇**主題樣式**（*Fluent、Liquid Glass、Bookshelf 或 Invisible Canvas*），啟用**桌面時鐘小工具**，或從**主題庫**匯入 `.cde` 主題——見證桌面即刻煥然一新！
