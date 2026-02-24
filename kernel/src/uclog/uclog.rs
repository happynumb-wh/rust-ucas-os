// #![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "std")]
use std::fmt::Write;

#[allow(unused)]
use sbi_rt::{console_write, console_read};

use log::{Log, LevelFilter};

use core::{fmt};
use core::fmt::{Write};

// log ``
struct Logger;


/// Prints to the console.
///
/// Equivalent to the [`uc_println!`] macro except that a newline is not printed at
/// the end of the message.
#[macro_export]
macro_rules! uc_print {
    ($($arg:tt)*) => {
        $crate::uclog::uclog::__print_impl(format_args!($($arg)*));
    }
}


/// Prints to the console, with a newline.
#[macro_export]
macro_rules! uc_println {
    () => { $crate::uc_print!("\n") };
    ($($arg:tt)*) => {
        $crate::uclog::uclog::__print_impl(format_args!("{}\n", format_args!($($arg)*)));
    }
}


// impl msg for Logger
impl Log for Logger {
    fn enabled(&self, metadata: &log::Metadata) -> bool {
        metadata.level() <= log::Level::Warn
    }

    fn log(&self, record: &log::Record) {
        if self.enabled(record.metadata()) {
            print_fmt(format_args!("[{}] - {}", record.level(), record.args())).unwrap();
        }
    }

    fn flush(&self) {}

}

// Call init before use all log macros
pub fn init() {
    log::set_logger(&Logger).unwrap();
    log::set_max_level(LevelFilter::Warn);
}


impl Write for Logger {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        cfg_if::cfg_if! {
            if #[cfg(feature = "std")] {
                println!("{}", s);
            } else {
                console_write(sbi_rt::Physical::new(s.len(), s.as_ptr() as usize, 0));
            }
        }
        Ok(())
    }
}

pub fn __print_impl(args: fmt::Arguments) {
    print_fmt(args).unwrap();
}

pub fn print_fmt(args: fmt::Arguments) -> fmt::Result {
    Logger.write_fmt(args)
}

