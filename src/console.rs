use crate::sbi::console_putchar;
use core::fmt::{self,Write};

struct Stdout;

impl Write for Stdout {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for c in s.chars() {
            console_putchar(c as usize);
        }
        Ok(())
    }
}

pub fn print(args : fmt::Arguments) {
    Stdout.write_fmt(args).unwrap();
}

#[macro_export]
macro_rules! print {
    ($fmt: literal $(, $arg: expr)*) => {
        $crate::console::print(format_args!($fmt $(, $arg)*));
    };
}

#[macro_export]
macro_rules! println {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::console::print(format_args!(concat!($fmt, "\n") $(, $($arg)+)?));
    }
}
#[macro_export]
macro_rules! log_error {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::console::print(format_args!(concat!("\x1b[31m[ERROR] ",concat!($fmt, "\n\x1b[0m")) $(, $($arg)+)?));
    }
}
#[macro_export]
macro_rules! log_warn {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::console::print(format_args!(concat!("\x1b[93m[WARN] ",concat!($fmt, "\n\x1b[0m")) $(, $($arg)+)?));
    }
}
#[macro_export]
macro_rules! log_info {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::console::print(format_args!(concat!("\x1b[34m[INFO] ",concat!($fmt, "\n\x1b[0m")) $(, $($arg)+)?));
    }
}
#[macro_export]
macro_rules! log_debug {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::console::print(format_args!(concat!("\x1b[32m[DEBUG] ",concat!($fmt, "\n\x1b[0m")) $(, $($arg)+)?));
    }
}
#[macro_export]
macro_rules! log_trace {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::console::print(format_args!(concat!("\x1b[90m[TRACE] ",concat!($fmt, "\n\x1b[0m")) $(, $($arg)+)?));
    }
}