#![cfg_attr(not(test), no_main)]

use std::sync::{Mutex, MutexGuard, OnceLock};

use rand_core::RngCore;
use rand_pcg::Pcg64;

use crate::bindings::{
    exports::wasi::random::{
        insecure::Guest as WasiRandomInsecure, insecure_seed::Guest as WasiRandomInsecureSeed,
        random::Guest as WasiRandom,
    },
    // wasi::random::insecure_seed::insecure_seed as insecure_seed_host,
};

mod bindings {
    wit_bindgen::generate!({
        world: "fcbench:wasi/virtual-random@0.2.2",
        with: {
            "wasi:random/insecure@0.2.2": generate,
            "wasi:random/insecure-seed@0.2.2": generate,
            "wasi:random/random@0.2.2": generate,
        },
    });
}

pub enum VirtRandom {}

#[cfg(target_arch = "wasm32")]
#[expect(unsafe_code)]
mod export {
    use crate::VirtRandom;
    crate::bindings::export!(VirtRandom with_types_in crate::bindings);
}

fn rng() -> MutexGuard<'static, Pcg64> {
    static RNG: OnceLock<Mutex<Pcg64>> = OnceLock::new();

    #[cold]
    #[inline(never)]
    fn init_rng() -> &'static Mutex<Pcg64> {
        RNG.get_or_init(|| {
            const PCG64_DEFAULT_STREAM: u128 = 0x0a02_bdbf_7bb3_c0a7_ac28_fa16_a64a_bf96;

            // let (state_lo, state_hi) = insecure_seed_host();
            // let state = u128::from(state_lo) | (u128::from(state_hi) << 64);
            let state = 0xcafe_f00d_d15e_a5e5;

            Mutex::new(Pcg64::new(state, PCG64_DEFAULT_STREAM))
        })
    }

    #[expect(clippy::unwrap_used)]
    RNG.get().unwrap_or_else(|| init_rng()).lock().unwrap()
}

impl WasiRandomInsecureSeed for VirtRandom {
    fn insecure_seed() -> (u64, u64) {
        let mut rng = rng();
        (rng.next_u64(), rng.next_u64())
    }
}

impl WasiRandomInsecure for VirtRandom {
    fn get_insecure_random_bytes(len: u64) -> Vec<u8> {
        #[expect(clippy::unwrap_used)] // FIXME
        let mut buffer = vec![0; usize::try_from(len).unwrap()];
        rng().fill_bytes(&mut buffer);
        buffer
    }

    fn get_insecure_random_u64() -> u64 {
        rng().next_u64()
    }
}

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

#[cfg(test)]
mod tests {
    #[test]
    fn check_wit_deps() -> check_wit_deps::Result<()> {
        check_wit_deps::check_is_locked("wit")
    }
}
