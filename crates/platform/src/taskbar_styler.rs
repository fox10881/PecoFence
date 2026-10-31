//! Host-side controller for the taskbar XAML styler — the native
//! replacement for the Windhawk `windows-11-taskbar-styler` mod.
//!
//! `tap::ensure_resident` guarantees `pecofence_tap.dll` is resident in
//! `explorer.exe` with its command window up, then `WM_COPYDATA` delivers
//! the serialized theme (`pecofence_core::taskbar_themes::theme_wire`, a
//! `PECO-TBS1` wire) which the injected styler applies to the taskbar's
//! XAML tree in place. The DLL's death watch restores stock styling if
//! pecofence.exe dies.

use std::sync::Mutex;
use std::sync::atomic::{AtomicBool, Ordering};
use windows_core::{PCSTR, PCWSTR};

windows_core::link!("kernel32.dll" "system" fn GetModuleHandleW(name: PCWSTR) -> *mut core::ffi::c_void);
windows_core::link!("kernel32.dll" "system" fn GetProcAddress(hmodule: *mut core::ffi::c_void, lpprocname: PCSTR) -> *mut core::ffi::c_void);

/// `COPYDATASTRUCT.dwData` values — must match `crates/tap/src/command.rs`.
const CDS_TB_THEME: usize = u32::from_be_bytes(*b"TBST") as usize;
const CDS_TB_RESET: usize = u32::from_be_bytes(*b"TBSR") as usize;
const CDS_TB_HELLO: usize = u32::from_be_bytes(*b"TBHI") as usize;

struct Tb {
    /// The catalog id last delivered ("" = Windows Default — nothing armed).
    style: String,
    /// The wire last built — cached so `ensure` re-delivers after an
    /// explorer restart without re-serializing the catalog.
    wire: String,
    /// The resident command window the wire was delivered to (`HWND` as
    /// `usize` for `Send`) — resident identity, not `explorer_pid`: after a
    /// restart the `fresh` flag may already be consumed by another caller on
    /// this tick, so the hwnd is the only reliable "this resident never got
    /// the theme" signal.
    cmd_hwnd: usize,
    /// Whether `CDS_TB_HELLO` reached the current DLL residency (the death
    /// watch arms once per injection — a fresh resident needs it again).
    hello_sent: bool,
    /// The armed theme is intentionally withheld while the taskbar is
    /// vertically docked (left/right). Win11's taskbar has no vertical XAML
    /// mode; on builds that force one the layout churns without converging,
    /// and our property writes inside that churn freeze the shell UI thread
    /// — the taskbar never renders and explorer's input loop starves.
    suppressed: bool,
}

static TB: Mutex<Option<Tb>> = Mutex::new(None);
/// Whether a taskbar theme is armed — read lock-free so callers never race
/// the TB mutex order in `reset`.
static ARMED: AtomicBool = AtomicBool::new(false);

/// `true` while a taskbar style is armed — `tap` keeps the DLL resident.
pub fn is_armed() -> bool {
    ARMED.load(Ordering::Acquire)
}

fn lock() -> std::sync::MutexGuard<'static, Option<Tb>> {
    TB.lock().unwrap_or_else(|p| p.into_inner())
}

// ---- `RtlQueryFeatureConfiguration` (Squircle's weather-variant probe) ----

#[repr(C)]
struct RtlFeatureConfiguration {
    feature_id: u32,
    /// `group:4 | enabledState:2 | enabledStateOptions:1 | unused1:1 |
    /// variant:6 | variantPayloadKind:2 | unused2:16`
    flags: u32,
    payload: u32,
}

type RtlQueryFn =
    unsafe extern "system" fn(u32, i32, *mut i64, *mut RtlFeatureConfiguration) -> i32;

/// `IsOsFeatureEnabled` — enabledState 2 = Enabled (1 = Disabled, 0 =
/// Default). `false` when the query fails — the base payload then applies.
fn feature_enabled(feature_id: u32) -> bool {
    let ntdll = unsafe { GetModuleHandleW(PCWSTR(widestr("ntdll.dll").as_ptr())) };
    if ntdll.is_null() {
        return false;
    }
    let f = unsafe {
        GetProcAddress(
            ntdll,
            PCSTR(c"RtlQueryFeatureConfiguration".as_ptr() as *const u8),
        )
    };
    if f.is_null() {
        return false;
    }
    let f: RtlQueryFn = unsafe { core::mem::transmute(f) };
    let mut cfg = RtlFeatureConfiguration {
        feature_id: 0,
        flags: 0,
        payload: 0,
    };
    let mut stamp = 0i64;
    if unsafe { f(feature_id, 1, &mut stamp, &mut cfg) } < 0 {
        return false;
    }
    (cfg.flags >> 4) & 0b11 == 2
}

fn widestr(s: &str) -> Vec<u16> {
    s.encode_utf16().chain(core::iter::once(0)).collect()
}

/// `true` when any taskbar window is taller than wide — docked left or
/// right. Measured from the real `Shell_TrayWnd`/`Shell_SecondaryTrayWnd`
/// rects (not the work area, which side app-bars also deform). While this
/// holds the styler must not deliver a theme: the vertical XAML layout
/// never converges, and each property write inside the visual-tree churn
/// keeps explorer's UI thread saturated.
fn taskbar_is_vertical() -> bool {
    use crate::bindings::{FindWindowExW, GetWindowRect};
    for class in ["Shell_TrayWnd", "Shell_SecondaryTrayWnd"] {
        let wide = widestr(class);
        let mut hwnd = unsafe { FindWindowExW(None, None, PCWSTR(wide.as_ptr()), PCWSTR::null()) };
        while !hwnd.0.is_null() {
            let mut rc = crate::bindings::RECT::default();
            if unsafe { GetWindowRect(hwnd, &mut rc) }.as_bool()
                && rc.bottom - rc.top > rc.right - rc.left
            {
                return true;
            }
            hwnd =
                unsafe { FindWindowExW(None, Some(hwnd), PCWSTR(wide.as_ptr()), PCWSTR::null()) };
        }
    }
    false
}

/// Sends the stored wire to the resident DLL. A resident this session never
/// delivered to (hwnd differs) gets the hello re-armed first — the wire
/// itself is (re)sent by the caller's decision.
fn deliver(hwnd: crate::bindings::HWND, tb: &mut Tb) -> bool {
    if tb.cmd_hwnd != hwnd.0 as usize {
        tb.hello_sent = false;
    }
    if !tb.hello_sent {
        let pid = std::process::id();
        if crate::tap::send_copydata(hwnd, CDS_TB_HELLO, &pid.to_ne_bytes()) {
            tb.hello_sent = true;
        }
    }
    if crate::tap::send_copydata(hwnd, CDS_TB_THEME, tb.wire.as_bytes()) {
        tb.cmd_hwnd = hwnd.0 as usize;
        true
    } else {
        false
    }
}

/// Applies the selected taskbar style + color tint. `id` is a catalog id
/// (`""` = no theme — tint-only when `tint` is set, `Windows Default` →
/// reset when both are empty). The tint rides the same wire as appended
/// `controlStyles` rules, so a later theme switch or tint clear restores
/// cleanly through the normal re-apply path. Returns `false` only for an
/// unknown id so settings reverts.
pub fn apply(id: &str, tint: &str) -> bool {
    let use_variant =
        pecofence_core::taskbar_themes::variant_feature(id).is_some_and(feature_enabled);
    let wire = if id.is_empty() {
        pecofence_core::color_tint::tint_wire(
            "PECO-TBS1",
            pecofence_core::color_tint::TASKBAR_TARGETS,
            tint,
        )
    } else {
        pecofence_core::taskbar_themes::theme_wire(id, use_variant).map(|mut w| {
            pecofence_core::color_tint::apply_to_wire(
                &mut w,
                pecofence_core::color_tint::TASKBAR_TARGETS,
                tint,
            );
            w
        })
    };
    let Some(wire) = wire else {
        if id.is_empty() {
            return reset();
        }
        return false;
    };
    let vertical = taskbar_is_vertical();
    let mut guard = lock();
    let tb = guard.get_or_insert_with(|| Tb {
        style: String::new(),
        wire: String::new(),
        cmd_hwnd: 0,
        hello_sent: false,
        suppressed: false,
    });
    tb.style = id.to_string();
    tb.wire = wire;
    ARMED.store(true, Ordering::Release);
    if let Some((hwnd, _)) = crate::tap::ensure_resident() {
        if vertical {
            // Keep the theme armed but send the reset: the resident restores
            // originals and our property writes stay out of the vertical
            // layout churn until the taskbar goes horizontal again.
            let _ = crate::tap::send_copydata(hwnd, CDS_TB_RESET, &[]);
            tb.suppressed = true;
            tb.cmd_hwnd = 0;
        } else {
            tb.suppressed = false;
            if !deliver(hwnd, tb) {
                tb.cmd_hwnd = 0; // delivery pending — `ensure` retries next tick
            }
        }
    }
    true
}

/// `Windows Default` — tell the resident styler to restore originals and
/// stop keeping the DLL alive on our account. Does **not** inject: a dead
/// resident has nothing to reset.
pub fn reset() -> bool {
    let mut guard = lock();
    if let Some(hwnd) = crate::tap::resident_window() {
        let _ = crate::tap::send_copydata(hwnd, CDS_TB_RESET, &[]);
    }
    if let Some(tb) = guard.as_mut() {
        tb.style.clear();
        tb.wire.clear();
    }
    ARMED.store(false, Ordering::Release);
    true
}

/// Periodic liveness driven by the app's anchor tick: re-injects and
/// re-delivers the wire when explorer restarted under an armed style.
/// Otherwise sends a `HELLO` — cheap poke that re-arms the DLL's watcher
/// retry and the death watch if either was lost.
pub fn ensure() {
    if !is_armed() {
        return;
    }
    let Some((hwnd, _)) = crate::tap::ensure_resident() else {
        return;
    };
    let vertical = taskbar_is_vertical();
    let mut guard = lock();
    let Some(tb) = guard.as_mut() else {
        return;
    };
    if vertical {
        // Once per transition — the reset is idempotent but each send runs
        // a full restore sweep inside the host.
        if !tb.suppressed {
            let _ = crate::tap::send_copydata(hwnd, CDS_TB_RESET, &[]);
            tb.suppressed = true;
            tb.cmd_hwnd = 0;
        }
        return;
    }
    if tb.suppressed {
        tb.suppressed = false;
        tb.cmd_hwnd = 0; // force re-delivery of the armed theme
    }
    // Resident identity decides re-delivery — a same-tick reinjection may
    // have already consumed the `fresh` flag.
    if tb.cmd_hwnd != hwnd.0 as usize {
        let _ = deliver(hwnd, tb);
    }
}

/// App shutdown — restore stock styling; the transparency teardown in
/// `tap::shutdown` unloads the resident itself.
pub fn shutdown() {
    let _ = reset();
}
