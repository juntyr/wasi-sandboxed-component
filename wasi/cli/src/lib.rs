#![cfg_attr(not(test), no_main)]

pub mod environment;
pub mod exit;
pub mod stdio;
pub mod terminal;

mod bindings {
    #[cfg(not(feature = "merged"))]
    wit_bindgen::generate!({
        world: "wasi-sandboxed:cli/exports@0.2.3",
        with: {
            "wasi:cli/environment@0.2.3": generate,
            "wasi:cli/exit@0.2.3": generate,
            "wasi:cli/stderr@0.2.3": generate,
            "wasi:cli/stdin@0.2.3": generate,
            "wasi:cli/stdout@0.2.3": generate,
            "wasi:cli/terminal-input@0.2.3": generate,
            "wasi:cli/terminal-output@0.2.3": generate,
            "wasi:cli/terminal-stderr@0.2.3": generate,
            "wasi:cli/terminal-stdin@0.2.3": generate,
            "wasi:cli/terminal-stdout@0.2.3": generate,

            // direct dependencies
            "wasi:io/error@0.2.3": generate,
            "wasi:io/poll@0.2.3": generate,
            "wasi:io/streams@0.2.3": generate,

            "wasi:null/io@0.2.3": generate,
        },
    });
    #[cfg(feature = "merged")]
    wit_bindgen::generate!({
        world: "wasi-sandboxed:cli/merged-exports@0.2.3",
        with: {
            "wasi:cli/environment@0.2.3": generate,
            "wasi:cli/exit@0.2.3": generate,
            "wasi:cli/stderr@0.2.3": generate,
            "wasi:cli/stdin@0.2.3": generate,
            "wasi:cli/stdout@0.2.3": generate,
            "wasi:cli/terminal-input@0.2.3": generate,
            "wasi:cli/terminal-output@0.2.3": generate,
            "wasi:cli/terminal-stderr@0.2.3": generate,
            "wasi:cli/terminal-stdin@0.2.3": generate,
            "wasi:cli/terminal-stdout@0.2.3": generate,

            // direct dependencies
            "wasi:io/error@0.2.3": wasi_sandboxed_io::exports::error,
            "wasi:io/poll@0.2.3": wasi_sandboxed_io::exports::poll,
            "wasi:io/streams@0.2.3": wasi_sandboxed_io::exports::streams,
        },
    });
}

pub enum VirtCli {}

#[cfg(target_arch = "wasm32")]
#[expect(unsafe_code)]
mod export {
    use crate::VirtCli;
    crate::bindings::export!(VirtCli with_types_in crate::bindings);
}

#[cfg(feature = "merged")]
pub mod exports {
    pub use crate::bindings::exports::wasi::cli::*;
}

#[cfg(test)]
mod tests {
    #[test]
    fn check_wit_deps() -> check_wit_deps::Result<()> {
        check_wit_deps::check_is_locked("wit")
    }
}
