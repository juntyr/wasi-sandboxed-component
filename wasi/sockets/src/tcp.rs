use crate::bindings::exports::wasi::sockets::network::{
    ErrorCode, IpAddressFamily, IpSocketAddress, NetworkBorrow,
};
use crate::bindings::exports::wasi::sockets::tcp::{
    Guest as WasiSocketsTcp, GuestTcpSocket, ShutdownType, TcpSocket,
};
use crate::bindings::wasi::clocks::monotonic_clock::Duration;
use crate::bindings::wasi::io::poll::Pollable;
use crate::bindings::wasi::io::streams::{InputStream, OutputStream};

use super::VirtSockets;

impl WasiSocketsTcp for VirtSockets {
    type TcpSocket = VirtTcpSocket;
}

pub enum VirtTcpSocket {}

#[expect(clippy::uninhabited_references)] // FIXME
impl GuestTcpSocket for VirtTcpSocket {
    fn start_bind(
        &self,
        _network: NetworkBorrow<'_>,
        _local_address: IpSocketAddress,
    ) -> Result<(), ErrorCode> {
        match *self {}
    }

    fn finish_bind(&self) -> Result<(), ErrorCode> {
        match *self {}
    }

    fn start_connect(
        &self,
        _network: NetworkBorrow<'_>,
        _remote_address: IpSocketAddress,
    ) -> Result<(), ErrorCode> {
        match *self {}
    }

    fn finish_connect(&self) -> Result<(InputStream, OutputStream), ErrorCode> {
        match *self {}
    }

    fn start_listen(&self) -> Result<(), ErrorCode> {
        match *self {}
    }

    fn finish_listen(&self) -> Result<(), ErrorCode> {
        match *self {}
    }

    fn accept(&self) -> Result<(TcpSocket, InputStream, OutputStream), ErrorCode> {
        match *self {}
    }

    fn local_address(&self) -> Result<IpSocketAddress, ErrorCode> {
        match *self {}
    }

    fn remote_address(&self) -> Result<IpSocketAddress, ErrorCode> {
        match *self {}
    }

    fn is_listening(&self) -> bool {
        match *self {}
    }

    fn address_family(&self) -> IpAddressFamily {
        match *self {}
    }

    fn set_listen_backlog_size(&self, _value: u64) -> Result<(), ErrorCode> {
        match *self {}
    }

    fn keep_alive_enabled(&self) -> Result<bool, ErrorCode> {
        match *self {}
    }

    fn set_keep_alive_enabled(&self, _value: bool) -> Result<(), ErrorCode> {
        match *self {}
    }

    fn keep_alive_idle_time(&self) -> Result<Duration, ErrorCode> {
        match *self {}
    }

    fn set_keep_alive_idle_time(&self, _value: Duration) -> Result<(), ErrorCode> {
        match *self {}
    }

    fn keep_alive_interval(&self) -> Result<Duration, ErrorCode> {
        match *self {}
    }

    fn set_keep_alive_interval(&self, _value: Duration) -> Result<(), ErrorCode> {
        match *self {}
    }

    fn keep_alive_count(&self) -> Result<u32, ErrorCode> {
        match *self {}
    }

    fn set_keep_alive_count(&self, _value: u32) -> Result<(), ErrorCode> {
        match *self {}
    }

    fn hop_limit(&self) -> Result<u8, ErrorCode> {
        match *self {}
    }

    fn set_hop_limit(&self, _value: u8) -> Result<(), ErrorCode> {
        match *self {}
    }

    fn receive_buffer_size(&self) -> Result<u64, ErrorCode> {
        match *self {}
    }

    fn set_receive_buffer_size(&self, _value: u64) -> Result<(), ErrorCode> {
        match *self {}
    }

    fn send_buffer_size(&self) -> Result<u64, ErrorCode> {
        match *self {}
    }

    fn set_send_buffer_size(&self, _value: u64) -> Result<(), ErrorCode> {
        match *self {}
    }

    fn subscribe(&self) -> Pollable {
        match *self {}
    }

    fn shutdown(&self, _shutdown_type: ShutdownType) -> Result<(), ErrorCode> {
        match *self {}
    }
}
