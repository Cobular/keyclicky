use libloading::os::windows::{Library, Symbol};
use windows::Win32::UI::WindowsAndMessaging::HHOOK;


pub fn lib_set_hook() -> Result<HHOOK, Box<dyn std::error::Error>> {
  unsafe {
      let lib = Library::new("./target/debug/keyclicky_lib.dll")?;
      let func: Symbol<unsafe extern fn() -> HHOOK> = lib.get(b"set_hook")?;
      Ok(func())
  }
}

pub fn lib_unset_hook(hook: HHOOK) -> Result<(), Box<dyn std::error::Error>> {
  unsafe {
      let lib = Library::new("./target/debug/keyclicky_lib.dll")?;
      let func: Symbol<unsafe extern fn(hook: HHOOK) -> ()> = lib.get(b"unset_hook")?;
      func(hook);
      Ok(())
  }
}

pub fn lib_block_on_msg() -> Result<(), Box<dyn std::error::Error>> {
  unsafe {
      let lib = Library::new("./target/debug/keyclicky_lib.dll")?;
      let func: Symbol<unsafe extern fn() -> ()> = lib.get(b"block_on_msg")?;
      func();
      Ok(())
  }
}