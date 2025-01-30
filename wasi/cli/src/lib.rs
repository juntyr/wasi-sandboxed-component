#![cfg_attr(not(test), no_main)]

mod environment;
mod exit;
mod stdio;
mod terminal;

mod bindings {
    wit_bindgen::generate!({
        world: "fcbench:wasi/virtual-cli@0.2.2",
        with: {
            "wasi:cli/environment@0.2.2": generate,
            "wasi:cli/exit@0.2.2": generate,
            "wasi:cli/stderr@0.2.2": generate,
            "wasi:cli/stdin@0.2.2": generate,
            "wasi:cli/stdout@0.2.2": generate,
            "wasi:cli/terminal-input@0.2.2": generate,
            "wasi:cli/terminal-output@0.2.2": generate,
            "wasi:cli/terminal-stderr@0.2.2": generate,
            "wasi:cli/terminal-stdin@0.2.2": generate,
            "wasi:cli/terminal-stdout@0.2.2": generate,

            // direct dependencies
            "wasi:io/error@0.2.2": generate,
            "wasi:io/poll@0.2.2": generate,
            "wasi:io/streams@0.2.2": generate,

            "wasi:null/io@0.2.2": generate,
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

#[cfg(test)]
mod tests {
    #[test]
    fn check_wit_deps() -> check_wit_deps::Result<()> {
        check_wit_deps::check_is_locked("wit")
    }
}
