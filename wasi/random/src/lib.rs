#![cfg_attr(not(test), no_main)]

pub mod insecure;
pub mod insecure_seed;
pub mod random;

mod bindings {
    wit_bindgen::generate!({
        world: "wasi-sandboxed:random/exports@0.2.3",
        with: {
            "wasi:random/insecure@0.2.3": generate,
            "wasi:random/insecure-seed@0.2.3": generate,
            "wasi:random/random@0.2.3": generate,
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

#[cfg(test)]
mod tests {
    #[test]
    fn check_wit_deps() -> check_wit_deps::Result<()> {
        check_wit_deps::check_is_locked("wit")
    }
}
