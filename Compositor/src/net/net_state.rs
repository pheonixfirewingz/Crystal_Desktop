use crate::net::connection_state::ConnectionStats;
use crate::net::error::UnixSocketError;
use crate::net::Packet;
use crate::net::{PacketHandler};
use libprotocol::Packet::{APIVersion, Close, RequestAPIVersion};
use libprotocol::{PROTOCOL_VERSION, WINDOW_UNIX_SOCKET_NAME};
use std::collections::HashMap;
use std::io::ErrorKind;
use std::os::unix::net::{UnixListener, UnixStream};
use std::path::Path;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use std::{fs, io};

pub struct ServerState {
    listener: UnixListener,
    connections: HashMap<u64, (UnixStream, ConnectionStats)>,
    packet_handler: Arc<Mutex<Box<dyn PacketHandler + Send>>>,
    max_recovery_attempts: u32,
}

impl ServerState {
    pub fn new(
        listener: UnixListener,
        packet_handler: Arc<Mutex<Box<dyn PacketHandler + Send>>>,
        max_recovery_attempts: u32,
    ) -> Self {
        Self {
            listener,
            connections: HashMap::new(),
            packet_handler,
            max_recovery_attempts,
        }
    }

    pub fn make_random_id(&self) -> u64 {
        loop {
            let id = std::random::random();
            if !self.connections.contains_key(&id) {
                return id;
            }
        }
    }

    pub fn accept_connections(&mut self) -> crate::net::Result<()> {
        for stream in self.listener.incoming() {
            match stream {
                Ok(stream) => {
                    let window_id = self.make_random_id();
                    stream.set_nonblocking(true)?;
                    stream.set_read_timeout(Some(Duration::from_secs(30)))?;
                    stream.set_write_timeout(Some(Duration::from_secs(5)))?;

                    let stats = ConnectionStats::new(Instant::now(), Instant::now(), 0, 0);

                    let stream_clone = stream.try_clone()?;
                    self.connections.insert(window_id, (stream_clone, stats));

                    println!("New connection established for window {}", window_id);
                }
                Err(e) if e.kind() == ErrorKind::WouldBlock => {
                    break;
                }
                Err(e) => {
                    eprintln!("Error accepting connection: {}", e);
                }
            }
        }
        Ok(())
    }

    pub fn process_packets(&mut self) -> crate::net::Result<()> {
        let mut connections = std::mem::take(&mut self.connections);
        for (&window_id, (stream, stats)) in &mut connections {
            if let Err(e) = self.handle_connection(window_id, stream) {
                eprintln!("Error handling connection {}: {}", window_id, e);
                stats.add_error();

                if stats.errors() >= 3 {
                    if !self.attempt_recovery(window_id)? {
                        self.handle_connection_failure(window_id)?;
                    }
                }
            }
        }
        self.connections = connections;
        Ok(())
    }

    pub fn handle_connection(
        &mut self,
        window_id: u64,
        stream: &mut UnixStream,
    ) -> crate::net::Result<()> {
        let packet = receive_packet(stream)?;
        if packet == RequestAPIVersion {
            let (major, minor, patch) = PROTOCOL_VERSION;
            let packet = APIVersion {
                major,
                minor,
                patch,
            };
            send_packet(stream, &packet)?;
            return Ok(());
        }
        let mut packet_out: Option<Packet> = None;
        if let Ok(mut handler) = self.packet_handler.lock() {
            packet_out = handler.handle_packet(window_id, packet)?
        }
        if let Some(packet_out) = packet_out {
            send_packet(stream, &packet_out)?;
        }
        Ok(())
    }

    pub(crate) fn check_connection_health(&mut self) -> crate::net::Result<()> {
        let now = Instant::now();
        let stale_connections: Vec<u64> = self
            .connections
            .iter()
            .filter(|(_, (_, stats))| {
                now.duration_since(stats.last_activity()) > Duration::from_secs(60)
            })
            .map(|(&id, _)| id)
            .collect();

        for window_id in stale_connections {
            println!("WARN: Stale connection detected for window {}", window_id);
            if !self.attempt_recovery(window_id)? {
                self.handle_connection_failure(window_id)?;
            }
        }
        Ok(())
    }

    fn handle_connection_failure(&mut self, window_id: u64) -> crate::net::Result<()> {
        println!("WARN:Handling connection failure for window {}", window_id);
        let failure_packet = Close { window_id };
        self.broadcast_to_others(window_id, &failure_packet)?;
        self.connections.remove(&window_id);
        println!("Connection failure handled for window {}", window_id);
        Ok(())
    }

    fn attempt_recovery(&mut self, window_id: u64) -> crate::net::Result<bool> {
        let mut connections = std::mem::take(&mut self.connections);
        if let Some((stream, stats)) = connections.get_mut(&window_id) {
            if stats.recovery_attempts() >= self.max_recovery_attempts {
                eprintln!("Max recovery attempts reached for window {}", window_id);
                return Ok(false);
            }

            println!("Attempting recovery for window {}", window_id);
            stats.recovery_attempts_increase();
            stream.set_nonblocking(false)?;
            stream.set_read_timeout(Some(Duration::from_secs(30)))?;
            stream.set_write_timeout(Some(Duration::from_secs(5)))?;
            let ping = RequestAPIVersion;
            if let Err(e) = send_packet(stream, &ping) {
                println!(
                    "WARN: Recovery attempt failed for window {}: {}",
                    window_id, e
                );
                return Ok(false);
            }

            println!("Recovery successful for window {}", window_id);
            stats.error_rest();
            self.connections = connections;
            Ok(true)
        } else {
            Err(UnixSocketError::ConnectionNotFound(window_id))
        }
    }

    fn get_stream(
        &mut self,
        window_id: u64,
    ) -> crate::net::Result<&mut (UnixStream, ConnectionStats)> {
        self.connections
            .get_mut(&window_id)
            .ok_or(UnixSocketError::ConnectionNotFound(window_id))
    }

    fn broadcast_to_others(&mut self, sender_id: u64, packet: &Packet) -> crate::net::Result<()> {
        let connections = std::mem::take(&mut self.connections);
        for (&window_id, _) in &connections {
            if window_id != sender_id {
                let (stream,_) = self.get_stream(window_id)?;
                if let Err(e) = send_packet(stream, &packet) {
                    println!("WARN: Failed to broadcast to window {}: {}", window_id, e);
                }
            }
        }
        self.connections = connections;
        Ok(())
    }

    pub fn cleanup(&mut self) -> crate::net::Result<()> {
        println!("Starting compositor cleanup");

        for (window_id, (_, _)) in self.connections.drain() {
            println!("Closing connection for window {}", window_id);
        }

        if Path::new(&WINDOW_UNIX_SOCKET_NAME).exists() {
            fs::remove_file(&WINDOW_UNIX_SOCKET_NAME).map_err(|e| {
                eprintln!("Failed to remove socket file: {}", e);
                UnixSocketError::Io(e)
            })?;
        }

        println!("Compositor cleanup completed successfully");
        Ok(())
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