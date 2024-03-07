#![no_main]
#![no_std]
#![feature(panic_info_message)]
mod lang_item;
mod sbi;
mod console;
use core::arch::global_asm;

global_asm!(include_str!("entry.S"));



// This function is called in the entry.S file.
#[no_mangle]
pub fn rust_main() -> ! {
    extern "C" {
        fn sbss();
        fn ebss();
        fn stext();
        fn etext();
        fn srodata();
        fn erodata();
        fn sdata();
        fn edata();
    }
    for addr in sbss as usize..ebss as usize {
        unsafe {
            (addr as *mut u8).write_volatile(0);
        }
    }
    println!("FrogOS is booting...");
    println!("FrogOS is running on RISC-V!");
    println!("FrogOS is written in Rust!");
    log_debug!(".text [{:#x}, {:#x})", stext as usize, etext as usize);
    log_debug!(".rodata [{:#x}, {:#x})", srodata as usize, erodata as usize);
    log_debug!(".data [{:#x}, {:#x})", sdata as usize, edata as usize);
    println!("FrogOS is shutting down...");
    sbi::shutdown(0);
}
