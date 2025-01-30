use crate::{
    bindings::{
        exports::wasi::io::{
            poll::Pollable,
            streams::{
                Guest as WasiIoStreams, GuestInputStream, GuestOutputStream, InputStream,
                InputStreamBorrow, OutputStream, StreamError,
            },
        },
        fcbench::wasi::stdio::{flush_stderr, flush_stdout, write_stderr, write_stdout},
    },
    poll::VirtPollable,
    VirtIO,
};

impl WasiIoStreams for VirtIO {
    type InputStream = VirtInputStream;
    type OutputStream = VirtOutputStream;
}

#[non_exhaustive]
pub enum VirtInputStream {
    Closed,
}

#[non_exhaustive]
pub enum VirtOutputStream {
    Stdout,
    Stderr,
    Sink,
}

impl GuestInputStream for VirtInputStream {
    fn read(&self, _len: u64) -> Result<Vec<u8>, StreamError> {
        Err(StreamError::Closed)
    }

    fn blocking_read(&self, _len: u64) -> Result<Vec<u8>, StreamError> {
        Err(StreamError::Closed)
    }

    fn skip(&self, _len: u64) -> Result<u64, StreamError> {
        Err(StreamError::Closed)
    }

    fn blocking_skip(&self, _len: u64) -> Result<u64, StreamError> {
        Err(StreamError::Closed)
    }

    fn subscribe(&self) -> Pollable {
        VirtPollable::ready()
    }
}

impl VirtInputStream {
    #[must_use]
    pub fn closed() -> InputStream {
        InputStream::new(Self::Closed)
    }
}

impl GuestOutputStream for VirtOutputStream {
    fn check_write(&self) -> Result<u64, StreamError> {
        Ok(4096)
    }

    fn write(&self, contents: Vec<u8>) -> Result<(), StreamError> {
        match self {
            Self::Stdout => write_stdout(&contents),
            Self::Stderr => write_stderr(&contents),
            Self::Sink => (),
        }

        Ok(())
    }

    fn blocking_write_and_flush(&self, contents: Vec<u8>) -> Result<(), StreamError> {
        self.write(contents)?;
        self.flush()
    }

    fn flush(&self) -> Result<(), StreamError> {
        match self {
            Self::Stdout => flush_stdout(),
            Self::Stderr => flush_stderr(),
            Self::Sink => (),
        }

        Ok(())
    }

    fn blocking_flush(&self) -> Result<(), StreamError> {
        self.flush()
    }

    fn subscribe(&self) -> Pollable {
        VirtPollable::ready()
    }

    fn write_zeroes(&self, mut len: u64) -> Result<(), StreamError> {
        if matches!(self, Self::Sink) {
            return Ok(());
        };

        while let (Err(_), Ok(max)) = (usize::try_from(len), u64::try_from(usize::MAX)) {
            self.write(vec![0_u8; usize::MAX])?;
            len -= max;
        }

        if let Ok(len) = usize::try_from(len) {
            self.write(vec![0_u8; len])?;
        }

        Ok(())
    }

    fn blocking_write_zeroes_and_flush(&self, len: u64) -> Result<(), StreamError> {
        self.write_zeroes(len)?;
        self.flush()
    }

    fn splice(&self, _src: InputStreamBorrow, _len: u64) -> Result<u64, StreamError> {
        Err(StreamError::Closed)
    }

    fn blocking_splice(&self, _src: InputStreamBorrow, _len: u64) -> Result<u64, StreamError> {
        Err(StreamError::Closed)
    }
}

impl VirtOutputStream {
    #[must_use]
    pub fn stdout() -> OutputStream {
        OutputStream::new(Self::Stdout)
    }

    #[must_use]
    pub fn stderr() -> OutputStream {
        OutputStream::new(Self::Stderr)
    }

    #[must_use]
    pub fn sink() -> OutputStream {
        OutputStream::new(Self::Sink)
    }
}
