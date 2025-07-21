use std::fmt;

#[derive(Debug)]
pub enum InputError {
        SendEscFail {
            sent: u32,
            expected: u32,
        },
        SendCtrlFail {
            sent: u32,
            expected: u32,
        },
}

impl fmt::Display for InputError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            InputError::SendEscFail { sent, expected } => {
                write!(f, "SendInput failed for Esc key: sent {sent} of {expected}")
            }
            InputError::SendCtrlFail { sent, expected } => {
                write!(f, "SendInput failed for Ctrl key: sent {sent} of {expected}")
            }
        }
    }
}

#[derive(Debug)]
pub enum WorkerInitError {
    SenderAlreadySet,
    ThreadFail,
}

impl fmt::Display for WorkerInitError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            WorkerInitError::SenderAlreadySet => write!(f, "Sender already initialized"),
            WorkerInitError::ThreadFail => write!(f, "Thread Failed"),
        }
    }
}

#[derive(Debug)]
pub enum HookError {
    StartHookFail(windows::core::Error),
}

impl fmt::Display for HookError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            HookError::StartHookFail(e) => {
                write!(f, "Failed to start hook: {e}")
            }
        }
    }
}
