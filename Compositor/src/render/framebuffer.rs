use crate::render::colour::Colour;
use crate::render::rect::Rect;

#[derive(Debug, Clone)]
pub struct FrameBuffer{
    width: u32,
    height: u32,
    buffer:  Box<[Colour]>,
}

impl FrameBuffer {
    pub fn new(width: u32, height: u32) -> Self {
        let size = (width * height) as usize;
        let buffer = vec![Colour { r: 0, g: 0, b: 0, a: 0 }; size].into_boxed_slice();
        Self { width, height, buffer }
    }
    
    pub fn width(&self) -> u32 {
        self.width
    }
    
    pub fn height(&self) -> u32 {
        self.height
    }
    
    pub fn clear(&mut self, colour: Colour) {
        for i in 0..self.buffer.len() {
            self.buffer[i] = colour.clone();
        }
    }
    
    pub unsafe fn as_ptr(&self) -> *const Colour {
        self.buffer.as_ptr()
    }

    pub fn blit(&mut self, rect: Rect, source: &Vec<Colour>) {
        let source_width = rect.width;
        let source_height = rect.height;

        // Ensure the source framebuffer fits within the destination rectangle
        if rect.x + source_width > self.width || rect.y + source_height > self.height {
            panic!("Source framebuffer does not fit within the destination rectangle.");
        }

        // Copy pixels from the source framebuffer to the destination framebuffer
        for y in 0..source_height {
            for x in 0..source_width {
                let source_index = (y * source_width + x) as usize;
                let dest_index = ((rect.y + y) * self.width + (rect.x + x)) as usize;
                self.buffer[dest_index] = source[source_index].clone();
            }
        }
    }
}