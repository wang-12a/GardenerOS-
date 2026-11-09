// Student ID: 24301101
// Name: Wang Jinghang
// Date: 2026-11-09
// Change: implement formatted output.

use core::fmt::{self, Write};
use crate::sys_write;

struct Stdout;

impl Write for Stdout {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        sys_write(1, s.as_bytes());
        Ok(())
    }
}

pub fn print(args: fmt::Arguments) {
    Stdout.write_fmt(args).unwrap();
}
#[macro_export]
macro_rules! print {
    ($fmt:literal $(, $($arg:tt)+)?) => {
        $crate::console::print(
            format_args!($fmt $(, $($arg)+)?)
        );
    };
}

#[macro_export]
macro_rules! println {
    ($fmt:literal $(, $($arg:tt)+)?) => {
        $crate::console::print(
            format_args!(concat!($fmt, "\n") $(, $($arg)+)?)
        );
    };
}
