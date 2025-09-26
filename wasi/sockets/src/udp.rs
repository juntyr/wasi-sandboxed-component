use crate::bindings::exports::wasi::sockets::{
    network::{ErrorCode, IpAddressFamily, IpSocketAddress, NetworkBorrow},
    udp::{
        Guest as WasiSocketsUdp, GuestIncomingDatagramStream, GuestOutgoingDatagramStream,
        GuestUdpSocket, IncomingDatagram, IncomingDatagramStream, OutgoingDatagram,
        OutgoingDatagramStream,
    },
};
use crate::bindings::wasi::io::poll::Pollable;

use super::VirtSockets;

impl WasiSocketsUdp for VirtSockets {
    type UdpSocket = VirtUdpSocket;

    type IncomingDatagramStream = VirtIncomingDatagramStream;

    type OutgoingDatagramStream = VirtOutgoingDatagramStream;
}

pub enum VirtUdpSocket {}

#[expect(clippy::uninhabited_references)] // FIXME
impl GuestUdpSocket for VirtUdpSocket {
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

    fn stream(
        &self,
        _remote_address: Option<IpSocketAddress>,
    ) -> Result<(IncomingDatagramStream, OutgoingDatagramStream), ErrorCode> {
        match *self {}
    }

    fn local_address(&self) -> Result<IpSocketAddress, ErrorCode> {
        match *self {}
    }

    fn remote_address(&self) -> Result<IpSocketAddress, ErrorCode> {
        match *self {}
    }

    fn address_family(&self) -> IpAddressFamily {
        match *self {}
    }

    fn unicast_hop_limit(&self) -> Result<u8, ErrorCode> {
        match *self {}
    }

    fn set_unicast_hop_limit(&self, _value: u8) -> Result<(), ErrorCode> {
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
}

pub enum VirtIncomingDatagramStream {}

#[expect(clippy::uninhabited_references)] // FIXME
impl GuestIncomingDatagramStream for VirtIncomingDatagramStream {
    fn receive(&self, _max_results: u64) -> Result<Vec<IncomingDatagram>, ErrorCode> {
        match *self {}
    }

    fn subscribe(&self) -> Pollable {
        match *self {}
    }
}

pub enum VirtOutgoingDatagramStream {}

#[expect(clippy::uninhabited_references)] // FIXME
impl GuestOutgoingDatagramStream for VirtOutgoingDatagramStream {
    fn check_send(&self) -> Result<u64, ErrorCode> {
        match *self {}
    }

    fn send(&self, _datagrams: Vec<OutgoingDatagram>) -> Result<u64, ErrorCode> {
        match *self {}
    }

    fn subscribe(&self) -> Pollable {
        match *self {}
    }
}
