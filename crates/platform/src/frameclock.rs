//! A frame tick aligned with the compositor (`DCompositionWaitForCompositorClock`, or
//! `DwmFlush` on Windows 10, which does not have the compositor clock).
//!
//! Compositor-side animations (opacity, offset, clip…) never need this. It exists for the few
//! values that are rasterised *inside* a Direct2D surface each frame — a hover highlight
//! fading, the scroll offset gliding — where the app itself has to redraw. `SetTimer` ticks
//! at 15.6 ms granularity and drifts against vsync; the compositor clock does not.
//!
//! One thread waits on the clock and calls `on_frame` (typically a `PostMessage`) once per
//! compositor tick (60 Hz idle material, up to 120 Hz interaction) **while armed**.
//! Consumers call [`FrameClock::request`] whenever they still have
//! work for the next frame; the clock disarms itself after each delivered frame, so an idle
//! app costs nothing.

use crate::bindings::*;
use crate::wide::to_wide;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Condvar, Mutex};
use std::time::{Duration, Instant};
use windows_core::{PCSTR, PCWSTR};

/// A missing compositor tick must not hold a short client animation past its endpoint.
const COMPOSITOR_TIMEOUT_MS: u32 = 16;
/// Some Windows builds occasionally return from the zero-handle wait without waiting at all.
const IMMEDIATE_WAIT: Duration = Duration::from_millis(1);
/// Idle material cadence and faster interaction cadence, both with bounded delivery.
const CLIENT_FRAME: Duration = Duration::from_nanos(16_666_667);
const INTERACTION_FRAME: Duration = Duration::from_nanos(8_333_334);

/// `DCompositionWaitForCompositorClock(count, handles, timeout_ms)`.
type WaitForCompositorClock = unsafe extern "system" fn(u32, *const HANDLE, u32) -> u32;

/// The compositor clock exists from Windows 11 on. It is looked up at run time so the
/// executable still loads on Windows 10.
fn compositor_clock() -> Option<WaitForCompositorClock> {
    let name = to_wide("dcomp.dll");
    // SAFETY: loading a system DLL by name from System32 and looking up an export; the
    // transmute matches the documented signature of that export.
    unsafe {
        let dll = LoadLibraryExW(
            PCWSTR(name.as_ptr()),
            None,
            LOAD_LIBRARY_SEARCH_SYSTEM32 as u32,
        );
        if dll.0.is_null() {
            return None;
        }
        let proc = GetProcAddress(
            dll,
            PCSTR(c"DCompositionWaitForCompositorClock".as_ptr().cast()),
        )?;
        Some(std::mem::transmute::<
            unsafe extern "system" fn() -> isize,
            WaitForCompositorClock,
        >(proc))
    }
}

struct Shared {
    armed: AtomicBool,
    delivery_busy: AtomicBool,
    fast_motion: AtomicBool,
    quit: AtomicBool,
    wake: Mutex<()>,
    cv: Condvar,
}

pub struct FrameClock {
    shared: Arc<Shared>,
}

impl FrameClock {
    /// Starts the clock thread. `on_frame` runs on that thread once per requested frame and
    /// must be cheap and thread-safe (post a message; never touch UI state).
    pub fn start(on_frame: impl Fn() + Send + 'static) -> Self {
        let shared = Arc::new(Shared {
            armed: AtomicBool::new(false),
            delivery_busy: AtomicBool::new(false),
            fast_motion: AtomicBool::new(false),
            quit: AtomicBool::new(false),
            wake: Mutex::new(()),
            cv: Condvar::new(),
        });
        let worker = shared.clone();
        std::thread::Builder::new()
            .name("frame-clock".into())
            .spawn(move || {
                let clock = compositor_clock();
                if clock.is_none() {
                    tracing::info!("compositor clock unavailable; pacing frames with DwmFlush");
                }
                let mut last_frame = None;
                loop {
                    {
                        let mut guard = worker.wake.lock().unwrap();
                        while (!worker.armed.load(Ordering::Acquire)
                            || worker.delivery_busy.load(Ordering::Acquire))
                            && !worker.quit.load(Ordering::Acquire)
                        {
                            guard = worker.cv.wait(guard).unwrap();
                        }
                    }
                    if worker.quit.load(Ordering::Acquire) {
                        return;
                    }
                    let wait_started = Instant::now();
                    let waited = match clock {
                        // SAFETY: no handles, plain timeout wait. A real compositor signal
                        // keeps the display's native cadence; timeout bounds a missing signal
                        // to 60 Hz.
                        Some(wait) => unsafe { wait(0, std::ptr::null(), COMPOSITOR_TIMEOUT_MS) },
                        // Windows 10: block until the next desktop composition pass.
                        // SAFETY: plain FFI call without arguments.
                        None => {
                            if unsafe { DwmFlush() }.is_ok() {
                                0
                            } else {
                                WAIT_FAILED
                            }
                        }
                    };
                    let waited_for = wait_started.elapsed();
                    let elapsed = last_frame.map_or(waited_for, |t: Instant| t.elapsed());
                    let period = if worker.fast_motion.load(Ordering::Acquire) {
                        INTERACTION_FRAME
                    } else {
                        CLIENT_FRAME
                    };
                    if last_frame.is_some() || waited == WAIT_FAILED || waited_for < IMMEDIATE_WAIT
                    {
                        if let Some(left) = period.checked_sub(elapsed) {
                            std::thread::sleep(left);
                        } else if waited == WAIT_FAILED || waited_for < IMMEDIATE_WAIT {
                            std::thread::sleep(IMMEDIATE_WAIT);
                        }
                    }
                    if worker.armed.swap(false, Ordering::AcqRel) {
                        worker.delivery_busy.store(true, Ordering::Release);
                        last_frame = Some(Instant::now());
                        on_frame();
                    }
                }
            })
            .expect("frame-clock thread");
        Self { shared }
    }

    /// Asks for one more frame callback. Idempotent within a frame.
    pub fn request(&self) {
        if !self.shared.armed.swap(true, Ordering::AcqRel) {
            let _guard = self.shared.wake.lock().unwrap();
            self.shared.cv.notify_one();
        }
    }

    /// Interaction starts without waiting for the next idle-material frame.
    pub fn request_fast(&self) {
        self.shared.fast_motion.store(true, Ordering::Release);
        self.request();
    }

    /// A posted frame remains outstanding until UI work completes, not merely until
    /// PostMessage returns. At most one frame can be queued or executing at a time.
    pub fn finish_frame(&self, more: bool, fast_motion: bool) {
        self.shared
            .fast_motion
            .store(fast_motion, Ordering::Release);
        if more {
            self.shared.armed.store(true, Ordering::Release);
        }
        self.shared.delivery_busy.store(false, Ordering::Release);
        let _guard = self.shared.wake.lock().unwrap();
        self.shared.cv.notify_one();
    }
}

impl Drop for FrameClock {
    fn drop(&mut self) {
        self.shared.quit.store(true, Ordering::Release);
        let _guard = self.shared.wake.lock().unwrap();
        self.shared.cv.notify_one();
    }
}
