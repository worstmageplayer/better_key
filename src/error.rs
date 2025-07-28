use std::fmt;
use windows::core::Error;

#[derive(Debug)]
pub enum Errors {
    SendInput {
        sent: u32,
        expected: u32,
    },
    WorkerInit,
    ThreadFail,
    StartHook(Error),
}

impl fmt::Display for Errors {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Errors::SendInput { sent, expected } => {
                write!(f, "SendInput failed: sent {sent} of {expected}")
            }
            Errors::WorkerInit => {
                write!(f, "Worker already initialized")
            }
            Errors::ThreadFail => {
                write!(f, "Thread failed to start")
            }
            Errors::StartHook(e) => {
                write!(f, "Failed to start hook: {e}")
            }
        }
    }
}
