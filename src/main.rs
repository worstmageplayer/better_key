#![windows_subsystem = "windows"]
use crate::hook::{
    start_hook
};

mod key;
mod error;
mod hook;

fn main() {
    let _ = start_hook();
}
