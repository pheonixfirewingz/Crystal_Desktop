pub mod error;
mod net_state;
pub mod packet;
mod connection_state;
pub mod handle;

use crate::net::error::UnixSocketError;
use crate::net::handle::{ControlMessage, NetHandle};
use crate::net::net_state::ServerState;
use crate::net::packet::Packet;
use std::fs;
use std::os::unix::net::UnixListener;
use std::path::Path;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::{channel, Receiver};
use std::sync::{Arc, Mutex};
use std::thread::{self};
use std::time::Duration;

const WINDOW_UNIX_SOCKET_NAME: &'static str = "/tmp/prism_comp";
pub type Result<T> = std::result::Result<T, UnixSocketError>;

pub trait PacketHandler: Send {
    fn handle_packet(&mut self, window_id: u64, packet: Packet) -> Result<Option<Packet>>;
}

pub struct CompositorServer {
    max_recovery_attempts: u32,
    packet_handler: Arc<Mutex<Box<dyn PacketHandler + Send>>>,
}

impl CompositorServer {
    pub fn new(packet_handler: Box<dyn PacketHandler + Send>) -> Self {
        Self {
            max_recovery_attempts: 3,
            packet_handler: Arc::new(Mutex::new(packet_handler)),
        }
    }

    pub fn spawn(self) -> Result<NetHandle> {
        let (control_tx, control_rx) = channel();
        let running = Arc::new(AtomicBool::new(true));
        let running_clone = running.clone();

        let thread_handle = thread::Builder::new()
            .name("compositor-server".into())
            .spawn(move || {
                self.run_server_loop(control_rx, running_clone);
            })
            .map_err(|e| UnixSocketError::Io(e))?;

        Ok(NetHandle::new(
            control_tx,
            running,
            Some(thread_handle),
        ))
    }

    fn initialize_server(&self) -> Result<ServerState> {
        println!("Initializing server state");

        if Path::new(&WINDOW_UNIX_SOCKET_NAME).exists() {
            fs::remove_file(&WINDOW_UNIX_SOCKET_NAME)?;
        }

        let listener = UnixListener::bind(&WINDOW_UNIX_SOCKET_NAME)?;
        listener.set_nonblocking(true)?;

        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            fs::set_permissions(&WINDOW_UNIX_SOCKET_NAME, fs::Permissions::from_mode(0o666))?;
        }

        Ok(ServerState::new(
            listener,
            self.packet_handler.clone(),
            self.max_recovery_attempts,
        ))
    }
    
    fn run_server_loop(self, control_rx: Receiver<ControlMessage>, running: Arc<AtomicBool>) {
        println!("Starting compositor server thread");

        let mut server_state = match self.initialize_server() {
            Ok(state) => state,
            Err(e) => {
                eprintln!("Failed to initialize server: {}", e);
                return;
            }
        };

        let mut paused = false;

        while running.load(Ordering::SeqCst) {
            match control_rx.try_recv() {
                Ok(ControlMessage::Stop) => break,
                Ok(ControlMessage::Pause) => paused = true,
                Ok(ControlMessage::Resume) => paused = false,
                Err(_) => {}
            }

            if !paused {
                if let Err(e) = server_state.accept_connections() {
                    eprintln!("Error accepting connections: {}", e);
                }

                if let Err(e) = server_state.process_packets() {
                    eprintln!("Error processing packets: {}", e);
                }

                if let Err(e) = server_state.check_connection_health() {
                    eprintln!("Error checking connection health: {}", e);
                }
            }

            thread::sleep(Duration::from_millis(16));
        }

        println!("Compositor server thread stopping");
        if let Err(e) = server_state.cleanup() {
            eprintln!("Error during server cleanup: {}", e);
        }
    }
}
