pub const WASI_SANDBOXED_CLI: &[u8] =
    include_bytes!("../artefacts/wasi_sandboxed_cli.component.wasm");
pub const WASI_SANDBOXED_CLOCKS: &[u8] =
    include_bytes!("../artefacts/wasi_sandboxed_clocks.component.wasm");
pub const WASI_SANDBOXED_FILESYSTEM: &[u8] =
    include_bytes!("../artefacts/wasi_sandboxed_filesystem.component.wasm");
pub const WASI_SANDBOXED_IO: &[u8] =
    include_bytes!("../artefacts/wasi_sandboxed_io.component.wasm");
pub const WASI_SANDBOXED_RANDOM: &[u8] =
    include_bytes!("../artefacts/wasi_sandboxed_random.component.wasm");

pub const ALL_COMPONENTS: &[(&str, &[u8])] = &[
    ("wasi-sandboxed:cli", WASI_SANDBOXED_CLI),
    ("wasi-sandboxed:clocks", WASI_SANDBOXED_CLOCKS),
    ("wasi-sandboxed:filesystem", WASI_SANDBOXED_FILESYSTEM),
    ("wasi-sandboxed:io", WASI_SANDBOXED_IO),
    ("wasi-sandboxed:random", WASI_SANDBOXED_RANDOM),
];

pub const MERGED_COMPONENT: &[u8] =
    include_bytes!("../artefacts/wasi_sandboxed_merged.component.wasm");
