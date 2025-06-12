#![cfg_attr(not(test), no_main)]

#[cfg(not(feature = "merged"))]
use crate::bindings::exports::wasi::{
    io::{
        poll::Pollable,
        streams::{InputStream, OutputStream},
    },
    null::io::Guest as WasiVirtNullIO,
};

pub mod error;
pub mod poll;
pub mod streams;

mod bindings {
    #[cfg(not(feature = "merged"))]
    wit_bindgen::generate!({
        world: "wasi-sandboxed:io/exports@0.2.3",
        with: {
            "wasi:io/error@0.2.3": generate,
            "wasi:io/poll@0.2.3": generate,
            "wasi:io/streams@0.2.3": generate,

            "wasi:null/io@0.2.3": generate,
        },
    });
    #[cfg(feature = "merged")]
    wit_bindgen::generate!({
        world: "wasi-sandboxed:io/merged-exports@0.2.3",
        with: {
            "wasi:io/error@0.2.3": generate,
            "wasi:io/poll@0.2.3": generate,
            "wasi:io/streams@0.2.3": generate,
        },
    });
}

pub enum VirtIO {}

#[cfg(target_arch = "wasm32")]
#[expect(unsafe_code)]
mod export {
    use crate::VirtIO;
    crate::bindings::export!(VirtIO with_types_in crate::bindings);
}

#[cfg(not(feature = "merged"))]
impl WasiVirtNullIO for VirtIO {
    fn ready_pollable() -> Pollable {
        poll::VirtPollable::ready()
    }

    fn closed_input() -> InputStream {
        streams::VirtInputStream::closed()
    }

    fn output_sink() -> OutputStream {
        streams::VirtOutputStream::sink()
    }

    fn stdout() -> OutputStream {
        streams::VirtOutputStream::stdout()
    }

    fn stderr() -> OutputStream {
        streams::VirtOutputStream::stderr()
    }
}

#[cfg(feature = "merged")]
pub mod exports {
    pub use crate::bindings::exports::wasi::io::*;
}

#[cfg(test)]
mod tests {
    #[test]
    fn check_wit_deps() -> check_wit_deps::Result<()> {
        check_wit_deps::check_is_locked("wit")
    }
}
