#[inline]
pub unsafe fn inb(port: u16) -> u8 {
    let mut value: u8;
    asm!(
        "in {1}, {0:x}",
        in(reg) port,
        lateout(reg_byte) value
    );
    value
}

#[inline]
pub unsafe fn inw(port: u16) -> u16 {
    let mut value: u16;
    asm!(
        "in {1:x}, {0:x}",
        in(reg) port,
        lateout(reg) value
    );
    value
}

#[inline]
pub unsafe fn inl(port: u16) -> u32 {
    let mut value: u32;
    asm!(
        "in {1:e}, {0:x}",
        in(reg) port,
        lateout(reg) value
    );
    value
}

#[inline]
pub unsafe fn outb(port: u16, value: u8) {
    asm!(
        "out {0:x}, {1}",
        in(reg) port,
        in(reg_byte) value
    );
}

#[inline]
pub unsafe fn outw(port: u16, value: u16) {
    asm!(
        "out {0:x}, {1:x}",
        in(reg) port,
        in(reg) value
    );
}

#[inline]
pub unsafe fn outl(port: u16, value: u32) {
    asm!(
        "out {0:x}, {1:e}",
        in(reg) port,
        in(reg) value
    );
}