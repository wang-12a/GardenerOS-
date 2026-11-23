#![no_std]
#![no_main]

#[macro_use]
extern crate user_lib;

#[no_mangle]
fn main() -> i32 {
    println!("Student ID: 24301101");
    println!("My application: sum from 1 to 100");

    let mut sum: u32 = 0;
    for i in 1..=100 {
        sum += i;
    }
    println!("Sum = {}", sum);

    if sum == 5050 {
        println!("Test sum OK!");
        0
    } else {
        println!("Test sum FAILED!");
        1
    }
}
