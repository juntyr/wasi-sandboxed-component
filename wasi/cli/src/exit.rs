use crate::{bindings::exports::wasi::cli::exit::Guest as WasiCliExit, VirtCli};

impl WasiCliExit for VirtCli {
    fn exit(_status: Result<(), ()>) {
        std::process::abort()
    }
}
