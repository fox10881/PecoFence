//! Host-side controller for the Notification Center styler — the native
//! replacement for the Windhawk `windows-11-notification-center-styler` mod.
//!
//! `pecofence_tap.dll` is injected into `ShellExperienceHost.exe` and
//! `ShellHost.exe` — primarily via thread-scoped `WH_GETMESSAGE` hooks
//! (Windows itself maps the DLL on message retrieval; the only vector
//! that reaches AppContainer hosts like SEH, whose loader stalls
//! foreign-thread `LoadLibrary` calls), with `CreateRemoteThread` as the
//! fallback. Once resident, the DLL runs the styling engine
//! itself: `WM_COPYDATA` delivers the serialized theme
//! (`pecofence_core::notification_center_themes::theme_wire`) to the
//! injected command window (`PecoFenceTAPNC` class family), which styles
//! the Action/Notification Center's `Windows.UI.Xaml` tree in place.
//!
//! Both hosts are spawned lazily by the shell and can idle-exit any time,
//! so `apply` stores the selected style and `ensure()` (driven by the
//! app's periodic tick) re-injects and re-sends it whenever a host
//! restarts.

use crate::bindings::{HWND, LPARAM, SendMessageTimeoutW, WPARAM};
use crate::wide::from_wide;
use std::collections::BTreeMap;
use std::sync::Mutex;
use std::time::{Duration, Instant};
use windows_core::{BOOL, PCSTR, PCWSTR};

windows_core::link!("kernel32.dll" "system" fn CloseHandle(hobject: *mut core::ffi::c_void) -> BOOL);
windows_core::link!("kernel32.dll" "system" fn CreateRemoteThread(process: *mut core::ffi::c_void, attrs: *const core::ffi::c_void, stack: usize, start: *mut core::ffi::c_void, param: *const core::ffi::c_void, flags: u32, tid: *mut u32) -> HANDLE);
windows_core::link!("kernel32.dll" "system" fn CreateToolhelp32Snapshot(dwflags: u32, th32processid: u32) -> *mut core::ffi::c_void);
windows_core::link!("kernel32.dll" "system" fn GetExitCodeThread(thread: HANDLE, code: *mut u32) -> BOOL);
windows_core::link!("kernel32.dll" "system" fn GetProcAddress(hmodule: *mut core::ffi::c_void, lpprocname: PCSTR) -> *mut core::ffi::c_void);
windows_core::link!("kernel32.dll" "system" fn GetModuleHandleW(name: PCWSTR) -> *mut core::ffi::c_void);
windows_core::link!("kernel32.dll" "system" fn OpenProcess(dwdesiredaccess: u32, binherithandle: BOOL, dwprocessid: u32) -> HANDLE);
windows_core::link!("user32.dll" "system" fn PostThreadMessageW(idthread: u32, msg: u32, wparam: WPARAM, lparam: LPARAM) -> BOOL);
windows_core::link!("kernel32.dll" "system" fn Process32FirstW(hsnapshot: *mut core::ffi::c_void, lppe: *mut core::ffi::c_void) -> BOOL);
windows_core::link!("kernel32.dll" "system" fn Process32NextW(hsnapshot: *mut core::ffi::c_void, lppe: *mut core::ffi::c_void) -> BOOL);
windows_core::link!("kernel32.dll" "system" fn Thread32First(hsnapshot: *mut core::ffi::c_void, lpte: *mut core::ffi::c_void) -> BOOL);
windows_core::link!("kernel32.dll" "system" fn Thread32Next(hsnapshot: *mut core::ffi::c_void, lpte: *mut core::ffi::c_void) -> BOOL);
windows_core::link!("kernel32.dll" "system" fn Sleep(dwmilliseconds: u32));
windows_core::link!("kernel32.dll" "system" fn VirtualAllocEx(process: HANDLE, addr: *const core::ffi::c_void, size: usize, alloctype: u32, protect: u32) -> *mut core::ffi::c_void);
windows_core::link!("kernel32.dll" "system" fn VirtualFreeEx(process: HANDLE, addr: *mut core::ffi::c_void, size: usize, freetype: u32) -> BOOL);
windows_core::link!("kernel32.dll" "system" fn WaitForSingleObject(handle: HANDLE, ms: u32) -> u32);
windows_core::link!("kernel32.dll" "system" fn WriteProcessMemory(process: HANDLE, addr: *mut core::ffi::c_void, buf: *const core::ffi::c_void, size: usize, written: *mut usize) -> BOOL);
windows_core::link!("kernel32.dll" "system" fn LoadLibraryW(name: PCWSTR) -> *mut core::ffi::c_void);
windows_core::link!("kernel32.dll" "system" fn FreeLibrary(hmodule: *mut core::ffi::c_void) -> BOOL);
windows_core::link!("user32.dll" "system" fn IsWindow(hwnd: HWND) -> BOOL);
windows_core::link!("user32.dll" "system" fn RegisterWindowMessageW(lpstring: PCWSTR) -> u32);
windows_core::link!("user32.dll" "system" fn SetWindowsHookExW(idhook: i32, lpfn: HookProc, hmod: *mut core::ffi::c_void, dwthreadid: u32) -> *mut core::ffi::c_void);
windows_core::link!("user32.dll" "system" fn SetWinEventHook(eventmin: u32, eventmax: u32, hmod: *mut core::ffi::c_void, proc: *const core::ffi::c_void, idprocess: u32, idthread: u32, flags: u32) -> *mut core::ffi::c_void);
windows_core::link!("user32.dll" "system" fn UnhookWinEvent(hwineventhook: *mut core::ffi::c_void) -> BOOL);
windows_core::link!("user32.dll" "system" fn UnhookWindowsHookEx(hhk: *mut core::ffi::c_void) -> BOOL);

type HookProc = Option<unsafe extern "system" fn(i32, usize, isize) -> isize>;

use crate::bindings::HANDLE;

const SMTO_ABORTIFHUNG: u32 = 0x0002;
const SMTO_BLOCK: u32 = 0x0001;
const SMTO_ERRORONEXIT: u32 = 0x0020;
const WM_COPYDATA: u32 = 0x004A;
/// `COPYDATASTRUCT.dwData` values the injected DLL understands.
const CDS_THEME: usize = u32::from_be_bytes(*b"NCST") as usize;
const CDS_RESET: usize = u32::from_be_bytes(*b"NCSR") as usize;
/// Announces the host pid so the DLL can restore styles if pecofence dies.
const CDS_HELLO: usize = u32::from_be_bytes(*b"NCHI") as usize;

/// Resident class family inside the Notification Center hosts — distinct
/// from explorer's `PecoFenceTAP` and SMEH's `PecoFenceTAPSM` so discovery
/// never confuses residents across process types.
const NC_CLASS_BASE: &str = "PecoFenceTAPNC";
/// Process image names the Notification Center surfaces live in.
const NC_HOSTS: [&str; 2] = ["shellexperiencehost.exe", "shellhost.exe"];

const TH32CS_SNAPPROCESS: u32 = 0x0000_0002;
const TH32CS_SNAPTHREAD: u32 = 0x0000_0004;
const WH_GETMESSAGE: i32 = 3;
const EVENT_OBJECT_CREATE: u32 = 0x8000;
const EVENT_OBJECT_SHOW: u32 = 0x8002;
const WINEVENT_INCONTEXT: u32 = 0x0004;
/// Between failed inject attempts on one host — a PLM-frozen host can't
/// pump the hook anyway, so re-arming every ensure tick just stalls the
/// caller; the armed wake hook covers the thaw.
const INJECT_COOLDOWN: Duration = Duration::from_secs(10);
const PROCESS_CREATE_THREAD: u32 = 0x0002;
const PROCESS_VM_OPERATION: u32 = 0x0008;
const PROCESS_VM_READ: u32 = 0x0010;
const PROCESS_VM_WRITE: u32 = 0x0020;
const PROCESS_QUERY_INFORMATION: u32 = 0x0400;
const MEM_COMMIT: u32 = 0x1000;
const MEM_RESERVE: u32 = 0x2000;
const MEM_RELEASE: u32 = 0x8000;
const PAGE_READWRITE: u32 = 0x04;
const WM_NULL: u32 = 0x0000;

/// `THREADENTRY32` — every field; the struct is a fixed 28 bytes.
#[repr(C)]
struct ThreadEntry {
    size: u32,
    usage: u32,
    tid: u32,
    owner_pid: u32,
    base_pri: i32,
    delta_pri: i32,
    flags: u32,
}

/// `PROCESSENTRY32W` (the fields we read — `szExeFile` tail is MAX_PATH).
#[repr(C)]
struct ProcessEntry {
    size: u32,
    usage: u32,
    pid: u32,
    heap: usize,
    module: u32,
    threads: u32,
    parent_pid: u32,
    base_pri: i32,
    flags: u32,
    exe_file: [u16; 260],
}

#[repr(C)]
struct CopyDataStruct {
    dw_data: usize,
    cb_data: u32,
    lp_data: *const core::ffi::c_void,
}

/// One injected Notification Center host process.
struct NcHost {
    /// The in-host command window (`PecoFenceTAPNC` class, owned by `pid`).
    cmd_hwnd: HWND,
    /// Whether `CDS_HELLO` was delivered for this pid (arms the DLL's host
    /// death-watch once per host lifetime).
    hello_sent: bool,
    /// The `Nc::wire_gen` last ACKed by this resident. `!= wire_gen` means
    /// the current theme/reset hasn't reached the host — `ensure()`
    /// redelivers. Generational (not a bool) so a theme switch can't be
    /// clobbered by a stale in-flight delivery.
    delivered_gen: u64,
}
unsafe impl Send for NcHost {}

#[derive(Default)]
struct Nc {
    /// The catalog id last applied ("" = Windows Default).
    style: String,
    /// The theme serialized for transport — cached so a re-inject after a
    /// host restart doesn't rebuild it.
    wire: String,
    /// Bumped by every `apply()` — each host's `delivered_gen` is compared
    /// against this to decide what still needs delivering.
    wire_gen: u64,
    /// `pid -> resident channel` for each live Notification Center host.
    hosts: BTreeMap<u32, NcHost>,
    /// `pid -> (hook, lib)` — `WINEVENT_INCONTEXT` triggers armed on
    /// suspended hosts that don't carry a resident. The OS maps the DLL
    /// into the host when it thaws and fires its first accessibility
    /// event — exactly when the panel is about to render.
    wake_hooks: BTreeMap<u32, (*mut core::ffi::c_void, *mut core::ffi::c_void)>,
    /// `pid -> next allowed inject attempt` — cooldown so a frozen host
    /// doesn't stall an `ensure` tick on a poll that can't succeed.
    next_inject: BTreeMap<u32, Instant>,
}
unsafe impl Send for Nc {}

static NC: Mutex<Nc> = Mutex::new(Nc {
    style: String::new(),
    wire: String::new(),
    wire_gen: 0,
    hosts: BTreeMap::new(),
    wake_hooks: BTreeMap::new(),
    next_inject: BTreeMap::new(),
});

fn lock() -> std::sync::MutexGuard<'static, Nc> {
    NC.lock().unwrap_or_else(|p| p.into_inner())
}

fn wide(s: &str) -> Vec<u16> {
    s.encode_utf16().chain(core::iter::once(0)).collect()
}

/// Pids of every running `ShellExperienceHost.exe`/`ShellHost.exe` — the
/// shell spawns them lazily and they can idle-exit any time.
fn find_nc_pids() -> Vec<u32> {
    let snap = unsafe { CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, 0) };
    if snap.is_null() {
        return Vec::new();
    }
    let mut pe = ProcessEntry {
        size: core::mem::size_of::<ProcessEntry>() as u32,
        usage: 0,
        pid: 0,
        heap: 0,
        module: 0,
        threads: 0,
        parent_pid: 0,
        base_pri: 0,
        flags: 0,
        exe_file: [0; 260],
    };
    let mut found = Vec::new();
    let mut cur = unsafe { Process32FirstW(snap, (&mut pe as *mut ProcessEntry).cast()) };
    while cur.0 != 0 {
        let len = pe
            .exe_file
            .iter()
            .position(|&c| c == 0)
            .unwrap_or(pe.exe_file.len());
        let name = from_wide(&pe.exe_file[..len]);
        if NC_HOSTS.iter().any(|h| name.eq_ignore_ascii_case(h)) {
            found.push(pe.pid);
        }
        cur = unsafe { Process32NextW(snap, (&mut pe as *mut ProcessEntry).cast()) };
    }
    unsafe {
        let _ = CloseHandle(snap);
    }
    found
}

/// A host's `PecoFenceTAPNC` command window (message-only — `EnumWindows`
/// can't enumerate these; the versioned-class enumeration in `tap`
/// resolves them). Only THIS build's resident counts: a same-protocol
/// older-hash window is a stale engine that would silently drop wires it
/// can't parse, so discovery must skip it and let `inject` co-load a
/// fresh image beside it.
fn find_cmd_window(pid: u32) -> Option<HWND> {
    crate::tap::resident_current(pid, NC_CLASS_BASE)
}

/// `WM_COPYDATA` payload send — waits up to 3 s for the injected DLL to
/// consume the serialized theme.
fn send_copydata(hwnd: HWND, tag: usize, payload: &[u8]) -> bool {
    let cds = CopyDataStruct {
        dw_data: tag,
        cb_data: payload.len() as u32,
        lp_data: payload.as_ptr() as *const core::ffi::c_void,
    };
    let mut result = 0usize;
    unsafe {
        SendMessageTimeoutW(
            hwnd,
            WM_COPYDATA,
            WPARAM(0),
            LPARAM(&cds as *const CopyDataStruct as isize),
            SMTO_ABORTIFHUNG | SMTO_BLOCK | SMTO_ERRORONEXIT,
            3_000,
            Some(&mut result),
        )
        .0 != 0
    }
}

/// Outcome of one `inject_hook` attempt — distinguishes "no hook could
/// be armed" (worth trying remote-thread) from "hooks armed but the
/// resident never booted" (remote-thread would only leave a hung thread
/// in an AppContainer host).
enum HookResult {
    Done(HWND),
    NoHooks,
    TimedOut,
}

/// Maps `pecofence_tap.dll` into a Notification Center host, then waits
/// for its command window. Thread-scoped `WH_GETMESSAGE` hooks are tried
/// first — Windows maps the DLL itself on message retrieval, which is
/// the only vector that reaches AppContainer hosts (`ShellExperienceHost`
/// stalls foreign-thread `LoadLibrary` calls indefinitely). Remote-thread
/// injection remains the fallback for hosts whose threads can't be
/// hooked at all.
fn inject(pid: u32) -> Option<HWND> {
    match inject_hook(pid) {
        HookResult::Done(cmd) => Some(cmd),
        HookResult::TimedOut => None,
        HookResult::NoHooks => inject_remote_thread(pid),
    }
}

/// Remote-thread injection — the classic `CreateRemoteThread(LoadLibraryW)`
/// vector (kernel32's image base is shared per-boot, so the local
/// `LoadLibraryW` address is the remote one). Works for `ShellHost`; for
/// `ShellExperienceHost` the remote thread stalls inside the AppContainer
/// loader and never returns, so this runs only when hooks can't be armed.
fn inject_remote_thread(pid: u32) -> Option<HWND> {
    // Unique-per-build module name — a stale pinned resident can't be
    // evicted by re-injecting the same path (LoadLibrary dedups), so the
    // versioned copy is the only way a fresh resident co-loads. The
    // AppContainer-readable copy keeps packaged hosts able to load it.
    let path = match crate::tap::tap_injectable_path_appcontainer() {
        Some(p) if p.exists() => p,
        _ => return None,
    };
    let wide_path = wide(&path.to_string_lossy());
    let k32 = unsafe { GetModuleHandleW(PCWSTR(wide("kernel32.dll").as_ptr())) };
    if k32.is_null() {
        return None;
    }
    let load = unsafe { GetProcAddress(k32, PCSTR(c"LoadLibraryW".as_ptr() as *const u8)) };
    if load.is_null() {
        return None;
    }
    let proc = unsafe {
        OpenProcess(
            PROCESS_CREATE_THREAD
                | PROCESS_VM_OPERATION
                | PROCESS_VM_READ
                | PROCESS_VM_WRITE
                | PROCESS_QUERY_INFORMATION,
            BOOL(0),
            pid,
        )
    };
    if proc.0.is_null() {
        return None;
    }
    let ok = unsafe {
        let bytes = wide_path.len() * 2;
        let mem = VirtualAllocEx(
            proc,
            core::ptr::null(),
            bytes,
            MEM_COMMIT | MEM_RESERVE,
            PAGE_READWRITE,
        );
        let mut wrote = 0usize;
        if mem.is_null()
            || WriteProcessMemory(
                proc,
                mem,
                wide_path.as_ptr() as *const core::ffi::c_void,
                bytes,
                &mut wrote,
            )
            .0 == 0
            || wrote != bytes
        {
            let _ = CloseHandle(proc.0);
            return None;
        }
        let thread = CreateRemoteThread(
            proc.0,
            core::ptr::null(),
            0,
            load,
            mem,
            0,
            core::ptr::null_mut(),
        );
        if thread.0.is_null() {
            let _ = VirtualFreeEx(proc, mem, 0, MEM_RELEASE);
            let _ = CloseHandle(proc.0);
            return None;
        }
        // The remote thread exits when LoadLibrary returns — bounded wait.
        let _ = WaitForSingleObject(thread, 5_000);
        let mut code = 0u32;
        let _ = GetExitCodeThread(thread, &mut code);
        let _ = CloseHandle(thread.0);
        if code == 0x103 {
            // STILL_ACTIVE: the remote LoadLibrary is hung (AppContainer
            // loader lock). Freeing `mem` here leaves the live remote
            // thread holding a dangling path pointer — a guaranteed AV
            // inside the host when it finally runs. Leak the one page
            // deliberately; the resident still registers itself if the
            // thread ever completes, and `ensure` discovers the window.
            let _ = CloseHandle(proc.0);
            return None;
        }
        let _ = VirtualFreeEx(proc, mem, 0, MEM_RELEASE);
        let _ = CloseHandle(proc.0);
        code != 0 // exit code = the remote HMODULE
    };
    if !ok {
        return None;
    }
    // Up to 5 s for the DLL to boot its command window inside the host.
    for _ in 0..100 {
        if let Some(cmd) = find_cmd_window(pid) {
            // One resident per process — unwind any stale co-loaded build.
            crate::tap::unload_foreign_residents_matching(pid, cmd, NC_CLASS_BASE);
            return Some(cmd);
        }
        unsafe { Sleep(50) };
    }
    None
}

/// Hook injection for the Notification Center hosts — a thread-scoped
/// `WH_GETMESSAGE` hook lets Windows map the DLL into the target on
/// message retrieval, which is the only vector that reaches
/// `ShellExperienceHost` (its AppContainer loader stalls foreign-thread
/// `LoadLibrary` calls). The host keeps no top-level windows at idle, so
/// EVERY one of its threads is hooked and each is kicked with a
/// `PostThreadMessageW(WM_NULL)` — any queued thread's next `GetMessage`
/// loads the resident. Threads without a queue reject the post
/// harmlessly. The resident self-pins in `DllMain`, so the local handle
/// unloads once the hooks are down.
fn inject_hook(pid: u32) -> HookResult {
    // AppContainer-readable copy — hook-driven loads resolve the file
    // inside the host's security context.
    let Some(path) = crate::tap::tap_injectable_path_appcontainer() else {
        return HookResult::TimedOut;
    };
    let lib = unsafe { LoadLibraryW(PCWSTR(wide(&path.to_string_lossy()).as_ptr())) };
    if lib.is_null() {
        return HookResult::NoHooks;
    }
    let proc = unsafe { GetProcAddress(lib, PCSTR(c"CallWndProc".as_ptr() as *const u8)) };
    if proc.is_null() {
        unsafe {
            let _ = FreeLibrary(lib);
        }
        return HookResult::NoHooks;
    }
    // Hook every thread the host owns — SEH has dozens and only some
    // pump messages; coverage costs nothing.
    let mut hooks = Vec::new();
    for tid in thread_ids(pid) {
        let h = unsafe {
            SetWindowsHookExW(
                WH_GETMESSAGE,
                core::mem::transmute::<*mut core::ffi::c_void, HookProc>(proc),
                lib,
                tid,
            )
        };
        if !h.is_null() {
            hooks.push(h);
        }
    }
    if hooks.is_empty() {
        unsafe {
            let _ = FreeLibrary(lib);
        }
        return HookResult::NoHooks;
    }
    // Kick every queue so the hook fires now rather than on the host's
    // next UI activity — posts to queue-less threads just fail.
    for tid in thread_ids(pid) {
        unsafe {
            let _ = PostThreadMessageW(tid, WM_NULL, WPARAM(0), LPARAM(0));
        }
    }
    // Up to ~1.5 s for the DLL to boot its command window inside the host —
    // an awake host maps the DLL on the first kicked message; a frozen one
    // never will, so a longer poll only stalls the caller before the wake
    // hook takes over.
    let mut result = HookResult::TimedOut;
    for _ in 0..30 {
        if let Some(cmd) = find_cmd_window(pid) {
            crate::tap::unload_foreign_residents_matching(pid, cmd, NC_CLASS_BASE);
            result = HookResult::Done(cmd);
            break;
        }
        unsafe { Sleep(50) };
    }
    unsafe {
        for h in hooks {
            let _ = UnhookWindowsHookEx(h);
        }
        let _ = FreeLibrary(lib);
    }
    result
}

/// Every thread id owned by `pid` (Toolhelp snapshot).
fn thread_ids(pid: u32) -> Vec<u32> {
    let snap = unsafe { CreateToolhelp32Snapshot(TH32CS_SNAPTHREAD, 0) };
    if snap.is_null() {
        return Vec::new();
    }
    let mut te = ThreadEntry {
        size: core::mem::size_of::<ThreadEntry>() as u32,
        usage: 0,
        tid: 0,
        owner_pid: 0,
        base_pri: 0,
        delta_pri: 0,
        flags: 0,
    };
    let mut out = Vec::new();
    let mut cur = unsafe { Thread32First(snap, (&mut te as *mut ThreadEntry).cast()) };
    while cur.0 != 0 {
        if te.owner_pid == pid {
            out.push(te.tid);
        }
        cur = unsafe { Thread32Next(snap, (&mut te as *mut ThreadEntry).cast()) };
    }
    unsafe {
        let _ = CloseHandle(snap);
    }
    out
}

/// Sends `PecoFence.TAP.Unload` + a reset to one host's resident, best
/// effort — the host may already be dead, in which case both sends fail
/// silently and the slot is dropped anyway.
fn unload_host(hwnd: HWND) {
    if hwnd.0.is_null() {
        return;
    }
    let _ = send_copydata(hwnd, CDS_RESET, &[]);
    let msg = unsafe { RegisterWindowMessageW(PCWSTR(wide("PecoFence.TAP.Unload").as_ptr())) };
    if msg == 0 {
        return;
    }
    let mut result = 0usize;
    unsafe {
        let _ = SendMessageTimeoutW(
            hwnd,
            msg,
            WPARAM(0),
            LPARAM(0),
            SMTO_ABORTIFHUNG | SMTO_BLOCK | SMTO_ERRORONEXIT,
            3_000,
            Some(&mut result),
        );
    }
}

/// Per-host delivery: find-or-inject a channel, then push the armed wire
/// (or `NCSR` reset). Blocking Win32 calls run WITHOUT the `nc` mutex —
/// state is snapshotted and committed in short locked sections so a
/// frozen host can never stall `apply`/`ensure`/`shutdown`.
fn deliver_host(pid: u32) -> bool {
    let stale = {
        let nc = lock();
        nc.hosts
            .get(&pid)
            .map(|h| h.cmd_hwnd.0.is_null() || unsafe { IsWindow(h.cmd_hwnd) }.0 == 0)
            .unwrap_or(true)
    };
    if stale {
        let found = find_cmd_window(pid).or_else(|| {
            if lock().wire.is_empty() {
                return None; // reset semantics — a resident-less host is already stock
            }
            // Cooldown between inject attempts — a frozen host can't pump
            // the message hook, so polling again within the window just
            // stalls the caller.
            {
                let nc = lock();
                if nc
                    .next_inject
                    .get(&pid)
                    .is_some_and(|t| Instant::now() < *t)
                {
                    return None;
                }
            }
            let cmd = inject(pid);
            if cmd.is_none() {
                lock()
                    .next_inject
                    .insert(pid, Instant::now() + INJECT_COOLDOWN);
            }
            cmd
        });
        match found {
            Some(cmd) => {
                disarm_wake_hook(pid);
                lock().hosts.insert(
                    pid,
                    NcHost {
                        cmd_hwnd: cmd,
                        hello_sent: false,
                        delivered_gen: 0,
                    },
                );
            }
            None => {
                lock().hosts.remove(&pid);
                if lock().wire.is_empty() {
                    return true;
                }
                // No resident and injection timed out — the host is almost
                // certainly PLM-frozen. Arm the wake trigger: Windows maps
                // the resident itself the moment the host thaws and fires
                // its first accessibility event, then `ensure` lands the
                // armed wire. Failing THAT means no channel exists at all.
                return arm_wake_hook(pid);
            }
        }
    }
    // Snapshot the delivery — the blocking sends below run lock-free.
    let (hwnd, wire, generation, hello_done) = {
        let nc = lock();
        match nc.hosts.get(&pid) {
            Some(h) => (h.cmd_hwnd, nc.wire.clone(), nc.wire_gen, h.hello_sent),
            None => return false,
        }
    };
    let ok = if wire.is_empty() {
        send_copydata(hwnd, CDS_RESET, &[])
    } else {
        // Arm the DLL's host death-watch once per host lifetime — styles
        // revert to stock if pecofence dies.
        if !hello_done {
            let own = std::process::id();
            if send_copydata(hwnd, CDS_HELLO, &own.to_ne_bytes()) {
                if let Some(h) = lock().hosts.get_mut(&pid) {
                    h.hello_sent = true;
                }
            }
        }
        send_copydata(hwnd, CDS_THEME, wire.as_bytes())
    };
    if ok {
        // Commit the generation actually sent — a concurrent apply() that
        // bumped wire_gen keeps this host "undelivered" for the new wire,
        // so a stale in-flight send can't clobber a newer theme.
        if let Some(h) = lock().hosts.get_mut(&pid) {
            h.delivered_gen = generation;
        }
    }
    true
}

/// Arms a `WINEVENT_INCONTEXT` trigger scoped to `pid`: the OS maps the
/// resident DLL into the host when it fires an accessibility event —
/// for a PLM-frozen host that's the first thing it does after thawing,
/// so the resident lands exactly when the panel needs styling. Already
/// armed = already guaranteed eventual delivery.
fn arm_wake_hook(pid: u32) -> bool {
    if lock().wake_hooks.contains_key(&pid) {
        return true;
    }
    let Some(path) = crate::tap::tap_injectable_path_appcontainer() else {
        return false;
    };
    let lib = unsafe { LoadLibraryW(PCWSTR(wide(&path.to_string_lossy()).as_ptr())) };
    if lib.is_null() {
        return false;
    }
    let proc = unsafe { GetProcAddress(lib, PCSTR(c"WinEventProc".as_ptr() as *const u8)) };
    if proc.is_null() {
        unsafe {
            let _ = FreeLibrary(lib);
        }
        return false;
    }
    let hook = unsafe {
        SetWinEventHook(
            EVENT_OBJECT_CREATE,
            EVENT_OBJECT_SHOW,
            lib,
            proc,
            pid,
            0,
            WINEVENT_INCONTEXT,
        )
    };
    if hook.is_null() {
        unsafe {
            let _ = FreeLibrary(lib);
        }
        return false;
    }
    lock().wake_hooks.insert(pid, (hook, lib));
    true
}

/// Drops a host's armed wake trigger (spent once a resident is live).
/// `UnhookWinEvent` on a dead pid is a harmless no-op.
fn disarm_wake_hook(pid: u32) {
    let entry = lock().wake_hooks.remove(&pid);
    if let Some((hook, lib)) = entry {
        unsafe {
            let _ = UnhookWinEvent(hook);
            let _ = FreeLibrary(lib);
        }
    }
}

/// Applies the selected Notification Center style.
///
/// `id` is a catalog id (`""` = Windows Default). Returns `false` only when
/// a running host rejected the command channel — when no host is running
/// the selection is stored and applied on each host's next launch, so the
/// call still reports success.
pub fn apply(id: &str) -> bool {
    let wire = if id.is_empty() {
        String::new()
    } else {
        match pecofence_core::notification_center_themes::theme_wire(id) {
            Some(w) => w,
            None => return false,
        }
    };
    {
        let mut nc = lock();
        nc.style = id.to_string();
        nc.wire = wire;
        nc.wire_gen += 1;
    }
    // Style cleared — pending wake triggers only ever deliver a RESET,
    // so disarm them rather than inject a resident that does nothing.
    if id.is_empty() {
        let armed: Vec<u32> = lock().wake_hooks.keys().copied().collect();
        for pid in armed {
            disarm_wake_hook(pid);
        }
    }
    let pids = find_nc_pids();
    // Drop hosts that died — their windows are gone anyway.
    let mut dead_hwnds = Vec::new();
    {
        let mut nc = lock();
        let dead: Vec<u32> = nc
            .hosts
            .keys()
            .copied()
            .filter(|p| !pids.contains(p))
            .collect();
        for pid in dead {
            nc.next_inject.remove(&pid);
            if let Some(host) = nc.hosts.remove(&pid) {
                dead_hwnds.push(host.cmd_hwnd);
            }
        }
    }
    for hwnd in dead_hwnds {
        unload_host(hwnd);
    }
    let mut ok = true;
    for pid in pids {
        if !deliver_host(pid) {
            ok = false;
        }
    }
    ok
}

/// Cheap liveness check driven by the app's periodic tick: re-injects and
/// re-delivers the selected style when a Notification Center host restarted
/// or launched after the style was chosen.
pub fn ensure() {
    {
        let nc = lock();
        // Idle fast-path — nothing armed, nothing tracked, nothing pending.
        if nc.style.is_empty() && nc.hosts.is_empty() && nc.wake_hooks.is_empty() {
            return;
        }
    }
    let pids = find_nc_pids();
    // Which hosts still need a push — undelivered generation (theme OR
    // reset), dead window, or a missing death-watch hello. A frozen host's
    // failed reset lands here on the next tick once it thaws.
    let needs: Vec<u32> = {
        let nc = lock();
        let generation = nc.wire_gen;
        let themed = !nc.wire.is_empty();
        pids.iter()
            .copied()
            .filter(|pid| {
                nc.hosts.get(pid).is_none_or(|h| {
                    h.cmd_hwnd.0.is_null()
                        || unsafe { IsWindow(h.cmd_hwnd) }.0 == 0
                        || h.delivered_gen != generation
                        || (themed && !h.hello_sent)
                })
            })
            .collect()
    };
    for pid in needs {
        let _ = deliver_host(pid);
    }
    // Prune slots for hosts that exited — windows, cooldowns, wake hooks.
    let live = find_nc_pids();
    let (dead_hwnds, dead_hooks) = {
        let mut nc = lock();
        let dead: Vec<u32> = nc
            .hosts
            .keys()
            .copied()
            .filter(|p| !live.contains(p))
            .collect();
        let mut hwnds = Vec::new();
        for pid in dead {
            if let Some(host) = nc.hosts.remove(&pid) {
                hwnds.push(host.cmd_hwnd);
            }
        }
        let dh: Vec<u32> = nc
            .wake_hooks
            .keys()
            .copied()
            .filter(|p| !live.contains(p))
            .collect();
        let mut hooks = Vec::new();
        for pid in dh {
            if let Some(e) = nc.wake_hooks.remove(&pid) {
                hooks.push(e);
            }
        }
        let ni: Vec<u32> = nc
            .next_inject
            .keys()
            .copied()
            .filter(|p| !live.contains(p))
            .collect();
        for pid in ni {
            nc.next_inject.remove(&pid);
        }
        (hwnds, hooks)
    };
    for hwnd in dead_hwnds {
        unload_host(hwnd);
    }
    for (hook, lib) in dead_hooks {
        unsafe {
            let _ = UnhookWinEvent(hook);
            let _ = FreeLibrary(lib);
        }
    }
}

/// Call on shutdown: restores the Notification Center and unloads the DLL
/// from every injected host.
pub fn shutdown() {
    let (hwnds, hooks) = {
        let mut nc = lock();
        let hosts = std::mem::take(&mut nc.hosts);
        let hooks = std::mem::take(&mut nc.wake_hooks);
        (
            hosts
                .into_values()
                .map(|h| h.cmd_hwnd)
                .collect::<Vec<HWND>>(),
            hooks.into_values().collect::<Vec<_>>(),
        )
    };
    for hwnd in hwnds {
        unload_host(hwnd);
    }
    for (hook, lib) in hooks {
        unsafe {
            let _ = UnhookWinEvent(hook);
            let _ = FreeLibrary(lib);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Diagnoses the live injection path against the running shell hosts —
    /// prints the pids found; does not apply a theme (run `apply` through
    /// the settings CLI for that).
    #[test]
    #[ignore]
    fn probe_nc() {
        let pids = find_nc_pids();
        eprintln!("find_nc_pids -> {pids:?}");
        for pid in pids {
            eprintln!(
                "cmd_window[{pid}] -> {:?}",
                find_cmd_window(pid).map(|h| h.0)
            );
        }
    }
}
