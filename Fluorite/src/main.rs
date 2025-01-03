use libcrystalmatrix::{open_window};
use std::sync::{Arc, Mutex};
use libcrystalmatrix::libprotocol::Packet;

fn main() {
    let should_close = Arc::new(Mutex::new(false));
    let should_close_clone = Arc::clone(&should_close);

    let client = open_window(Some("Flourite".to_string()),1280/2 ,720/2,move |packet| {
        match packet {
            Packet::Closed => {
                let mut should_close = should_close_clone.lock().unwrap();
                *should_close = true;
            }
            _ => {}
        }
        None
    }).expect("Failed to open window");

    loop {
        if *should_close.lock().unwrap() {
            break;
        }
        client.pump_window();
    }

    client.close_window();
}

