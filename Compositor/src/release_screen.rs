//TODO

use std::sync::{Arc, RwLock};
use std::sync::atomic::{AtomicBool};
//use evdev::{Device, InputEventKind};
use crate::DisplayServer;

pub fn handle_hardware(_display_server: Arc<RwLock<DisplayServer>>, _running: Arc<AtomicBool>) {
    //TODO: this needs alot of work
    return;
   /* // Open mouse and keyboard devices
    let mut lid_switch = Device::open("/dev/input/event0").expect("failed to open laptop lid switch");
    let mut mouse = Device::open("/dev/input/event5").expect("failed to open mouse");
    let mut keyboard = Device::open("/dev/input/event3").expect("failed to open keyboard");

    // Get device names
    let mouse_name = mouse.name().expect("failed to get mouse name");
    let keyboard_name = keyboard.name().expect("failed to get keyboard name");
    println!("ATTACHED MOUSE: {mouse_name}");
    println!("ATTACHED KEYBOARD: {keyboard_name}");
    println!("MOUSE: {mouse}");

    // Loop to process events from both devices
    while !running.load(Ordering::SeqCst) {
        if let Ok(events) = lid_switch.fetch_events() {
            for event in events {
                match event.kind() {
                    InputEventKind::Switch(lid) => {
                        println!("LID: {lid:?}");
                    }
                    _ => {}
                }
            }
        }
        // Handle mouse events
        if let Ok(events) = mouse.fetch_events() {
            for event in events {
                match event.kind() {
                    InputEventKind::RelAxis(axis) => {
                        println!("Mouse move event: {:?}, value: {}", axis, event.value());
                    }
                    InputEventKind::AbsAxis(axis) => {
                        println!("Mouse move event: {:?}, value: {}", axis, event.value());
                    }
                    InputEventKind::Key(key_code) => {
                        println!("Mouse button event: {:?}", key_code);
                    }
                    InputEventKind::Synchronization(sync) => {
                        println!("Mouse synchronization event");
                    }
                    InputEventKind::Led(led) => {
                        println!("Mouse led event: {:?}, value: {}", led, event.value());
                    }
                    _ => {
                        println!("Unhandled mouse event: {:?}", event);
                    }
                }
            }
        }
        // Handle keyboard events
        if let Ok(events) = keyboard.fetch_events() {
            for event in events {
                match event.kind() {
                    InputEventKind::Key(key_code) => {
                        println!("Key event: {:?}, value: {}", key_code, event.value());
                    }
                    InputEventKind::Synchronization(sync) => {
                        println!("Keyboard synchronization event");
                    }
                    InputEventKind::Led(led) => {
                        println!("Key led event: {:?}, value: {}", led, event.value());
                    }
                    evdev::InputEventKind::Misc(evdev::MiscType::MSC_SCAN) => {
                        println!("Scan code event: value={}", event.value());
                    }
                    _ => {
                        println!("Unhandled keyboard event: {:?}", event);
                    }
                }
            }
        }
    }
    println!("Exiting handle_hardware loop.");*/
}

pub fn start_screen(_dm_server: Arc<RwLock<DisplayServer>>, _shutdown: Arc<AtomicBool>) {
    
}