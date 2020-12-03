#[no_mangle]
pub extern "C" fn _start(_boot_info: &'static bootloader::BootInfo) -> ! {
    crate::kmain();
}