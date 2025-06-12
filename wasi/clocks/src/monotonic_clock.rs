use std::sync::atomic::{AtomicU64, Ordering};

use crate::bindings::exports::wasi::clocks::monotonic_clock::{
    Duration, Guest as WasiClocksMonotonicClock, Instant,
};
use crate::bindings::wasi::{io::poll::Pollable, null::io::ready_pollable};

use super::VirtClock;

static CLOCK_NS: AtomicU64 = AtomicU64::new(0);

impl WasiClocksMonotonicClock for VirtClock {
    fn now() -> Instant {
        CLOCK_NS.load(Ordering::SeqCst)
    }

    fn resolution() -> Duration {
        1 // ns
    }

    fn subscribe_instant(when: Instant) -> Pollable {
        CLOCK_NS.fetch_max(when, Ordering::SeqCst);
        ready_pollable()
    }

    fn subscribe_duration(when: Duration) -> Pollable {
        CLOCK_NS.fetch_add(when, Ordering::SeqCst);
        ready_pollable()
    }
}
