#![cfg_attr(not(test), no_main)]

pub mod preopens;
pub mod types;

mod bindings {
    wit_bindgen::generate!({
        world: "wasi-sandboxed:filesystem/exports@0.2.4",
        with: {
            "wasi:filesystem/preopens@0.2.4": generate,
            "wasi:filesystem/types@0.2.4": generate,

            // direct dependencies
            "wasi:io/error@0.2.4": generate,
            "wasi:io/poll@0.2.4": generate,
            "wasi:io/streams@0.2.4": generate,

            "wasi:clocks/wall-clock@0.2.4": generate,
        },
    });
}

pub enum VirtFilesystem {}

type WasiIoErrorRef<'a> = &'a crate::bindings::wasi::io::error::Error;

#[cfg(target_arch = "wasm32")]
#[expect(unsafe_code)]
mod export {
    use crate::VirtFilesystem;
    crate::bindings::export!(VirtFilesystem with_types_in crate::bindings);
}

#[cfg(test)]
mod tests {
    #[test]
    fn check_wit_deps() -> check_wit_deps::Result<()> {
        check_wit_deps::check_is_locked("wit")
    }
}
