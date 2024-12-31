use rusttype::{Font, Point, PositionedGlyph, Scale};
use std::collections::HashMap;
use crate::common::SScreenSize;
use crate::render::api::texture::Texture;

pub struct TextRenderer {
    font: Font<'static>,
    texture: Texture,
    cache: HashMap<String, TextCache>,
}

pub struct TextCache {
    pub actual_width: f32,
    pub actual_height: f32,
}

impl TextRenderer {
    pub fn new() -> Self {
        let font = Font::try_from_bytes(include_bytes!("/usr/share/fonts/truetype/ubuntu/Ubuntu-B.ttf")).unwrap();
        Self {
            font,
            texture: Texture::opengl(0,0),
            cache: HashMap::new(),
        }
    }

    fn layout_text(&self, text: &str, scale: Scale) -> (Vec<PositionedGlyph>, (f32, f32)) {
        let v_metrics = self.font.v_metrics(scale);
        let glyphs: Vec<PositionedGlyph> = self
            .font
            .layout(text, scale, Point {x:0.0, y:v_metrics.ascent})
            .collect();

        let width = if let Some(last_glyph) = glyphs.last() {
            if let Some(bbox) = last_glyph.pixel_bounding_box() {
                bbox.max.x as f32
            } else {
                0.0
            }
        } else {
            0.0
        };

        let height = v_metrics.ascent - v_metrics.descent;
        (glyphs, (width, height))
    }

    fn cache_text(&mut self, text: String, size: f32) -> Option<(f32, f32)> {
        let scale = Scale::uniform(size);
        let (glyphs, (width, height)) = self.layout_text(&text, scale);

        // Round up to power of 2
        let texture_width = width.ceil() as u32;
        let texture_height = height.ceil() as u32;

        let mut buffer = vec![0u8; (texture_width * texture_height * 4) as usize];

        // Render glyphs to buffer
        for glyph in glyphs {
            if let Some(bbox) = glyph.pixel_bounding_box() {
                glyph.draw(|x, y, v| {
                    let x = x as SScreenSize + bbox.min.x;
                    let y = y as SScreenSize + bbox.min.y;
                    if x >= 0 && x < texture_width as SScreenSize && y >= 0 && y < texture_height as SScreenSize {
                        let idx = ((y * texture_width as SScreenSize + x) * 4) as usize;
                        let alpha = (v * 255.0) as u8;
                        buffer[idx] = 255;
                        buffer[idx + 1] = 255;
                        buffer[idx + 2] = 255;
                        buffer[idx + 3] = alpha;
                    }
                });
            }
        }
        self.texture.write(buffer.as_slice(),texture_width,texture_height);
        self.cache.insert(
            text,
            TextCache {
                actual_width: width,
                actual_height: height,
            },
        );

        Some((width, height))
    }

    pub fn remove_from_cache(&mut self, text: &str) {
        self.cache.remove(text);
    }

    pub fn clear_cache(&mut self) {
        self.cache.clear();
    }

    pub fn get_cached_text(&mut self, text: &str, size: f32) -> &TextCache {
        if !self.cache.contains_key(text) {
            self.cache_text(text.to_string(), size).expect("failed to create text");
        }
        &self.cache[text]
    }

    pub fn get_texture(&self) -> &Texture {
        &self.texture
    }

    pub fn cleanup(&mut self) {
        self.texture.cleanup();
        self.cache.clear();
    }
}