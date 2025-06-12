use crate::bindings::exports::wasi::random::{
    insecure::Guest as WasiRandomInsecure, insecure_seed::Guest as WasiRandomInsecureSeed,
};

use super::VirtRandom;

impl WasiRandomInsecureSeed for VirtRandom {
    fn insecure_seed() -> (u64, u64) {
        (
            Self::get_insecure_random_u64(),
            Self::get_insecure_random_u64(),
        )
    }
}
