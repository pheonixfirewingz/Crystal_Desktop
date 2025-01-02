use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::Sender;
use std::thread::JoinHandle;
use crate::net::error::UnixSocketError;

pub struct NetHandle {
    control_sender: Sender<ControlMessage>,
    running: Arc<AtomicBool>,
    thread_handle: Option<JoinHandle<()>>,
}

pub enum ControlMessage {
    Stop,
    Pause,
    Resume,
}

impl NetHandle {
    pub fn new(control_sender: Sender<ControlMessage>,running: Arc<AtomicBool>,thread_handle: Option<JoinHandle<()>>) -> Self {
        Self {
            control_sender,
            running,
            thread_handle,
        }
    }
    
    pub fn stop(&mut self) -> crate::net::Result<()> {
        println!("Stopping compositor server");
        self.running.store(false, Ordering::SeqCst);
        let _ = self.control_sender.send(ControlMessage::Stop);

        if let Some(handle) = self.thread_handle.take() {
            handle.join().map_err(|_| {
                UnixSocketError::RecoveryFailed("Failed to join compositor thread".into())
            })?;
        }

        Ok(())
    }

    pub fn pause(&self) -> crate::net::Result<()> {
        println!("Pausing compositor server");
        self.control_sender
            .send(ControlMessage::Pause)
            .map_err(|_| UnixSocketError::SendError("Failed to send pause message".into()))?;
        Ok(())
    }

    pub fn resume(&self) -> crate::net::Result<()> {
        println!("Resuming compositor server");
        self.control_sender
            .send(ControlMessage::Resume)
            .map_err(|_| UnixSocketError::SendError("Failed to send resume message".into()))?;
        Ok(())
    }

    pub fn is_running(&self) -> bool {
        self.running.load(Ordering::SeqCst)
    }
}

impl Drop for NetHandle {
    fn drop(&mut self) {
        let _ = self.stop();
    }
}