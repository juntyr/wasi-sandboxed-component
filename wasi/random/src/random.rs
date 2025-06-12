use super::VirtRandom;
use crate::bindings::exports::wasi::random::{
    insecure::Guest as WasiRandomInsecure, random::Guest as WasiRandom,
};

// FIXME: we should *not* implement this interface, as we only provide
//         deterministic randomness
//        however, the wasip1 adapter maps the hashmap insecure seed
//         initialisation to the secure RNG
impl WasiRandom for VirtRandom {
    fn get_random_bytes(len: u64) -> Vec<u8> {
        Self::get_insecure_random_bytes(len)
    }

    fn get_random_u64() -> u64 {
        Self::get_insecure_random_u64()
    }
}
