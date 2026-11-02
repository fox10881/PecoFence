//! Frameless-window support for the glass hosts.
//!
//! The windows keep `WS_CAPTION` (DWM needs it for shadows, snapping, and
//! the Win11 maximize-flyout) but `WM_NCCALCSIZE` returns 0, so the client
//! covers the whole window and the native caption is never drawn — the
//! transparent composition WebView then reaches the top edge. `hit_test`
//! supplies the zones the removed frame used to give us for free: drag,
//! resize edges/corners, and real window buttons (`HTCLOSE`/`HTMAXBUTTON`/
//! `HTMINBUTTON` get SC_* behaviour and the snap flyout from DefWindowProc).

use crate::bindings::{
    self as b, GetClientRect, GetWindowLongPtrW, GetWindowRect, ScreenToClient, GWL_STYLE, HWND,
    POINT, RECT,
};
use crate::window::dpi_of;

// The page titlebar uses the same DIP layout — change both or neither.
const TITLE_H_DIP: i32 = 32;
const BUTTON_W_DIP: i32 = 46;
const BUTTON_H_DIP: i32 = 32;
const EDGE_DIP: i32 = 6;

const WS_MAXIMIZE: isize = 0x0100_0000;
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
