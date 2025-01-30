use crate::{
    bindings::exports::wasi::io::poll::{
        Guest as WasiIoPoll, GuestPollable, Pollable, PollableBorrow,
    },
    VirtIO,
};

impl WasiIoPoll for VirtIO {
    type Pollable = VirtPollable;

    fn poll(r#in: Vec<PollableBorrow>) -> Vec<u32> {
        (0..).take(r#in.len()).collect()
    }
}

#[non_exhaustive]
pub enum VirtPollable {
    Ready,
}

impl GuestPollable for VirtPollable {
    fn ready(&self) -> bool {
        true
    }

    fn block(&self) {
        // no-op
    }
}

impl VirtPollable {
    #[must_use]
    pub fn ready() -> Pollable {
        Pollable::new(Self::Ready)
    }
}
