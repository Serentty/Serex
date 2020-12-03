#![feature(asm)]
#![allow(dead_code)]
#![no_std]
#![no_main]

mod arch;
mod console;

use core::fmt::Write;
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

fn kmain() -> ! {
    print!("北越急行ほくほく線は、新潟県南魚沼市の六日町駅を起点とし、新潟県上越市の犀潟駅までを結ぶ、北越急行が運営する鉄道路線である。 ");
    loop{}
}