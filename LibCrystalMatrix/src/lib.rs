use crate::common::ScreenSize;
use crate::net::packat::Packet;
use lazy_static::lazy_static;
pub mod common;
pub mod net;
mod manager;
lazy_static! {
    
}
pub fn open_window(_title: Option<String>, _width: ScreenSize, _height: ScreenSize,call_back: impl Fn(Packet) -> Option<Packet> + Send + Sync + 'static) {
}

pub fn pump_window() {
}

pub fn get_window_size() -> (ScreenSize, ScreenSize) {
    (0,0)
}

pub fn get_window_position() -> (ScreenSize, ScreenSize) {
    (0,0)
}

pub fn close_window() {}