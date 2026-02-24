#![no_std]
#![no_main]

// use uclog;
// use config::config;
// use arch::riscv64::{boot};


mod uclog;
mod arch;
mod config;

use core::panic::PanicInfo;


#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}


#[unsafe(no_mangle)]
fn main() {
    uclog::uclog::init();
    log::warn!("Hello, world! This is a warning message.");
    uc_println!("{}", config::logo::LOGO);
    loop {}
}