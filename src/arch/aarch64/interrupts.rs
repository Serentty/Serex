pub fn halt() {
    cortex_a::asm::wfi();
}

pub fn halt_loop() -> ! {
    loop {
        halt();
    }
}

pub fn without_interrupts<F, R>(f: F) -> R
    where F: FnOnce() -> R
{
    // TODO
    f()
}