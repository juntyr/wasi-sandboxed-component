#![cfg_attr(not(test), no_main)]

pub mod outgoing_handler;
pub mod types;

#[allow(unknown_lints, clippy::same_length_and_capacity)]
mod bindings {
    wit_bindgen::generate!({
        world: "wasi-sandboxed:http/exports@0.2.6",
        with: {
            "wasi:http/outgoing-handler@0.2.6": generate,
            "wasi:http/types@0.2.6": generate,

            // direct dependencies
            "wasi:io/error@0.2.6": generate,
            "wasi:io/poll@0.2.6": generate,
            "wasi:io/streams@0.2.6": generate,

            "wasi:null/io@0.2.6": generate,

            "wasi:clocks/monotonic-clock@0.2.6": generate,
        },
    });
}

pub enum VirtHttp {}

type WasiIoErrorRef<'a> = &'a crate::bindings::wasi::io::error::Error;

#[cfg(target_arch = "wasm32")]
#[expect(unsafe_code)]
mod export {
    use crate::VirtHttp;
    crate::bindings::export!(VirtHttp with_types_in crate::bindings);
}

#[cfg(test)]
mod tests {
    #[test]
    fn check_wit_deps() -> check_wit_deps::Result<()> {
        check_wit_deps::check_is_locked("wit")
    }
}
