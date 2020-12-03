use spin::Mutex;

#[inline(always)]
pub fn get_console() -> &'static Mutex<impl crate::console::Console> {
    &crate::arch::x86::unique::vga_graphic_console::VGA_GRAPHIC_CONSOLE
}