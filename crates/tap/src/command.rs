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
/// it must read as stale);
/// `8` = + in-process `IImageList2::SetOverlayImage` shortcut-overlay heal
/// (`SLNK` marker + periodic sync against `HKCU\...\Shell Icons\29`).
/// Keep in sync with `crates/platform/src/tap.rs::TAP_PROTOCOL_VERSION`.
pub const TAP_PROTOCOL_VERSION: usize = 8;
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
        sync_link_overlay(None);
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
/// ExplorerBlurMica relay — `EBML` carries the UTF-16 path to
/// `ExplorerBlurMica.dll` (LoadLibrary in-process), `EBMU` unloads it.
/// The vendored EBM DllMain self-installs inside explorer.exe.
const CDS_EBM_LOAD: usize = u32::from_be_bytes(*b"EBML") as usize;
const CDS_EBM_UNLOAD: usize = u32::from_be_bytes(*b"EBMU") as usize;
/// File Explorer layout (`PECO-FES1`) — pure positioning rules only,
/// applied by the mux (WinUI3) engine; `FELR` restores stock layout.
/// Independent of EBM: the material pipeline owns every surface brush.
const CDS_FE_LAYOUT: usize = u32::from_be_bytes(*b"FELT") as usize;
const CDS_FE_LAYOUT_RESET: usize = u32::from_be_bytes(*b"FELR") as usize;
/// In-process shortcut-overlay override (`SLNK`, 1 byte: `1` = stock arrow,
/// `0` = blank). `windows.storage.dll` reads `Shell Icons\29` from `HKLM`
/// only, so when `HKLM` holds a poisoned resource (`shell32.dll,-50`) and
/// the process is unelevated, we swap the `SIID_LINK` overlay slot in
/// Explorer's five `SHGetImageList` instances directly via `IImageList2`.
const CDS_LINK_OVERLAY: usize = u32::from_be_bytes(*b"SLNK") as usize;

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
        // registers it — a theme wire can land before any File Explorer
        // window exists, so keep re-trying while the mux engine wants one.
        if crate::styler::mux::wants_session() {
            crate::site::ensure_watching_winui3();
        }
        // Anchor props for late-created frames — a `Shell_TrayWnd` window
        // opened after this resident booted has no `CmdHwnd` prop until a
        // re-stamp, and the host would arm the hook forever.
        stamp_host_props(_hwnd);
        sync_link_overlay(None);
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
            CDS_HELLO | CDS_TB_HELLO | CDS_NC_HELLO
                if !cds.lp_data.is_null() && cds.cb_data == 4 =>
            {
                // Host announced itself — arm the death watch so styles
                // restore if pecofence.exe dies.
                let pid = unsafe { *(cds.lp_data as *const u32) };
                watch_host_process(pid);
            }
            CDS_EBM_LOAD if !cds.lp_data.is_null() && cds.cb_data >= 4 => {
                let bytes = unsafe {
                    core::slice::from_raw_parts(cds.lp_data as *const u8, cds.cb_data as usize)
                };
                crate::ebm::load(bytes);
            }
            CDS_EBM_UNLOAD => crate::ebm::unload(),
            CDS_FE_LAYOUT if !cds.lp_data.is_null() && cds.cb_data > 0 => {
                let bytes = unsafe {
                    core::slice::from_raw_parts(cds.lp_data as *const u8, cds.cb_data as usize)
                };
                let Ok(text) = core::str::from_utf8(bytes) else {
                    crate::tap_log!("fe layout wire not UTF-8");
                    return;
                };
                // The mux engine styles the WinUI3 islands — arm that
                // watcher (cheap no-op once advised; the 2s retry timer
                // re-pokes while `wants_session`).
                crate::site::ensure_watching_winui3();
                crate::styler::mux::apply_wire(text);
            }
            CDS_FE_LAYOUT_RESET => crate::styler::mux::reset(),
            CDS_LINK_OVERLAY if !cds.lp_data.is_null() && cds.cb_data >= 1 => {
                let shown = unsafe { *(cds.lp_data as *const u8) } != 0;
                sync_link_overlay(Some(shown));
            }
            CDS_RESET | CDS_TB_RESET | CDS_NC_RESET => crate::styler::wux::reset(),
            _ => {}
        }
    }
}

/// Repairs the shortcut-arrow overlay slot inside `explorer.exe`'s five
/// `SHGetImageList` instances (`SHIL_LARGE`..=`SHIL_JUMBO`).
///
/// Why this lives in the resident: `windows.storage.dll` reads
/// `Software\Microsoft\Windows\CurrentVersion\Explorer\Shell Icons\29` from
/// `HKLM` only (`HKCU` is ignored). When `HKLM` holds a broken entry like
/// `shell32.dll,-50` and PecoFence runs unelevated, `SHIL_JUMBO` (256x256)
/// punches a 128x128 transparent hole in the bottom-left corner of every
/// `.lnk` icon in File Explorer and on the Desktop. Calling
/// `IImageList2::SetOverlayImage` in-process replaces the overlay slot with
/// `imageres.dll,-1015` (Microsoft's blank icon, when arrows are hidden) or
/// `imageres.dll,-163` (Windows 11's stock arrow, when shown) without needing
/// admin rights or an Explorer restart.
fn sync_link_overlay(explicit_shown: Option<bool>) {
    if crate::is_smeh() || crate::is_nch() {
        return;
    }
    let shown = explicit_shown.unwrap_or_else(hkcu_wants_link_arrow);
    // -163 = stock shortcut arrow in imageres.dll; -1015 = stock blank icon.
    let icon_id: i32 = if shown { -163 } else { -1015 };
    let mut win_dir = [0u16; 260];
    let n = unsafe { GetWindowsDirectoryW(windows_core::PWSTR(win_dir.as_mut_ptr()), 260) };
    let base = if n > 0 && (n as usize) < win_dir.len() {
        String::from_utf16_lossy(&win_dir[..n as usize])
    } else {
        r"C:\Windows".to_owned()
    };
    let imageres: Vec<u16> = format!(r"{base}\System32\imageres.dll")
        .encode_utf16()
        .chain(core::iter::once(0))
        .collect();
    // IDO_SHGIOI_LINK = 0x0FFFFFFE -> 1-based overlay slot (typically 2).
    let ovl = unsafe { SHGetIconOverlayIndexW(windows_core::PCWSTR::null(), 0x0FFF_FFFE) };
    let ovl = if (1..=15).contains(&ovl) { ovl } else { 2 };
    let target_idx =
        unsafe { Shell_GetCachedImageIndexW(windows_core::PCWSTR(imageres.as_ptr()), icon_id, 0) };
    if target_idx < 0 {
        return;
    }
    const IID_IIMAGELIST2: windows_core::GUID =
        windows_core::GUID::from_u128(0x192b9d83_50fc_457b_90a0_2b82a8b5dae1);
    let mut changed = false;
    for shil in 0..=4 {
        let mut iml: *mut core::ffi::c_void = core::ptr::null_mut();
        if unsafe { SHGetImageList(shil, &IID_IIMAGELIST2, &mut iml) }.is_err() || iml.is_null() {
            continue;
        }
        unsafe {
            let vtable = *(iml as *const *const usize);
            let release: unsafe extern "system" fn(*mut core::ffi::c_void) -> u32 =
                core::mem::transmute(*vtable.add(2));
            let set_overlay: unsafe extern "system" fn(
                *mut core::ffi::c_void,
                i32,
                i32,
            ) -> windows_core::HRESULT = core::mem::transmute(*vtable.add(5));
            let get_overlay: unsafe extern "system" fn(
                *mut core::ffi::c_void,
                i32,
                *mut i32,
            ) -> windows_core::HRESULT = core::mem::transmute(*vtable.add(31));
            let force_present: unsafe extern "system" fn(
                *mut core::ffi::c_void,
                i32,
                u32,
            ) -> windows_core::HRESULT = core::mem::transmute(*vtable.add(37));

            let mut cur_idx = -1i32;
            let _ = get_overlay(iml, ovl, &mut cur_idx);
            if cur_idx != target_idx {
                let _ = force_present(iml, target_idx, 0);
                if set_overlay(iml, target_idx, ovl).is_ok() {
                    changed = true;
                }
            }
            let _ = release(iml);
        }
    }
    if changed || explicit_shown.is_some() {
        crate::tap_log!(
            "link overlay synced shown={shown} ovl={ovl} target_idx={target_idx} changed={changed}"
        );
        const SHCNE_ASSOCCHANGED: i32 = 0x0800_0000u32 as i32;
        const SHCNE_UPDATEIMAGE: i32 = 0x0000_8000;
        const SHCNF_IDLIST: u32 = 0x0000;
        const SHCNF_DWORD: u32 = 0x0003;
        unsafe {
            SHChangeNotify(
                SHCNE_UPDATEIMAGE,
                SHCNF_DWORD,
                29usize as *const core::ffi::c_void,
                core::ptr::null(),
            );
            SHChangeNotify(
                SHCNE_UPDATEIMAGE,
                SHCNF_DWORD,
                target_idx as usize as *const core::ffi::c_void,
                core::ptr::null(),
            );
            SHChangeNotify(
                SHCNE_ASSOCCHANGED,
                SHCNF_IDLIST,
                core::ptr::null(),
                core::ptr::null(),
            );
        }
        redraw_process_windows();
    }
}

/// Reads `HKCU\Software\Microsoft\Windows\CurrentVersion\Explorer\Shell Icons\29`:
/// `false` when it names our `-1015` blank sentinel; `true` otherwise.
fn hkcu_wants_link_arrow() -> bool {
    const HKEY_CURRENT_USER: isize = -0x7FFF_FFFFisize; // 0xFFFFFFFF80000001
    const RRF_RT_REG_SZ: u32 = 0x02;
    let subkey: Vec<u16> = r"Software\Microsoft\Windows\CurrentVersion\Explorer\Shell Icons"
        .encode_utf16()
        .chain(core::iter::once(0))
        .collect();
    let val: Vec<u16> = "29\0".encode_utf16().collect();
    let mut buf = [0u16; 512];
    let mut size = (buf.len() * 2) as u32;
    let status = unsafe {
        RegGetValueW(
            HKEY_CURRENT_USER,
            windows_core::PCWSTR(subkey.as_ptr()),
            windows_core::PCWSTR(val.as_ptr()),
            RRF_RT_REG_SZ,
            core::ptr::null_mut(),
            buf.as_mut_ptr().cast(),
            &mut size,
        )
    };
    if status != 0 {
        return true;
    }
    let len = buf.iter().position(|&c| c == 0).unwrap_or(buf.len());
    let s = String::from_utf16_lossy(&buf[..len]);
    !s.contains("-1015")
}

fn redraw_process_windows() {
    unsafe extern "system" fn cb(hwnd: HWND, _lparam: LPARAM) -> windows_core::BOOL {
        let mut pid = 0u32;
        unsafe {
            let _ = GetWindowThreadProcessId(hwnd, &mut pid);
            if pid == GetCurrentProcessId() {
                // RDW_INVALIDATE (1) | RDW_ERASE (4) | RDW_ALLCHILDREN (0x80)
                let _ = RedrawWindow(hwnd, core::ptr::null(), core::ptr::null_mut(), 0x85);
            }
        }
        windows_core::BOOL(1)
    }
    unsafe {
        let _ = EnumWindows(Some(cb), 0);
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
    crate::ebm::shutdown();
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
