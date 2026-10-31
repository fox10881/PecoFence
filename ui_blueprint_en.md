# PecoFence UI Blueprint

> Complete hierarchy of every UI surface: settings window, tray menu, context menus, dialogs. Control types in parentheses.


## Settings Window

  - **Settings navigation (left sidebar)**
    - Brand mark: PecoFence icon + name
    - General
    - Fences
    - Sorting rules
    - Layout and backups
    - About
    - Connecting to desktop… (workspace status summary)

### 📄 General

    - Theme style
        - *Options: Fluent acrylic · Liquid Glass · Classic wooden bookshelf · Fully transparent canvas*
    - Set video as desktop wallpaper (H.264 MP4 only; CloudConvert/FreeConvert links)
      - Open
    - Color mode
        - *Options: Follow Windows mode · Follow app mode · Light · Dark*
    - Text color (Auto: Liquid Glass picks from wallpaper brightness)
        - *Options: Auto · Per-fence auto · White · Black*
    - Title alignment
        - *Options: Left · Center · Right*
    - Tab bar position
        - *Options: Top · Left*
    - Default icon size (used for new fences)
    - Icon tint
        - *Options: None · Blue · Green · Orange · Red · Purple · Gray*
    - Icon tint strength
        - *Options: Weak · Medium · Strong*
    - Blend icons with background (Chameleon)
      - Blend intensity
    - Show shortcut arrows
    - Live wallpaper glass capture
    - Display language
        - *Options: Follow system + 12 languages*
    - **Desktop & startup**
      - Start with Windows
      - Hide Windows desktop icons
        - Hide "Desktop" fence while Windows icons are shown
      - Double-click empty desktop to hide/show fences
      - Offer "New fence here" when dragging a rectangle on empty desktop
      - When showing desktop (Win+D)
          - *Options: Keep fences visible · Hide with desktop*
      - Taskbar style
      - Start Menu style
      - Notification Center style
      - File Explorer style
      - Accent color overlay
          - *Overlay the chosen color on taskbar, Start menu and File Explorer; slash clears it*
      - Restart Explorer
    - **Behavior**
      - Auto-expand rolled-up fences on hover
      - Rolled-up fences expand on title click
      - Push fences below when expanding
      - Show title bar on hover only
      - Hide item count and collapse chevron on title bar right
      - Switch tabs on hover
      - Auto-hide scrollbar
    - **Snapping & alignment**
      - Snap-align when moving fences
        - Show alignment guides
      - Fence spacing
          - *Options: 0 / 4 / 8 / 12 / 16 / 24 / 32 px*
      - Snap to whole rows/columns when resizing
    - **Peek fences**
      - Hotkey brings all fences in front of windows
        - Hotkey
        - Include Win key
        - Preset combos
      - Dim background while peeking

### 📄 Fences

    - Fence picker (dropdown + color dot + count badge)
    - Empty-state card "No fences on the desktop yet…"
    - Fence name
    - Copy settings from another fence
    - **Appearance**
      - Opacity
          - *Options: Fully transparent · Clearer · Default · Solider*
      - Tint
      - Tint title bar only
      - Tab color
      - Title color
          - *Options: Follow theme · Follow tint · White · Black*
      - Title size
          - *Options: Small · Normal · Large · Extra large*
      - Show title bar
          - *Options: Follow general setting · Always · On hover · Never*
      - Show icon labels
      - Icon label display mode
          - *Options: Follow "show icon labels" toggle · Always · On hover · Never*
      - Label font
      - Label weight
          - *Options: Regular · Bold*
      - Icon frames
          - *Options: Off · Translucent · Solid color*
      - Text color
      - Frame color
    - **Layout**
      - Icon size
      - Icon spacing
          - *Options: Compact · Normal · Loose*
      - Icon shape
          - *Options: Standard · Poster (2:3 cover)*
      - Custom icon size
      - Horizontal gap
      - Vertical gap
      - Hide scrollbar
      - Lock scrolling
      - 3D tilted shelf
      - Depth scaling
      - Auto height
    - **Behavior**
      - Lock position and size
      - Keep this fence visible on double-click hide
      - Auto-hide when empty
      - Dock to top of screen
    - **Portal folder (portal fences only)**
      - Open subfolders inside this fence on double-click
      - Show folder icon in title bar

### 📄 Sorting rules

    - Auto-classify new items appearing on the desktop
    - Items that match no rule
        - *Options: Back to "Desktop" fence · Stay in current fence*
    - Apply rules to all desktop items now
    - Force-apply to every item
    - **Quick add**
      - Create a fence + rule in one click
          - *Options: Images · Music · Video · Archives · Installers · Cleanup*
    - **Rule list**
      - Dynamic: rule cards (index + name + condition summary + target fence badge)
          - *Per-row: move up · move down · enable toggle · delete*
    - **New rule (collapsible panel)**
      - Rule name
      - Target fence
      - Condition type
          - *Options: File type · Extension · Name contains · Name excludes · Name starts with · Name ends with · Full filename · Wildcard · Shortcut target contains · Created time range · Created weekday · Size range (MB) · Idle days · Folders only · Files only · Plain files*
      - Condition value
      - Type checkboxes (programs/shortcuts/folders/documents/images/music/video/archives/installers)
      - From / To time inputs
      - Weekday checkboxes (Mon…Sun)
      - Idle at least N days
      - At least / At most MB
      - Add rule

### 📄 Layout and backups

    - **Theme gallery**
      - Theme name (optional)
      - Save current configuration
      - Import .peco theme
      - Open themes folder
      - Dynamic: theme card grid (preview thumbnail + name)
          - *Per-card: Apply · Export .peco · Delete*
    - Always show full background regardless of aspect ratio
    - Auto-update the current theme daily
    - **Multi-monitor**
      - Swap fences between two monitors
    - Config file (path display)

### 📄 About

    - App info card (icon + PecoFence version + description + Apache 2.0 license + memory usage)
    - Update
    - User manual (animated demo per feature)
    - Enjoying PecoFence?
    - **Feedback**
      - Crash-recovery note (shown after unexpected shutdown)
      - Type
      - Message
      - Email (optional, for replies)
      - Attach diagnostic log (no personal info)
      - Preview what will be sent
      - Send
    - **Troubleshooting**
      - Windows desktop icons (status + restore / re-hide)
  - **Modal dialog**
    - Unsaved changes: prompt before switching themes
        - *Buttons: Save & switch · Don't save · Cancel*
  - **Transient toast bar**
    - Instant operation-result toast

## System Tray Icon Menu

  - Show all fences / Hide all fences
  - New fence
  - Show folder on desktop
  - Peek all fences (with hotkey)
  - Apply organization rules now
  - Restore desktop icons / Re-hide them (state-dependent)
  - Repair desktop icons (when they vanish)
  - Send feedback…
  - Settings…
  - Exit PecoFence

## Fence Right-Click Menu

  - Expand / Collapse
  - Rename…
  - Open folder in Explorer (portal fence)
  - Go up / Back to original folder (portal fence)
  - View ▸
      - *Options: Icons · List · Details · Small icons · Medium icons · Large icons · Extra large icons*
  - Sort by ▸
      - *Options: Manual · By name · By type · By date modified · By size · By open count · Reverse order · Group by time*
  - New ▸
      - *Options: Folder · Text document · Tab*
  - Paste
  - Tab ▸
      - *Options: Move up · Move down · Move left · Move right · Detach to own fence · Merge into…*
  - Unlock / Lock position and size
  - Fence options…
  - New fence
  - Show folder on desktop
  - Auto-hide when empty
  - Delete fence

## Item Right-Click Menu (inside a fence)

  - Open
  - Open file location
  - Rename
  - Delete
  - Show this folder on desktop (folder item)
  - Move to fence ▸ (per-fence submenu + move out to desktop)
  - Properties

## Desktop Drag-Rectangle Menu

  - New fence here

## List-View Column Header Menu (portal fence)

  - Name · Date modified · Type · Size (checkable columns)
  - Restore default column widths
