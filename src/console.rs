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
    crate::arch::native::console::write_format(args);   
}