use crate::{bindings::exports::wasi::cli::environment::Guest as WasiCliEnvironment, VirtCli};

impl WasiCliEnvironment for VirtCli {
    fn get_environment() -> Vec<(String, String)> {
        Vec::new()
    }

    fn get_arguments() -> Vec<String> {
        Vec::new()
    }

    fn initial_cwd() -> Option<String> {
        None
    }
}
