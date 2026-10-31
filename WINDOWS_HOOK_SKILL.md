# PecoFence — Windows Hooking & XAML Injection Guardrails

Permanent architectural constitution for this repository. These rules are
non-negotiable; they encode hard-won failure modes of the `pecofence_tap.dll`
resident and the host-side injection layer. If a change appears to violate
one of them, stop and re-check the design before touching code.

## Rule 1 — Warmup Gate (`host_warm_enough`)

**Never call `InitializeXamlDiagnosticsEx` before the process-age gate
opens.**

- The first `IXDEx` call must not run until the host process is past its
  XAML bootstrap. For the shell explorer this is **30 s** (the taskbar's
  `DXamlCore` registers ~10–25 s after spawn).
- A failed first attempt (`0x80070490`) **permanently poisons** the
  process's diagnostics session — every retry fails for the life of that
  process. Observed: 80+ consecutive failures; every success ever logged
  was attempt-1 `S_OK`.
- Never `LoadLibraryW` a XAML framework DLL on a worker thread — only
  `GetModuleHandleW` on a module the host itself already loaded. Bootstrapping
  the framework on our thread produces the same poisoned state.
- Retries must ride the existing `WM_TIMER` cadence — a thread that has
  called `ixde` can never block again (stray framework APC hangs
  `Sleep`/`join`/waits).

## Rule 2 — Engine (wux)

**Taskbar and Start Menu are `Windows.UI.Xaml` surfaces. All styling goes
through the single `styler::wux` engine.**

- **Taskbar** — `Windows.UI.Xaml` (UWP XAML / wux) inside the *shell*
  `explorer.exe` (the `find_taskbar` process). Styled by `styler::wux`
  over `styler_bindings`, wire `PECO-TBS1` (`TBST`/`TBSR`).
- `bindings` and `styler_bindings` contain identically
  named COM types with **different IIDs** — they are not interchangeable.
  Share code through the `xb` facade in `styler.rs`; convert only through
  explicit `Interface::cast`/QI, never by type-name equality.
- `IXamlDiagnostics`/`IVisualTreeService*` are framework-agnostic —
  session plumbing is shared; element-level casts are not.

## Rule 3 — Lifecycle & Death-Watch (self-heal)

**Every injected resident MUST ride the `WM_TIMER` cadence and revert
cleanly when `pecofence.exe` exits.**

- The command window's `WM_TIMER` (~2 s) drives `site::ensure_watching`
  (diagnostics install + watcher advise). Never move this work to
  blocking loops on the ixde thread.
- The host-pid `HELLO` (`TBHI`/`SMHI`) arms the resident's death watch:
  when pecofence.exe dies, the resident restores all original property
  values, unadvises watchers, and unloads. Every new resident must
  receive the hello before it counts as armed.
- Armed intent is recorded host-side *before* delivery (`apply` stores
  style+wire, then delivers best-effort) — the anchor-tick `ensure`
  re-delivers to new residents (explorer restart) without the user
  re-selecting.
- `RestartExplorer` (تحديث) kills `explorer.exe`; everything must
  self-heal: shell residents re-inject via the anchor tick,
  theme wires re-deliver on `SetSite`, video wallpaper re-attaches via
  the wallpaper poll's `is_alive` check.

## Rule 5 — Unique Module Name (stale-resident eviction)

**Inject only the versioned copy `pecofence_tap_v<ver>_<filehash>.dll`,
never `pecofence_tap.dll` directly.**

- A resident image can be pinned inside its host for the process's whole
  life (IXDEx holds COM references; `FreeLibrary` can't evict it).
  Re-injecting the *same path* then dedups inside the target —
  `LoadLibrary` returns the stale module and the new code never runs.
  The only way a fresh resident co-loads is a module name the stale
  image doesn't own.
- The resident derives its window class from its own module filename
  (`pecofence_tap_v4_<h>.dll` → `PecoFenceTAP_v4_<h>`), so two builds can
  register command windows side by side — `RegisterClassExW` on an
  existing atom would fail, and a colliding `CreateWindowEx` would
  silently deliver the new resident's wires to the stale wndproc.
- Discovery must therefore be class-*prefix* matching
  (`resident_windows_matching`), ordered our-build-first then newest
  protocol — never `FindWindowW` on a fixed class name. A resident only
  counts as usable when its class equals `expected_resident_class`
  (version ≥ `TAP_PROTOCOL_VERSION` *and* our build's module name).
- A stale resident's `WM_TIMER` keeps re-stamping `PecoFence.TAP.CmdHwnd`
  with *its* HWND forever — the prop ping-pongs between builds. Always
  fall back to enumeration when the stamped prop fails `resident_usable`;
  never trust the prop alone.
- On discovering our resident, `unload_foreign_residents` sends
  `PecoFence.TAP.Unload` to every other resident window in the process —
  one resident applies styles per process.

## Rule 4 — No External Dependencies

**PecoFence is 100% standalone.**

- Never call into, bridge to, or depend on Windhawk, TranslucentTB,
  ExplorerPatcher, StartAllBack, or any third-party mod — including for
  "temporary" fixes. Windhawk theme definitions are a *data source*
  (imported into `crates/core` catalogs), never a runtime dependency.
- No symbol/PDB downloads, no function detours — backdrop and frame
  changes go through supported APIs (`DwmSetWindowAttribute`,
  `DwmExtendFrameIntoClientArea`, direct per-window application on the
  timer). `explorerFrameContainerHeight` remains provenance-only for
  this reason.
- The only privileged integration is the `WH_CALLWNDPROC` map-in plus
  the documented XAML Diagnostics/COM surface.

---

### Quick map

| Surface | Process | Framework | Engine | Wire | Session |
|---|---|---|---|---|---|
| Taskbar | shell `explorer.exe` | `Windows.UI.Xaml` | `styler::wux` | `PECO-TBS1` (`TBST`/`TBSR`) | `Windows.UI.Xaml.dll` ixde |
| Start Menu | `StartMenuExperienceHost.exe` | `Windows.UI.Xaml` | `styler::wux` | `PECO-SMS1` (`SMST`/`SMSR`) | same, `PecoFenceTAPSM` class |

Resident protocol version: `TAP_PROTOCOL_VERSION` — kept in sync between
`crates/tap/src/command.rs` and `crates/platform/src/tap.rs`; bump on any
wire/marker change so pinned old residents read as stale.
