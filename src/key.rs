use windows::Win32::UI::Input::KeyboardAndMouse::{
    SendInput,
    INPUT,
    INPUT_0,
    INPUT_KEYBOARD,
    KEYBDINPUT,
    KEYBD_EVENT_FLAGS,
    KEYEVENTF_KEYUP,
    VK_ESCAPE,
    VIRTUAL_KEY,
    VK_SHIFT,
    VK_LSHIFT,
    VK_RSHIFT,
    VK_CONTROL,
    VK_LCONTROL,
    VK_RCONTROL,
};
use std::{
    sync::atomic::{AtomicBool, Ordering::SeqCst},
    mem::size_of,
};
use crate::error::InputError;
use InputError::{SendCtrlFail, SendEscFail};

pub const KEY: u32 = 124; // f13
pub static KEY_STATE: AtomicBool = AtomicBool::new(false);

static OTHER_KEY_PRESSED: AtomicBool = AtomicBool::new(false);

const INPUT_SIZE: i32 = size_of::<INPUT>() as i32;

pub fn key_handler(is_key_down: bool) {
    if is_key_down {
        OTHER_KEY_PRESSED.store(false, SeqCst);
    } else if !OTHER_KEY_PRESSED.load(SeqCst)
        && let Err(e) = send_esc() {
        eprintln!("{e}");
    }
}

pub fn ctrl_handler(vk_code: u32) {
    OTHER_KEY_PRESSED.store(true, SeqCst);
    let virtual_key = VIRTUAL_KEY(vk_code as u16);

    match virtual_key {
        VK_SHIFT | VK_LSHIFT | VK_RSHIFT | VK_CONTROL | VK_LCONTROL | VK_RCONTROL => return,
        _ => {}
    }

    if let Err(e) = send_ctrl(virtual_key) {
        eprintln!("{e}");
    }
}

const ESC_INPUTS: [INPUT; 2] = [
    INPUT {
        r#type: INPUT_KEYBOARD,
        Anonymous: INPUT_0 {
            ki: KEYBDINPUT {
                wVk: VK_ESCAPE,
                wScan: 0,
                dwFlags: KEYBD_EVENT_FLAGS(0),
                time: 0,
                dwExtraInfo: 0,
            }
        },
    },
    INPUT {
        r#type: INPUT_KEYBOARD,
        Anonymous: INPUT_0 {
            ki: KEYBDINPUT {
                wVk: VK_ESCAPE,
                wScan: 0,
                dwFlags: KEYEVENTF_KEYUP,
                time: 0,
                dwExtraInfo: 0,
            }
        },
    }
];
const ESC_INPUTS_LEN: u32 = ESC_INPUTS.len() as u32;

pub fn send_esc() -> Result<(), InputError> {
    let sent = unsafe { SendInput(&ESC_INPUTS, INPUT_SIZE) };
    if sent != ESC_INPUTS_LEN {
        return Err(SendEscFail {
            sent,
            expected: ESC_INPUTS_LEN
        });
    }
    Ok(())
}

pub fn send_ctrl(virtual_key: VIRTUAL_KEY) -> Result<(), InputError> {
    let inputs = &[
        INPUT {
            r#type: INPUT_KEYBOARD,
            Anonymous: INPUT_0 {
                ki: KEYBDINPUT {
                    wVk: VK_LCONTROL,
                    wScan: 0,
                    dwFlags: KEYBD_EVENT_FLAGS(0),
                    time: 0,
                    dwExtraInfo: 0,
                }
            },
        },
        INPUT {
            r#type: INPUT_KEYBOARD,
            Anonymous: INPUT_0 {
                ki: KEYBDINPUT {
                    wVk: virtual_key,
                    wScan: 0,
                    dwFlags: KEYBD_EVENT_FLAGS(0),
                    time: 0,
                    dwExtraInfo: 0,
                }
            },
        },
        INPUT {
            r#type: INPUT_KEYBOARD,
            Anonymous: INPUT_0 {
                ki: KEYBDINPUT {
                    wVk: virtual_key,
                    wScan: 0,
                    dwFlags: KEYEVENTF_KEYUP,
                    time: 0,
                    dwExtraInfo: 0,
                }
            },
        },
        INPUT {
            r#type: INPUT_KEYBOARD,
            Anonymous: INPUT_0 {
                ki: KEYBDINPUT {
                    wVk: VK_LCONTROL,
                    wScan: 0,
                    dwFlags: KEYEVENTF_KEYUP,
                    time: 0,
                    dwExtraInfo: 0,
                }
            },
        }
    ];
    let sent = unsafe { SendInput(inputs, INPUT_SIZE) };
    if sent != inputs.len() as u32 {
        return Err(SendCtrlFail {
            sent,
            expected: inputs.len() as u32
        });
    }
    Ok(())
}
