use spin::Mutex;
use lazy_static::lazy_static;
use crate::arch::x86_64::unique::{vga_console, vga_graphic_console};
use crate::console::Console;

struct ConsoleDispatcher;

impl ConsoleDispatcher {
    fn graphic_console_enabled() -> bool {
        unsafe { vga_graphic_console::BUFFER_BASE != core::ptr::null_mut() }
    }
}

impl core::fmt::Write for ConsoleDispatcher {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        if Self::graphic_console_enabled() {
            let mut console = vga_graphic_console::VGA_GRAPHIC_CONSOLE.lock();
            console.write_str(s).ok();
        } else {
            let mut console = vga_console::VGA_CONSOLE.lock();
            console.write_str(s).ok();
        }
        Ok(())
    }
}

impl Console for ConsoleDispatcher {}

lazy_static! {
    static ref CONSOLE_DISPATCHER: Mutex<ConsoleDispatcher> = Mutex::new(ConsoleDispatcher);
}

#[inline(always)]
pub fn get_console() -> &'static Mutex<impl Console> {
    &CONSOLE_DISPATCHER
}