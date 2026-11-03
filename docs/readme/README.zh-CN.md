<p align="center">
  <a href="../../README.md">English</a>
  &nbsp;·&nbsp;   <strong>简体中文</strong>
  &nbsp;·&nbsp;   <a href="README.zh-TW.md">繁體中文</a>
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

# 💠 PecoFence — 终极桌面定制与整理套件

**将 Windows 11 变身为生动鲜活、极度有序的玻璃工作空间。**

PecoFence 是一款用 Rust 打造的超强次世代桌面整理与 Windows 11 外壳定制套件。远超基础图标围栏，PecoFence 融合了磁性桌面整理、3D 透视书架、实时 H.264 视频壁纸、自适应桌面时钟小部件、Windows 11 外壳深度主题化（任务栏、开始菜单、通知中心和文件资源管理器 Mica/Acrylic），以及一键式 `.peco` 完整主题包。

<!-- TODO: ADD IMAGE 1 (Hero Desktop Showcase) -->
![PecoFence Hero Showcase](assets/hero-showcase.png)
*> **[IMAGE PLACEHOLDER 1]:** 添加一张完整桌面截图，展示 Invisible/Bookshelf 围栏、桌面时钟小部件和半透明任务栏。*

</div>

---

## ✨ 本版本的非凡之处

PecoFence 的每一层都经过重新设计，让你对桌面图标和 Windows 11 外壳本身拥有完全的创意掌控——同时 CPU 和 GPU 占用近乎为零。

---

## 🎨 1. 次世代围栏样式与视觉融合

以四种独特视觉范式超越标准容器：
- **Fluent（经典）：** 干净的 Windows 11 Fluent 卡片，边框锐利。
- **Liquid Glass：** 实时折射玻璃，可对静态或动态视频壁纸进行采样。
- **Bookshelf（`رف الكتب`）[新]：** 将围栏变成有触感的立体书架，图标整齐排列在书架上。
- **Invisible Canvas（`لوحة غير مرئية`）[新]：** 完全隐藏围栏背景和边框——保留磁性吸附、排序规则和网格对齐的全部能力，图标仿佛自然悬浮在原始壁纸上。

### 🦎 变色龙图标融合与快捷方式清理
- **变色龙模式（`دمج الأيقونات مع الخلفية`）[新]：** 通过可自定义的**融合强度滑块（0%–100%）**动态去饱和并将图标融入壁纸，营造平静、无干扰的美感。
- **快捷方式箭头开关（`إظهار/إخفاء أسهم الاختصارات`）[新]：** 在设置中即时移除或恢复杂乱的 Windows 快捷方式箭头。
- **完整阿拉伯语本地化与 RTL 界面 [新]：** 原生阿拉伯语界面（`العربية`），无缝从右到左布局与即时语言切换。

<!-- TODO: ADD IMAGE 2 (Fence Styles & Chameleon Mode) -->
![Fence Styles Showcase](assets/fence-styles.png)
*> **[IMAGE PLACEHOLDER 2]:** 添加「常规」选项卡截图，展示 4 种主题样式（Fluent、Liquid Glass、Bookshelf、Invisible Canvas）和变色龙融合。*

---

## 🎬 2. 原生 H.264 视频壁纸与自适应时钟小部件

### 🎞️ 硬件加速视频壁纸引擎 [新]
- 直接从 PecoFence 将任意 **MP4 (H.264)** 视频设为丝般顺滑的动态桌面壁纸。
- 内置 *CloudConvert* 和 *FreeConvert* 快捷链接，轻松将任何视频转换为超轻量、GPU 友好的壁纸。

### 🕰️ 无边框桌面时钟小部件（`ويدجت الساعة المكتبية`）[新]
- 从 **20 款手工打造的响应式时钟/日期/星期设计**（10 款 Gemini 签名 + 10 款 Devin 签名）中选择，或选 `بدون` 完全关闭。
- **真正的无边框交互：** 无标题栏、无杂物。在小部件**内部任意位置**点击拖动即可移动，抓住**任意边缘/角落**即可平滑缩放并自动缩放字体。
- **自适应壁纸对比度（`تنسيق ألوان الويدجت تلقائياً لتناسب الخلفية`）[新]：** PecoFence 持续采样时钟小部件正后方的精确壁纸像素。无论壁纸是刺眼的纯白还是纯黑，小部件都会自动翻转文字、玻璃和强调的对比度，使时间和日期始终 100% 清晰可读。

<!-- TODO: ADD IMAGE 3 (Desktop Clock Widget & Video Wallpaper) -->
![Desktop Clock Widget Showcase](assets/clock-widgets.png)
*> **[IMAGE PLACEHOLDER 3]:** 添加一张桌面时钟小部件悬浮截图及其设置与自适应对比度开关。*

---

## 🪟 3. Windows 11 外壳与文件资源管理器玻璃深度定制

何必安装五个独立调教工具？PecoFence 一站式统一你的整个 Windows 11 美学。

### 🛠️ 即时实时外壳样式（Windhawk 驱动预设）[新]
即时切换主题——选择即应用，零重启：
- **任务栏样式（`نمط شريط المهام`）：***Translucent Taskbar、Window Glass、OS26 Liquid Glass（Clear/Dark Mac Dock & Taskbar 变体）、Plasma、Surface、Luminosity、Layer Mica UI、Fluid、Tinted Glass、Frosted Acrylic、Pills、Blob、Minecraft Hotbar、One UI 8.5 Dock* 等。
- **开始菜单样式（`نمط قائمة ابدأ`）：***Down Aero、Translucent Start Menu、No Recommended Section、Side by Side、Windows 10、Fluent 2 Inspired、Rose Pine、Metro 10 Minimal、Everblush、Sun Valley、Liquid Glass、Borderless、Command Center、Full Screen、Frosty Glass*。
- **通知中心样式（`نمط مركز الإشعارات`）：***Layer Mica UI、Translucent Shell、Matte、Unified、JumpLists 10、Better Control 11、Fluid、Liquid Glass、Borderless、Dense、Frosty Glass、OS26 Tahoe Glass、Command Center*。

<!-- TODO: ADD IMAGE 4 (Taskbar, Start Menu & Notification Center Themes) -->
![Windows 11 Shell Customization](assets/windows-shell-customization.png)
*> **[IMAGE PLACEHOLDER 4]:** 添加自定义开始菜单、通知中心和任务栏主题下拉菜单实际运行的截图。*

### 🔍 文件资源管理器玻璃工作室与统一应用透明化 [新]
- **专属文件资源管理器玻璃编辑器（`تأثير خلفية مستكشف الملفات`）：** 打开带有交互式**文件资源管理器实时预览（`معاينة مباشرة`）**和可选预览背景的**高级设置（`الإعدادات المتقدمة`）**工作室！
- 自定义 **Blur、Acrylic、Mica、Mica Alt、Clear Blur**，切换地址栏/滚动条/WinUI 工具栏透明度，并微调自定义 `#AARRGGBB` 浅色与深色混合色调及不透明度。
- **真正的 DComp 应用镜像：** PecoFence 自身窗口（含无边框标题栏和客户区）以智能 `-70` 不透明度偏移实时镜像你的资源管理器玻璃材质——整个应用获得真正的透视硬件玻璃，无杂色覆盖。
- **一键外观刷新（`تحديث المظهر`）[新]：** 即时重启 `explorer.exe` 并干净地重新挂钩所有桌面围栏和活动视频壁纸。

<!-- TODO: ADD IMAGE 5 (File Explorer Glass Studio & Live Preview) -->
![File Explorer Glass Editor](assets/explorer-glass-editor.png)
*> **[IMAGE PLACEHOLDER 5]:** 添加文件资源管理器玻璃高级设置窗口的截图，展示交互式实时预览和自定义 RGBA 控件。*

---

## 📐 4. 围栏级精细定制与 3D 透视书架

在**围栏（`الأسوار`）**选项卡中，每个围栏都能精确到像素调整：

- **克隆围栏设置（`نسخ الإعدادات من سياج آخر`）[新]：** 设计好完美围栏？一键将其整体视觉外观和布局设置复制到任意其他围栏——不覆盖目标围栏的名称、内容或排序规则。
- **标题栏可见性（`إظهار شريط العنوان`）：** 在*始终*和*悬停时*之外新增**「永不显示」（`عدم الإظهار أبداً`）[新]**，实现完全无标题外观。
- **自定义图标框架与调色板（`إطارات الأيقونات`）[新]：** 用**透明（`شفاف`）**或**全彩（`ملون بالكامل`）**圆角卡片包裹单个图标，**文字颜色（`لون النص`）**和**框架颜色（`لون الإطار`）**各配专用 11 色选择器。
- **贴边图标模式（`شكل الأيقونة: ملتصق`）[新]：** 将图标裁剪为 `2:3` 边到边海报——游戏库和媒体围栏的完美之选！
- **精密像素滑块 [新]：**
  - **自定义图标大小（`حجم أيقونة مخصص`）：** `16px` 到 `512px` 自由滑块。
  - **列间距（`تباعد الأعمدة`）与行间距（`تباعد الصفوف`）：** 可缩至 `0px` 让图标无缝贴合，或拉大到 `48px+`。
- **滚动条控制 [新]：** 独立切换**隐藏滚动条（`إخفاء شريط التمرير`）**（鼠标滚轮滚动保持有效），或启用**锁定滚动（`قفل التمرير`）**将内容固定在顶部用于展示布局。
- **3D 倾斜书架模式与透视缩放（`وضع الرف المائل` + `معامل التكبير`）[新]：** 将围栏倾斜成 3D 透视平行四边形，与带角度的 3D 房间壁纸对齐！调整**透视缩放系数（`معامل التكبير`）**让图标向书架远端平滑缩小，营造真实纵深感。

<!-- TODO: ADD IMAGE 6 (Per-Fence Customization, Icon Frames & Tilted Shelf) -->
![Advanced Fence Layout & 3D Shelf](assets/fence-advanced-layout.png)
*> **[IMAGE PLACEHOLDER 6]:** 添加「围栏」选项卡截图，展示克隆设置、图标框架与颜色选择器、贴边 2:3 模式、间距滑块和倾斜书架模式。*

---

## ⚡ 5. 颜色编码排序引擎与强制应用

**排序规则（`قواعد الفرز`）**引擎已为可靠性和视觉清晰度完全重建：
- **颜色编码围栏徽章 [新]：** 每个围栏在规则列表中获得独特的颜色徽章，一眼即可识别哪条规则通往哪个围栏。
- **立即应用 vs 强制应用（`تطبيق إجباري على كل العناصر`）[新]：**
  - **立即应用（`تطبيق الآن`）：** 温和地引导未整理的桌面项目，同时尊重你手动放置的条目。
  - **强制应用（`تطبيق إجباري`）：** 覆盖所有手动放置，严格按照活动规则对桌面每一项进行无情重排。
- **一键快速预设（`إضافة سريعة`）：** 立即为*图片、音乐、视频、压缩包、安装程序*或*桌面清理*（仅扫走 30 天以上未动的安装程序和压缩包，不删除任何文件）生成预建围栏和规则。

<!-- TODO: ADD IMAGE 7 (Color-Coded Sorting Rules & Force Apply) -->
![Sorting Rules Engine](assets/sorting-rules-colored.png)
*> **[IMAGE PLACEHOLDER 7]:** 添加排序规则选项卡截图，突出颜色编码围栏徽章和橙色强制应用按钮。*

---

## 📦 6. `.peco` 完整主题包与智能壁纸适配

忘掉基础的 `.json` 布局备份——**布局与备份（`التخطيط والنسخ الاحتياطية`）**选项卡现在是完整的**可视化主题库（`معرض الثيمات`）**：
- **一体化 `.peco` 主题存档 [新]：** 保存主题将**一切**捕获进便携式 `.peco` 捆绑包——围栏精确坐标、围栏级样式、自定义图标大小、排序规则、玻璃设置，以及你的静态或动态视频壁纸，并附带自动桌面预览缩略图！
- **一键应用、导出与导入：** 数秒内在整套桌面配置间切换，或导出 `.peco` 与他人分享你的桌面主题。
- **智能宽高比壁纸适配（`ضبط الصورة لتظهر بالكامل مهما كانت أبعاد الشاشة`）[新]：** 壁纸与显示器宽高比不符？开启后可将完整未裁剪图像居中显示，边缘以氛围感模糊背景填充——关闭则为标准边到边裁剪填充。
- **每日自动保存（`حفظ تحديث الثيم تلقائياً كل يوم`）[新]：** 每天自动就地更新你的活动 `.peco` 主题文件——最新桌面调整始终保存且无重复。

<!-- TODO: ADD IMAGE 8 (Theme Gallery & .peco Packages) -->
![Theme Gallery Showcase](assets/themes-gallery.png)
*> **[IMAGE PLACEHOLDER 8]:** 添加主题库截图，展示带应用/导出 .peco/删除按钮的可视化主题卡片和智能壁纸适配开关。*

---

## 🚀 开始使用

1. 从 **Releases** 页面下载最新的 `setup.exe` 或 `portable.zip`。
2. 从系统托盘启动 **PecoFence** 并打开**设置（`إعدادات PecoFence`）**。
3. 选择**主题样式**（*Fluent、Liquid Glass、Bookshelf 或 Invisible Canvas*），启用**桌面时钟小部件**，或从**主题库**导入 `.peco` 主题——见证桌面即刻焕然一新！
