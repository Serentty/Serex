use spin::Mutex;

#[inline(always)]
pub fn get_console() -> &'static Mutex<impl crate::console::Console> {
    &crate::arch::x86::unique::vga_console::VGA_CONSOLE
}