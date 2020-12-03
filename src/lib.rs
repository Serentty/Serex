#![feature(asm)]
#![feature(global_asm)]
#![allow(dead_code)]
#![no_std]
#![no_main]

mod arch;
mod console;

use core::panic::PanicInfo;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("\nA kernel panic has occurred.");
    print!("{}", info);
    loop {}
}

static MESSAGE: &str =
r#"╔═══════════════════╗
║ Welcome to SEREX! ║
╚═══════════════════╝"#;

fn kmain(boot_information: multiboot2::BootInformation) -> ! {
    println!("{}", MESSAGE);
    for _ in 0..10000 {
        print!("ごいす〜！");
    }
    loop{}
}