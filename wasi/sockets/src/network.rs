use crate::bindings::exports::wasi::sockets::network::{Guest as WasiSocketsNetwork, GuestNetwork};

use super::VirtSockets;

impl WasiSocketsNetwork for VirtSockets {
    type Network = VirtNetwork;
}

pub struct VirtNetwork;

impl GuestNetwork for VirtNetwork {}
