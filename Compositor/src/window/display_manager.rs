use crate::common::mouse::Mouse;
use crate::render::Renderer;
use crate::window::window::{Window, WINDOW_PADDING};
use std::collections::HashMap;
use std::hash::{DefaultHasher, Hash, Hasher};
use libprotocol::ScreenSize;
use crate::render::api::texture::Texture;
use crate::render::util::rect::{Position, Rect, Size};

pub struct DisplayServer {
    windows: HashMap<u64,Window>,
    mouse: Mouse,
    low_state_mode: bool,
    is_mouse_dirty: bool,
    is_window_dirty: bool,
    is_background_dirty: bool,
    renderer: Option<Renderer>,
    width: ScreenSize,
    height: ScreenSize,
}

impl DisplayServer {
    pub fn new() -> Self {
        Self {
            windows: HashMap::new(),
            mouse: Mouse::new(),
            low_state_mode: false,
            is_window_dirty: false,
            is_mouse_dirty: false,
            is_background_dirty: false,
            renderer: None,
            width:0,
            height:0,
        }
    }
    
    pub fn get_center(&self,window_width:ScreenSize,window_height:ScreenSize) -> Rect {
        let width = self.width / 2 - window_width / 2;
        let height = self.height / 2 - window_height / 2;
        Rect::new(width,height,window_width,window_height)
    }
    
    pub fn update_mouse_pos(&mut self, x:ScreenSize, y:ScreenSize) {
        self.is_mouse_dirty = true;
        self.mouse.add_position(x, y);
    }
    
    pub fn update_mouse_wheel_delta(&mut self, x:f32, y:f32) {
        self.mouse.add_wheel_delta(x,y);
    }
    
    pub fn update_button_state(&mut self, button:u8,state:bool) {
        match button {
            0 => {
                self.mouse.set_left_button(state);
            }
            1 => {
                self.mouse.set_right_button(state);
            }
            2 => {
                self.mouse.set_middle_button(state);
            }
            _ => {}
        }
    }
    
    pub fn add_window(&mut self, mut window: Window) -> u64 {
        window.set_active(true);
        let mut hasher = DefaultHasher::new();
        window.get_title().hash(&mut hasher);
        let hash = hasher.finish();
        self.windows.insert(hash,window);
        self.is_window_dirty = true;
        hash
    }

    pub fn get_window_size(&self,window_id: &u64) -> Size {
        let windows = &self.windows[&window_id];
        let rect = &windows.get_render_rect();
        rect.size
    }
    
    pub fn update_window_frame_buffer(&mut self,_window_id: &u64,_frame_buffer_data:&Vec<u8>) {
        self.is_window_dirty = true;
    }
    
    pub fn get_window_pos(&self,window_id: &u64) -> Position {
        let windows = &self.windows[&window_id];
        let rect = &windows.get_render_rect();
        rect.position
    }

    pub fn remove_window(&mut self, window_id: &u64) {
        self.windows.remove(&window_id);
        self.is_window_dirty = true;
    }
    
    pub fn setup_renderer(&mut self,width: ScreenSize,height: ScreenSize) {
        self.renderer = Some(Renderer::new(width, height));
        self.width = width;
        self.height = height;
    }
    
    pub fn tick(&mut self) {
        if self.low_state_mode {
            return;
        }
        self.update();
        let renderer = self.renderer.as_mut().expect("Render not set up");
        if self.is_mouse_dirty {
            renderer.rerender_mouse(self.mouse.get_x(),self.mouse.get_y());
            self.is_mouse_dirty = false;
        }
        if self.is_window_dirty {
            renderer.rerender_windows(&self.windows);
            self.is_window_dirty = false;
        }
        if self.is_background_dirty {
            renderer.rerender_background(&Texture::not_owned(0,0,0));
            self.is_background_dirty = false;
        }
        renderer.render();
    }

    fn update(&mut self) {
        for (_key,window) in &mut self.windows {
            if window.is_active() {
                let rect = window.get_mut_render_rect();
                // resize or move the window
                if self.mouse.is_left_button_pressed() {
                    if rect.is_near_bottom_right(self.mouse.get_x(), self.mouse.get_y(), 7) {
                        rect.size.width = (rect.size.width + self.mouse.get_rel_x()).max(200);
                        rect.size.height = (rect.size.height + self.mouse.get_rel_y()).max(200);
                        self.is_window_dirty = true;
                    }
                    if rect.is_near_right(self.mouse.get_x(), self.mouse.get_y(), 7) {
                        rect.size.width = (rect.size.width + self.mouse.get_rel_x()).max(200);
                        self.is_window_dirty = true;
                    }
                    if rect.is_near_bottom(self.mouse.get_x(), self.mouse.get_y(), 7) {
                        rect.size.height = (rect.size.height + self.mouse.get_rel_y()).max(200);
                        self.is_window_dirty = true;
                    }
                    if rect.is_near_top(self.mouse.get_x(), self.mouse.get_y(), 30 + WINDOW_PADDING) {
                        rect.position.x = (rect.position.x + self.mouse.get_rel_x()).clamp(0, self.width - rect.size.width);
                        rect.position.y = (rect.position.y + self.mouse.get_rel_y()).clamp(0, self.height - rect.size.height);
                        self.is_window_dirty = true;
                    }
                }
                break;
            }
        }
    }
    
    pub fn cleanup(&mut self) {
        if let Some(renderer) = self.renderer.as_mut() {
            renderer.cleanup();
        }
        self.windows.clear();
    }
}

impl Drop for DisplayServer {
    fn drop(&mut self) {
        self.cleanup();
    }
}
