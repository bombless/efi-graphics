#![no_main]
#![no_std]

#[macro_use]
extern crate alloc;

mod characters;
mod source;

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

    image.guard().image(100, 100).draw(&mut display).unwrap();

    let text = MyImage::from(text());

    text.guard().image(300, 300).draw(&mut display).unwrap();

    let text = MyImage::from(source::main());

    text.guard().image(300, 500).draw(&mut display).unwrap();

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

impl From<Vec<u8>> for MyImage {
    fn from(data: Vec<u8>) -> Self {
        MyImage { data }
    }
}

fn text() -> MyImage {
    let mut buffer = vec![255; 300 * 32 * 3];
    let mut x_cursor = 0;
    for c in "中华人民共和国".chars() {
        let character = characters::character_get_bitmap(c as _);
        for i in 0..32 {
            for j in 0..32 {
                if character[j] & (1 << i) != 0 {
                    buffer[(x_cursor + i) * 3 + j * 300 * 3] = 0;
                    buffer[(x_cursor + i) * 3 + j * 300 * 3 + 1] = 0;
                    buffer[(x_cursor + i) * 3 + j * 300 * 3 + 2] = 0;
                }
            }
        }

        x_cursor += 32;
    }

    MyImage::from(buffer)
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
    fn image(&'a self, position_x: i32, position_y: i32) -> Image<'a, ImageRaw<'a, Rgb888>> {
        Image::new(&self.data, Point::new(position_x, position_y))
    }
}
