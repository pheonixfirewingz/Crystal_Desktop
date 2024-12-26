#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Colour {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Colour {
    pub fn new(r: u8, g: u8, b: u8, a: u8) -> Colour {
        Colour { r, g, b, a }
    }
}