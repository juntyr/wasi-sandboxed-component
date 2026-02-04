use crate::bindings::exports::wasi::sockets::network::{ErrorCode, IpAddressFamily};
use crate::bindings::exports::wasi::sockets::udp_create_socket::{
    Guest as WasiSocketsUdpCreateSocket, UdpSocket,
};

use super::VirtSockets;

impl WasiSocketsUdpCreateSocket for VirtSockets {
    fn create_udp_socket(_address_family: IpAddressFamily) -> Result<UdpSocket, ErrorCode> {
        Err(ErrorCode::AccessDenied)
    }
}
