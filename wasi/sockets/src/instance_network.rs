use crate::bindings::exports::wasi::sockets::instance_network::{
    Guest as WasiSocketsInstanceNetwork, Network,
};

use super::network::VirtNetwork;
use super::VirtSockets;

impl WasiSocketsInstanceNetwork for VirtSockets {
    fn instance_network() -> Network {
        Network::new(VirtNetwork)
    }
}
