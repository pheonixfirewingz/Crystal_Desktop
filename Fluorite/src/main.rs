use lib_crystal_matrix::net::packat::Packet;
use lib_crystal_matrix::{close_window, open_window, pump_window};
use std::sync::{Arc, Mutex};

fn main() {
    let should_close = Arc::new(Mutex::new(false));
    let should_close_clone = Arc::clone(&should_close);
    open_window(Some("Flourite".to_string()), 720, 1280, move |packet| {
        match packet {
            Packet::Closed => {
                let mut should_close = should_close_clone.lock().unwrap();
                *should_close = true; // Update the shared value
            }
            _ => {}
        }
        None
    });
    loop {
        if *should_close.lock().unwrap() {
            break;
        }
        pump_window();
    }
    close_window();
}

