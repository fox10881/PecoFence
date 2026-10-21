https://github.com/user-attachments/assets/c827f059-cfd7-4f6a-bed3-8b00411a7220

<p align="center">
  <strong>免费、开源的 Windows 11 桌面整理工具，可以替代 Stardock Fences。</strong><br>
  用玻璃栅栏收好文件，用标签页切换项目，还能让 AI 助手通过内置 CLI，直接调整布局、外观和自动整理规则。
</p>

<p align="center">
  <a href="https://pecofence.jiang.jp/zh-CN/"><strong>官网</strong></a>
  &nbsp;·&nbsp; <a href="#开始使用"><strong>下载使用 →</strong></a>
  &nbsp;·&nbsp; <a href="#桌面怎么配告诉-ai-就好"><strong>AI + CLI</strong></a>
  &nbsp;·&nbsp; <a href="#看看它怎么用">看看实际操作</a>
  &nbsp;·&nbsp; <a href="../README.md">项目文档</a>
</p>

<p align="center">
  <a href="../../README.md">English</a>
  &nbsp;·&nbsp; <strong>简体中文</strong>
  &nbsp;·&nbsp; <a href="README.zh-TW.md">繁體中文</a>
  &nbsp;·&nbsp; <a href="README.ja.md">日本語</a>
  &nbsp;·&nbsp; <a href="README.ko.md">한국어</a>
  &nbsp;·&nbsp; <a href="README.de.md">Deutsch</a>
  &nbsp;·&nbsp; <a href="README.fr.md">Français</a>
  &nbsp;·&nbsp; <a href="README.es.md">Español</a>
  &nbsp;·&nbsp; <a href="README.pt-BR.md">Português (Brasil)</a>
  &nbsp;·&nbsp; <a href="README.ru.md">Русский</a>
  &nbsp;·&nbsp; <a href="README.ar.md">العربية</a>
</p>

---

## 桌面静态海报书架整合：终极形态

![桌面静态海报书架整合](../assets/desktop-poster-shelves.png)

### 🧩 最后一块拼图：BoxArt Studio
PecoFence 提供了整齐的网格布局和零间距的 2:3 海报容器，但 Windows 原生生成的仍是标准方形图标。
要实现截图中的实体游戏盒书架效果，你需要 **[BoxArt Studio](https://github.com/fox10881/BoxArt-Studio)**。BoxArt Studio 是必备的搭配工具，可以把普通的 Windows 游戏快捷方式和文件夹批量转换成精美的高分辨率 2:3 实体游戏盒封面。

### 📸 便携布局快照

布局快照现在是可以完全带走的备份:直接在快照列表里重命名存档文件,还可以把当前桌面壁纸一并打进存档。这样在任何一台电脑上还原布局都分毫不差——哪怕那台电脑从未见过你的壁纸。

### 🧊 3D 倾斜层架(实验性)

把任意栅栏剪切成一个完美的平行四边形,让每一行都能贴合斜视角壁纸里的层板:拖动侧边上缘的竖向手柄倾斜栅栏的一侧,图标会朝更低、更近的一侧逐级缩放——可在栅栏设置中通过**纵深缩放**滑杆调节。

> **⚠️ 已知问题——求助。** 平行四边形内的鼠标移动、对齐和命中测试都完全正常。但使用 3D 透视壁纸时,第一行能与层板完美对齐,后续各行却因为壁纸的灭点透视而对不上。如果哪位开发者知道如何完美解决这种多行透视数学问题,我们非常欢迎 PR!

### 🖼️ 现成的 2D 壁纸

仓库自带一组正面平视的书架壁纸,位于 [`assets/wallpapers/`](../../assets/wallpapers/),与未倾斜的平直栅栏完美对齐——选一张,摆好层架,完成。

> **💡 小贴士:** 我们提供了一套与平直栅栏完美对齐的现成 2D 壁纸。想要惊艳的效果,强烈建议把这些 2D 图片交给 AI 视频生成器做成动画,再用 [Lively Wallpaper](https://github.com/rocksdanister/lively) 应用到桌面。

### 🎛️ 新主题与逐栅栏精细控制

设置新增两种风格:**书架**——经典木质书架,游戏封面直接坐落在木板上——以及**隐形画布**,完全透明的主题让图标悬浮在壁纸之上。每个栅栏的页面还可以用**从不显示**彻底隐藏**标题栏**、关闭**图标标签**,并通过**自定义图标大小**、**水平间距**、**垂直间距**滑杆精确调整网格;再加上**隐藏滚动条**、**锁定滚动**、**自动高度**开关,打造干净静态的展示柜。

<a id="让-ai-帮你整理"></a>

## 桌面怎么配，告诉 AI 就好

**内置 CLI，让 AI 直接配置桌面**

把你想要的桌面告诉 AI 助手。PecoFence 自带 `pecofence-cli`，Claude Code、Codex、Cursor 可以读取当前配置，并直接在 PecoFence 中执行修改。

- **说出需求，直接改配置。** 切换主题，调整透明度、图标大小和全局设置，也能一次改好所有栅栏。
- **整理一次，以后自动归位。** 创建项目栅栏、整理图标，再添加规则，让新文件自动进入对应栅栏。
- **喜欢的配置，保存下来。** 用快照保存栅栏布局，用配置导入导出保存和恢复设置、规则与布局。

**现在就让你的 AI 助手试试**

启动 PecoFence，把下面这段话发给你的 AI 编程助手：

> 请用 pecofence-cli 帮我配置桌面。先阅读 pecofence-cli skill 和 pecofence-cli describe，再查看当前设置与栅栏。修改前先备份配置，然后切换为深色模式，并把所有栅栏调得更透明。

CLI 已随应用附带。Microsoft Store 版会把 `pecofence-cli` 加入 PATH；使用便携版时，告诉助手 `pecofence-cli.exe` 的完整路径即可。

<details>
<summary><strong>与编程助手的对话示例</strong></summary>

> 把桌面上的 PDF 放进 Docs 栅栏，以后的 PDF 也自动归进去，切成深色模式，再把栅栏调得更透明一点。

```powershell
pecofence-cli config export "$env:USERPROFILE\pecofence-before-ai.json"
pecofence-cli snapshot save before-cleanup
pecofence-cli fence create --title Docs
pecofence-cli rule add --name PDFs --ext pdf --to Docs --index 0
pecofence-cli rule apply
pecofence-cli settings set theme dark
pecofence-cli fence set --all opacity clear
```

</details>

为 AI 和脚本设计：`describe` 输出命令目录和 JSON Schema，`skill` 输出使用指南。每次执行都用 JSON 说明改了什么；出错时返回结构化错误，助手可以据此决定下一步。

[查看 CLI 上手指南 →](../CLI.md#start-with-your-ai-agent)

## 给每件事留一个位置

<p align="center">
  <img src="../assets/hero-zh-CN.png" alt="PecoFence — 项目文件夹、PDF 和设计海报，收在 Liquid Glass 栅栏里。" width="1280">
</p>

## 看看它怎么用

### 一个栅栏，切换多个项目

把相关栅栏合成标签页，点一下就能从 Project 切到 Ideas。
需要同时看两组文件时，把标签页拖出来，就变回两个独立栅栏。

![把两个栅栏合成标签页，在 Project 和 Ideas 之间切换，再把标签拖出来，拆回独立栅栏。](../assets/tabs.gif)

### 文件就在当前应用前面

按 **Ctrl + Alt + 空格** 速览栅栏，所有栅栏显示在当前应用上方。
取用需要的文件后，按 **Esc** 回去接着工作。

![用快捷键速览栅栏，栅栏显示在应用上方；点一下栅栏外面或打开文件，就回到应用。](../assets/peek.gif)

<sub>动图来自<a href="https://pecofence.jiang.jp/zh-CN/manual/">使用手册</a>，会自动循环播放。</sub>

## 日常好用，藏在这些细节里

| 体验 | 能做什么 |
| :--- | :--- |
| **AI + CLI** | `pecofence-cli` — **说出需求，直接改配置。** 切换主题，调整透明度、图标大小和全局设置，也能一次改好所有栅栏。 |
| **少一点手动整理** | 按类型、扩展名、名称、通配符、快捷方式目标、时间和大小设置规则，新文件自动找到位置。 |
| **配得上你的壁纸** | Fluent、Liquid Glass、书架、隐形画布四种风格,支持深浅色;每个栅栏可单独设置色调和不透明度,还能给图标着色。 |
| **每个栅栏随你调** | 按栅栏设置标题栏显示(直至**从不显示**)、可关的图标标签、图标大小与行/列间距滑杆,以及滚动条、滚动锁定和自动高度开关。 |
| **熟悉的文件操作** | 资源管理器右键菜单、拖放、复制粘贴、多选、缩略图，以及图标／列表／详细信息视图。 |
| **把文件夹放在手边** | 把真实文件夹变成桌面上的窗口；可以进入子文件夹，内容变化自动同步。 |
| **用时展开，闲时收好** | 把栅栏收起成一条标题栏，鼠标悬停即可展开；也可以锁定已经摆好的位置和大小。双击桌面空白处隐藏全部栅栏，再次双击恢复。 |
| **喜欢的布局，留得住** | 保存布局快照、每日自动备份、导入导出配置、交换两个显示器上的栅栏。 |
| **3D 倾斜层架(实验性)** | 把栅栏剪切成平行四边形,让各行贴合斜视角壁纸的层板;图标朝更近的一侧逐级缩放。 |
| **轻巧地待在桌面上** | Rust 编写的原生应用，空闲时内存占用约 40 MB（任务管理器显示）。WebView2 设置面板按需加载。 |

自动整理只改变文件所属的栅栏，保留文件原来的位置。
你主动发起的移动、重命名和删除，则像资源管理器一样操作真实文件。

[查看完整功能清单 →](../FEATURES.md)

## 开始使用

<a href="https://apps.microsoft.com/detail/9MV6WG3XNWSX?mode=direct"><img src="https://get.microsoft.com/images/zh-cn%20dark.svg" alt="从 Microsoft Store 获取" width="200"></a>

Microsoft Store 版由微软签名，自动更新，不会出现 SmartScreen 提示。同一个程序也可以这样安装：

- **便携版 ZIP**：在本仓库的 **Releases** 页面下载 `pecofence-v<版本>-x64-portable.zip`，**完整解压**到一个文件夹，运行 `pecofence.exe`。
- **安装包**：同一页面上的 `pecofence-v<版本>-x64-setup.exe` 只为当前用户安装，会添加开始菜单项和卸载程序。
- **winget**：`winget install DayuanJiang.PecoFence` 安装的是便携版，不会触发 SmartScreen 提示。

<details>
<summary><strong>系统要求、配置位置与使用说明</strong></summary>

- 支持 Windows 10 22H2，以及 Windows 11 22H2 及以上版本。设置面板需要 Microsoft Edge WebView2 Runtime，Windows 11 已自带，Windows 10 如打不开设置请从微软官网安装。
- 首次运行会按所选语言创建“程序”“文件夹”“文件与文档”和“桌面”四个栅栏。退出程序时，Windows 桌面图标会恢复显示。
- 配置保存在 `%APPDATA%\PecoFence\config.json`。用 `--portable` 启动，可改为保存在程序旁的 `config` 文件夹。
- 便携版和安装包都没有代码签名。首次运行若出现 Windows SmartScreen 提示，点击**更多信息 → 仍要运行**。

[便携版说明](../PORTABLE.md) · [多语言说明](../LOCALIZATION.md) · [Windows 安装版说明](../INSTALLER.md)

</details>

## 开源，也欢迎你的想法

改进一句翻译、修好一次拖拽、让某个日常操作更顺手，都很欢迎。

[参与贡献](../../CONTRIBUTING.md) · [完善翻译](../LOCALIZATION.md) · [开发文档](../DEVELOPMENT.md)

---

**让每次回到桌面，都更舒服一点。**  
[Apache 2.0 许可证](../../LICENSE) · [第三方许可说明](../../third_party/README.md)

感谢 [LINUX DO](https://linux.do) 社区。
