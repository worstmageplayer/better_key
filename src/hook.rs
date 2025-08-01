use windows::{
    Win32::{
        Foundation::{
            LRESULT,
            WPARAM,
            LPARAM,
        },
        UI::{
            WindowsAndMessaging::{
                SetWindowsHookExA,
                CallNextHookEx,
                GetMessageA,
                TranslateMessage,
                DispatchMessageA,
                WH_KEYBOARD_LL,
                KBDLLHOOKSTRUCT,
                LLKHF_INJECTED,
                WM_KEYDOWN,
                WM_SYSKEYDOWN,
                WM_KEYUP,
                WM_SYSKEYUP,
                MSG,
            },
        }
    }
};
use std::{
    sync::{
        atomic::{
            Ordering::Relaxed,
        },
        mpsc::{
            channel,
            Sender,
        },
        OnceLock,
    },
    thread,
};
use crate::{
    key::{
        KEY,
        KEY_STATE,
        ctrl_handler,
        key_handler,
    },
    error::Errors
};

enum KeyAction {
    KeyHandler(bool),
    CtrlHandler(u32, bool),
}

static SENDER: OnceLock<Sender<KeyAction>> = OnceLock::new();

pub fn init_worker() -> Result<(), Errors> {
    let (sender, receiver) = channel::<KeyAction>();

    if SENDER.set(sender).is_err() {
        return Err(Errors::WorkerInit);
    }

    thread::spawn(move || {
        for action in receiver {
            let _ = match action {
                KeyAction::KeyHandler(is_down) => key_handler(is_down),
                KeyAction::CtrlHandler(vk, is_key_event_down) => ctrl_handler(vk, is_key_event_down),
            };
        }
    });
    Ok(())
}

const VK_SHIFT_RAW: u16 = 0x10;
const VK_LSHIFT_RAW: u16 = 0xA0;
const VK_RSHIFT_RAW: u16 = 0xA1;
const VK_MENU_RAW: u16 = 0x12;
const VK_LMENU_RAW: u16 = 0xA4;
const VK_RMENU_RAW: u16 = 0xA5;

#[unsafe(no_mangle)]
unsafe extern "system" fn hook_proc(n_code: i32, w_param: WPARAM, l_param: LPARAM) -> LRESULT {
    if n_code < 0 {
        return unsafe { CallNextHookEx(None, n_code, w_param, l_param) }
    }

    // l_param is a pointer to a KBDLLHOOKSTRUCT struct
    let kb = unsafe { &*(l_param.0 as *const KBDLLHOOKSTRUCT) };

    if kb.flags.contains(LLKHF_INJECTED) {
        return unsafe { CallNextHookEx(None, n_code, w_param, l_param) }
    }

    let vk_code = kb.vkCode;
    // w_param is the type of event
    // Lets you check if it is a key press or release
    let is_key_event_down = match w_param.0 as u32 {
        WM_KEYDOWN | WM_SYSKEYDOWN => true,
        WM_KEYUP | WM_SYSKEYUP => false,
        _ => return unsafe { CallNextHookEx(None, n_code, w_param, l_param) }
    };

    if vk_code == KEY {
        KEY_STATE.store(is_key_event_down, Relaxed);
        if let Some(sender) = SENDER.get() {
            let _ = sender.send(KeyAction::KeyHandler(is_key_event_down));
        };
        return LRESULT(1);
    }

    match vk_code as u16 {
        VK_SHIFT_RAW | VK_LSHIFT_RAW | VK_RSHIFT_RAW | VK_MENU_RAW | VK_LMENU_RAW | VK_RMENU_RAW => {
            return unsafe { CallNextHookEx(None, n_code, w_param, l_param) };
        },
        _ => {}
    }

    // When f13 key is held, send all key presses to ctrl_handler
    if KEY_STATE.load(Relaxed) {
        if let Some(sender) = SENDER.get() {
            let _ = sender.send(KeyAction::CtrlHandler(vk_code, is_key_event_down));
        };
        return LRESULT(1);
    }

    unsafe { CallNextHookEx(None, n_code, w_param, l_param) }
}

pub fn start_hook() -> Result<(), Errors> {
    // This thread installs the hook
    if unsafe { SetWindowsHookExA(WH_KEYBOARD_LL, Some(hook_proc), None, 0) }.is_err() {
        return Err(Errors::StartHook);
    }

    let mut msg = MSG::default();
    // Loop to keep the thread alive
    // Windows sends hook events to the thread that installed the hook
    while unsafe { GetMessageA(&mut msg, None, 0, 0).0 } > 0 {
        unsafe {
            let _ = TranslateMessage(&msg);
            DispatchMessageA(&msg);
        }
    }
    Ok(())
}
