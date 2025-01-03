#![feature(random)]

use crate::net::{CompositorServer, PacketHandler};
use crate::window::display_manager::DisplayServer;
use crate::window::window::Window;
use std::collections::HashMap;
use std::sync::atomic::AtomicBool;
use std::sync::{Arc, RwLock};
use libprotocol::Packet;

mod common;
mod debug_screen;
mod math;
mod net;
mod release_screen;
mod render;
mod window;

struct Prism {
    pub display: Arc<RwLock<DisplayServer>>,
    id_map: HashMap<u64, u64>,
}

impl Prism {
    pub fn new() -> Self {
        Self {
            display: Arc::new(RwLock::new(DisplayServer::new())),
            id_map: HashMap::new(),
        }
    }
}

impl PacketHandler for Prism {
    fn handle_packet(&mut self, window_id: u64, packet: Packet) -> net::Result<Option<Packet>> {
        match packet {
            Packet::Create {
                width,
                height,
                title,
            } => {
                let window = if let Some(title) = title {
                    let dm = self.display.read().unwrap();
                    Window::new_titled(title,dm.get_center(width, height))
                } else {
                    let dm = self.display.read().unwrap();
                    Window::new_non_titled(dm.get_center(width, height))
                };
                let mut dm = self.display.write().unwrap();
                let id = dm.add_window(window);
                self.id_map.insert(window_id,id);
                return Ok(Option::from(Packet::CreateSuccess { window_id }));
            }
            /*Packet::RequestWindowPosition { window_id} => {
                let dm = self.display.read().unwrap();
                let pos = dm.get_window_pos(&self.id_map[&window_id]);
                return Ok(Option::from(Packet::Position {x:pos.x,y:pos.y}));
            }
            Packet::RequestWindowSize { window_id } => {
                let dm = self.display.read().unwrap();
                let size = dm.get_window_size(&self.id_map[&window_id]);
                return Ok(Option::from(Packet::Size {width:size.width,height:size.height}));
            }
            Packet::Paint { window_id, buffer} => {
                let mut  dm = self.display.write().unwrap();
                dm.update_window_frame_buffer(&self.id_map[&window_id], &buffer);
            }*/
            Packet::Close { window_id } => {
                let mut dm = self.display.write().unwrap();
                dm.remove_window(&self.id_map[&window_id]);
                self.id_map.remove(&window_id);
                return Ok(Option::from(Packet::Closed));
            }
            _ => {
                println!("New packet received: {:?}", packet);
            }
        }
        Ok(None)
    }
}

fn main() {
    let prism = Prism::new();
    let term = Arc::new(AtomicBool::new(false));
    let dm_server = Arc::clone(&prism.display);
    let net_net_server = CompositorServer::new(Box::new(prism));
    let mut control = net_net_server
        .spawn()
        .expect("net_net_server failed to start");
    signal_hook::flag::register(signal_hook::consts::SIGTERM, Arc::clone(&term))
        .expect("failed to set up SIGTERM hook");
    signal_hook::flag::register(signal_hook::consts::SIGQUIT, Arc::clone(&term))
        .expect("failed to set up SIGTERM hook");

    #[cfg(debug_assertions)]
    {
        debug_screen::start_screen(dm_server, term);
    }
    #[cfg(not(debug_assertions))]
    {
        release_screen::start_screen(dm_server, term);
    }
    control.stop().expect("failed to stop net server");
    println!("Shutting down compositor server...");
}
