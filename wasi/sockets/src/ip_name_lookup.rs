use crate::bindings::exports::wasi::sockets::ip_name_lookup::{
    Guest as WasiSocketsIpNameLookup, GuestResolveAddressStream, ResolveAddressStream,
};
use crate::bindings::exports::wasi::sockets::network::{ErrorCode, IpAddress, NetworkBorrow};
use crate::bindings::wasi::io::poll::Pollable;

use super::VirtSockets;

impl WasiSocketsIpNameLookup for VirtSockets {
    type ResolveAddressStream = VirtResolveAddressStream;

    fn resolve_addresses(
        _network: NetworkBorrow<'_>,
        _name: String,
    ) -> Result<ResolveAddressStream, ErrorCode> {
        Err(ErrorCode::AccessDenied)
    }
}

pub enum VirtResolveAddressStream {}

impl GuestResolveAddressStream for VirtResolveAddressStream {
    fn resolve_next_address(&self) -> Result<Option<IpAddress>, ErrorCode> {
        match *self {}
    }

    fn subscribe(&self) -> Pollable {
        match *self {}
    }
}
