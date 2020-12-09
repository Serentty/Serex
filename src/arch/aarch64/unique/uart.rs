use spin::Mutex;

use core::mem::transmute;
use volatile::Volatile;

type MmmioRegister = &'static mut Volatile<u32>;

struct Uart;

impl Uart {
    const UART_DR: usize = 0x3F201000;
    const UART_FR: usize = 0x3F201018;

    fn transmit_fifo_full(&self) -> bool {
        let fr: MmmioRegister = unsafe { transmute(Self::UART_FR) };
        fr.read() & (1 << 5) > 0
    }
    
    fn receive_fifo_empty(&self) -> bool {
        let fr: MmmioRegister = unsafe { transmute(Self::UART_FR) };
        fr.read() & (1 << 4) > 0
    }
    
    fn write_byte(&self, byte: u8) {
        while self.transmit_fifo_full() {}
        let dr: MmmioRegister = unsafe { transmute(Self::UART_DR) };
        dr.write(byte as u32);
    }
    
    fn read_byte(&self) -> u8 {
        while self.receive_fifo_empty() {}
        let dr: MmmioRegister = unsafe { transmute(Self::UART_DR) };
        dr.read() as u8
    }
    
}

static UART: Mutex<Uart> = Mutex::new(Uart);

pub fn write_string(s: &str) {
    let uart = UART.lock();
    for byte in s.bytes() {
        uart.write_byte(byte);
    }
}