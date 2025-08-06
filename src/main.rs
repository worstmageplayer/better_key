#![windows_subsystem = "windows"]

mod key;
mod error;
mod hook;

use hook::start_hook;

fn main() {
    let _ = start_hook();
}
