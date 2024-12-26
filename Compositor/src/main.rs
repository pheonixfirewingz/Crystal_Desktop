use crate::render::framebuffer::FrameBuffer;
use crate::render::rect;
use crate::window::Window;
use std::fs;
use std::os::unix::net::{UnixListener, UnixStream};
use std::path::Path;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc,RwLock};
use std::time::{Duration, Instant};

mod window;
mod util;
mod net;
mod render;

const WINDOW_UNIX_SOCKET_NAME: &'static str = "/tmp/prism_comp";

struct DisplayServer
{
    windows: Vec<Window>,
    low_state_mode: bool,
}

fn handle_hardware(_: Arc<RwLock<DisplayServer>>,_: Arc<AtomicBool>) {
    
}

fn handle_client(_: UnixStream,_:Arc<RwLock<DisplayServer>>){
}
fn handle_net(dm_server: Arc<RwLock<DisplayServer>>,term: Arc<AtomicBool>) {
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

fn main() {
    let dm_server = Arc::new(RwLock::new(DisplayServer{windows: Vec::new(), low_state_mode: false }));
    let term = Arc::new(AtomicBool::new(false));
    let fb = Arc::new(RwLock::new(FrameBuffer::new(1920/4,1080/4)));
    util::debug_screen::start_debug_screen(fb.clone(), term.clone());
    signal_hook::flag::register(signal_hook::consts::SIGTERM, Arc::clone(&term)).expect("failed to set up SIGTERM hook");
    signal_hook::flag::register(signal_hook::consts::SIGQUIT, Arc::clone(&term)).expect("failed to set up SIGTERM hook");
    // launch hardware handler
    {
        let term_clone = Arc::clone(&term);
        let dm_server_clone = Arc::clone(&dm_server);
        std::thread::spawn(move || { handle_hardware(dm_server_clone, term_clone)});
    }
    // launch unix socket communication
    {
        let term_clone = Arc::clone(&term);
        let dm_server_clone = Arc::clone(&dm_server);
        std::thread::spawn(move || handle_net(dm_server_clone, term_clone));
    }
    let font = render::load_ascii(32.0f32);
    let target_fps = 60; // Target FPS
    let target_frame_time = Duration::from_secs(1) / target_fps;
    while !term.load(Ordering::Relaxed) {
        let start_time = Instant::now();
        {
            let fb = fb.write().expect("failed to get framebuffer");
            let c = fb.clone();
            let char_c = font[&'C'].clone();
            c.clone().blit(rect::Rect::new(0, 0, char_c.width, char_c.height), &char_c.buffer);
            //println!("FrameBuffer Data: {c:?}");
        }
        let elapsed = start_time.elapsed();
        if elapsed < target_frame_time {
            let sleep_time = target_frame_time - elapsed;
            std::thread::sleep(sleep_time);
        }
    }
    println!("Shutting down compositor server...");
    fs::remove_file(WINDOW_UNIX_SOCKET_NAME).unwrap();
}