#![no_main]
#![no_std]

use uefi::{prelude::*, println};

use uefi::proto::console::gop::GraphicsOutput;

#[entry]
fn main() -> Status {
    uefi::helpers::init().unwrap();

    println!("hello");

    let gop_handle = boot::get_handle_for_protocol::<GraphicsOutput>().unwrap();

    println!("got gop_handle {gop_handle:?}");
    let mut gop = if let Ok(gop) = boot::open_protocol_exclusive::<GraphicsOutput>(gop_handle) {
        gop
    } else {
        println!("Failed to open GraphicsOutput protocol");
        loop {}
    };

    println!("gop");
    // Create UefiDisplay
    let mode = gop.current_mode_info();
    println!("mode");

    let frame_buffer = gop.frame_buffer();

    println!("mode: {mode:?}, frame_buffer: {frame_buffer:?}");
    loop {}
}
