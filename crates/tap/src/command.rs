//! Command channel: a message-only window inside Explorer that the host
//! process `PostMessage`s state changes to — the lightweight replacement for
//! TranslucentTB's COM `TaskbarAppearanceService` RPC. Also owns the orderly
//! `WM_TAP_UNLOAD` teardown and the host-death fallback
//! (`RestoreAllTaskbarsToDefaultWhenProcessDies`).

use crate::bindings::HWND;
use crate::shared;
use crate::win32::*;
use std::sync::atomic::{AtomicU32, Ordering};

/// Class names are derived from this resident's own module filename so two
/// different builds can co-exist in one host process: the host injects a
/// unique-per-build copy (`pecofence_tap_v<ver>_<hash>.dll`), and if a stale
/// image is still mapped it already owns `PecoFenceTAP` — `RegisterClassExW`
/// on an existing atom fails, and a colliding `CreateWindowEx` would deliver
/// the new resident's wires to the stale resident's wndproc instead.
/// `pecofence_tap.dll` → `PecoFenceTAP`; `pecofence_tap_v4_<h>.dll` →
/// `PecoFenceTAP_v4_<h>` (SMEH variant: `PecoFenceTAPSM…`; Notification
/// Center hosts: `PecoFenceTAPNC…`).
fn window_class() -> &'static [u16] {
    static CLASS: std::sync::OnceLock<Vec<u16>> = std::sync::OnceLock::new();
    CLASS
        .get_or_init(|| {
            let mut buf = [0u16; 260];
            let len = unsafe {
                GetModuleFileNameW(
                    crate::dll_instance(),
                    windows_core::PWSTR(buf.as_mut_ptr()),
                    buf.len() as u32,
                )
            };
            let stem = if len > 0 {
                let path = String::from_utf16_lossy(&buf[..len as usize]);
                let file = path.rsplit(['\\', '/']).next().unwrap_or_default();
                file.strip_suffix(".dll").unwrap_or(file).to_owned()
            } else {
                "pecofence_tap".to_owned()
            };
            let suffix = stem.strip_prefix("pecofence_tap").unwrap_or("");
            let base = if crate::is_smeh() {
                "PecoFenceTAPSM"
            } else if crate::is_nch() {
                "PecoFenceTAPNC"
            } else {
                "PecoFenceTAP"
            };
            format!("{base}{suffix}\0").encode_utf16().collect()
        })
        .as_slice()
}
const UNLOAD_NAME: &[u16] = &[
    0x50, 0x65, 0x63, 0x6f, 0x46, 0x65, 0x6e, 0x63, 0x65, 0x2e, 0x54, 0x41, 0x50, 0x2e, 0x55, 0x6e,
    0x6c, 0x6f, 0x61, 0x64, 0x00, // "PecoFence.TAP.Unload"
];

static UNLOAD_MSG: AtomicU32 = AtomicU32::new(0);

/// `true` once `WM_TAP_UNLOAD` ran — a stale pinned resident must go fully
/// inert: no retry ticks, no `CmdHwnd` re-stamps, no diagnostics re-advise,
/// no wires, no death watch. Without this the stale build keeps fighting
/// the live resident for the prop and the diagnostics channel every 2s.
static UNLOADED: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);

pub(crate) fn is_unloaded() -> bool {
    UNLOADED.load(Ordering::Acquire)
}

/// Command-protocol build stamp — set as a window property
/// (`PecoFence.TAP.Build`) so the host can tell a stale resident apart:
/// `InitializeXamlDiagnosticsEx` pins the DLL in the host process, so an
/// older image survives unload/rebuild cycles with its older protocol.
/// Bump whenever the command surface changes incompatibly.
/// `1` = transparency + Start Menu styler; `2` = + taskbar styler
/// (`PECO-TBS1` COPYDATA markers); `3` = legacy accent command surface
/// removed (`PecoFence.TAP.Command` no longer exists — a v2 resident can
/// still carry an armed accent detour, so it must read as stale);
/// `4` = + File Explorer styler (`PECO-FES1` markers + WinUI3 session);
/// `5` = File Explorer styler removed (`PECO-FES1` markers + the WinUI3
/// diagnostics session gone — a v4 resident still carries the whole mux
/// machinery, so it must read as stale and go inert);
/// `6` = + Notification Center styler (`PECO-NCS1` markers +
/// `PecoFenceTAPNC` class in `ShellExperienceHost.exe`/`ShellHost.exe` —
/// a v5 resident can't register there, so it must read as stale);
/// `7` = + File Explorer styler restored (`PECO-FES1` markers + a second
/// diagnostics session on `Microsoft.Internal.FrameworkUdk.dll` for the
/// CabinetWClass WinUI3 islands — a v6 resident has no FEST handler, so
/// it must read as stale).
/// Keep in sync with `crates/platform/src/tap.rs::TAP_PROTOCOL_VERSION`.
pub const TAP_PROTOCOL_VERSION: usize = 7;
const BUILD_PROP_NAME: &[u16] = &[
    0x50, 0x65, 0x63, 0x6f, 0x46, 0x65, 0x6e, 0x63, 0x65, 0x2e, 0x54, 0x41, 0x50, 0x2e, 0x42, 0x75,
    0x69, 0x6c, 0x64, 0x00, // "PecoFence.TAP.Build"
];

pub fn run() {
    unsafe {
        UNLOAD_MSG.store(
            RegisterWindowMessageW(pcwstr(UNLOAD_NAME)),
            Ordering::Release,
        );

        let class_name = window_class();
        let class = WNDCLASSEXW {
            cbSize: core::mem::size_of::<WNDCLASSEXW>() as u32,
            style: 0,
            lpfnWndProc: Some(wndproc),
            cbClsExtra: 0,
            cbWndExtra: 0,
            hInstance: crate::dll_instance(),
            hIcon: core::ptr::null_mut(),
            hCursor: core::ptr::null_mut(),
            hbrBackground: core::ptr::null_mut(),
            lpszMenuName: windows_core::PCWSTR::null(),
            lpszClassName: pcwstr(class_name),
            hIconSm: core::ptr::null_mut(),
        };
        if RegisterClassExW(&class) == 0 {
            return;
        }
        let hwnd = CreateWindowExW(
            0,
            pcwstr(class_name),
            windows_core::PCWSTR::null(),
            0,
            0,
            0,
            0,
            0,
            HWND_MESSAGE,
            core::ptr::null_mut(),
            crate::dll_instance(),
            core::ptr::null(),
        );
        if hwnd.0.is_null() {
            return;
        }
        // Stamp the protocol version — the host refuses to talk to a stale
        // resident instead of letting its messages drop silently.
        let _ = SetPropW(
            hwnd,
            pcwstr(BUILD_PROP_NAME),
            TAP_PROTOCOL_VERSION as *mut core::ffi::c_void,
        );
        shared::lock().command_window = hwnd;
        crate::tap_log!("command window ready hwnd={:?}", hwnd.0);
        // Message-only windows don't appear in EnumWindows, so a per-process
        // resident can't be found by class enumeration from the host. Stamp
        // the command HWND on this process's enumerable anchor window —
        // `Shell_TrayWnd` (shell explorer's taskbar) — so the host reads it
        // back with GetPropW instead of guessing among same-class residents.
        stamp_host_props(hwnd);
        // The IXDEx retry cadence: a fresh Explorer has no XAML island yet,
        // so early install attempts fail until the taskbar comes up. `site::
        // ensure_watching` is a cheap no-op once the watcher is advised.
        SetTimer(hwnd, RETRY_TIMER_ID, RETRY_TIMER_MS, core::ptr::null());

        let mut msg = core::mem::zeroed::<MSG>();
        while GetMessageW(&mut msg, HWND(core::ptr::null_mut()), 0, 0).0 > 0 {
            let _ = TranslateMessage(&msg);
            DispatchMessageW(&msg);
        }
        shared::lock().command_window = HWND(core::ptr::null_mut());
    }
}

/// `GetPropW` name the host reads off `Shell_TrayWnd` to discover this
/// process's command window — see `stamp_host_props`.
pub const CMD_PROP_NAME: &[u16] = &[
    0x50, 0x65, 0x63, 0x6f, 0x46, 0x65, 0x6e, 0x63, 0x65, 0x2e, 0x54, 0x41, 0x50, 0x2e, 0x43, 0x6d,
    0x64, 0x48, 0x77, 0x6e, 0x64, 0x00, // "PecoFence.TAP.CmdHwnd"
];

fn stamp_host_props(cmd: HWND) {
    unsafe extern "system" fn cb(hwnd: HWND, lparam: LPARAM) -> windows_core::BOOL {
        let _ = std::panic::catch_unwind(core::panic::AssertUnwindSafe(|| {
            let mut pid = 0u32;
            unsafe {
                let _ = crate::win32::GetWindowThreadProcessId(hwnd, &mut pid);
            }
            if pid != unsafe { crate::win32::GetCurrentProcessId() } {
                return;
            }
            let mut cls = [0u16; 64];
            let n = unsafe {
                crate::win32::GetClassNameW(hwnd, windows_core::PWSTR(cls.as_mut_ptr()), 64)
            };
            if n > 0 {
                let name = String::from_utf16_lossy(&cls[..n as usize]);
                if name == "Shell_TrayWnd" {
                    unsafe {
                        let _ = crate::win32::SetPropW(
                            hwnd,
                            windows_core::PCWSTR(CMD_PROP_NAME.as_ptr()),
                            lparam as *mut core::ffi::c_void,
                        );
                    }
                }
            }
        }));
        windows_core::BOOL(1)
    }
    unsafe {
        let _ = crate::win32::EnumWindows(Some(cb), cmd.0 as isize);
    }
}

/// The unload counterpart to `stamp_host_props`: drop `CmdHwnd` where it
/// still names *this* command window. A co-loaded live resident's stamp is
/// left untouched — and its next timer tick re-stamps anyway. Without this,
/// the prop can keep pointing at an inert resident until the live one's
/// cadence wins.
fn unstamp_host_props(cmd: HWND) {
    unsafe extern "system" fn cb(hwnd: HWND, lparam: LPARAM) -> windows_core::BOOL {
        let _ = std::panic::catch_unwind(core::panic::AssertUnwindSafe(|| {
            let mut pid = 0u32;
            unsafe {
                let _ = crate::win32::GetWindowThreadProcessId(hwnd, &mut pid);
            }
            if pid != unsafe { crate::win32::GetCurrentProcessId() } {
                return;
            }
            let name = windows_core::PCWSTR(CMD_PROP_NAME.as_ptr());
            let cur = unsafe { crate::win32::GetPropW(hwnd, name) };
            if cur as isize == lparam {
                unsafe {
                    let _ = crate::win32::RemovePropW(hwnd, name);
                }
            }
        }));
        windows_core::BOOL(1)
    }
    unsafe {
        let _ = crate::win32::EnumWindows(Some(cb), cmd.0 as isize);
    }
}

fn pcwstr(s: &'static [u16]) -> windows_core::PCWSTR {
    windows_core::PCWSTR(s.as_ptr())
}

unsafe extern "system" fn wndproc(hwnd: HWND, msg: u32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    let _ = std::panic::catch_unwind(core::panic::AssertUnwindSafe(|| {
        handle_message(hwnd, msg, wparam, lparam)
    }));
    unsafe { DefWindowProcW(hwnd, msg, wparam, lparam) }
}

/// `COPYDATASTRUCT` — the host delivers the serialized Start Menu theme
/// (`WM_COPYDATA`, UTF-8 wire text) to the command window.
#[repr(C)]
struct CopyDataStruct {
    dw_data: usize,
    cb_data: u32,
    lp_data: *const core::ffi::c_void,
}

const WM_COPYDATA: u32 = 0x004A;
const WM_TIMER: u32 = 0x0113;
/// Re-pokes `site::ensure_watching` until the visual-tree watcher is advised;
/// cheap forever after.
const RETRY_TIMER_ID: usize = 1;
const RETRY_TIMER_MS: u32 = 2000;
/// Start Menu (`PECO-SMS1`) markers — sent to the `PecoFenceTAPSM` window.
const CDS_THEME: usize = u32::from_be_bytes(*b"SMST") as usize;
const CDS_RESET: usize = u32::from_be_bytes(*b"SMSR") as usize;
const CDS_HELLO: usize = u32::from_be_bytes(*b"SMHI") as usize;
/// Taskbar (`PECO-TBS1`) markers — sent to the explorer `PecoFenceTAP` window.
const CDS_TB_THEME: usize = u32::from_be_bytes(*b"TBST") as usize;
const CDS_TB_RESET: usize = u32::from_be_bytes(*b"TBSR") as usize;
const CDS_TB_HELLO: usize = u32::from_be_bytes(*b"TBHI") as usize;
/// Notification Center (`PECO-NCS1`) markers — sent to the `PecoFenceTAPNC`
/// window inside `ShellExperienceHost.exe`/`ShellHost.exe`.
const CDS_NC_THEME: usize = u32::from_be_bytes(*b"NCST") as usize;
const CDS_NC_RESET: usize = u32::from_be_bytes(*b"NCSR") as usize;
const CDS_NC_HELLO: usize = u32::from_be_bytes(*b"NCHI") as usize;
/// File Explorer (`PECO-FES1`) markers — sent to the explorer
/// `PecoFenceTAP` window; the wire applies inside the WinUI3 diagnostics
/// session (the mux engine), not the taskbar's WUX one.
const CDS_FE_THEME: usize = u32::from_be_bytes(*b"FEST") as usize;
const CDS_FE_RESET: usize = u32::from_be_bytes(*b"FESR") as usize;
const CDS_FE_HELLO: usize = u32::from_be_bytes(*b"FEHI") as usize;

fn handle_message(_hwnd: HWND, msg: u32, _wparam: WPARAM, lparam: LPARAM) {
    if msg == UNLOAD_MSG.load(Ordering::Acquire) && msg != 0 {
        crate::tap_log!("unload requested");
        unload(_hwnd);
        return;
    }
    // An unloaded resident is inert: its timer is killed, and any wire or
    // hello still landing here must not re-arm styling or the death watch.
    if is_unloaded() {
        return;
    }
    if msg == WM_TIMER {
        crate::site::ensure_watching();
        // The mux session's endpoint exists only once a WinUI3 island
        // registers it — an early FEST can land before any File Explorer
        // window exists, so keep re-trying while the mux engine wants one.
        if crate::styler::mux::wants_session() {
            crate::site::ensure_watching_winui3();
        }
        // Anchor props for late-created frames — a `Shell_TrayWnd` window
        // opened after this resident booted has no `CmdHwnd` prop until a
        // re-stamp, and the host would arm the hook forever.
        stamp_host_props(_hwnd);
        return;
    }
    if msg == WM_COPYDATA {
        // COPYDATA carries a UTF-8 theme wire (`PECO-SMS1` / `PECO-TBS1` /
        // `PECO-NCS1`) or a host-pid hello; the marker decides the engine.
        let cds = lparam as *const CopyDataStruct;
        if cds.is_null() {
            return;
        }
        let cds = unsafe { &*cds };
        crate::tap_log!(
            "wire rx marker={:08x} bytes={}",
            cds.dw_data as u32,
            cds.cb_data
        );
        match cds.dw_data {
            CDS_THEME | CDS_TB_THEME | CDS_NC_THEME
                if !cds.lp_data.is_null() && cds.cb_data > 0 =>
            {
                let bytes = unsafe {
                    core::slice::from_raw_parts(cds.lp_data as *const u8, cds.cb_data as usize)
                };
                let Ok(text) = core::str::from_utf8(bytes) else {
                    crate::tap_log!("styler wire not UTF-8");
                    return;
                };
                // The styler needs the visual-tree feed — re-arm/retry the
                // watcher (cheap no-op once advised).
                crate::site::ensure_watching();
                crate::styler::wux::apply_wire(text);
            }
            CDS_FE_THEME if !cds.lp_data.is_null() && cds.cb_data > 0 => {
                let bytes = unsafe {
                    core::slice::from_raw_parts(cds.lp_data as *const u8, cds.cb_data as usize)
                };
                let Ok(text) = core::str::from_utf8(bytes) else {
                    crate::tap_log!("FE styler wire not UTF-8");
                    return;
                };
                // The WinUI3 session may not exist yet (no File Explorer
                // window open) — the engine holds the theme as pending and
                // the timer keeps retrying `ensure_watching_winui3`.
                crate::site::ensure_watching_winui3();
                crate::styler::mux::apply_wire(text);
            }
            CDS_HELLO | CDS_TB_HELLO | CDS_NC_HELLO | CDS_FE_HELLO
                if !cds.lp_data.is_null() && cds.cb_data == 4 =>
            {
                // Host announced itself — arm the death watch so styles
                // restore if pecofence.exe dies.
                let pid = unsafe { *(cds.lp_data as *const u32) };
                watch_host_process(pid);
            }
            CDS_RESET | CDS_TB_RESET | CDS_NC_RESET => crate::styler::wux::reset(),
            CDS_FE_RESET => crate::styler::mux::reset(),
            _ => {}
        }
    }
}

fn watch_host_process(pid: u32) {
    let mut shared = shared::lock();
    if !shared.host_wait.is_null() {
        return; // already watching
    }
    let process = unsafe {
        OpenProcess(
            SYNCHRONIZE | PROCESS_QUERY_LIMITED_INFORMATION,
            windows_core::BOOL(0),
            pid,
        )
    };
    if process.is_null() {
        return;
    }
    let mut wait: HANDLE = core::ptr::null_mut();
    let ok = unsafe {
        RegisterWaitForSingleObject(
            &mut wait,
            process,
            Some(host_died),
            core::ptr::null(),
            INFINITE,
            WT_EXECUTEONLYONCE,
        )
    };
    if ok.0 == 0 {
        unsafe {
            let _ = CloseHandle(process);
        }
        return;
    }
    shared.host_wait = wait;
    // The wait does NOT own `process` — closing it while the wait is pending
    // is undefined behavior and silently kills the death watch. Keep the
    // handle until `host_died`/`unload` closes it.
    shared.host_process = process;
}

/// Host vanished: restore whichever surface the styler was armed on (taskbar
/// in explorer, Start Menu in SMEH). The fired wait auto-unregisters
/// (`WT_EXECUTEONLYONCE`); clearing the stored handle lets the *next* host's
/// hello re-arm the death watch.
unsafe extern "system" fn host_died(_context: *mut core::ffi::c_void, _fired: windows_core::BOOL) {
    let _ = std::panic::catch_unwind(core::panic::AssertUnwindSafe(|| {
        crate::styler::wux::reset();
        crate::styler::mux::reset();
        let mut shared = shared::lock();
        shared.host_wait = core::ptr::null_mut();
        if !shared.host_process.is_null() {
            unsafe {
                let _ = CloseHandle(shared.host_process);
            }
            shared.host_process = core::ptr::null_mut();
        }
    }));
}

/// `WM_TAP_UNLOAD`: full restore — the styler returns original property
/// values synchronously, the watcher unadvises, the death watch disarms.
///
/// The command window and the DLL stay alive: `InitializeXamlDiagnosticsEx`
/// pins the DLL in the host permanently, so a destroyed pump could never be
/// recreated on re-injection — an inert resident is exactly what
/// TranslucentTB keeps. Everything it *applied* is still fully unwound.
///
/// Inert also means silent: the retry timer is killed (it would otherwise
/// re-stamp `CmdHwnd` and re-advise the diagnostics channel forever —
/// fighting the live resident for both), our prop stamps are removed where
/// they still point at this window, and `is_unloaded()` rejects any late
/// wire/hello.
fn unload(hwnd: HWND) {
    UNLOADED.store(true, Ordering::Release);
    unsafe {
        let _ = KillTimer(hwnd, RETRY_TIMER_ID);
    }
    unstamp_host_props(hwnd);
    crate::tap_log!("unload: inert — timer killed, CmdHwnd unstamped");
    crate::styler::wux::shutdown();
    crate::styler::mux::shutdown();
    // Snapshot the service/watcher under the lock, then release it — the
    // unadvise call marshals to the UI thread and must not race callbacks
    // that take `shared`. Runs on a worker: a dead island pump parks the
    // unadvise forever, and the command pump must stay responsive.
    let both = {
        let mut shared = shared::lock();
        [
            shared.tree_service.clone().zip(shared.watcher.take()),
            shared
                .tree_service_winui3
                .clone()
                .zip(shared.watcher_winui3.take()),
        ]
    };
    struct SendPair(
        crate::bindings::IVisualTreeService3,
        crate::bindings::IVisualTreeServiceCallback2,
    );
    unsafe impl Send for SendPair {}
    let pairs: Vec<SendPair> = both
        .into_iter()
        .flatten()
        .map(|(s, w)| SendPair(s, w))
        .collect();
    std::thread::spawn(move || {
        for pair in &pairs {
            unsafe {
                let _ = pair.0.UnadviseVisualTreeChange(&*pair.1);
            }
        }
    });
    let mut shared = shared::lock();
    if !shared.host_wait.is_null() {
        unsafe {
            let _ = UnregisterWaitEx(shared.host_wait, core::ptr::null_mut());
        }
        shared.host_wait = core::ptr::null_mut();
    }
    if !shared.host_process.is_null() {
        unsafe {
            let _ = CloseHandle(shared.host_process);
        }
        shared.host_process = core::ptr::null_mut();
    }
}
