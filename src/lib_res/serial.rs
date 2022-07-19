use serialport::SerialPort;

pub static mut PORT: Option<Box<dyn SerialPort>> = None;

pub fn send_click() {
    unsafe {
        if let Some(port) = PORT.as_mut() {
            let output = "This is a test. This is only a test.\n".as_bytes();
            port.write_all(output).expect("Write failed!");
        }
    }
}
