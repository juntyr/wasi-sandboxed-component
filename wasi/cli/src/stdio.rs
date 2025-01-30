use crate::{
    bindings::{
        exports::wasi::cli::{
            stderr::Guest as WasiCliStderr, stdin::Guest as WasiCliStdin,
            stdout::Guest as WasiCliStdout,
        },
        wasi::{
            io::streams::{InputStream, OutputStream},
            null::io::{closed_input, stderr, stdout},
        },
    },
    VirtCli,
};

impl WasiCliStdin for VirtCli {
    fn get_stdin() -> InputStream {
        closed_input()
    }
}

impl WasiCliStdout for VirtCli {
    fn get_stdout() -> OutputStream {
        stdout()
    }
}

impl WasiCliStderr for VirtCli {
    fn get_stderr() -> OutputStream {
        stderr()
    }
}
