#![windows_subsystem = "windows"]
use crate::hook::{
    init_worker,
    start_hook
};

mod key;
mod error;
mod hook;

fn main() {
    let _ = init_worker();
    let _ = start_hook();
}
