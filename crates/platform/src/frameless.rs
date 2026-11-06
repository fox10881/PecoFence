//! Frameless-window support for the glass hosts.
//!
//! The windows keep `WS_CAPTION` (DWM needs it for shadows, snapping, and
//! the Win11 maximize-flyout) but `WM_NCCALCSIZE` returns 0, so the client
//! covers the whole window and the native caption is never drawn — the
//! transparent composition WebView then reaches the top edge. `hit_test`
//! supplies the zones the removed frame used to give us for free: drag,
//! resize edges/corners, and real window buttons (`HTCLOSE`/`HTMAXBUTTON`/
//! `HTMINBUTTON` get SC_* behaviour and the snap flyout from DefWindowProc).

use std::cell::Cell;
use std::time::Instant;

use crate::bindings::{
    self as b, GWL_STYLE, GetClientRect, GetWindowLongPtrW, GetWindowRect, HWND, POINT, RECT,
    SM_CXDOUBLECLK, SM_CYDOUBLECLK, ScreenToClient, ShowWindow,
};
use crate::msg;
use crate::window::{self, dpi_of};

// The page titlebar uses the same DIP layout — change both or neither.
const TITLE_H_DIP: i32 = 32;
const BUTTON_W_DIP: i32 = 46;
const BUTTON_H_DIP: i32 = 32;
const EDGE_DIP: i32 = 6;

const WS_MAXIMIZE: isize = 0x0100_0000;
const SW_MAXIMIZE: i32 = 3;
const SW_MINIMIZE: i32 = 6;
const SW_RESTORE: i32 = 9;
const DWMWA_EXTENDED_FRAME_BOUNDS: u32 = 9;
const SM_CXSIZEFRAME: i32 = 32;
const SM_CYSIZEFRAME: i32 = 33;
const SM_CXPADDEDBORDER: i32 = 92;

const HTCLIENT: isize = 1;
const HTCAPTION: isize = 2;
const HTMINBUTTON: isize = 8;
const HTMAXBUTTON: isize = 9;
const HTLEFT: isize = 10;
const HTRIGHT: isize = 11;
const HTTOP: isize = 12;
const HTTOPLEFT: isize = 13;
const HTTOPRIGHT: isize = 14;
const HTBOTTOM: isize = 15;
const HTBOTTOMLEFT: isize = 16;
const HTBOTTOMRIGHT: isize = 17;
const HTCLOSE: isize = 20;

/// `NCCALCSIZE_PARAMS` — only `rgrc` is ever touched.
#[repr(C)]
struct NcCalcSizeParams {
    rgrc: [RECT; 3],
    lppos: *mut core::ffi::c_void,
}

pub fn is_zoomed(hwnd: HWND) -> bool {
    // SAFETY: any live HWND is valid for a GWL_STYLE read.
    unsafe { GetWindowLongPtrW(hwnd, GWL_STYLE) & WS_MAXIMIZE != 0 }
}

fn dip(hwnd: HWND, v: i32) -> i32 {
    v * dpi_of(hwnd) as i32 / 96
}

/// `WM_NCCALCSIZE` answer for the frameless look: the client covers the
/// whole window so the native caption is never painted. A maximized window
/// is positioned off-screen by the invisible DWM resize borders, so the
/// proposed client rect must be re-inset by that thickness or the content
/// (and the taskbar edge) clips.
pub fn calc_size(hwnd: HWND, wparam: usize, lparam: isize) -> isize {
    if wparam == 0 {
        return 0;
    }
    if is_zoomed(hwnd) {
        // SAFETY: lParam is NCCALCSIZE_PARAMS for the duration of this
        // message; both rect reads use out-pointers to locals.
        let params = unsafe { &mut *(lparam as *mut NcCalcSizeParams) };
        let mut wr = RECT::default();
        let mut efb = RECT::default();
        unsafe {
            let _ = GetWindowRect(hwnd, &mut wr);
        }
        let ok = unsafe {
            b::DwmGetWindowAttribute(
                hwnd,
                DWMWA_EXTENDED_FRAME_BOUNDS,
                &mut efb as *mut _ as *mut core::ffi::c_void,
                core::mem::size_of::<RECT>() as u32,
            )
            .is_ok()
        };
        if ok {
            params.rgrc[0].left += efb.left - wr.left;
            params.rgrc[0].top += efb.top - wr.top;
            params.rgrc[0].right -= wr.right - efb.right;
            params.rgrc[0].bottom -= wr.bottom - efb.bottom;
        } else {
            // SAFETY: metric reads on a live HWND.
            let dpi = unsafe { b::GetDpiForWindow(hwnd) };
            let (bx, by) = unsafe {
                (
                    b::GetSystemMetricsForDpi(SM_CXSIZEFRAME, dpi)
                        + b::GetSystemMetricsForDpi(SM_CXPADDEDBORDER, dpi),
                    b::GetSystemMetricsForDpi(SM_CYSIZEFRAME, dpi)
                        + b::GetSystemMetricsForDpi(SM_CXPADDEDBORDER, dpi),
                )
            };
            params.rgrc[0].left += bx;
            params.rgrc[0].top += by;
            params.rgrc[0].right -= bx;
            params.rgrc[0].bottom -= by;
        }
    }
    0
}

/// `WM_NCHITTEST` zones, ordered corners → edges → caption buttons →
/// caption strip → client. The button rects mirror the page titlebar's
/// right-aligned 3×46px cluster inset by the resize edge.
pub fn hit_test(hwnd: HWND, lparam: isize) -> isize {
    let pt = POINT {
        x: (lparam & 0xffff) as u16 as i16 as i32,
        y: ((lparam >> 16) & 0xffff) as u16 as i16 as i32,
    };
    let mut pt = pt;
    // SAFETY: in/out pointer to a local on a live HWND.
    unsafe {
        let _ = ScreenToClient(hwnd, &mut pt);
    }
    let mut rc = RECT::default();
    // SAFETY: out-pointer to a local on a live HWND.
    unsafe {
        let _ = GetClientRect(hwnd, &mut rc);
    }

    let border = dip(hwnd, EDGE_DIP);
    let title_h = dip(hwnd, TITLE_H_DIP);
    let btn_w = dip(hwnd, BUTTON_W_DIP);
    let btn_h = dip(hwnd, BUTTON_H_DIP);

    if !is_zoomed(hwnd) {
        // Corners first, then edges.
        if pt.x < border && pt.y < border {
            return HTTOPLEFT;
        }
        if pt.x >= rc.right - border && pt.y < border {
            return HTTOPRIGHT;
        }
        if pt.x < border && pt.y >= rc.bottom - border {
            return HTBOTTOMLEFT;
        }
        if pt.x >= rc.right - border && pt.y >= rc.bottom - border {
            return HTBOTTOMRIGHT;
        }
        if pt.x < border {
            return HTLEFT;
        }
        if pt.x >= rc.right - border {
            return HTRIGHT;
        }
        if pt.y < border {
            return HTTOP;
        }
        if pt.y >= rc.bottom - border {
            return HTBOTTOM;
        }
    }

    // Right-aligned button cluster, inset by the resize edge.
    let btn_right = rc.right - border;
    let btn_top = (title_h - btn_h) / 2;
    if pt.y >= btn_top && pt.y < btn_top + btn_h {
        if pt.x >= btn_right - btn_w && pt.x < btn_right {
            return HTCLOSE;
        }
        if pt.x >= btn_right - 2 * btn_w && pt.x < btn_right - btn_w {
            return HTMAXBUTTON;
        }
        if pt.x >= btn_right - 3 * btn_w && pt.x < btn_right - 2 * btn_w {
            return HTMINBUTTON;
        }
    }
    if pt.y < title_h {
        return HTCAPTION;
    }
    HTCLIENT
}

const WIDGET_EDGE_DIP: i32 = 8;

/// Widget variant for chrome-free surfaces (the desktop clock): the `8px`
/// perimeter resizes and *every* interior point drags — `HTCAPTION` makes
/// `DefWindowProc` run the `SC_MOVE` loop, no title bar needed. Corners
/// first, then edges; a `WS_POPUP` has no frame to subtract.
pub fn widget_hit_test(hwnd: HWND, lparam: isize) -> isize {
    let mut pt = POINT {
        x: (lparam & 0xffff) as u16 as i16 as i32,
        y: ((lparam >> 16) & 0xffff) as u16 as i16 as i32,
    };
    // SAFETY: in/out pointer to a local on a live HWND.
    unsafe {
        let _ = ScreenToClient(hwnd, &mut pt);
    }
    let mut rc = RECT::default();
    // SAFETY: out-pointer to a local on a live HWND.
    unsafe {
        let _ = GetClientRect(hwnd, &mut rc);
    }
    let border = dip(hwnd, WIDGET_EDGE_DIP);
    if pt.x < border && pt.y < border {
        return HTTOPLEFT;
    }
    if pt.x >= rc.right - border && pt.y < border {
        return HTTOPRIGHT;
    }
    if pt.x < border && pt.y >= rc.bottom - border {
        return HTBOTTOMLEFT;
    }
    if pt.x >= rc.right - border && pt.y >= rc.bottom - border {
        return HTBOTTOMRIGHT;
    }
    if pt.x < border {
        return HTLEFT;
    }
    if pt.x >= rc.right - border {
        return HTRIGHT;
    }
    if pt.y < border {
        return HTTOP;
    }
    if pt.y >= rc.bottom - border {
        return HTBOTTOM;
    }
    HTCAPTION
}

/// `WM_NCLBUTTON{DOWN,DBLCLK}` handling for the caption zones `hit_test`
/// reports.
///
/// `DefWindowProc`'s own button tracker computes the hit rects from real
/// frame metrics — a window whose `WM_NCCALCSIZE` client covers everything
/// has no frame for those rects, so its track never completes and Min/Max/
/// Close clicks silently die (verified live: hit codes correct, `ShowWindow`
/// fine, the DOWN->SC_* hop dead). And on a maximized window the
/// drag-to-restore track swallows the second caption press before Windows
/// can convert it to `WM_NCLBUTTONDBLCLK` — `last_cap` carries the previous
/// caption press (time + screen point) so a quick second `NCLBUTTONDOWN`
/// toggles zoom ourselves. (`CS_DBLCLKS` still delivers a real DBLCLK for
/// the restored-state case; both paths end here.)
pub fn nc_button(
    hwnd: HWND,
    message: u32,
    wparam: usize,
    lparam: isize,
    last_cap: &Cell<Option<(Instant, i32, i32)>>,
) -> Option<isize> {
    match message {
        m if m == msg::WM_NCLBUTTONDOWN => match wparam as isize {
            HTMINBUTTON => unsafe {
                // SAFETY: plain FFI call.
                let _ = ShowWindow(hwnd, SW_MINIMIZE);
            },
            HTMAXBUTTON => unsafe {
                // SAFETY: plain FFI call; SW_RESTORE also unmaximizes.
                let _ = ShowWindow(
                    hwnd,
                    if is_zoomed(hwnd) {
                        SW_RESTORE
                    } else {
                        SW_MAXIMIZE
                    },
                );
            },
            HTCLOSE => {
                window::post_message(hwnd, msg::WM_CLOSE, 0, 0);
            }
            HTCAPTION => {
                let (x, y) = nc_point(lparam);
                if let Some((t, lx, ly)) = last_cap.replace(None) {
                    let fast =
                        t.elapsed().as_millis() <= unsafe { b::GetDoubleClickTime() } as u128;
                    let (dx, dy) = ((x - lx).abs(), (y - ly).abs());
                    let (cx, cy) = unsafe {
                        (
                            b::GetSystemMetricsForDpi(SM_CXDOUBLECLK, dpi_of(hwnd)),
                            b::GetSystemMetricsForDpi(SM_CYDOUBLECLK, dpi_of(hwnd)),
                        )
                    };
                    if fast && dx <= cx && dy <= cy {
                        unsafe {
                            // SAFETY: plain FFI call.
                            let _ = ShowWindow(
                                hwnd,
                                if is_zoomed(hwnd) {
                                    SW_RESTORE
                                } else {
                                    SW_MAXIMIZE
                                },
                            );
                        }
                        return Some(0);
                    }
                }
                last_cap.set(Some((Instant::now(), x, y)));
                return None;
            }
            _ => return None,
        },
        m if m == msg::WM_NCLBUTTONDBLCLK => match wparam as isize {
            HTCAPTION | HTMAXBUTTON => {
                last_cap.set(None);
                unsafe {
                    // SAFETY: plain FFI call.
                    let _ = ShowWindow(
                        hwnd,
                        if is_zoomed(hwnd) {
                            SW_RESTORE
                        } else {
                            SW_MAXIMIZE
                        },
                    );
                }
            }
            _ => return None,
        },
        _ => return None,
    }
    Some(0)
}

fn nc_point(lparam: isize) -> (i32, i32) {
    (
        (lparam & 0xffff) as u16 as i16 as i32,
        ((lparam >> 16) & 0xffff) as u16 as i16 as i32,
    )
}
