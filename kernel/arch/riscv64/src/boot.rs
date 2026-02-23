// use config::config::plat;


// let hello = "Hello, world!";
#[allow(unused)]
static BOOT_MSG: &str = "rust ucas os booting...";

#[unsafe(link_section = ".bss.stack")]
static mut BOOT_STACK: [u8; 4096] = [0; 4096];




fn write_str() {
    sbi_rt::console_write(sbi_rt::Physical::new(BOOT_MSG.len(), BOOT_MSG.as_ptr() as usize, 0));
    // loop {}
    // for byte in BOOT_MSG.as_bytes() {
    //     sbi_rt::console_write_byte(*byte);
    // }

}

#[unsafe(naked)]
#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.boot")]
unsafe extern "C" fn _start() -> ! {
    core::arch::naked_asm!(
        "la sp , {stack_top}",
        "li t0, 4096",
        "add sp , sp , t0",
        // "li a7 , 0x00",
        // "la a0 , 24",
        // "la a1 , {boot_msg}",
        // "addi a2 , a1 , 24",
        // "ecall",
        "call {write_str}",
        "1: wfi",
        "j 1b",
        stack_top = sym BOOT_STACK,
        write_str = sym write_str,
    );

    // sbi_rt::console_write(sbi_rt::Physical::new(BOOT_MSG.as_ptr() as usize, BOOT_MSG.len(),0));
    // loop {}
}

