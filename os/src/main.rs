// Student ID: 24301101
// Name: Wang Jinghang

#![no_std]
#![no_main]

mod console;
mod lang_items;
mod sbi;

use core::arch::global_asm;

global_asm!(include_str!("entry.asm"));

fn clear_bss() {
    unsafe extern "C" {
        fn sbss();
        fn ebss();
    }

    let start = sbss as *const () as usize;
    let end = ebss as *const () as usize;

    for addr in start..end {
        unsafe { (addr as *mut u8).write_volatile(0) };
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn rust_main() -> ! {
    unsafe extern "C" {
        fn stext();
        fn etext();
        fn srodata();
        fn erodata();
        fn sdata();
        fn edata();
        fn boot_stack();
        fn boot_stack_top();
        fn sbss();
        fn ebss();
    }

    clear_bss();
    crate::println!("Hello, world!");
    crate::println!(
        ".text [{:#x}, {:#x})",
        stext as *const () as usize,
        etext as *const () as usize
    );
    crate::println!(
        ".rodata [{:#x}, {:#x})",
        srodata as *const () as usize,
        erodata as *const () as usize
    );
    crate::println!(
        ".data [{:#x}, {:#x})",
        sdata as *const () as usize,
        edata as *const () as usize
    );
    crate::println!(
        "boot_stack [{:#x}, {:#x})",
        boot_stack as *const () as usize,
        boot_stack_top as *const () as usize
    );
    crate::println!(
        ".bss [{:#x}, {:#x})",
        sbss as *const () as usize,
        ebss as *const () as usize
    );
    crate::println!("Hello, world!");
    panic!("Shutdown machine!");
}
