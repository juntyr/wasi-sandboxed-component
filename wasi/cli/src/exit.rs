use super::VirtCli;
use crate::bindings::exports::wasi::cli::exit::Guest as WasiCliExit;

impl WasiCliExit for VirtCli {
    fn exit(_status: Result<(), ()>) {
        std::process::abort()
    }
}
