pub mod outgoing_handler;
pub mod types;

use crate::VirtMerged as VirtHttp;

type WasiIoErrorRef<'a> = crate::bindings::exports::wasi::io::error::ErrorBorrow<'a>;
