use serde::{Deserialize, Serialize};
use crate::common::ScreenSize;
pub const PROTOCOL_VERSION: (u8, u8, u8) = (0, 0, 1);
#[derive(Debug, Clone, Serialize, Deserialize,PartialEq, Eq)]
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
        buffer: Vec<u8>,
    },
    RequestAPIVersion,
    RequestWindowPosition{
        window_id: u64,
    },
    RequestWindowSize{
        window_id: u64,
    },
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
    Size {
        width: ScreenSize,
        height: ScreenSize,
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
#[derive(Debug, Clone, Copy, Serialize, Deserialize,PartialEq, Eq)]
pub enum MouseButton {
    Left,
    Right,
    Middle,
    Other(u8),
}
#[derive(Debug, Clone, Copy, Serialize, Deserialize,PartialEq, Eq)]
pub struct Modifiers {
    pub shift: bool,
    pub ctrl: bool,
    pub alt: bool,
    pub logo: bool,
}
#[derive(Debug, Clone, Copy, Serialize, Deserialize,PartialEq, Eq)]
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


pub fn deserialize(packet: &Vec<u8>) -> bincode::Result<Packet> {
    bincode::deserialize(&packet)
}

pub fn serialize(packet: &Packet) -> bincode::Result<Vec<u8>> {
    bincode::serialize(&packet)
}