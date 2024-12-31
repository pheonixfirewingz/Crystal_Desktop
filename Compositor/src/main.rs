use std::sync::atomic::AtomicBool;
use std::sync::{Arc, RwLock};
use std::time::Duration;
use crate::net::handle_net;
use crate::window::display_manager::DisplayServer;
use crate::render::util::rect::Rect;
use crate::window::window::Window;
mod net;
mod render;
mod window;
mod math;
mod common;
pub mod debug_screen;
pub mod release_screen;

fn main() {
    let dm_server = Arc::new(RwLock::new(DisplayServer::new()));
    let term = Arc::new(AtomicBool::new(false));
    signal_hook::flag::register(signal_hook::consts::SIGTERM, Arc::clone(&term))
        .expect("failed to set up SIGTERM hook");
    signal_hook::flag::register(signal_hook::consts::SIGQUIT, Arc::clone(&term))
        .expect("failed to set up SIGTERM hook");
    // launch unix socket communication
    {
        let term_clone = Arc::clone(&term);
        let dm_server_clone = Arc::clone(&dm_server);
        std::thread::spawn(move || handle_net(dm_server_clone, term_clone));
    }
    //render::load_ascii(18f32);
    let target_frame_time = Duration::from_secs(1);
    {
        let mut dm = dm_server.write().expect("failed to get DM server");
        dm.add_window(Window::new("test window".to_string(),true,Rect::new(10, 10, 1280/2, 720/2)));
    }
    debug_screen::start_screen(dm_server.clone(), term.clone());
    //util::release_screen::start_screen(dm_server.clone(), term.clone());
    println!("Shutting down compositor server...");
}
