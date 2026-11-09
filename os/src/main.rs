// Student ID: 24301101
// Name: Wang Jinghang
// Date: 2026-11-09
// Change: implement the exit system call.

#![no_std]
#![no_main]
mod console;
use core::panic::PanicInfo;
use core::arch::asm;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

const SYSCALL_EXIT: usize = 93;

fn syscall(id: usize, args: [usize; 3]) -> isize {
    let mut ret: isize;
    unsafe {
        asm!(
            "ecall",
            in("x10") args[0],
            in("x11") args[1],
            in("x12") args[2],
            in("x17") id,
            lateout("x10") ret
        );
    }
    ret
}

const SYSCALL_WRITE: usize = 64;

pub fn sys_write(fd: usize, buffer: &[u8]) -> isize {
    syscall(
        SYSCALL_WRITE,
        [fd, buffer.as_ptr() as usize, buffer.len()],
    )
}

pub fn sys_exit(xstate: i32) -> isize {
    syscall(SYSCALL_EXIT, [xstate as usize, 0, 0])
}

#[unsafe(no_mangle)]
extern "C" fn _start() {
    crate::println!("Hello, world!");
    sys_exit(9);
}























