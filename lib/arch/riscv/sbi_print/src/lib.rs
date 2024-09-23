#![no_std]

fn sbi_print_str(s: &str) {
    for c in s.bytes() {
        // .filter(|c| c.is_ascii())
        sbi::legacy::console_putchar(c);
    }
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => (sbi_print::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

pub struct ConsoleWriter;

impl core::fmt::Write for ConsoleWriter {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        sbi_print_str(s);
        Ok(())
    }
}

#[doc(hidden)]
pub fn _print(args: core::fmt::Arguments) {
    use core::fmt::Write;
    ConsoleWriter.write_fmt(args).unwrap();
}
