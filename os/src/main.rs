// Student ID: 24301101
// Name: Wang Jinghang
// Date: 2026-11-09
// Change: remove the standard library dependency.

#![no_std]
#![no_main]

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}







