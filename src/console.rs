pub trait Console: core::fmt::Write {}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::console::write_format(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

pub fn write_format(args: core::fmt::Arguments) {
    use core::fmt::Write;
    crate::arch::native::console::get_console().lock().write_fmt(args).ok();    
}