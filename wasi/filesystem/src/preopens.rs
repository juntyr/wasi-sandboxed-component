use crate::bindings::exports::wasi::filesystem::{
    preopens::Guest as WasiFilesystemPreopens, types::Descriptor,
};

use super::VirtFilesystem;

impl WasiFilesystemPreopens for VirtFilesystem {
    fn get_directories() -> Vec<(Descriptor, String)> {
        Vec::new()
    }
}
