use crate::common::mouse::Mouse;
use crate::render::Renderer;
use crate::window::window::{Window, WINDOW_PADDING};
use std::sync::atomic::AtomicBool;
use crate::common::{SScreenSize, ScreenSize};

pub struct DisplayServer {
    windows: Vec<Window>,
    pub mouse: Mouse,
    low_state_mode: AtomicBool,
    is_window_dirty: AtomicBool,
    renderer: Option<Renderer>,
}

impl DisplayServer {
    pub fn new() -> Self {
        Self {
            windows: Vec::new(),
            mouse: Mouse::new(),
            low_state_mode: AtomicBool::new(false),
            is_window_dirty: AtomicBool::new(false),
            renderer: None
        }
    }
    
    pub fn add_window(&mut self, window: Window) {
        self.windows.push(window);
        self.is_window_dirty.store(true, std::sync::atomic::Ordering::Relaxed);
    }
    
    pub fn setup_renderer(&mut self,width: ScreenSize,height: ScreenSize) {
        self.renderer = Some(Renderer::new(width, height));
    }
    
    pub fn tick(&mut self) {
        if self.low_state_mode.load(std::sync::atomic::Ordering::Relaxed) {
            return;
        }
        self.update();
        let renderer = self.renderer.as_mut().expect("Render not set up");
        if self.is_window_dirty.load(std::sync::atomic::Ordering::Relaxed) {
            renderer.rerender_windows(&self.windows);
            self.is_window_dirty.store(false, std::sync::atomic::Ordering::Relaxed);
        }
        renderer.render();
    }

    fn update(&mut self) {
        for window in &mut self.windows {
            let rect = window.get_mut_render_rect();
            // resize or move the window
            if self.mouse.is_left_button_pressed() {
                if rect.is_near_side(self.mouse.get_x(), self.mouse.get_y(), 15) {
                    rect.width = (rect.width as SScreenSize + self.mouse.get_rel_x()).max(200) as ScreenSize;
                    self.is_window_dirty.store(true, std::sync::atomic::Ordering::Relaxed);
                }
                if rect.is_near_bottom(self.mouse.get_x(), self.mouse.get_y(), 15) {
                    rect.height = (rect.height as SScreenSize + self.mouse.get_rel_y()).max(200) as ScreenSize;
                    self.is_window_dirty.store(true, std::sync::atomic::Ordering::Relaxed);
                }
                if rect.is_near_top(self.mouse.get_x(), self.mouse.get_y(), 30 + WINDOW_PADDING) {
                    rect.x = (rect.x as SScreenSize + self.mouse.get_rel_x()) as ScreenSize;
                    rect.y = (rect.y as SScreenSize + self.mouse.get_rel_y()) as ScreenSize;
                    self.is_window_dirty.store(true, std::sync::atomic::Ordering::Relaxed);
                }
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
