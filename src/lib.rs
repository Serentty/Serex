#![feature(asm)]
#![feature(abi_x86_interrupt)]
#![feature(const_fn_fn_ptr_basics)]
#![allow(dead_code)]
#![no_std]

mod arch;
mod console;
mod timer;

use core::panic::PanicInfo;
use arch::native;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("\n🯁🯂🯃 A kernel panic has occurred.");
    print!("{}", info);
    native::interrupts::halt_loop();
}

static MESSAGE: &str =
r#"╔═══════════════════╗
║ Welcome to SEREX! ║
╚═══════════════════╝"#;

fn kmain(_boot_information: multiboot2::BootInformation) -> ! {
    println!("{}", MESSAGE);
    println!("Initializing memory...");
    native::memory::initialize();
    println!("Initializing I/O...");
    native::io::initialize();
    println!("Now chilling, waiting for interrupts.");  
    native::interrupts::halt_loop();
}
