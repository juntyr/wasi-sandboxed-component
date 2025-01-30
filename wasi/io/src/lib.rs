#![cfg_attr(not(test), no_main)]

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
    wit_bindgen::generate!({
        world: "fcbench:wasi/virtual-io@0.2.2",
        with: {
            "wasi:io/error@0.2.2": generate,
            "wasi:io/poll@0.2.2": generate,
            "wasi:io/streams@0.2.2": generate,

            "wasi:null/io@0.2.2": generate,
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

#[cfg(test)]
mod tests {
    #[test]
    fn check_wit_deps() -> check_wit_deps::Result<()> {
        check_wit_deps::check_is_locked("wit")
    }
}
