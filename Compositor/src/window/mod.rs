use crate::render::rect;
use crate::render::rect::Rect;

#[derive(Clone)]
pub struct Window {
    rect: rect::Rect,
    title: String,
}
impl Window {
    pub fn new(title: String, rect:rect::Rect) -> Window {
        Window { title, rect }
    }
    pub fn move_window(&mut self, move_x: u32, y: u32) {
        let rect = &self.rect;
        self.rect.set_pos(rect.x + move_x,rect.y + y);
    }
    pub fn resize_window(&mut self, width: u32, height: u32) {
        self.rect.set_size(width, height);
    }
    pub fn get_size(&self) -> (u32, u32) {
        (self.rect.width,self.rect.height)
    }
    pub fn get_position(&self) -> (u32, u32) {
        (self.rect.x,self.rect.y)
    }
    pub fn update_title(&mut self, title: String) {
        self.title = title;
    }
    pub fn get_title(&self) -> String {
        self.title.clone()
    }
    pub fn get_render_rect(&self) -> Rect {
        self.rect.clone()
    }
}