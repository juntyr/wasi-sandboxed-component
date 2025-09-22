#![cfg_attr(not(test), no_main)]

pub mod cli;
pub mod clocks;
pub mod filesystem;
pub mod io;
pub mod random;

mod bindings {
    wit_bindgen::generate!({
        world: "wasi-sandboxed:io/exports@0.2.6",
        with: {
            "wasi:cli/environment@0.2.6": generate,
            "wasi:cli/exit@0.2.6": generate,
            "wasi:cli/stderr@0.2.6": generate,
            "wasi:cli/stdin@0.2.6": generate,
            "wasi:cli/stdout@0.2.6": generate,
            "wasi:cli/terminal-input@0.2.6": generate,
            "wasi:cli/terminal-output@0.2.6": generate,
            "wasi:cli/terminal-stderr@0.2.6": generate,
            "wasi:cli/terminal-stdin@0.2.6": generate,
            "wasi:cli/terminal-stdout@0.2.6": generate,

            "wasi:clocks/monotonic-clock@0.2.6": generate,
            "wasi:clocks/wall-clock@0.2.6": generate,

            "wasi:filesystem/preopens@0.2.6": generate,
            "wasi:filesystem/types@0.2.6": generate,

            "wasi:io/error@0.2.6": generate,
            "wasi:io/poll@0.2.6": generate,
            "wasi:io/streams@0.2.6": generate,

            "wasi:random/insecure@0.2.6": generate,
            "wasi:random/insecure-seed@0.2.6": generate,
            "wasi:random/random@0.2.6": generate,
        },
    });

    pub mod wasi {
        pub mod io {
            pub mod poll {
                pub use crate::bindings::exports::wasi::io::poll::Pollable;
            }

            pub mod streams {
                pub use crate::bindings::exports::wasi::io::streams::{InputStream, OutputStream};
            }
        }

        pub mod null {
            pub mod io {
                pub fn ready_pollable() -> crate::bindings::exports::wasi::io::poll::Pollable {
                    crate::io::poll::VirtPollable::ready()
                }

                pub fn closed_input() -> crate::bindings::exports::wasi::io::streams::InputStream {
                    crate::io::streams::VirtInputStream::closed()
                }

                pub fn stdout() -> crate::bindings::exports::wasi::io::streams::OutputStream {
                    crate::io::streams::VirtOutputStream::stdout()
                }

                pub fn stderr() -> crate::bindings::exports::wasi::io::streams::OutputStream {
                    crate::io::streams::VirtOutputStream::stderr()
                }
            }
        }
    }
}

pub enum VirtMerged {}

#[cfg(target_arch = "wasm32")]
#[expect(unsafe_code)]
mod export {
    use crate::VirtMerged;
    crate::bindings::export!(VirtMerged with_types_in crate::bindings);
}

#[cfg(test)]
mod tests {
    #[test]
    fn check_wit_deps() -> check_wit_deps::Result<()> {
        check_wit_deps::check_is_locked("wit")
    }
}
