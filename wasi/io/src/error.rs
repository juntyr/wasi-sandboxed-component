use crate::{
    bindings::exports::wasi::io::error::{Guest as WasiIoError, GuestError},
    VirtIO,
};

impl WasiIoError for VirtIO {
    type Error = VirtError;
}

pub enum VirtError {}

impl GuestError for VirtError {
    fn to_debug_string(&self) -> String {
        #[expect(clippy::uninhabited_references)] // FIXME
        match *self {}
    }
}
