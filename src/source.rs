use alloc::vec::Vec;
use core::iter::repeat_n;
use rusttype::{Font, Scale, point};

pub(crate) fn main() -> super::TextData {
    let font = {
        let font_data = include_bytes!("../../WenQuanYiMicroHei.ttf");
        Font::try_from_bytes(font_data as &[u8]).expect("error constructing a Font from bytes")
    };

    let height: f32 = 18.0;

    let scale = Scale {
        x: height,
        y: height,
    };

    let v_metrics = font.v_metrics(scale);
    let offset = point(0.0, v_metrics.ascent);

    let text = format!("offset {offset:?}");

    let glyphs: Vec<_> = font.layout(&text, scale, offset).collect();

    let mut pixel_data_first_line = vec![0u8; 300 * 32 * 3];
    for g in glyphs {
        if let Some(bb) = g.pixel_bounding_box() {
            g.draw(|x, y, v| {
                // v should be in the range 0.0 to 1.0
                let value = (v * 255.0) as u8;

                let x = x as i32 + bb.min.x;
                let y = y as i32 + bb.min.y;
                if x >= 300 || y >= 32 {
                    return;
                }
                let offset = (x * 3 + y * 300 * 3) as usize;
                pixel_data_first_line[offset] = value;
                pixel_data_first_line[offset + 1] = value;
                pixel_data_first_line[offset + 2] = value;
            })
        }
    }
    let text = format!("你好，我也好。");

    let glyphs: Vec<_> = font.layout(&text, scale, offset).collect();

    let mut pixel_data_second_line = vec![0u8; 300 * 32 * 3];
    for g in glyphs {
        if let Some(bb) = g.pixel_bounding_box() {
            g.draw(|x, y, v| {
                // v should be in the range 0.0 to 1.0
                let value = (v * 255.0) as u8;

                let x = x as i32 + bb.min.x;
                let y = y as i32 + bb.min.y;
                let offset = (x * 3 + y * 300 * 3) as usize;
                pixel_data_second_line[offset] = value;
                pixel_data_second_line[offset + 1] = value;
                pixel_data_second_line[offset + 2] = value;
            })
        }
    }
    super::TextData {
        width: 300,
        data: pixel_data_first_line
            .into_iter()
            .chain(pixel_data_second_line)
            .collect(),
    }
}

pub(crate) fn text(width: usize, text: &str) -> Vec<u8> {
    let font = {
        let font_data = include_bytes!("../../WenQuanYiMicroHei.ttf");
        Font::try_from_bytes(font_data as &[u8]).expect("error constructing a Font from bytes")
    };

    let height: f32 = 28.0;

    let scale = Scale {
        x: height,
        y: height,
    };

    let v_metrics = font.v_metrics(scale);

    let mut data = vec![0u8; width * 32 * 3];
    let mut line_count = 1;
    let mut cursor = point(0.0, v_metrics.ascent);
    let mut last_glyph = None;
    for c in text.chars() {
        let glyph = font.glyph(c);
        let id = glyph.id();
        let scaled = glyph.scaled(scale);
        let w = scaled.h_metrics().advance_width;
        if let Some(last) = last_glyph {
            cursor.x += font.pair_kerning(scale, last, id);
        }
        let glyph = scaled.positioned(cursor);
        cursor.x += w;
        if let Some(bounding_box) = glyph.pixel_bounding_box() {
            if c == '\n' || bounding_box.max.x + libm::ceilf(cursor.x) as i32 >= width as i32 {
                cursor.x = 0.0;
                cursor.y += 32.0;
                line_count += 1;
                data.extend(repeat_n(0, width * 32 * 3))
            }
            if c == '\n' {
                continue;
            }
            // Draw the glyph into the image per-pixel by using the draw closure
            glyph.draw(|x, y, v| {
                let x = x as i32 + bounding_box.min.x + 1;
                let y = y as i32 + bounding_box.min.y;
                if x >= 0 && y >= 0 && x < width as i32 && y < line_count * 32 {
                    let x = x as usize;
                    let y = y as usize;
                    data[(y * width + x) * 3] = (v * 255.0) as u8;
                    data[(y * width + x) * 3 + 1] = (v * 255.0) as u8;
                    data[(y * width + x) * 3 + 2] = (v * 255.0) as u8;
                }
            });
        }
        last_glyph = Some(id);
    }
    data
}
