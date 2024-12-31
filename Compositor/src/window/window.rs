use crate::render::api::texture::Texture;
use std::sync::atomic::{AtomicBool, Ordering};
use crate::common::ScreenSize;
use crate::render::util::rect::Rect;

pub struct Window { 
    pub(crate) rect: Rect,
    pub(crate) icon: Texture,
    frame_buffer: Texture,
    title: String,
    has_title_bar: bool,
    is_dirty: AtomicBool,
}

pub const WINDOW_PADDING: ScreenSize = 2;

impl Window {
    pub fn new(title: String, has_title_bar: bool, rect: Rect) -> Window {
        Window {
            title,
            has_title_bar,
            icon: Texture::none(),
            frame_buffer: Texture::none(),
            rect,
            is_dirty: AtomicBool::new(true),
        }
    }
    pub fn move_window(&mut self, move_x: ScreenSize, y: ScreenSize) {
        self.is_dirty.store(true, Ordering::Release);
        let rect = &self.rect;
        self.rect.set_pos(rect.x + move_x, rect.y + y);
    }
    pub fn resize_window(&mut self, width: ScreenSize, height: ScreenSize) {
        self.is_dirty.store(true, Ordering::Release);
        self.rect.set_size(width, height);
    }
    pub fn toggle_title_bar(&mut self) {
        self.is_dirty.store(true, Ordering::Release);
        self.has_title_bar = !self.has_title_bar;
    }
    pub fn update_title(&mut self, title: String) {
        self.is_dirty.store(true, Ordering::Release);
        self.title = title;
    }
    pub fn draw_title_bar(&self) -> bool {
        self.has_title_bar
    }
    pub fn is_dirty(&self) -> bool {
        let is = self.is_dirty.load(Ordering::Acquire);
        self.is_dirty.swap(false, Ordering::Release);
        is
    }
    pub fn get_size(&self) -> (ScreenSize, ScreenSize) {
        (self.rect.width, self.rect.height)
    }
    pub fn get_position(&self) -> (ScreenSize, ScreenSize) {
        (self.rect.x, self.rect.y)
    }
    pub fn get_title(&self) -> &String {
        &self.title
    }
    pub fn get_mut_render_rect(&mut self) -> &mut Rect {
        &mut self.rect
    }
    pub fn get_render_rect(&self) -> &Rect {
        &self.rect
    }
}
