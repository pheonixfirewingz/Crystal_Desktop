use std::io;
use std::io::{ErrorKind, Read, Write};
use std::os::unix::net::UnixStream;
use std::sync::{Arc, Mutex};

pub use libprotocol;
use libprotocol::{Packet, ScreenSize, PROTOCOL_VERSION, WINDOW_UNIX_SOCKET_NAME};
struct ClientState {
    stream: UnixStream,
    callback: Box<dyn Fn(Packet) -> Option<Packet> + Send + Sync>,
    already_closed: bool,
    size: (ScreenSize, ScreenSize),
    position: (ScreenSize, ScreenSize),
    window_id: u64,
}

// Each instance gets its own isolated state
pub struct Client {
    state: Arc<Mutex<Option<ClientState>>>,
}

// Thread-safe singleton pattern to track all active clients
struct ClientRegistry {
    clients: Mutex<Vec<Arc<Mutex<Option<ClientState>>>>>,
}

impl ClientRegistry {
    fn get_instance() -> Arc<ClientRegistry> {
        static mut INSTANCE: Option<Arc<ClientRegistry>> = None;
        static INIT: std::sync::Once = std::sync::Once::new();

        #[allow(static_mut_refs)]
        unsafe {
            INIT.call_once(|| {
                INSTANCE = Some(Arc::new(ClientRegistry {
                    clients: Mutex::new(Vec::new()),
                }));
            });
            INSTANCE.clone().unwrap()
        }
    }

    fn register_client(&self, state: Arc<Mutex<Option<ClientState>>>) {
        let mut clients = self.clients.lock().unwrap();
        clients.push(state);
    }

    fn unregister_client(&self, state: &Arc<Mutex<Option<ClientState>>>) {
        let mut clients = self.clients.lock().unwrap();
        clients.retain(|x| !Arc::ptr_eq(x, state));
    }
}

pub fn open_window(
    title: Option<String>,
    width: ScreenSize,
    height: ScreenSize,
    callback: impl Fn(Packet) -> Option<Packet> + Send + Sync + 'static,
) -> std::io::Result<Arc<Client>> {
    let client = Client {
        state: Arc::new(Mutex::new(None)),
    };

    // Connect to the
    let stream = UnixStream::connect(&WINDOW_UNIX_SOCKET_NAME)?;
    let create_packet = Packet::Create {
        width,
        height,
        title,
    };

    let mut state = ClientState {
        stream,
        callback: Box::new(callback),
        already_closed: false,
        size: (width, height),
        position: (-1, -1),
        window_id: 0,
    };

    send_packet(&mut state.stream, &create_packet)?;
    std::thread::sleep(std::time::Duration::from_millis(5));
    {
        let packet = receive_packet(&mut state.stream)?;
        match packet {
            Packet::CreateSuccess { window_id } => {
                state.window_id = window_id;
            }
            _ => {
                return Err(std::io::Error::new(
                    ErrorKind::ConnectionAborted,
                    "compositor sent the wrong packet type",
                ));
            }
        }
    }
    send_packet(&mut state.stream, &Packet::RequestAPIVersion)?;
    std::thread::sleep(std::time::Duration::from_millis(5));
    {
        let packet = receive_packet(&mut state.stream)?;
        match packet {
            Packet::APIVersion {
                major,
                minor,
                patch,
            } => {
                let (expected_major, expected_minor, expected_patch) = PROTOCOL_VERSION;
                if major != expected_major || minor != expected_minor || patch != expected_patch {
                    send_packet(&mut state.stream, &Packet::Close { window_id: 0 }).expect("");
                    return Err(std::io::Error::new(
                        ErrorKind::ConnectionAborted,
                        "ABI version is not the same as the windowing lib refused to connect",
                    ));
                }
            }
            _ => {
                return Err(std::io::Error::new(
                    ErrorKind::ConnectionAborted,
                    "compositor sent the wrong packet type",
                ));
            }
        }
    }

    // Register the client
    let registry = ClientRegistry::get_instance();
    *client.state.lock().unwrap() = Some(state);
    registry.register_client(client.state.clone());

    Ok(Arc::new(client))
}

impl Client {
    pub fn pump_window(&self) {
        if let Some(state) = self.state.lock().unwrap().as_mut() {
            if let Ok(packet) = receive_packet(&mut state.stream) {
                match packet {
                    Packet::Closed => {
                        state.already_closed = true;
                    }
                    Packet::Resize { width, height } => {
                        state.size = (width, height);
                        return;
                    }
                    Packet::Position { x, y } => {
                        state.position = (x, y);
                        return;
                    }
                    _ => {}
                }
                if let Some(response) = (state.callback)(packet) {
                    let _ = send_packet(&mut state.stream, &response);
                }
            }
        }
    }

    pub fn get_window_size(&self) -> (ScreenSize, ScreenSize) {
        self.state.lock().unwrap().as_ref().unwrap().size.clone()
    }

    pub fn get_window_position(&self) -> (ScreenSize, ScreenSize) {
        self.state
            .lock()
            .unwrap()
            .as_ref()
            .unwrap()
            .position
            .clone()
    }

    pub fn close_window(&self) {
        let registry = ClientRegistry::get_instance();

        if let Some(state) = self.state.lock().unwrap().as_mut() {
            // Only send close packet if we haven't received Closed from
            if !state.already_closed {
                let close_packet = Packet::Close {
                    window_id: state.window_id,
                };
                let _ = send_packet(&mut state.stream, &close_packet);
            }
        }

        // Unregister the client
        registry.unregister_client(&self.state);
        *self.state.lock().unwrap() = None;
    }
}

impl Drop for Client {
    fn drop(&mut self) {
        self.close_window();
    }
}

fn receive_packet(stream: &mut UnixStream) -> io::Result<Packet> { 
    let  ret = libprotocol::receive_packet(stream);
    if let Ok(packet) = &ret {
        println!("LibCrystalMatrix: Received packet: {:?}", packet);
    }
    ret
}

fn send_packet(stream: &mut UnixStream, packet: &Packet) -> io::Result<()> {
    println!("LibCrystalMatrix: Sending packet {:?}", packet);
    libprotocol::send_packet(stream, packet)
}
