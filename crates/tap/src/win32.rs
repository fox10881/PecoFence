//! Win32 declarations used inside the injected DLL. Same `windows_core::link!`
//! convention as `pecofence-platform`'s bindings; only what the TAP needs.
#![allow(
    non_snake_case,
    non_camel_case_types,
    clippy::upper_case_acronyms,
    dead_code
)]

pub use crate::bindings::HWND;
pub use windows_core::{BOOL, PCSTR, PCWSTR, PWSTR};

pub type WPARAM = usize;
pub type LPARAM = isize;
pub type LRESULT = isize;

pub type HANDLE = *mut core::ffi::c_void;
pub type HINSTANCE = *mut core::ffi::c_void;
pub type HMODULE = *mut core::ffi::c_void;
pub type FARPROC = *mut core::ffi::c_void;
pub type ATOM = u16;
pub type WNDPROC = Option<unsafe extern "system" fn(HWND, u32, WPARAM, LPARAM) -> LRESULT>;
pub type WAITORTIMERCALLBACK = Option<unsafe extern "system" fn(*mut core::ffi::c_void, BOOL)>;

#[repr(C)]
pub struct POINT {
    pub x: i32,
    pub y: i32,
}

#[repr(C)]
pub struct MSG {
    pub hwnd: HWND,
    pub message: u32,
    pub wParam: WPARAM,
    pub lParam: LPARAM,
    pub time: u32,
    pub pt: POINT,
    pub lPrivate: u32,
}

#[repr(C)]
pub struct WNDCLASSEXW {
    pub cbSize: u32,
    pub style: u32,
    pub lpfnWndProc: WNDPROC,
    pub cbClsExtra: i32,
    pub cbWndExtra: i32,
    pub hInstance: HINSTANCE,
    pub hIcon: *mut core::ffi::c_void,
    pub hCursor: *mut core::ffi::c_void,
    pub hbrBackground: *mut core::ffi::c_void,
    pub lpszMenuName: PCWSTR,
    pub lpszClassName: PCWSTR,
    pub hIconSm: *mut core::ffi::c_void,
}

pub const DLL_PROCESS_ATTACH: u32 = 1;
pub const HWND_MESSAGE: HWND = HWND(-3isize as *mut core::ffi::c_void);
pub const WM_QUIT: u32 = 0x0012;
pub const INFINITE: u32 = 0xFFFF_FFFF;
pub const WT_EXECUTEONLYONCE: u32 = 0x0000_0008;
pub const SYNCHRONIZE: u32 = 0x0010_0000;
pub const PROCESS_QUERY_LIMITED_INFORMATION: u32 = 0x1000;

windows_core::link!("kernel32.dll" "system" fn CloseHandle(hobject: HANDLE) -> BOOL);
windows_core::link!("kernel32.dll" "system" fn CreateEventW(lpeventattributes: *const core::ffi::c_void, bmanualreset: BOOL, binitialstate: BOOL, lpname: PCWSTR) -> HANDLE);
windows_core::link!("kernel32.dll" "system" fn DisableThreadLibraryCalls(hlibmodule: HMODULE) -> BOOL);
windows_core::link!("kernel32.dll" "system" fn GetCurrentProcessId() -> u32);
windows_core::link!("kernel32.dll" "system" fn GetModuleFileNameW(hmodule: HMODULE, lpfilename: PWSTR, nsize: u32) -> u32);
windows_core::link!("kernel32.dll" "system" fn GetModuleHandleW(lpmodulename: PCWSTR) -> HMODULE);
windows_core::link!("kernel32.dll" "system" fn GetProcAddress(hmodule: HMODULE, lpprocname: PCSTR) -> FARPROC);
windows_core::link!("kernel32.dll" "system" fn LoadLibraryW(lplibfilename: PCWSTR) -> HMODULE);
windows_core::link!("kernel32.dll" "system" fn OpenProcess(dwdesiredaccess: u32, binherithandle: BOOL, dwprocessid: u32) -> HANDLE);
windows_core::link!("kernel32.dll" "system" fn RegisterWaitForSingleObject(phnewwaitobject: *mut HANDLE, hobject: HANDLE, callback: WAITORTIMERCALLBACK, context: *const core::ffi::c_void, dwmilliseconds: u32, dwflags: u32) -> BOOL);
windows_core::link!("kernel32.dll" "system" fn SetEvent(hevent: HANDLE) -> BOOL);
windows_core::link!("kernel32.dll" "system" fn Sleep(dwmilliseconds: u32));
windows_core::link!("kernel32.dll" "system" fn UnregisterWaitEx(waithandle: HANDLE, completionevent: HANDLE) -> BOOL);
windows_core::link!("kernel32.dll" "system" fn WaitForSingleObject(hhandle: HANDLE, dwmilliseconds: u32) -> u32);
windows_core::link!("ole32.dll" "system" fn RoInitialize(roinittype: i32) -> windows_core::HRESULT);
windows_core::link!("user32.dll" "system" fn CallNextHookEx(hhk: *mut core::ffi::c_void, ncode: i32, wparam: WPARAM, lparam: LPARAM) -> LRESULT);
windows_core::link!("user32.dll" "system" fn CreateWindowExW(dwexstyle: u32, lpclassname: PCWSTR, lpwindowname: PCWSTR, dwstyle: u32, x: i32, y: i32, nwidth: i32, nheight: i32, hwndparent: HWND, hmenu: *mut core::ffi::c_void, hinstance: HINSTANCE, lpparam: *const core::ffi::c_void) -> HWND);
windows_core::link!("user32.dll" "system" fn DefWindowProcW(hwnd: HWND, msg: u32, wparam: WPARAM, lparam: LPARAM) -> LRESULT);
windows_core::link!("user32.dll" "system" fn DestroyWindow(hwnd: HWND) -> BOOL);
windows_core::link!("user32.dll" "system" fn DispatchMessageW(lpmsg: *const MSG) -> LRESULT);
windows_core::link!("user32.dll" "system" fn FindWindowExW(hwndparent: HWND, hwndchildafter: HWND, lpszclass: PCWSTR, lpszwindow: PCWSTR) -> HWND);
windows_core::link!("user32.dll" "system" fn GetClassNameW(hwnd: HWND, lpclassname: PWSTR, nmaxcount: i32) -> i32);
windows_core::link!("user32.dll" "system" fn GetMessageW(lpmsg: *mut MSG, hwnd: HWND, wmsgfiltermin: u32, wmsgfiltermax: u32) -> BOOL);
windows_core::link!("user32.dll" "system" fn PostQuitMessage(nexitcode: i32));
windows_core::link!("user32.dll" "system" fn RegisterClassExW(lpwndclassex: *const WNDCLASSEXW) -> ATOM);
windows_core::link!("user32.dll" "system" fn SetTimer(hwnd: HWND, nidevent: usize, uelapse: u32, lptimerfunc: *const core::ffi::c_void) -> usize);
windows_core::link!("user32.dll" "system" fn RegisterWindowMessageW(lpstring: PCWSTR) -> u32);
windows_core::link!("user32.dll" "system" fn TranslateMessage(lpmsg: *const MSG) -> BOOL);
