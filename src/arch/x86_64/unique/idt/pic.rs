use lazy_static::lazy_static;
use x86_64::structures::idt::InterruptStackFrame;
use pic8259_simple::ChainedPics;
use spin::Mutex;
use PicInterrupt::*;

pub const PIC_1_OFFSET: u8 = 32;
pub const PIC_2_OFFSET: u8 = PIC_1_OFFSET + 8;

pub static PICS: Mutex<ChainedPics> = Mutex::new(unsafe { ChainedPics::new(PIC_1_OFFSET, PIC_2_OFFSET) });

#[derive(Debug, Copy, Clone)]
#[repr(u8)]
pub enum PicInterrupt {
    Timer = PIC_1_OFFSET + 0,
    Keyboard = PIC_1_OFFSET + 1
}

unsafe fn eoi(interrupt: PicInterrupt) {
    PICS.lock().notify_end_of_interrupt(interrupt as u8);
}

pub extern "x86-interrupt" fn idt_timer(_frame: &mut InterruptStackFrame) {
    unsafe { eoi(Timer) };
}

pub extern "x86-interrupt" fn idt_keyboard(_frame: &mut InterruptStackFrame) {
    use x86_64::instructions::port::Port;
    use pc_keyboard::{layouts, DecodedKey, HandleControl, Keyboard, ScancodeSet1};
    lazy_static! {
        static ref KEYBOARD: Mutex<Keyboard<layouts::Us104Key, ScancodeSet1>> =
            Mutex::new(Keyboard::new(layouts::Us104Key, ScancodeSet1, HandleControl::Ignore));
    }
    let mut keyboard = KEYBOARD.lock();
    let mut ps2 = Port::new(0x60);
    let scancode: u8 = unsafe { ps2.read() };
    if let Ok(Some(key_event)) = keyboard.add_byte(scancode) {
        if let Some(key) = keyboard.process_keyevent(key_event) {
            match key {
                DecodedKey::Unicode(ch) => crate::print!("{}", ch),
                DecodedKey::RawKey(key) => crate::print!("{:?}", key)
            }
        }
    }
    unsafe { eoi(Keyboard) };
}