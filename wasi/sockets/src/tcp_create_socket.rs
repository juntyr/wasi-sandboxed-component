use crate::bindings::exports::wasi::sockets::network::{ErrorCode, IpAddressFamily};
use crate::bindings::exports::wasi::sockets::tcp_create_socket::{
    Guest as WasiSocketsTcpCreateSocket, TcpSocket,
};

use super::VirtSockets;

impl WasiSocketsTcpCreateSocket for VirtSockets {
    fn create_tcp_socket(_address_family: IpAddressFamily) -> Result<TcpSocket, ErrorCode> {
        Err(ErrorCode::AccessDenied)
    }
}
