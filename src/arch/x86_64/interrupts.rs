#[inline(always)]
pub fn halt() {
    x86_64::instructions::hlt();
}

pub fn halt_loop() -> ! {
    loop {
        halt();
    }
}

pub use x86_64::instructions::interrupts::without_interrupts;