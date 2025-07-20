#![windows_subsystem = "windows"]
use crate::hook::{init_worker, start_hook};

mod key;
mod error;
mod hook;


fn main() {
    if let Err(e) = init_worker() {
        eprintln!("Failed to initialize worker: {e:?}");
        return;
    }

    if let Err(e) = start_hook() {
        eprintln!("Failed to start hook: {e}");
    }
}
