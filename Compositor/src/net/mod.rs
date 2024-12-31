use std::fs;
use std::os::unix::net::{UnixListener, UnixStream};
use std::path::Path;
use std::sync::{Arc, RwLock};
use std::sync::atomic::{AtomicBool, Ordering};
use crate::DisplayServer;

pub enum Packet {
    Create,
    Paint,
    MouseEnter,
    MouseLeave,
    MouseMove,
    MouseDown,
    MouseUp,
    KeyDown,
    KeyUp,
    Move,
    Resize,
    Suspend,
    Resume,
    Close,
}

const WINDOW_UNIX_SOCKET_NAME: &'static str = "/tmp/prism_comp";

pub fn handle_client(_: UnixStream,_:Arc<RwLock<DisplayServer>>){
}

pub fn handle_net(dm_server: Arc<RwLock<DisplayServer>>,term: Arc<AtomicBool>) {
    let socket = Path::new(WINDOW_UNIX_SOCKET_NAME);
    // Delete old socket if necessary
    if socket.exists() {
        fs::remove_file(&socket).unwrap();
    }
    let listener = UnixListener::bind(WINDOW_UNIX_SOCKET_NAME).unwrap();
    println!("Compositor server listening on unix socket {}", WINDOW_UNIX_SOCKET_NAME);
    while !term.load(Ordering::Relaxed) {
        for stream in listener.incoming() {
            let dm_server = dm_server.clone();
            std::thread::spawn(|| {handle_client(stream.unwrap(), dm_server)});
        }
    }
    println!("Compositor server disconnected");
    fs::remove_file(&socket).unwrap();
}