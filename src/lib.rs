#![feature(asm)]
#![feature(abi_x86_interrupt)]
#![feature(const_fn_fn_ptr_basics)]
#![feature(alloc_error_handler)]
#![allow(dead_code)]
#![no_std]

extern crate alloc;
#[macro_use]
extern crate mopa;

mod arch;
mod console;
mod timer;
mod allocation;
mod filesystem;

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

fn kmain() -> ! {
    println!("{}", MESSAGE);
    println!("Initializing memory...");
    native::memory::initialize();
    println!("Initializing the heap...");
    allocation::initialize();
    println!("Initializing I/O...");
    native::io::initialize();
    use arch::aarch64::unique::mailbox;
    let fb = mailbox::initialize_framebuffer();
    println!("{:?}", fb);
    native::interrupts::halt_loop();
}
