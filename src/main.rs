#![windows_subsystem = "windows"]

mod key;
mod error;
mod hook;

fn main() {
    let _ = hook::start_hook();
}
