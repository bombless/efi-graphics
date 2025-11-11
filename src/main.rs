#![no_main]
#![no_std]

#[macro_use]
extern crate alloc;

mod bitmap;
mod characters;
mod gray8;
mod source;

use alloc::vec::Vec;

use uefi::proto::console::text::{Input, Key, ScanCode};
use uefi::{Result as OutputResult, ResultExt, boot};

use alloc::boxed::Box;

#[cfg(not(test))]
use core::panic::PanicInfo;

use embedded_graphics::Drawable;
use embedded_graphics::geometry::Point;
use embedded_graphics::image::{Image, ImageRaw};
use embedded_graphics::pixelcolor::{Rgb888, RgbColor};
use embedded_graphics::prelude::{DrawTarget, Size};
use embedded_graphics::primitives::{PrimitiveStyle, Rectangle, StyledDrawable};
use uefi::boot::{OpenProtocolAttributes, OpenProtocolParams};
use uefi::prelude::*;
use uefi::println;
use uefi::proto::console::gop::GraphicsOutput;

use uefi_graphics2::UefiDisplay;

#[cfg(not(test))]
#[panic_handler]
fn panic(i: &PanicInfo) -> ! {
    println!("panic {:?}", i.location());
    loop {}
}

#[entry]
fn main() -> Status {
    println!("phase zero");

    // boot::stall(3_000_000);
    if uefi::helpers::init().is_err() {
        println!("uefi::helpers::init() failed");

        // boot::stall(3_000_000);

        return Status::ABORTED;
    }
    println!("uefi::helpers::init() okay");

    // boot::stall(3_000_000);

    // Disable the watchdog timer

    if boot::set_watchdog_timer(0, 0x10000, None).is_err() {
        println!("boot::set_watchdog_timer() failed");

        // boot::stall(3_000_000);

        return Status::ABORTED;
    }
    println!("boot::set_watchdog_timer() okay");

    // boot::stall(3_000_000);

    // Get gop
    let gop_handle = if let Ok(h) = boot::get_handle_for_protocol::<GraphicsOutput>() {
        h
    } else {
        println!("boot::get_handle_for_protocol() failed");

        // boot::stall(3_000_000);

        return Status::ABORTED;
    };
    println!("boot::get_handle_for_protocol() okay {gop_handle:?}");

    // boot::stall(3_000_000);

    let params = OpenProtocolParams {
        handle: gop_handle,
        agent: gop_handle,
        controller: None,
    };
    let mut gop = if let Ok(gop) = unsafe {
        boot::open_protocol::<GraphicsOutput>(params, OpenProtocolAttributes::GetProtocol)
    } {
        gop
    } else {
        println!("boot::open_protocol_exclusive() failed");

        // boot::stall(3_000_000);

        return Status::ABORTED;
    };
    // println!("boot::open_protocol_exclusive() okay");

    // boot::stall(3_000_000);

    // Create UefiDisplay
    let mode = gop.current_mode_info();
    // println!("mode {mode:?}");

    // boot::stall(3_000_000);
    let mut display = UefiDisplay::new(gop.frame_buffer(), mode).unwrap();

    // println!("first phase: draw yellow rectangle");
    // Flush everything
    display.flush();

    // boot::stall(3_000_000);

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
    // Flush everything
    display.flush();

    // boot::stall(3_000_000);

    // println!("second phase: draw colored board");
    // Flush everything
    display.flush();

    // boot::stall(3_000_000);

    let image = TextData::demo();

    image.position(100, 100).draw(&mut display).unwrap();

    // boot::stall(3_000_000);

    // println!("third phase: draw one line of text");
    // Flush everything
    display.flush();

    // boot::stall(3_000_000);

    let text = text();

    text.position(300, 300).draw(&mut display).unwrap();
    // Flush everything
    display.flush();

    // boot::stall(3_000_000);

    // println!("fourth phase: draw beautiful lines of text");
    // Flush everything
    display.flush();

    let text = new_text();

    text.position(0, 560).draw(&mut display).unwrap();

    let text = gray8_text();

    text.position(300, 560).draw(&mut display).unwrap();

    display.flush();

    // boot::stall(13_000_000);

    let text = source::main();

    text.position(300, 500).draw(&mut display).unwrap();

    let text = TextData::text(1800, &format!("mode {mode:?}\n{:?}", display.log()));

    text.position(0, 0).draw(&mut display).unwrap();

    // Flush everything
    display.flush();

    // Create a new rectangle
    let rectangle = Rectangle::new(
        Point { x: 0, y: 700 },
        Size {
            width: 1024,
            height: 30,
        },
    );

    // Draw the text on the display
    rectangle
        .draw_styled(&mut PrimitiveStyle::with_fill(Rgb888::YELLOW), &mut display)
        .unwrap();
    // Flush everything
    display.flush();

    let data = generate_frame(42);

    let some_data: &[u8] = unsafe { core::mem::transmute(&data[..]) };
    let text = TextData::text(800, &format!("{some_data:?}"));

    text.position(0, 0).draw(&mut display).unwrap();

    display.flush();

    // boot::stall(13_000_000);

    let image_raw = ImageRaw::<Rgb888>::new(some_data, 800);
    Image::new(&image_raw, Point { x: 0, y: 0 })
        .draw(&mut display)
        .unwrap();
    display.flush();

    read_keyboard_events(&mut display).unwrap();

    for step in 1..900 {
        let data = generate_frame(step);

        let image_raw = ImageRaw::<Rgb888>::new(unsafe { core::mem::transmute(&data[..]) }, 800);
        Image::new(&image_raw, Point { x: 0, y: 0 })
            .draw(&mut display)
            .unwrap();

        let text = TextData::text(200, &format!("step {step}"));

        text.position(800, 0).draw(&mut display).unwrap();

        // Flush everything
        display.flush();

        boot::stall(10_000);
    }

    read_keyboard_events(&mut display).unwrap();

    Status::SUCCESS
}

fn generate_frame(step: u16) -> Box<[[(u8, u8, u8); 800]; 800]> {
    let mut buf = Box::new([[(0u8, 0u8, 0u8); 800]; 800]);

    for row in buf.iter_mut() {
        for (r, ..) in row.iter_mut() {
            *r = step as u8;
        }
    }

    buf
}

struct TextData {
    width: usize,
    data: Vec<u8>,
}

struct ImageGuard<'a> {
    data: ImageRaw<'a, Rgb888>,
    position: Point,
    line_count: usize,
}

fn text() -> TextData {
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

    TextData {
        width: 300,
        data: buffer,
    }
}

fn new_text() -> TextData {
    let mut buffer = vec![255; 300 * 32 * 3];
    let mut x_cursor = 0;
    for c in "吃菜啊别光喝酒".chars() {
        let character = bitmap::bitmap(c as _);
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

    TextData {
        width: 300,
        data: buffer,
    }
}

fn gray8_text() -> TextData {
    let mut buffer = vec![0; 500 * 32 * 3 * 3];
    let mut height_offset = 0;
    let mut graylevel = 2;
    for graylevel_pointer_limit in 0..3 {
        let mut x_cursor = 0;
        for c in format!("今天又是新的一天 graylevel {graylevel}").chars() {
            let character = gray8::bitmap(c as _);
            for i in 0..32 {
                for j in 0..32 {
                    for offset in 0..8 {
                        let pointer = offset.clamp(0, graylevel_pointer_limit);
                        if character[pointer][j] & (1 << i) != 0 {
                            buffer[height_offset + (x_cursor + i) * 3 + j * 500 * 3] |=
                                1 << (7 - offset);
                            buffer[height_offset + (x_cursor + i) * 3 + j * 500 * 3 + 1] |=
                                1 << (7 - offset);
                            buffer[height_offset + (x_cursor + i) * 3 + j * 500 * 3 + 2] |=
                                1 << (7 - offset);
                        }
                    }
                }
            }

            x_cursor += if c > '~' { 32 } else { 16 };
        }
        height_offset += 500 * 32 * 3;
        graylevel *= 2;
    }

    TextData {
        width: 500,
        data: buffer,
    }
}

impl TextData {
    fn text(width: usize, text: &str) -> Self {
        TextData {
            width,
            data: source::text(width, text),
        }
    }
    fn demo() -> Self {
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
        TextData {
            width: 300,
            data: data,
        }
    }
    fn position<'a>(&'a self, x: i32, y: i32) -> ImageGuard<'a> {
        ImageGuard {
            data: ImageRaw::new(&self.data, self.width as _),
            position: Point::new(x, y),
            line_count: self.data.len() / self.width / 3 / 32,
        }
    }
}

impl<'a> ImageGuard<'a> {
    fn draw(
        &'a self,
        display: &mut UefiDisplay,
    ) -> Result<usize, <UefiDisplay as DrawTarget>::Error> {
        Image::new(&self.data, self.position).draw(display)?;
        Ok(self.line_count)
    }
}

fn read_keyboard_events(display: &mut UefiDisplay) -> OutputResult {
    let handle = boot::get_handle_for_protocol::<Input>().unwrap();
    let mut input = boot::open_protocol_exclusive::<Input>(handle).unwrap();
    let mut height = 32;

    let text = TextData::text(800, &format!("key any key"));

    text.position(0, 0).draw(display).unwrap();
    display.flush();
    loop {
        // Pause until a keyboard event occurs.
        let mut events = [input.wait_for_key_event().unwrap()];
        boot::wait_for_event(&mut events).discard_errdata()?;

        match input.read_key()? {
            // Example of handling a printable key: print a message when
            // the 'u' key is pressed.
            Some(Key::Printable(key)) => {
                let text = TextData::text(800, &format!("key {} pressed", char::from(key)));

                let line_count = text.position(0, height as i32).draw(display).unwrap();
                height += line_count * 32;
                display.flush();
            }

            // Example of handling a special key: exit the loop when the
            // escape key is pressed.
            Some(Key::Special(ScanCode::ESCAPE)) => {
                let text = TextData::text(800, &format!("ESC pressed"));

                text.position(0, height as i32).draw(display).unwrap();
                display.flush();
                break;
            }
            _ => {}
        }
    }

    Ok(())
}
