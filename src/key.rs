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

pub const KEY: u32 = 124; // f13
pub static mut KEY_STATE: bool = false;
static mut OTHER_KEY_PRESSED: bool = false;

const INPUT_SIZE: i32 = size_of::<INPUT>() as i32;

#[inline]
pub fn key_handler(is_key_down: bool) {
    if is_key_down {
        unsafe { OTHER_KEY_PRESSED = false; }
    } else if unsafe{ !OTHER_KEY_PRESSED } {
        send_esc()
    }
}

#[inline]
pub fn ctrl_handler(vk_code: u32, is_key_event_down: bool) {
    if !is_key_event_down { return }

    unsafe { OTHER_KEY_PRESSED = true; }

    send_ctrl(VIRTUAL_KEY(vk_code as u16))
}

#[inline]
pub fn send_esc() {
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
    unsafe { SendInput(&inputs, INPUT_SIZE) };
}

#[inline]
pub fn send_ctrl(virtual_key: VIRTUAL_KEY) {
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
    unsafe { SendInput(&inputs, INPUT_SIZE) };
}
