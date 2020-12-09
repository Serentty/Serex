#[no_mangle]
pub extern "C" fn rust_start() -> ! {
    crate::kmain();
}
