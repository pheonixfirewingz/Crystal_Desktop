use serde::{Deserialize, Serialize};
use std::{fmt, io};
use std::io::{Read, Write};
use std::os::unix::net::UnixStream;

pub type ScreenSize = i32;
pub const WINDOW_UNIX_SOCKET_NAME: &'static str = "/tmp/prism_comp";
pub const PROTOCOL_VERSION: (u8, u8, u8) = (0, 0, 1);
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum Packet {
    // TO Compositor
    Create {
        width: ScreenSize,
        height: ScreenSize,
        title: Option<String>,
    },
    Close {
        window_id: u64,
    },
    Paint {
        window_id: u64,
    },
    RequestAPIVersion,
    //TO Client
    CreateSuccess {
        window_id: u64,
    },
    Closed,
    MouseEnter,
    MouseLeave,
    MousePosition {
        x: ScreenSize,
        y: ScreenSize,
    },
    MouseDown {
        button: MouseButton,
        x: ScreenSize,
        y: ScreenSize,
    },
    MouseUp {
        button: MouseButton,
        x: ScreenSize,
        y: ScreenSize,
    },
    KeyDown {
        key: KeyCode,
        modifiers: Modifiers,
    },
    KeyUp {
        key: KeyCode,
        modifiers: Modifiers,
    },
    Position {
        x: ScreenSize,
        y: ScreenSize,
    },
    Resize {
        width: ScreenSize,
        height: ScreenSize,
    },
    Suspend,
    Resume,
    DemandPaint,
    APIVersion {
        major: u8,
        minor: u8,
        patch: u8,
    },
}
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum MouseButton {
    Left,
    Right,
    Middle,
    Other(u8),
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub struct Modifiers {
    pub shift: bool,
    pub ctrl: bool,
    pub alt: bool,
    pub logo: bool,
}
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum KeyCode {
    Backspace,
    Tab,
    Enter,
    Escape,
    Space,
    Delete,
    Character(char),
    Function(u8),
}

fn deserialize(packet: &Vec<u8>) -> bincode::Result<Packet> {
    bincode::deserialize(&packet)
}
fn serialize(packet: &Packet) -> bincode::Result<Vec<u8>> {
    bincode::serialize(&packet)
}

pub fn receive_packet(stream: &mut UnixStream) -> io::Result<Packet> {
    let mut size_buf = [0u8; 4];
    stream.read_exact(&mut size_buf)?;
    let size = u32::from_le_bytes(size_buf) as usize;
    let mut packet_buf = vec![0u8; size];
    stream.read_exact(&mut packet_buf)?;
    bincode::deserialize(&packet_buf)
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e.to_string()))
}

pub fn send_packet(stream: &mut UnixStream, packet: &Packet) -> io::Result<()> {
    let data = serialize(packet)
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?;

    let size = (data.len() as u32).to_le_bytes();
    stream.write_all(&size)?;
    stream.write_all(&data)?;
    stream.flush()?;

    Ok(())
}



impl fmt::Display for Packet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Packet::Create {
                width,
                height,
                title,
            } => write!(
                f,
                "Create: width={}, height={}, title={:?}",
                width, height, title
            ),
            Packet::Close { window_id } => write!(f, "Close: window_id={}", window_id),
            Packet::Paint { window_id } => {
                write!(f, "Paint: window_id={}", window_id)
            }
            Packet::RequestAPIVersion => write!(f, "RequestAPIVersion"),
            Packet::CreateSuccess { window_id } => {
                write!(f, "CreateSuccess: window_id={}", window_id)
            }
            Packet::Closed => write!(f, "Closed"),
            Packet::MouseEnter => write!(f, "MouseEnter"),
            Packet::MouseLeave => write!(f, "MouseLeave"),
            Packet::MousePosition { x, y } => write!(f, "MousePosition: x={}, y={}", x, y),
            Packet::MouseDown { button, x, y } => {
                write!(f, "MouseDown: button={}, x={}, y={}", button, x, y)
            }
            Packet::MouseUp { button, x, y } => {
                write!(f, "MouseUp: button={}, x={}, y={}", button, x, y)
            }
            Packet::KeyDown { key, modifiers } => {
                write!(f, "KeyDown: key={}, modifiers={}", key, modifiers)
            }
            Packet::KeyUp { key, modifiers } => {
                write!(f, "KeyUp: key={}, modifiers={}", key, modifiers)
            }
            Packet::Position { x, y } => write!(f, "Position: x={}, y={}", x, y),
            Packet::Resize { width, height } => {
                write!(f, "Resize: width={}, height={}", width, height)
            }
            Packet::Suspend => write!(f, "Suspend"),
            Packet::Resume => write!(f, "Resume"),
            Packet::DemandPaint => write!(f, "DemandPaint"),
            Packet::APIVersion {
                major,
                minor,
                patch,
            } => write!(f, "APIVersion: {}.{}.{}", major, minor, patch),
        }
    }
}

impl fmt::Display for MouseButton {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MouseButton::Left => write!(f, "Left"),
            MouseButton::Right => write!(f, "Right"),
            MouseButton::Middle => write!(f, "Middle"),
            MouseButton::Other(code) => write!(f, "Other({})", code),
        }
    }
}

impl fmt::Display for Modifiers {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Modifiers(shift={}, ctrl={}, alt={}, logo={})",
            self.shift, self.ctrl, self.alt, self.logo
        )
    }
}

impl fmt::Display for KeyCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            KeyCode::Backspace => write!(f, "Backspace"),
            KeyCode::Tab => write!(f, "Tab"),
            KeyCode::Enter => write!(f, "Enter"),
            KeyCode::Escape => write!(f, "Escape"),
            KeyCode::Space => write!(f, "Space"),
            KeyCode::Delete => write!(f, "Delete"),
            KeyCode::Character(c) => write!(f, "Character({})", c),
            KeyCode::Function(num) => write!(f, "Function({})", num),
        }
    }
}
