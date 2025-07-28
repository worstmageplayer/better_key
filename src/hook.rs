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
            Input::KeyboardAndMouse::{
                VIRTUAL_KEY,
                VK_SHIFT,
                VK_LSHIFT,
                VK_RSHIFT,
                VK_MENU,
                VK_LMENU,
                VK_RMENU,
            }
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
    error::{
        HookError,
        WorkerInitError,
    }
};

enum KeyAction {
    KeyHandler(bool),
    CtrlHandler(u32, bool),
}

static SENDER: OnceLock<Sender<KeyAction>> = OnceLock::new();

pub fn init_worker() -> Result<(), WorkerInitError> {
    let (sender, receiver) = channel::<KeyAction>();

    if SENDER.set(sender).is_err() {
        return Err(WorkerInitError::SenderAlreadySet);
    }

    let thread = thread::Builder::new().name("key-action-thread".to_string());
    match thread.spawn(move || {
        for action in receiver {
            match action {
                KeyAction::KeyHandler(is_down) => key_handler(is_down),
                KeyAction::CtrlHandler(vk, is_key_event_down) => ctrl_handler(vk, is_key_event_down),
            }
        }
    }) {
        Ok(_) => Ok(()),
        Err(_) => Err(WorkerInitError::ThreadFail),
    }
}

#[unsafe(no_mangle)]
unsafe extern "system" fn hook_proc(n_code: i32, w_param: WPARAM, l_param: LPARAM) -> LRESULT {
    if n_code < 0 {
        return unsafe { CallNextHookEx(None, n_code, w_param, l_param) }
    }

    // l_param is a pointer to a KBDLLHOOKSTRUCT struct
    let kb = unsafe { &*(l_param.0 as *const KBDLLHOOKSTRUCT) };

    // Skip injected events
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

    // Ignore shift and alt keys
    match VIRTUAL_KEY(vk_code as u16) {
        VK_SHIFT | VK_LSHIFT | VK_RSHIFT | VK_MENU | VK_LMENU | VK_RMENU => {
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

pub fn start_hook() -> Result<(), HookError> {
    {
        // This thread installs the hook
        match unsafe { SetWindowsHookExA(WH_KEYBOARD_LL, Some(hook_proc), None, 0) } {
            Ok(result) => result,
            Err(e) => {
                return Err(HookError::StartHookFail(e));
            },
        };
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
