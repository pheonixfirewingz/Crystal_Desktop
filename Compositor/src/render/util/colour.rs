use nalgebra::Vector4;

pub struct Colour {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Colour {
    #[inline]
    pub fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }
    pub fn rgb(r: u8,g:u8,b:u8) -> Self {
        Self::new(r, g, b, 255)
    }
    #[inline]
    pub fn grayscale(gray: u8) -> Self {
        Self::new(gray, gray, gray, 255)
    }
    #[inline]
    pub fn grayscale_alpha(gray: u8, a: u8) -> Self {
        Self::new(gray, gray, gray, a)
    }
    #[inline]
    pub fn black() -> Self {
        Self::new(0, 0, 0, 255)
    }
    #[inline]
    pub fn white() -> Self {
        Self::new(255, 255, 255, 255)
    }
    #[inline]
    pub fn red() -> Self {
        Self::new(255, 0, 0, 255)
    }
    #[inline]
    pub fn green() -> Self {
        Self::new(0, 255, 0, 255)
    }
    #[inline]
    pub fn blue() -> Self {
        Self::new(0, 0, 255, 255)
    }
    #[inline]
    pub fn transparent() -> Self {
        Self::new(0, 0, 0, 0)
    }
    #[inline]
    fn convert_range(value: u8) -> f32 {
        // Convert the value from 0-255 range to 0-1 range
        (value as f32) / 255.0
    }
    #[inline]
    pub fn to_gl(&self) -> Vector4<f32> {
        Vector4::new(Self::convert_range(self.r), Self::convert_range(self.g), Self::convert_range(self.b), Self::convert_range(self.a))
    }
}