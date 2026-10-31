//! Host-side residency controller for `pecofence_tap.dll` — injects the DLL
//! into `explorer.exe` with a `WH_CALLWNDPROC` hook on the taskbar thread
//! (TranslucentTB's `InjectExplorerTAP`) and owns the channel the taskbar
//! styler rides on:
//!
//! - `WM_COPYDATA` to the in-Explorer `PecoFenceTAP` command window:
//!   `PECO-TBS1` theme wires, the host-pid hello (death watch), and the
//!   reset marker.
//! - `"PecoFence.TAP.Unload"` (registered message): synchronous full
//!   teardown — original property values back, visual-tree watcher
//!   unadvised, death watch disarmed.

use crate::bindings::{
    FindWindowExW, FindWindowW, GetClassNameW, GetModuleFileNameW, GetPropW,
    GetWindowThreadProcessId, HWND, HWND_MESSAGE, LPARAM, SendMessageTimeoutW, WPARAM,
};
use std::path::PathBuf;
use std::sync::Mutex;
use std::sync::atomic::{AtomicU32, Ordering};
use windows_core::{BOOL, PCSTR, PCWSTR, PWSTR};

windows_core::link!("kernel32.dll" "system" fn FreeLibrary(hlibmodule: *mut core::ffi::c_void) -> BOOL);
windows_core::link!("kernel32.dll" "system" fn GetProcAddress(hmodule: *mut core::ffi::c_void, lpprocname: PCSTR) -> *mut core::ffi::c_void);
windows_core::link!("kernel32.dll" "system" fn LoadLibraryW(lplibfilename: PCWSTR) -> *mut core::ffi::c_void);
windows_core::link!("kernel32.dll" "system" fn LocalFree(hlocal: *mut core::ffi::c_void) -> *mut core::ffi::c_void);
windows_core::link!("kernel32.dll" "system" fn Sleep(dwmilliseconds: u32));
windows_core::link!("advapi32.dll" "system" fn ConvertStringSidToSidW(sid: PCWSTR, sid_out: *mut *mut core::ffi::c_void) -> BOOL);
windows_core::link!("advapi32.dll" "system" fn GetNamedSecurityInfoW(name: PCWSTR, objtype: i32, si: u32, owner: *mut *mut core::ffi::c_void, group: *mut *mut core::ffi::c_void, dacl: *mut *mut core::ffi::c_void, sacl: *mut *mut core::ffi::c_void, sd: *mut *mut core::ffi::c_void) -> u32);
windows_core::link!("advapi32.dll" "system" fn SetEntriesInAclW(count: u32, entries: *const ExplicitAccessW, old: *const core::ffi::c_void, new: *mut *mut core::ffi::c_void) -> u32);
windows_core::link!("advapi32.dll" "system" fn SetNamedSecurityInfoW(name: PCWSTR, objtype: i32, si: u32, owner: *const core::ffi::c_void, group: *const core::ffi::c_void, dacl: *const core::ffi::c_void, sacl: *const core::ffi::c_void) -> u32);
windows_core::link!("user32.dll" "system" fn IsWindow(hwnd: HWND) -> BOOL);
windows_core::link!("user32.dll" "system" fn RegisterWindowMessageW(lpstring: PCWSTR) -> u32);
windows_core::link!("user32.dll" "system" fn SetWindowsHookExW(idhook: i32, lpfn: *const core::ffi::c_void, hmod: *mut core::ffi::c_void, dwthreadid: u32) -> *mut core::ffi::c_void);
windows_core::link!("user32.dll" "system" fn UnhookWindowsHookEx(hhk: *mut core::ffi::c_void) -> BOOL);

/// `EXPLICIT_ACCESS_W` + `TRUSTEE_W` — DACL merge for the AppContainer ACE.
#[repr(C)]
struct TrusteeW {
    multiple: *mut u16,
    multiple_op: u32,
    form: u32,
    trustee_type: u32,
    name: *mut u16,
}

#[repr(C)]
struct ExplicitAccessW {
    permissions: u32,
    mode: u32,
    inheritance: u32,
    trustee: TrusteeW,
}

const SET_ACCESS: u32 = 2;
const NO_INHERITANCE: u32 = 0;
const TRUSTEE_IS_SID: u32 = 0;
const TRUSTEE_IS_WELL_KNOWN_GROUP: u32 = 5;
const SE_FILE_OBJECT: i32 = 1;
const DACL_SECURITY_INFORMATION: u32 = 0x4;
const GENERIC_READ: u32 = 0x8000_0000;
const GENERIC_EXECUTE: u32 = 0x2000_0000;
/// `ALL APPLICATION PACKAGES` — the audience AppContainer file ACLs use.
const APP_PACKAGES_SID: &str = "S-1-15-2-1";

/// Grants `ALL APPLICATION PACKAGES` read+execute on `path` — packaged
/// hosts (`ShellExperienceHost`) can't read user-profile files, so any DLL
/// a hook or remote load maps into them must carry this ACE. A plain DACL
/// merge: the existing entries stay untouched.
fn grant_app_packages_access(path: &std::path::Path) {
    let mut sid = core::ptr::null_mut();
    if unsafe { ConvertStringSidToSidW(PCWSTR(wide(APP_PACKAGES_SID).as_ptr()), &mut sid) }.0 == 0 {
        return;
    }
    let ea = ExplicitAccessW {
        permissions: GENERIC_READ | GENERIC_EXECUTE,
        mode: SET_ACCESS,
        inheritance: NO_INHERITANCE,
        trustee: TrusteeW {
            multiple: core::ptr::null_mut(),
            multiple_op: 0,
            form: TRUSTEE_IS_SID,
            trustee_type: TRUSTEE_IS_WELL_KNOWN_GROUP,
            name: sid.cast(),
        },
    };
    unsafe {
        let mut old_dacl = core::ptr::null_mut();
        let mut sd = core::ptr::null_mut();
        let wname = wide(&path.to_string_lossy());
        if GetNamedSecurityInfoW(
            PCWSTR(wname.as_ptr()),
            SE_FILE_OBJECT,
            DACL_SECURITY_INFORMATION,
            core::ptr::null_mut(),
            core::ptr::null_mut(),
            &mut old_dacl,
            core::ptr::null_mut(),
            &mut sd,
        ) == 0
        {
            let mut new_dacl = core::ptr::null_mut();
            if SetEntriesInAclW(1, &ea, old_dacl, &mut new_dacl) == 0 {
                let _ = SetNamedSecurityInfoW(
                    PCWSTR(wname.as_ptr()),
                    SE_FILE_OBJECT,
                    DACL_SECURITY_INFORMATION,
                    core::ptr::null(),
                    core::ptr::null(),
                    new_dacl,
                    core::ptr::null(),
                );
                let _ = LocalFree(new_dacl);
            }
            let _ = LocalFree(sd);
        }
        let _ = LocalFree(sid);
    }
}

const WH_CALLWNDPROC: i32 = 4;
const SMTO_ABORTIFHUNG: u32 = 0x0002;
const SMTO_BLOCK: u32 = 0x0001;
const SMTO_ERRORONEXIT: u32 = 0x0020;

struct Tap {
    /// Our `WH_CALLWNDPROC` hook while it is still armed (kept until the DLL
    /// has actually been loaded by Explorer; `WH_CALLWNDPROC` hooks stay
    /// installed even after the target loads the DLL, so we unhook once the
    /// command window answers).
    hook: *mut core::ffi::c_void,
    /// Our own load of `pecofence_tap.dll` (keeps the path resolvable for the
    /// hook; Explorer keeps its own mapping).
    lib: *mut core::ffi::c_void,
    /// The in-Explorer command window (`PecoFenceTAP` class).
    cmd_hwnd: HWND,
    /// Registered `PecoFence.TAP.Unload` message id — per-session.
    msg_unload: u32,
    /// Pid of the Explorer process the DLL is currently loaded in.
    explorer_pid: u32,
}

// Raw handles cross no threads other than the message pump's; access is
// serialised by the Mutex like the rest of this module's state.
unsafe impl Send for Tap {}

static TAP: Mutex<Option<Tap>> = Mutex::new(None);

fn lock() -> std::sync::MutexGuard<'static, Option<Tap>> {
    TAP.lock().unwrap_or_else(|p| p.into_inner())
}

fn wide(s: &str) -> Vec<u16> {
    s.encode_utf16().chain(core::iter::once(0)).collect()
}

fn find_taskbar() -> Option<(HWND, u32)> {
    let hwnd = unsafe { FindWindowW(PCWSTR(wide("Shell_TrayWnd").as_ptr()), PCWSTR::null()) };
    if hwnd.0.is_null() {
        return None;
    }
    let mut pid = 0u32;
    unsafe {
        GetWindowThreadProcessId(hwnd, Some(&mut pid));
    }
    (pid != 0).then_some((hwnd, pid))
}

fn tap_dll_path() -> Option<PathBuf> {
    let mut buf = [0u16; 512];
    let len = unsafe { GetModuleFileNameW(None, PWSTR(buf.as_mut_ptr()), buf.len() as u32) };
    if len == 0 {
        return None;
    }
    let exe = PathBuf::from(String::from_utf16_lossy(&buf[..len as usize]));
    Some(exe.with_file_name("pecofence_tap.dll"))
}

/// The path actually injected into hosts — a *unique-per-build* copy of the
/// resident DLL. `InitializeXamlDiagnosticsEx` (and the self-pin) keeps a
/// resident's image mapped in its host for the process's life, so a stale
/// build can never be evicted by re-injecting `pecofence_tap.dll` under the
/// same path: `LoadLibrary` dedups by path inside the target and the new
/// code never loads — the stale resident keeps its command window, drops
/// wires it can't parse, and only an `explorer.exe` restart evicts it.
/// Injecting `pecofence_tap_v<protocol>_<filehash>.dll` instead lets the
/// fresh resident co-load beside the stale image (which then reads as
/// stale via `PecoFence.TAP.Build` and is told to unload).
pub(crate) fn tap_injectable_path() -> Option<PathBuf> {
    static PATH: std::sync::OnceLock<Option<PathBuf>> = std::sync::OnceLock::new();
    PATH.get_or_init(|| {
        let real = tap_dll_path()?;
        let bytes = std::fs::read(&real).ok()?;
        // FNV-1a over the whole image — any rebuild yields a new module name.
        let mut h: u64 = 0xcbf29ce484222325;
        for &b in &bytes {
            h = (h ^ b as u64).wrapping_mul(0x100000001b3);
        }
        let name = format!("pecofence_tap_v{TAP_PROTOCOL_VERSION}_{h:016x}.dll");
        let dir = std::env::var_os("LOCALAPPDATA")
            .map(PathBuf::from)?
            .join("PecoFence")
            .join("residents");
        std::fs::create_dir_all(&dir).ok()?;
        let out = dir.join(&name);
        if !out.is_file() {
            // Garbage-collect superseded copies — pinned ones (loaded in a
            // live host) fail removal harmlessly and get a retry next run.
            if let Ok(rd) = std::fs::read_dir(&dir) {
                for e in rd.flatten() {
                    let p = e.path();
                    let stale = p
                        .file_name()
                        .is_some_and(|n| n.to_string_lossy().starts_with("pecofence_tap_v"));
                    if stale && p != out {
                        let _ = std::fs::remove_file(&p);
                    }
                }
            }
            std::fs::copy(&real, &out).ok()?;
        }
        Some(out)
    })
    .clone()
}

/// The same unique-per-build resident copy but under `%ProgramData%` with
/// an `ALL APPLICATION PACKAGES` read+execute ACE — AppContainer hosts
/// (`ShellExperienceHost`, `ShellHost`) can't read `%LOCALAPPDATA%` paths,
/// so the NC pipeline injects this copy instead. Same filename → the
/// resident derives the identical command-window class.
pub(crate) fn tap_injectable_path_appcontainer() -> Option<PathBuf> {
    static PATH: std::sync::OnceLock<Option<PathBuf>> = std::sync::OnceLock::new();
    PATH.get_or_init(|| {
        let local = tap_injectable_path()?;
        let dir = PathBuf::from(std::env::var_os("ProgramData")?)
            .join("PecoFence")
            .join("residents");
        std::fs::create_dir_all(&dir).ok()?;
        let out = dir.join(local.file_name()?);
        if !out.is_file() {
            std::fs::copy(&local, &out).ok()?;
        }
        grant_app_packages_access(&out);
        Some(out)
    })
    .clone()
}

const WM_COPYDATA: u32 = 0x004A;

/// `COPYDATASTRUCT` — mirrors the DLL side; pointer only needs to live for
/// the duration of the (synchronous) `SendMessage` call.
#[repr(C)]
struct CopyDataStruct {
    dw_data: usize,
    cb_data: u32,
    lp_data: *const core::ffi::c_void,
}

/// `WM_COPYDATA` delivery to the in-Explorer command window — used by the
/// taskbar styler for `PECO-TBS1` theme wires + the host-pid hello.
pub fn send_copydata(hwnd: HWND, dw_data: usize, bytes: &[u8]) -> bool {
    let cds = CopyDataStruct {
        dw_data,
        cb_data: bytes.len() as u32,
        lp_data: bytes.as_ptr() as *const core::ffi::c_void,
    };
    send_cmd(hwnd, WM_COPYDATA, 0, &cds as *const CopyDataStruct as isize)
}

/// Sends a registered message to the in-Explorer command window and waits for
/// it to be processed — ordering guarantee for "state set" before "accent
/// applied".
fn send_cmd(hwnd: HWND, msg: u32, wparam: usize, lparam: isize) -> bool {
    let mut result = 0usize;
    unsafe {
        SendMessageTimeoutW(
            hwnd,
            msg,
            WPARAM(wparam),
            LPARAM(lparam),
            SMTO_ABORTIFHUNG | SMTO_BLOCK | SMTO_ERRORONEXIT,
            2_000,
            Some(&mut result),
        )
        .0 != 0
    }
}

/// `PecoFence.TAP.CmdHwnd` — the resident stamps its message-only command
/// window as a prop on its process's `Shell_TrayWnd` (message-only windows
/// are skipped by `EnumWindows`, and `FindWindowW` is ambiguous among
/// same-class residents).
fn cmd_prop(hwnd: HWND) -> HWND {
    let name = wide("PecoFence.TAP.CmdHwnd");
    let val = unsafe { GetPropW(hwnd, PCWSTR(name.as_ptr())) };
    HWND(val.0 as _)
}

/// A resident command window's registered class name — encodes its module
/// build (`PecoFenceTAP_v<ver>_<filehash>`), which is a stricter identity
/// than the `PecoFence.TAP.Build` protocol stamp alone: two builds can
/// share a protocol version while differing in code.
fn resident_class_name(hwnd: HWND) -> String {
    let mut cls = [0u16; 96];
    let n = unsafe { GetClassNameW(hwnd, PWSTR(cls.as_mut_ptr()), cls.len() as i32) };
    String::from_utf16_lossy(&cls[..n.max(0) as usize])
}

/// The class our current build's resident registers — `base` + the suffix
/// after `pecofence_tap` in the injected module filename (the resident
/// derives it the same way from its own `GetModuleFileNameW`). When the
/// DLL can't be hashed the suffix is empty and matching degrades to
/// protocol version only.
fn expected_resident_class(base: &str) -> String {
    let suffix = tap_injectable_path()
        .and_then(|p| p.file_stem().map(|s| s.to_string_lossy().into_owned()))
        .and_then(|stem| stem.strip_prefix("pecofence_tap").map(|s| s.to_string()))
        .unwrap_or_default();
    format!("{base}{suffix}")
}

/// Discovery acceptance: live window, right process, new-enough protocol,
/// and our build's class — a same-protocol older-hash resident is still
/// foreign (it carries the bug being upgraded away).
fn resident_usable(cmd: HWND, pid: u32, min_version: usize) -> bool {
    !cmd.0.is_null()
        && unsafe { IsWindow(cmd) }.0 != 0
        && resident_owner(cmd) == pid
        && resident_version(cmd) >= min_version
        && resident_class_name(cmd) == expected_resident_class("PecoFenceTAP")
}

/// All resident command windows of the `base` class family owned by `pid`,
/// our-build-first then newest protocol — message-only windows are
/// invisible to `EnumWindows` but `FindWindowExW` off `HWND_MESSAGE` walks
/// them. Prefix-matched because each build registers a unique class
/// derived from its module filename so a fresh image can co-load beside a
/// stale pinned resident. `base` is matched exactly or with a `_` suffix,
/// so `PecoFenceTAP` never catches `PecoFenceTAPSM`.
pub(crate) fn resident_windows_matching(pid: u32, base: &str) -> Vec<HWND> {
    let expected = expected_resident_class(base);
    let prefixed = format!("{base}_");
    let mut out = Vec::new();
    let mut after = HWND(core::ptr::null_mut());
    loop {
        let h = unsafe {
            FindWindowExW(
                Some(HWND_MESSAGE),
                Some(after),
                PCWSTR::null(),
                PCWSTR::null(),
            )
        };
        if h.0.is_null() {
            break;
        }
        let name = resident_class_name(h);
        if (name == base || name.starts_with(&prefixed)) && resident_owner(h) == pid {
            out.push(h);
        }
        after = h;
    }
    out.sort_by_key(|h| {
        (
            resident_class_name(*h) != expected,
            usize::MAX - resident_version(*h),
        )
    });
    out
}

/// Explorer-resident windows (`PecoFenceTAP` family) owned by `pid`,
/// newest protocol first.
fn resident_windows_for_pid(pid: u32) -> Vec<HWND> {
    resident_windows_matching(pid, "PecoFenceTAP")
}

/// The resident command window owned by `pid` — the highest protocol build
/// wins, so a fresh co-loaded resident beats a stale pinned one whose
/// `CmdHwnd` re-stamp may still be winning the prop race.
fn find_cmd_for_pid(pid: u32) -> Option<HWND> {
    resident_windows_for_pid(pid).into_iter().next()
}

/// The command window of THIS build's resident in `pid` — same-protocol
/// older-hash windows are skipped (a stale resident's engine can't parse
/// new wire directives; it must be evicted, not served).
pub(crate) fn resident_current(pid: u32, base: &str) -> Option<HWND> {
    let expected = expected_resident_class(base);
    resident_windows_matching(pid, base)
        .into_iter()
        .find(|h| resident_class_name(*h) == expected)
}

/// Tells every resident window of the `base` family in `pid` that isn't
/// `keep` to unwind its applied styles. Since each build injects under a
/// unique module name, a stale image can stay mapped in the process — and
/// two residents styling the same visual tree would fight over originals.
pub(crate) fn unload_foreign_residents_matching(pid: u32, keep: HWND, base: &str) {
    let msg = unsafe { RegisterWindowMessageW(PCWSTR(wide("PecoFence.TAP.Unload").as_ptr())) };
    if msg == 0 {
        return;
    }
    for h in resident_windows_matching(pid, base) {
        if h != keep {
            let _ = send_cmd(h, msg, 0, 0);
        }
    }
}

fn unload_foreign_residents(pid: u32, keep: HWND) {
    unload_foreign_residents_matching(pid, keep, "PecoFenceTAP");
}

/// Command-protocol build the resident DLL must report via its
/// `PecoFence.TAP.Build` window property — must match
/// `crates/tap/src/command.rs::TAP_PROTOCOL_VERSION`. `1` = transparency +
/// Start Menu styler; `2` = + taskbar styler (`PECO-TBS1` COPYDATA markers);
/// `3` = legacy accent surface removed — a v2 resident may still carry an
/// armed `TAP_STATE`/SWCA detour from a previous session, so it must read
/// as stale here (an explorer restart loads the clean build);
/// `5` = File Explorer styler removed — a v4 resident still carries the
/// WinUI3/mux machinery, so it must read as stale and go inert;
/// `6` = + Notification Center styler — a v5 resident registers no
/// `PecoFenceTAPNC` class in the Shell hosts, so it must read as stale;
/// `7` = + File Explorer styler restored (`PECO-FES1` markers + the WinUI3
/// `FrameworkUdk` diagnostics session — a v6 resident has no FEST handler,
/// so it must read as stale).
pub const TAP_PROTOCOL_VERSION: usize = 7;

/// Explorer pid carrying a stale pinned resident that can't serve the
/// taskbar-styler protocol — remembered so the periodic `ensure` doesn't
/// burn the inject poll (4s) on every tick. Cleared implicitly when the
/// pid changes (explorer restart → fresh inject).
static STALE_RESIDENT_PID: AtomicU32 = AtomicU32::new(0);

/// The build stamp a resident command window advertises — `0` when the
/// property is missing (a resident older than this handshake, or a dying
/// window mid-teardown).
fn resident_version(hwnd: HWND) -> usize {
    let name = wide("PecoFence.TAP.Build");
    unsafe { GetPropW(hwnd, PCWSTR(name.as_ptr())).0 as usize }
}

/// Owner pid of a resident command window — `0` for a dead HWND. A
/// restarting explorer leaves its resident window alive briefly during
/// teardown, and `FindWindowW` is global, so windows from other processes
/// must neither satisfy the handshake nor poison the stale-pid latch.
fn resident_owner(hwnd: HWND) -> u32 {
    let mut pid = 0;
    unsafe {
        let _ = GetWindowThreadProcessId(hwnd, Some(&mut pid));
    }
    pid
}

/// Arms the `WH_CALLWNDPROC` hook so Windows maps our DLL into `explorer.exe`,
/// then waits for its command window to appear.
/// `min_version`: minimum resident build stamp to accept — older builds
/// silently drop `PECO-TBS1` wires or may carry stale accent state.
/// Arms the `WH_CALLWNDPROC` hook so Windows maps our DLL into the target
/// window's thread, pokes the thread, and returns immediately — the caller
/// decides how long (if at all) to wait for the resident's command window.
fn arm_inject(tap: &mut Tap, target: HWND) -> Option<u32> {
    // The versioned copy, not the install-side `pecofence_tap.dll`: only a
    // unique module name can co-load past a stale pinned resident (the
    // whole point of the FE live-update channel existing).
    let path = match tap_injectable_path() {
        Some(p) if p.exists() => p,
        _ => return None,
    };
    let wide_path = path
        .to_string_lossy()
        .encode_utf16()
        .chain(core::iter::once(0))
        .collect::<Vec<u16>>();
    let lib = unsafe { LoadLibraryW(PCWSTR(wide_path.as_ptr())) };
    if lib.is_null() {
        return None;
    }
    let proc = unsafe { GetProcAddress(lib, PCSTR(c"CallWndProc".as_ptr() as *const u8)) };
    if proc.is_null() {
        unsafe {
            let _ = FreeLibrary(lib);
        }
        return None;
    }
    let mut tid_pid = 0u32;
    let tid = unsafe { GetWindowThreadProcessId(target, Some(&mut tid_pid)) };
    if tid == 0 {
        unsafe {
            let _ = FreeLibrary(lib);
        }
        return None;
    }
    let hook = unsafe { SetWindowsHookExW(WH_CALLWNDPROC, proc, lib, tid) };
    if hook.is_null() {
        tracing::warn!(tid, pid = tid_pid, "inject: SetWindowsHookExW failed");
        unsafe {
            let _ = FreeLibrary(lib);
        }
        return None;
    }
    tracing::info!(tid, pid = tid_pid, "inject: hook armed");
    tap.hook = hook;
    tap.lib = lib;
    tap.msg_unload =
        unsafe { RegisterWindowMessageW(PCWSTR(wide("PecoFence.TAP.Unload").as_ptr())) };
    // Trigger the hook NOW: `WH_CALLWNDPROC` only fires on *sent* messages —
    // a posted WM_NULL waits in the queue and, on an idle cabinet thread that
    // receives no SendMessage traffic, the DLL never maps at all (the reason
    // File Explorer residents silently never existed). A synchronous send
    // forces the CallWndProc chain through immediately.
    let mut result = 0usize;
    unsafe {
        let _ = SendMessageTimeoutW(
            target,
            0, // WM_NULL — no-op for the receiver, enough to fire the hook
            WPARAM(0),
            LPARAM(0),
            SMTO_ABORTIFHUNG | SMTO_BLOCK | SMTO_ERRORONEXIT,
            500,
            Some(&mut result),
        );
    }
    Some(tid_pid)
}

/// Blocking inject for the shell path — arms the hook, then polls up to 4s
/// for the resident's command window. Discovery is prop-first with an
/// enumeration fallback that sees every resident build in the process and
/// prefers the newest protocol.
fn inject(tap: &mut Tap, taskbar: HWND, min_version: usize) -> bool {
    let Some(tid_pid) = arm_inject(tap, taskbar) else {
        return false;
    };
    // Up to 4s for the DLL to boot its command window inside the target
    // process. The resident stamps `PecoFence.TAP.CmdHwnd` on the anchor
    // window we hooked (cabinet frame or taskbar) — the prop is the
    // authoritative channel once stamped; `FindWindowW`+owner is the
    // fallback for residents that predate stamping.
    let mut stale_seen = false;
    for _ in 0..80 {
        let prop_cmd = cmd_prop(taskbar);
        let prop_live = !prop_cmd.0.is_null()
            && unsafe { IsWindow(prop_cmd) }.0 != 0
            && resident_owner(prop_cmd) == tid_pid;
        // Prefer the stamped prop only while it is ours — a stale resident
        // keeps re-stamping its HWND from its own WM_TIMER, so the prop can
        // point at an old build while a fresh co-loaded resident (unique
        // module name) sits beside it.
        let found = if resident_usable(prop_cmd, tid_pid, min_version) {
            Some(prop_cmd)
        } else {
            let all = resident_windows_for_pid(tid_pid);
            all.iter()
                .copied()
                .find(|h| resident_usable(*h, tid_pid, min_version))
                .or(if prop_live {
                    Some(prop_cmd)
                } else {
                    all.into_iter().next()
                })
        };
        if let Some(cmd) = found {
            if resident_version(cmd) >= min_version {
                tap.cmd_hwnd = cmd;
                // One resident per process — unwind stale co-loaded builds.
                unload_foreign_residents(tid_pid, cmd);
                return true;
            }
            // A resident from an older build — IXDEx pins it in Explorer so
            // re-injection can't replace it. Keep polling briefly: if it was
            // unpinned our `disarm` may have unloaded it (its window dies and
            // a fresh bootstrap creates a stamped one).
            stale_seen = true;
        }
        unsafe { Sleep(50) };
    }
    if stale_seen && min_version > 0 {
        STALE_RESIDENT_PID.store(tid_pid, Ordering::Release);
        tracing::warn!(
            min_version,
            "taskbar styler: resident pecofence_tap.dll predates the command protocol \
             — restart explorer.exe to load the new engine"
        );
    }
    false
}

/// Drops the hook + our DLL reference once Explorer's copy is running.
fn disarm(tap: &mut Tap) {
    unsafe {
        if !tap.hook.is_null() {
            let _ = UnhookWindowsHookEx(tap.hook);
            tap.hook = core::ptr::null_mut();
        }
        if !tap.lib.is_null() {
            let _ = FreeLibrary(tap.lib);
            tap.lib = core::ptr::null_mut();
        }
    }
}

/// Full teardown: tell the DLL to restore everything, then forget it.
fn unload_locked(tap: &mut Tap) {
    if !tap.cmd_hwnd.0.is_null() && tap.msg_unload != 0 {
        // `unload()` inside the DLL restores brushes before returning.
        let _ = send_cmd(tap.cmd_hwnd, tap.msg_unload, 0, 0);
    }
    disarm(tap);
    tap.cmd_hwnd = HWND(core::ptr::null_mut());
    tap.explorer_pid = 0;
}

/// Makes sure the TAP is resident in the current explorer — injects when
/// the pid changed or the command window died. Returns `(hwnd, fresh)`:
/// `fresh` is true when this call (re)injected, meaning per-residency
/// payloads (theme wires, hello) must be re-sent.
fn ensure_resident_locked(
    guard: &mut Option<Tap>,
    taskbar: HWND,
    pid: u32,
    min_version: usize,
) -> Option<(HWND, bool)> {
    let stale = guard
        .as_ref()
        .map(|t| {
            t.explorer_pid != pid
                || unsafe { IsWindow(t.cmd_hwnd) }.0 == 0
                || resident_version(t.cmd_hwnd) < min_version
        })
        .unwrap_or(true);
    if stale && let Some(mut tap) = guard.take() {
        unload_locked(&mut tap);
    }
    let tap = guard.get_or_insert_with(|| Tap {
        hook: core::ptr::null_mut(),
        lib: core::ptr::null_mut(),
        cmd_hwnd: HWND(core::ptr::null_mut()),
        msg_unload: 0,
        explorer_pid: 0,
    });
    let fresh = tap.explorer_pid != pid;
    if fresh {
        // A stale pinned resident was already diagnosed for this explorer —
        // it can't be replaced until explorer restarts, so don't burn the
        // inject poll on every tick. Re-verify before honoring the latch:
        // it may have been set by a foreign window that has since died.
        if min_version > 0 && STALE_RESIDENT_PID.load(Ordering::Acquire) == pid {
            // Prop first (authoritative once stamped); class+owner is the
            // fallback for pre-stamp residents.
            let prop_cmd = cmd_prop(taskbar);
            let cmd = if !prop_cmd.0.is_null() {
                prop_cmd
            } else {
                find_cmd_for_pid(pid).unwrap_or(HWND(core::ptr::null_mut()))
            };
            if !cmd.0.is_null() && resident_owner(cmd) == pid && resident_version(cmd) < min_version
            {
                return None;
            }
            STALE_RESIDENT_PID.store(0, Ordering::Release);
        }
        if !inject(tap, taskbar, min_version) {
            unload_locked(tap);
            return None;
        }
        disarm(tap);
        tap.explorer_pid = pid;
    }
    Some((tap.cmd_hwnd, fresh))
}

/// Residency for the taskbar XAML styler — injects when the pid changed or
/// the command window died. Returns the command window + whether this call
/// (re)injected (callers re-send the theme wire + hello then).
pub fn ensure_resident() -> Option<(HWND, bool)> {
    let (taskbar, pid) = find_taskbar()?;
    let mut guard = lock();
    ensure_resident_locked(&mut guard, taskbar, pid, TAP_PROTOCOL_VERSION)
}

/// The resident command window when the TAP is live in explorer — does
/// **not** inject; `reset` paths use this so they can't resurrect a dead
/// resident.
pub fn resident_window() -> Option<HWND> {
    let guard = lock();
    let tap = guard.as_ref()?;
    let live = !tap.cmd_hwnd.0.is_null() && unsafe { IsWindow(tap.cmd_hwnd) }.0 != 0;
    live.then_some(tap.cmd_hwnd)
}

/// Restarts `explorer.exe` — the troubleshooting escape hatch when the shell
/// (or the resident engine inside it) misbehaves. `taskkill /F` first, then
/// wait for the shell to come back on its own before respawning it: a bare
/// `explorer.exe` launched while the shell already runs opens a stray File
/// Explorer window instead of a shell.
///
/// Everything re-arms itself afterwards: the next anchor tick sees the new
/// pid, `ensure_resident` re-injects, the resident's own `WM_TIMER` retries
/// `InitializeXamlDiagnosticsEx` until the XAML island is up, and
/// `taskbar_styler::ensure` re-delivers the armed theme.
pub fn restart_explorer() -> bool {
    use std::os::windows::process::CommandExt;
    const CREATE_NO_WINDOW: u32 = 0x0800_0000;
    let old_pid = find_taskbar().map(|(_, pid)| pid);
    if old_pid.is_some() {
        let _ = std::process::Command::new("taskkill.exe")
            .args(["/F", "/IM", "explorer.exe"])
            .creation_flags(CREATE_NO_WINDOW)
            .status();
    }
    // Only a different explorer pid counts as "back": `taskkill` returns as
    // soon as the kill is signaled, and the dying tray window lingers during
    // teardown. The shell sometimes respawns on its own — give it a few
    // seconds before starting a second instance (a bare `explorer.exe`
    // launched while the shell runs opens a stray file window instead).
    for _ in 0..40 {
        if let Some((_, pid)) = find_taskbar()
            && Some(pid) != old_pid
        {
            return true;
        }
        std::thread::sleep(std::time::Duration::from_millis(250));
    }
    match find_taskbar() {
        // `spawn_detached` passes lpApplicationName — which skips the PATH
        // search entirely; `Command` puts the name in lpCommandLine where
        // %SystemRoot% is on the path. Null stdio keeps the child from
        // inheriting our pipes (same call `openConfigFolder` uses).
        None => std::process::Command::new("explorer.exe")
            .stdin(std::process::Stdio::null())
            .stdout(std::process::Stdio::null())
            .stderr(std::process::Stdio::null())
            .spawn()
            .is_ok(),
        Some((_, pid)) => Some(pid) != old_pid, // still the old shell = kill failed
    }
}

/// Call on shutdown: restores the taskbar and unloads the DLL if injected.
pub fn shutdown() {
    let mut guard = lock();
    if let Some(mut tap) = guard.take() {
        unload_locked(&mut tap);
    }
}

#[cfg(test)]
mod tests {
    /// `cargo test -p pecofence-platform tap::tests::restart -- --ignored`
    /// exercises the failsafe path against the live shell.
    #[test]
    #[ignore = "restarts the live explorer.exe"]
    fn restart() {
        assert!(super::restart_explorer());
    }
}
