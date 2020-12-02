#![feature(asm)]
#![no_std]
#![no_main]

mod arch;
mod console;

use core::fmt::Write;
use core::panic::PanicInfo;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("A kernel panic has occurred.");
    print!("{}", info);
    loop {}
}

static MESSAGE: &str =
r#"╔═══════════════════╗
║ Welcome to SEREX! ║
╚═══════════════════╝"#;

fn kmain() -> ! {
    println!("{}", MESSAGE);
    loop{}
}