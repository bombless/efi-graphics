use alloc::vec::Vec;
use rusttype::{Font, Scale, point};

pub(crate) fn main() -> Vec<u8> {
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
    return pixel_data_first_line
        .into_iter()
        .chain(pixel_data_second_line)
        .collect();
}
