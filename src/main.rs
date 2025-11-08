#![no_main]
#![no_std]

use core::panic::PanicInfo;

use uefi::{prelude::*, println};

use boot::protocols_per_handle;

#[entry]
fn main() -> Status {
    uefi::helpers::init().unwrap();
    println!("hello");
    loop {}

    Status::SUCCESS
}
