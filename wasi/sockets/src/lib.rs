#![cfg_attr(not(test), no_main)]

pub mod instance_network;
pub mod ip_name_lookup;
pub mod network;
pub mod tcp;
pub mod tcp_create_socket;
pub mod udp;
pub mod udp_create_socket;

mod bindings {
    wit_bindgen::generate!({
        world: "wasi-sandboxed:sockets/exports@0.2.6",
        with: {
            "wasi:sockets/instance-network@0.2.6": generate,
            "wasi:sockets/network@0.2.6": generate,
            "wasi:sockets/udp@0.2.6": generate,
            "wasi:sockets/udp-create-socket@0.2.6": generate,
            "wasi:sockets/tcp@0.2.6": generate,
            "wasi:sockets/tcp-create-socket@0.2.6": generate,
            "wasi:sockets/ip-name-lookup@0.2.6": generate,

            // direct dependencies
            "wasi:io/error@0.2.6": generate,
            "wasi:io/poll@0.2.6": generate,
            "wasi:io/streams@0.2.6": generate,

            "wasi:clocks/monotonic-clock@0.2.6": generate,
        },
    });
}

pub enum VirtSockets {}

#[cfg(target_arch = "wasm32")]
#[expect(unsafe_code)]
mod export {
    use crate::VirtSockets;
    crate::bindings::export!(VirtSockets with_types_in crate::bindings);
}

#[cfg(test)]
mod tests {
    #[test]
    fn check_wit_deps() -> check_wit_deps::Result<()> {
        check_wit_deps::check_is_locked("wit")
    }
}
