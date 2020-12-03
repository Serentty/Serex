use uart_16550::SerialPort;
use spin::Mutex;
use lazy_static::lazy_static;

fn make_port(port: u16) -> Mutex<SerialPort> {
    let mut serial_port = unsafe { SerialPort::new(port) };
    serial_port.init();
    Mutex::new(serial_port)
}

lazy_static! {
    pub static ref COM1: Mutex<SerialPort> = make_port(0x3F8);
    pub static ref COM2: Mutex<SerialPort> = make_port(0x2F8);
    pub static ref COM3: Mutex<SerialPort> = make_port(0x3E8);
    pub static ref COM4: Mutex<SerialPort> = make_port(0x2E8);
}