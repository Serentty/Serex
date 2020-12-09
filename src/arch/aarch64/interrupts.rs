pub fn halt() {
    unsafe { asm!(
        "wfe"
    )};
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