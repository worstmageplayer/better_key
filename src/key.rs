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
use crate::error::Errors;

pub const KEY: u32 = 124; // f13
pub static mut KEY_STATE: bool = false;
static mut OTHER_KEY_PRESSED: bool = false;

const INPUT_SIZE: i32 = size_of::<INPUT>() as i32;

#[inline]
pub fn key_handler(is_key_down: bool) -> Result<(), Errors> {
    if is_key_down {
        unsafe { OTHER_KEY_PRESSED = false; }
    } else if unsafe{ !OTHER_KEY_PRESSED } {
        return send_esc()
    }
    Ok(())
}

#[inline]
pub fn ctrl_handler(vk_code: u32, is_key_event_down: bool) -> Result<(), Errors> {
    // Ignore key releases
    if !is_key_event_down { return Ok(()) }

    unsafe { OTHER_KEY_PRESSED = true; }

    send_ctrl(VIRTUAL_KEY(vk_code as u16))
}

/// Returns an Error if number of sent events does not match the expected number of events
#[inline]
pub fn send_esc() -> Result<(), Errors> {
    let inputs = [
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
    let sent = unsafe { SendInput(&inputs, INPUT_SIZE) };
    if sent != 2 {
        return Err(Errors::SendInput);
    }
    Ok(())
}

/// Returns an Error if number of sent events does not match the expected number of events
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
    if sent != 4 {
        return Err(Errors::SendInput);
    }
    Ok(())
}
