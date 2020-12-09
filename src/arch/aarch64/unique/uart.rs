const UART_DR: u32 = 0x3F201000;
const UART_FR: u32 = 0x3F201018;

fn mmio_write(reg: u32, val: u32) {
    unsafe { *(reg as *mut u32) = val; }
}

fn mmio_read(reg: u32) -> u32 {
    unsafe { *(reg as *const u32) }
}

fn transmit_fifo_full() -> bool {
    mmio_read(UART_FR) & (1 << 5) > 0
}

fn receive_fifo_empty() -> bool {
    mmio_read(UART_FR) & (1 << 4) > 0
}

fn write_byte(byte: u8) {
    while transmit_fifo_full() {}
    mmio_write(UART_DR, byte as u32);
}

fn read_byte() -> u8 {
    while receive_fifo_empty() {}
    mmio_read(UART_DR) as u8
}

pub fn write_string(s: &str) {
    for byte in s.bytes() {
        write_byte(byte);
    }
}