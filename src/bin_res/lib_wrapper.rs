use libloading::os::windows::{Library, Symbol};
use windows::Win32::UI::WindowsAndMessaging::HHOOK;
use lazy_static::lazy_static;

lazy_static! {
  static ref LIB: Library = {
    unsafe {
      Library::new("./target/debug/keyclicky_lib.dll").expect("Failed to load the dll library")
    }
  };
}

pub fn lib_init() -> Result<(), Box<dyn std::error::Error>> {
  unsafe {
      let func: Symbol<unsafe extern fn() -> ()> = LIB.get(b"init")?;
      func();
      Ok(())
  }
}

pub fn lib_set_hook() -> Result<HHOOK, Box<dyn std::error::Error>> {
  unsafe {
      let func: Symbol<unsafe extern fn() -> HHOOK> = LIB.get(b"set_hook")?;
      Ok(func())
  }
}

pub fn lib_unset_hook(hook: HHOOK) -> Result<(), Box<dyn std::error::Error>> {
  unsafe {
      let func: Symbol<unsafe extern fn(hook: HHOOK) -> ()> = LIB.get(b"unset_hook")?;
      func(hook);
      Ok(())
  }
}

pub fn lib_block_on_msg() -> Result<(), Box<dyn std::error::Error>> {
  unsafe {
      let func: Symbol<unsafe extern fn() -> ()> = LIB.get(b"block_on_msg")?;
      func();
      Ok(())
  }
}