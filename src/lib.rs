use windows::core::{IntoParam, PCSTR, Param};
use windows::Win32::{Media::Audio::{PlaySoundA, SND_ASYNC}};
use windows::{
    Win32::{
        Foundation::{HINSTANCE, HWND, LPARAM, LRESULT, WPARAM, BOOL},
        UI::WindowsAndMessaging::{
            CallNextHookEx, DispatchMessageA, GetMessageA, SetWindowsHookExA, TranslateMessage,
            UnhookWindowsHookEx, HHOOK, MSG, WH_KEYBOARD_LL,
        },
    },
};
use log::{debug, info, warn};

static mut RUNNING: bool = true;

// #[no_mangle]
// pub extern "C" fn init() -> HHOOK {
    
// }

extern "system" fn hook(hook_code: i32, v_key_code: WPARAM, key_message_info: LPARAM) -> LRESULT {
    if hook_code < 0 {
        unsafe { return CallNextHookEx(HHOOK(0), hook_code, v_key_code, key_message_info) }
    }

    match v_key_code {
        WPARAM(257) => debug!("Key Up"),
        WPARAM(256) => debug!("Key Down"),
        _ => debug!("Something Else"),
    }

    let sound_str: Param<PCSTR> = "./src/gun.wav".into_param();

    unsafe {
        PlaySoundA(sound_str.abi(), HINSTANCE::default(), SND_ASYNC);
    }

    unsafe { CallNextHookEx(HHOOK(0), hook_code, v_key_code, key_message_info) }
}

#[no_mangle]
pub extern "C" fn set_hook() -> HHOOK {
    unsafe {
        RUNNING = true;

        let hook = SetWindowsHookExA(WH_KEYBOARD_LL, Some(hook), HINSTANCE(0), 0)
            .expect("Failed to set hook");
        debug!("Hook set: {hook:?}");
        hook
    }
}

#[no_mangle]
pub extern "C" fn unset_hook(hook: HHOOK) {
    debug!("Shutting down hook: {hook:?}");
    unsafe {
        RUNNING = false;
        UnhookWindowsHookEx(hook).unwrap();
    }
}

#[no_mangle]
pub extern "C" fn block_on_msg() {
    let mut msg: MSG = MSG::default();

    unsafe {
        while RUNNING && GetMessageA(&mut msg, HWND::default(), 0, 0).into() {
            debug!("message");
            TranslateMessage(&msg);
            DispatchMessageA(&msg);
        }
    }
}
