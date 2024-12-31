use crate::common::ScreenSize;

#[derive(Copy, Clone, Debug)]
pub struct Rect {
    pub x: ScreenSize,
    pub y: ScreenSize,
    pub width: ScreenSize,
    pub height: ScreenSize,
}
impl Rect {
    #[inline]
    pub fn new(x: ScreenSize, y: ScreenSize, width: ScreenSize, height: ScreenSize) -> Rect {
        Rect {x, y, width, height }
    }
    #[inline]
    pub fn set_pos(&mut self, x: ScreenSize, y: ScreenSize) {
        self.x = x;
        self.y = y - self.height;
    }
    #[inline]
    pub fn set_width(&mut self, width: ScreenSize) {
        self.width = width;
    }
    #[inline]
    pub fn set_height(&mut self, height: ScreenSize) {
        self.height = height;
    }
    #[inline]
    pub fn set_size(&mut self, width: ScreenSize, height: ScreenSize) {
        self.width = width;
        self.height = height;
    }
    #[inline]
    pub fn contains(&self, x: ScreenSize, y: ScreenSize) -> bool {
        x >= self.x && x <= self.x + self.width && y >= self.y && y <= self.y + self.height
    }
    #[inline]
    pub fn is_near_bottom(&self, x: ScreenSize, y: ScreenSize, threshold: ScreenSize) -> bool {
        x >= self.x && x <= self.x + self.width && y >= self.y + self.height - threshold && y <= self.y + self.height + threshold
    }
    #[inline]
    pub fn is_near_side(&self, x: ScreenSize, y: ScreenSize, threshold: ScreenSize) -> bool {
        (x >= self.x + self.width - threshold && x <= self.x + self.width + threshold && y >= self.y && y <= self.y + self.height)
    }
    #[inline]
    pub fn is_near_top(&self, x: ScreenSize, y: ScreenSize, threshold: ScreenSize) -> bool {
        x >= self.x && x <= self.x + self.width && y >= self.y && y <= self.y + threshold
    }
}