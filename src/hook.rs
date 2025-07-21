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
            }
        }
    }
};
use std::{
    sync::{
        atomic::{
            AtomicPtr,
            Ordering::SeqCst,
        },
        mpsc::{
            channel,
            Sender,
        },
        OnceLock,
    },
    thread,
    ffi::c_void,
    ptr::null_mut,
};
use crate::key::{
    KEY,
    KEY_STATE,
    ctrl_handler,
    key_handler,
};
use crate::error::{HookError, WorkerInitError};

enum KeyAction {
    KeyHandler(bool),
    CtrlHandler(u32),
}

static HOOK: AtomicPtr<c_void> = AtomicPtr::new(null_mut());
static SENDER: OnceLock<Sender<KeyAction>> = OnceLock::new();

pub fn init_worker() -> Result<(), WorkerInitError> {
    let (tx, rx) = channel::<KeyAction>();

    if SENDER.set(tx).is_err() {
        return Err(WorkerInitError::SenderAlreadySet);
    }

    if thread::Builder::new()
        .name("key-action-thread".into())
        .spawn(move || {
            for action in rx {
                match action {
                    KeyAction::KeyHandler(is_down) => key_handler(is_down),
                    KeyAction::CtrlHandler(vk) => ctrl_handler(vk),
                }
            }
        })
        .is_err() {
        return Err(WorkerInitError::ThreadFailed)
    }

    Ok(())
}

#[unsafe(no_mangle)]
unsafe extern "system" fn hook_proc(n_code: i32, w_param: WPARAM, l_param: LPARAM) -> LRESULT {
    if n_code < 0 {
        return unsafe { CallNextHookEx(None, n_code, w_param, l_param) }
    }

    let kb = unsafe { &*(l_param.0 as *const KBDLLHOOKSTRUCT) };
    if kb.flags.contains(LLKHF_INJECTED) {
        return unsafe { CallNextHookEx(None, n_code, w_param, l_param) }
    }

    let vk_code = kb.vkCode;
    let is_key_event_down = match w_param.0 as u32 {
        WM_KEYDOWN | WM_SYSKEYDOWN => true,
        WM_KEYUP | WM_SYSKEYUP => false,
        _ => return unsafe { CallNextHookEx(None, n_code, w_param, l_param) }
    };

    if vk_code == KEY {
        KEY_STATE.store(is_key_event_down, SeqCst);
        if let Some(sender) = SENDER.get() {
            let _ = sender.send(KeyAction::KeyHandler(is_key_event_down));
        };
        return LRESULT(1);
    }

    if KEY_STATE.load(SeqCst) && is_key_event_down {
        if let Some(sender) = SENDER.get() {
            let _ = sender.send(KeyAction::CtrlHandler(vk_code));
        };
        return LRESULT(1);
    }

    unsafe { CallNextHookEx(None, n_code, w_param, l_param) }
}

pub fn start_hook() -> Result<(), HookError> {
    {
        let hook = match unsafe { SetWindowsHookExA(WH_KEYBOARD_LL, Some(hook_proc), None, 0) } {
            Ok(result) => result,
            Err(e) => {
                return Err(HookError::StartHookFail(e));
            },
        };
        HOOK.store(hook.0, SeqCst);
    }

    let mut msg = MSG::default();
    while unsafe { GetMessageA(&mut msg, None, 0, 0).0 } > 0 {
        unsafe {
            let _ = TranslateMessage(&msg);
            DispatchMessageA(&msg);
        }
    }
    Ok(())
}
