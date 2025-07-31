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
    VK_LCONTROL,
};
use std::{
    sync::atomic::{
        AtomicBool,
        Ordering::Relaxed
    },
};
use crate::error::Errors;

pub const KEY: u32 = 124; // f13
pub static KEY_STATE: AtomicBool = AtomicBool::new(false);

static OTHER_KEY_PRESSED: AtomicBool = AtomicBool::new(false);

const INPUT_SIZE: i32 = size_of::<INPUT>() as i32;

#[inline]
pub fn key_handler(is_key_down: bool) -> Result<(), Errors> {
    if is_key_down {
        OTHER_KEY_PRESSED.store(false, Relaxed);
    } else if !OTHER_KEY_PRESSED.load(Relaxed) {
        return send_esc()
    }
    Ok(())
}

#[inline]
pub fn ctrl_handler(vk_code: u32, is_key_event_down: bool) -> Result<(), Errors> {
    // Ignore key releases
    if !is_key_event_down { return Ok(()) }

    OTHER_KEY_PRESSED.store(true, Relaxed);

    send_ctrl(VIRTUAL_KEY(vk_code as u16))
}

static ESC_INPUTS: [INPUT; 2] = [
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

#[inline]
pub fn send_esc() -> Result<(), Errors> {
    let sent = unsafe { SendInput(&ESC_INPUTS, INPUT_SIZE) };
    if sent != ESC_INPUTS_LEN {
        return Err(Errors::SendInput);
    }
    Ok(())
}

#[inline]
pub fn send_ctrl(virtual_key: VIRTUAL_KEY) -> Result<(), Errors> {
    let inputs = [
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
    let sent = unsafe { SendInput(&inputs, INPUT_SIZE) };
    if sent != inputs.len() as u32 {
        return Err(Errors::SendInput);
    }
    Ok(())
}
