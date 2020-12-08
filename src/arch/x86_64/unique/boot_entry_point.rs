use core::mem::transmute;
use crate::arch::x86_64::unique::vga_graphic_console;

#[no_mangle]
pub extern "C" fn rust_start(multiboot_ptr: u32) -> ! {
    let boot_information = unsafe { multiboot2::load(multiboot_ptr as usize) };
    if let Some(framebuffer) = boot_information.framebuffer_tag() {
        unsafe {
            vga_graphic_console::BUFFER_BASE = transmute(framebuffer.address);
        }
    }
    unsafe { asm!(
        "add rsp, 0x1000000"
    )};
    crate::kmain(boot_information);
}

