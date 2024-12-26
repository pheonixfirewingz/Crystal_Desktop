#[derive(Copy, Clone, Debug)]
pub struct Rect {
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
}
impl Rect {
    pub fn new(x: u32, y: u32, width: u32, height: u32) -> Rect {
        Rect {x, y, width, height }
    }
    
    pub fn top_left(&self) -> (u32, u32) {
        (self.x, self.y)
    }
    
    pub fn top_right(&self) -> (u32, u32) {
        (self.x + self.width, self.y)
    }
    pub fn bottom_right(&self) -> (u32, u32) {
        (self.x + self.width, self.y + self.height)
    }
    
    pub fn bottom_left(&self) -> (u32, u32) {
        (self.x, self.y + self.height)
    }
    
    pub fn set_pos(&mut self, x: u32, y: u32) {
        self.x = x;
        self.y = y;
    }
    
    pub fn set_width(&mut self, width: u32) {
        self.width = width;
    }
    
    pub fn set_height(&mut self, height: u32) {
        self.height = height;
    }
    
    pub fn set_size(&mut self, width: u32, height: u32) {
        self.width = width;
        self.height = height;
    }
}