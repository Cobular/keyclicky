mod lib_res;

use std::time::Duration;

use log::debug;
use windows::Win32::{
    Foundation::{HINSTANCE, HWND, LPARAM, LRESULT, WPARAM},
    UI::WindowsAndMessaging::{
        CallNextHookEx, DispatchMessageA, GetMessageA, SetWindowsHookExA, TranslateMessage,
        UnhookWindowsHookEx, HHOOK, MSG, WH_KEYBOARD_LL,
    },
};
use crate::lib_res::serial::{PORT, send_click};

static mut RUNNING: bool = true;

#[no_mangle]
extern "system" fn init() {
    env_logger::init();

    let my_ports: Vec<serialport::SerialPortInfo> = serialport::available_ports()
        .unwrap()
        .into_iter()
        .filter(|port| match &port.port_type {
            serialport::SerialPortType::UsbPort(port) => port.vid == 0x1209 && port.pid == 0x0745,
            _ => false,
        })
        .collect();

    if my_ports.is_empty() {
        panic!("No Keyclicky Device Found!");
    }

    let port = serialport::new(&my_ports[0].port_name, 9600)
        .timeout(Duration::from_millis(10))
        .open()
        .unwrap();

    unsafe {
        PORT = Some(port);
    }
}

extern "system" fn hook(hook_code: i32, v_key_code: WPARAM, key_message_info: LPARAM) -> LRESULT {
    if hook_code < 0 {
        unsafe { return CallNextHookEx(HHOOK(0), hook_code, v_key_code, key_message_info) }
    }

    match v_key_code {
        WPARAM(257) => debug!("Key Up"),
        WPARAM(256) => {
            send_click();
            debug!("Key Down")
        },
        _ => debug!("Something Else"),
    }

    unsafe { CallNextHookEx(HHOOK(0), hook_code, v_key_code, key_message_info) }
}

#[no_mangle]
pub extern "C" fn set_hook() -> HHOOK {
    unsafe {
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
