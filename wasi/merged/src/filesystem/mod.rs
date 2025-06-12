pub mod preopens;
pub mod types;

use crate::VirtMerged as VirtFilesystem;

type WasiIoErrorRef<'a> = crate::bindings::exports::wasi::io::error::ErrorBorrow<'a>;
