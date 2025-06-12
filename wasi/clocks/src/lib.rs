#![cfg_attr(not(test), no_main)]

pub mod monotonic_clock;
pub mod wall_clock;

mod bindings {
    wit_bindgen::generate!({
        world: "wasi-sandboxed:clocks/exports@0.2.3",
        with: {
            "wasi:clocks/monotonic-clock@0.2.3": generate,
            "wasi:clocks/wall-clock@0.2.3": generate,

            // direct dependencies
            "wasi:io/error@0.2.3": generate,
            "wasi:io/poll@0.2.3": generate,
            "wasi:io/streams@0.2.3": generate,

            "wasi:null/io@0.2.3": generate,
        },
    });
}

pub enum VirtClock {}

#[cfg(target_arch = "wasm32")]
#[expect(unsafe_code)]
mod export {
    use crate::VirtClock;
    crate::bindings::export!(VirtClock with_types_in crate::bindings);
}

#[cfg(test)]
mod tests {
    #[test]
    fn check_wit_deps() -> check_wit_deps::Result<()> {
        check_wit_deps::check_is_locked("wit")
    }
}
