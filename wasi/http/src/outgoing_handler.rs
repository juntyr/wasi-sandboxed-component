use crate::bindings::exports::wasi::http::outgoing_handler::{
    ErrorCode, FutureIncomingResponse, Guest as WasiHttpOutgoingHandler, OutgoingRequest,
    RequestOptions,
};

use super::VirtHttp;

impl WasiHttpOutgoingHandler for VirtHttp {
    fn handle(
        _request: OutgoingRequest,
        _options: Option<RequestOptions>,
    ) -> Result<FutureIncomingResponse, ErrorCode> {
        Err(ErrorCode::ConnectionRefused)
    }
}
