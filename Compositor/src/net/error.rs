use std::io;
use thiserror::Error;
#[derive(Error, Debug)]
pub enum UnixSocketError {
    #[error("IO error: {0}")]
    Io(#[from] io::Error),

    #[error("Socket already exists and couldn't be removed: {0}")]
    SocketExists(String),

    #[error("Failed to set socket permissions: {0}")]
    PermissionError(String),

    #[error("Serialization error: {0}")]
    SerializationError(String),

    #[error("Connection to window {0} not found")]
    ConnectionNotFound(u64),

    #[error("Failed to send packet: {0}")]
    SendError(String),

    #[error("Recovery failed: {0}")]
    RecoveryFailed(String),
}