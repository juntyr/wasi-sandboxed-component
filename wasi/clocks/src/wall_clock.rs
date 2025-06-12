use crate::bindings::exports::wasi::clocks::{
    monotonic_clock::Guest as WasiClocksMonotonicClock,
    wall_clock::{Datetime, Guest as WasiClocksWallClock},
};

use super::VirtClock;

impl WasiClocksWallClock for VirtClock {
    fn now() -> Datetime {
        const NANOSECONDS: u32 = 1_000_000_000;

        let now_ns = <Self as WasiClocksMonotonicClock>::now();

        #[expect(clippy::cast_possible_truncation)]
        let nanoseconds = (now_ns % u64::from(NANOSECONDS)) as u32;
        let seconds = now_ns / u64::from(NANOSECONDS);

        Datetime {
            seconds,
            nanoseconds,
        }
    }

    fn resolution() -> Datetime {
        Datetime {
            seconds: 0,
            nanoseconds: 1,
        }
    }
}
