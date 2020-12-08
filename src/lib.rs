#![feature(asm)]
#![feature(abi_x86_interrupt)]
#![allow(dead_code)]
#![no_std]
#![no_main]

mod arch;
mod console;

use core::panic::PanicInfo;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("\n🯁🯂🯃 A kernel panic has occurred.");
    print!("{}", info);
    loop {}
}

static MESSAGE: &str =
r#"╔═══════════════════╗
║ Welcome to SEREX! ║
╚═══════════════════╝"#;

fn kmain(boot_information: multiboot2::BootInformation) -> ! {
    println!("{}", MESSAGE);
    println!("Initializing memory...");
    arch::native::memory::initialize();
    x86_64::instructions::interrupts::int3();
    unsafe {
        *(0x1deadbeef as *mut u8) = 42; 
    }
    loop{}
}
