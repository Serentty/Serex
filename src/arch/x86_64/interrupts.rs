#[inline(always)]
pub fn halt() {
    x86_64::instructions::hlt();
}

pub fn halt_loop() -> ! {
    loop {
        halt();
    }
}