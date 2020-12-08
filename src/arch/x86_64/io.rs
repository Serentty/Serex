pub fn initialize() {
    unsafe {
        super::unique::idt::pic::PICS.lock().initialize();
    }
    x86_64::instructions::interrupts::enable();
}