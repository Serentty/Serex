use core::mem::transmute;
use crate::arch::x86_64::unique::vga_graphic_console;

#[no_mangle]
pub extern "C" fn rust_start(multiboot_ptr: u32) -> ! {
    let boot_information = unsafe { multiboot2::load(multiboot_ptr as usize) };
    if let Ok(boot_information) = boot_information {
        if let Some(framebuffer) = boot_information.framebuffer_tag() {
            if framebuffer.buffer_type != multiboot2::FramebufferType::Text {
                unsafe {
                    vga_graphic_console::BUFFER_BASE = transmute(framebuffer.address);
                }
            }
        }
    }
    crate::kmain();
}
