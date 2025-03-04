use std::ffi::CString;
use nix::sys::wait::WaitStatus;
use crate::{
    program::posix::builtin::Builtin,
    program::{Result, Runtime, parse_and_run},
};

/// Command builtin, I have no idea why you'd want this honestly.
pub struct Command;

impl Builtin for Command {
    fn run(self, argv: Vec<CString>, runtime: &mut Runtime) -> Result<WaitStatus> {
        let text = argv[1..].iter().map(|c| {
            c.to_str().unwrap()
        }).collect::<Vec<_>>().join(" ");
        parse_and_run(&text, runtime)
    }
}
