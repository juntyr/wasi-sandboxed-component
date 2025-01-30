use crate::{
    bindings::exports::wasi::cli::{
        terminal_input::{Guest as WasiCliTerminalInput, GuestTerminalInput, TerminalInput},
        terminal_output::{Guest as WasiCliTerminalOutput, GuestTerminalOutput, TerminalOutput},
        terminal_stderr::Guest as WasiCliTerminalStderr,
        terminal_stdin::Guest as WasiCliTerminalStdin,
        terminal_stdout::Guest as WasiCliTerminalStdout,
    },
    VirtCli,
};

impl WasiCliTerminalInput for VirtCli {
    type TerminalInput = VirtTerminalInput;
}

impl WasiCliTerminalOutput for VirtCli {
    type TerminalOutput = VirtTerminalOutput;
}

pub enum VirtTerminalInput {}

impl GuestTerminalInput for VirtTerminalInput {}

pub enum VirtTerminalOutput {}

impl GuestTerminalOutput for VirtTerminalOutput {}

impl WasiCliTerminalStdin for VirtCli {
    fn get_terminal_stdin() -> Option<TerminalInput> {
        None
    }
}

impl WasiCliTerminalStdout for VirtCli {
    fn get_terminal_stdout() -> Option<TerminalOutput> {
        None
    }
}

impl WasiCliTerminalStderr for VirtCli {
    fn get_terminal_stderr() -> Option<TerminalOutput> {
        None
    }
}
