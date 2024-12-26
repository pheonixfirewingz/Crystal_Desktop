pub mod colour;
pub mod framebuffer;
pub mod rect;

use colour::Colour;
use rusttype::{point, Font, Scale};
use std::collections::HashMap;

const DEFAULT_FONT: &'static [u8] = include_bytes!("/usr/share/fonts/truetype/ubuntu/Ubuntu-B.ttf");
const ASCII: &'static str = "aAbBcCdDeEfFgGhHiIjJkKlLmMnNoNpPqQrRsStTuUvVxXyYzZ0123456789!\"£$%^&*()[]{}+*.,?\\|/\'@;:#~_<>-=0123456789";

#[derive(Debug, Clone)]
pub struct Glyph
{
    pub width: u32,
    pub height: u32,
    pub buffer: Vec<Colour>,
}

pub fn load_ascii(size:f32) -> HashMap<char,Glyph> {
    let font = Font::try_from_bytes(DEFAULT_FONT).unwrap();
    let mut map: HashMap<char,Glyph> = HashMap::new();
    for c in ASCII.chars() {
        map.insert(c, generate_font_char(&font, size, c));
    }
    map
}

pub fn generate_font_char(font: &Font, font_size: f32, letter: char) -> Glyph {
    // Set the scale based on the desired font size
    let scale = Scale::uniform(font_size);

    let glyph = font.glyph(letter).scaled(scale).positioned(point(0.0, 0.0));
    let bounding_box = glyph
        .pixel_bounding_box()
        .expect("Failed to get glyph bounding box");
    let width = bounding_box.width() as u32;
    let height = bounding_box.height() as u32;
    // Create a vector to store the pixel data
    let mut pixels = vec![Colour::new(0, 0, 0, 0); (width * height) as usize];

    // Rasterize the glyph into the pixel data
    glyph.draw(|x, y, v| {
        let px = x as u32;
        let py = y as u32;
        if px < width && py < height {
            let index = (py * width + px) as usize;
            if v > 0.0 {
                pixels[index] = Colour::new(255, 255, 255, 255);
            } else {
                pixels[index] = Colour::new(0, 0, 0, (v * 255.0) as u8);
            }
        }
    });
    Glyph {width, height, buffer: pixels}
}
