#[cfg(not(feature = "merged"))]
use crate::bindings::wasi::{
    io::streams::{InputStream, OutputStream},
    null::io::{closed_input, stderr, stdout},
};
use crate::{
    bindings::exports::wasi::cli::{
        stderr::Guest as WasiCliStderr, stdin::Guest as WasiCliStdin,
        stdout::Guest as WasiCliStdout,
    },
    VirtCli,
};
#[cfg(feature = "merged")]
use wasi_sandboxed_io::exports::streams::{InputStream, OutputStream};

#[cfg(feature = "merged")]
fn closed_input() -> InputStream {
    wasi_sandboxed_io::streams::VirtInputStream::closed()
}

#[cfg(feature = "merged")]
fn stdout() -> OutputStream {
    wasi_sandboxed_io::streams::VirtOutputStream::stdout()
}

#[cfg(feature = "merged")]
fn stderr() -> OutputStream {
    wasi_sandboxed_io::streams::VirtOutputStream::stderr()
}

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
