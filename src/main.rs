#![no_main]
#![no_std]

extern crate alloc;

use alloc::vec::Vec;

use embedded_graphics::Drawable;
use embedded_graphics::geometry::Point;
use embedded_graphics::image::{Image, ImageRaw};
use embedded_graphics::pixelcolor::{Rgb888, RgbColor};
use embedded_graphics::prelude::Size;
use embedded_graphics::primitives::{PrimitiveStyle, Rectangle, StyledDrawable};
use uefi::prelude::*;
use uefi::proto::console::gop::GraphicsOutput;

use uefi_graphics2::UefiDisplay;

#[entry]
fn main() -> Status {
    uefi::helpers::init().unwrap();

    // Disable the watchdog timer

    boot::set_watchdog_timer(0, 0x10000, None).unwrap();

    // Get gop
    let gop_handle = boot::get_handle_for_protocol::<GraphicsOutput>().unwrap();
    let mut gop = boot::open_protocol_exclusive::<GraphicsOutput>(gop_handle).unwrap();

    // Create UefiDisplay
    let mode = gop.current_mode_info();
    let mut display = UefiDisplay::new(gop.frame_buffer(), mode).unwrap();

    // Create a new rectangle
    let rectangle = Rectangle::new(
        Point { x: 30, y: 100 },
        Size {
            width: 300,
            height: 150,
        },
    );

    // Draw the text on the display
    rectangle
        .draw_styled(&mut PrimitiveStyle::with_fill(Rgb888::YELLOW), &mut display)
        .unwrap();

    let image = MyImage::new();

    image.guard().image().draw(&mut display).unwrap();

    // Flush everything
    display.flush();

    boot::stall(100_000_000);

    Status::SUCCESS
}

struct MyImage {
    data: Vec<u8>,
}

struct ImageGuard<'a> {
    data: ImageRaw<'a, Rgb888>,
}

impl MyImage {
    fn new() -> Self {
        let mut data = Vec::new();
        for i in 0..300 {
            for j in 0..300 {
                let x = (i / 10 + j / 10) % 3;
                let r = if x == 0 { 255 } else { 0 };
                let g = if x == 1 { 255 } else { 0 };
                let b = if x == 2 { 255 } else { 0 };
                data.push(r);
                data.push(g);
                data.push(b);
            }
        }
        MyImage { data: data }
    }
    fn guard<'a>(&'a self) -> ImageGuard<'a> {
        ImageGuard {
            data: ImageRaw::new(&self.data, 300),
        }
    }
}

impl<'a> ImageGuard<'a> {
    fn image(&'a self) -> Image<'a, ImageRaw<'a, Rgb888>> {
        Image::new(&self.data, Point::new(100, 100))
    }
}
