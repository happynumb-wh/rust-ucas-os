use crate::config::config::config;
use crate::main;

#[allow(unused)]
pub static BOOT_MSG: &str = "rust ucas os booting...\n";

#[unsafe(link_section = ".bss.stack")]
static mut BOOT_STACK: [u8; config::plat::BOOT_STACK_SIZE] = [0; config::plat::BOOT_STACK_SIZE];




fn write_str() {
    sbi_rt::console_write(sbi_rt::Physical::new(BOOT_MSG.len(), BOOT_MSG.as_ptr() as usize, 0));
}

#[unsafe(naked)]
#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.boot")]
unsafe extern "C" fn _start() -> ! {
    core::arch::naked_asm!(
        "la sp , {stack_top}",
        "li t0, {stack_size}",
        "add sp , sp , t0",
        "call {write_str}",
        "call {entry}",
        "1: wfi",
        "j 1b",
        entry = sym main,
        stack_size = const config::plat::BOOT_STACK_SIZE,
        stack_top = sym BOOT_STACK,
        write_str = sym write_str,
    );

}

