use core::fmt::Write;

struct Writer;

impl Write for Writer {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        crate::arch::aarch64::unique::uart::write_string(s);
        Ok(())
    }
}

pub fn write_format(args: core::fmt::Arguments) {
    Writer.write_fmt(args).ok();
}