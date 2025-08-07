#![windows_subsystem = "windows"]

mod key;
mod hook;
mod error {
    pub enum Errors {
        StartHook,
    }
}

fn main() {
    let _ = hook::start_hook();
}
