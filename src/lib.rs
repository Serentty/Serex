#![feature(asm)]
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
    print!(r#"
    U+1FB0x 	🬀 	🬁 	🬂 	🬃 	🬄 	🬅 	🬆 	🬇 	🬈 	🬉 	🬊 	🬋 	🬌 	🬍 	🬎 	🬏
    U+1FB1x 	🬐 	🬑 	🬒 	🬓 	🬔 	🬕 	🬖 	🬗 	🬘 	🬙 	🬚 	🬛 	🬜 	🬝 	🬞 	🬟
    U+1FB2x 	🬠 	🬡 	🬢 	🬣 	🬤 	🬥 	🬦 	🬧 	🬨 	🬩 	🬪 	🬫 	🬬 	🬭 	🬮 	🬯
    U+1FB3x 	🬰 	🬱 	🬲 	🬳 	🬴 	🬵 	🬶 	🬷 	🬸 	🬹 	🬺 	🬻 	🬼 	🬽 	🬾 	🬿
    U+1FB4x 	🭀 	🭁 	🭂 	🭃 	🭄 	🭅 	🭆 	🭇 	🭈 	🭉 	🭊 	🭋 	🭌 	🭍 	🭎 	🭏
    U+1FB5x 	🭐 	🭑 	🭒 	🭓 	🭔 	🭕 	🭖 	🭗 	🭘 	🭙 	🭚 	🭛 	🭜 	🭝 	🭞 	🭟
    U+1FB6x 	🭠 	🭡 	🭢 	🭣 	🭤 	🭥 	🭦 	🭧 	🭨 	🭩 	🭪 	🭫 	🭬 	🭭 	🭮 	🭯
    U+1FB7x 	🭰 	🭱 	🭲 	🭳 	🭴 	🭵 	🭶 	🭷 	🭸 	🭹 	🭺 	🭻 	🭼 	🭽 	🭾 	🭿
    U+1FB8x 	🮀 	🮁 	🮂 	🮃 	🮄 	🮅 	🮆 	🮇 	🮈 	🮉 	🮊 	🮋 	🮌 	🮍 	🮎 	🮏
    U+1FB9x 	🮐 	🮑 	🮒 		🮔 	🮕 	🮖 	🮗 	🮘 	🮙 	🮚 	🮛 	🮜 	🮝 	🮞 	🮟
    U+1FBAx 	🮠 	🮡 	🮢 	🮣 	🮤 	🮥 	🮦 	🮧 	🮨 	🮩 	🮪 	🮫 	🮬 	🮭 	🮮 	🮯
    U+1FBBx 	🮰 	🮱 	🮲 	🮳 	🮴 	🮵 	🮶 	🮷 	🮸 	🮹 	🮺 	🮻 	🮼 	🮽 	🮾 	🮿
    U+1FBCx 	🯀 	🯁 	🯂 	🯃 	🯄 	🯅 	🯆 	🯇 	🯈 	🯉 	🯊 					
    U+1FBDx 																
    U+1FBEx 																
    U+1FBFx 	🯰 	🯱 	🯲 	🯳 	🯴 	🯵 	🯶 	🯷 	🯸 	🯹 			"#);
    loop{}
}