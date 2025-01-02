use crate::common::WindowID;
use crate::net::packat::Packet;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
pub type WindowCallback = Box<dyn Fn(Packet) -> Option<Packet> + Send + Sync>;

struct Window {
    pub window_callback: WindowCallback,
    pub owner_pid: std::thread::ThreadId,
}

struct WindowManager {
    registry: Mutex<HashMap<WindowID, Window>>,
}
impl WindowManager {
    // Create a new window manager.
    fn new() -> Self {
        Self {
            registry: Mutex::new(HashMap::new()),
        }
    }
    fn register_callback(&self, window: WindowID, callback: Window) {
        let mut registry = self.registry.lock().unwrap();
        registry.insert(window, callback);
    }
    fn trigger_event(&self, window_id: WindowID, event_data: Packet) -> Option<Packet> {
        let registry = self.registry.lock().unwrap();
        if let Some(window) = registry.get(&window_id) {
            let callback = &window.window_callback;
            return callback(event_data);
        }
        None
    }
}
lazy_static::lazy_static! {
    static ref WINDOW_MANAGER: Arc<WindowManager> = Arc::new(WindowManager::new());
}
pub fn manager_register(id: WindowID, callback: Window) {
    WINDOW_MANAGER.register_callback(id, callback);
}

pub fn window_trigger_event(window_id: WindowID, event_data: Packet) {
    WINDOW_MANAGER.trigger_event(window_id, event_data);
}
