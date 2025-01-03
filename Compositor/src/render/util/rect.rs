use libprotocol::ScreenSize;

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct Position {
    pub x: ScreenSize,
    pub y: ScreenSize,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct Size {
    pub width: ScreenSize,
    pub height: ScreenSize,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct Rect {
    pub position: Position,
    pub size: Size,
}

impl Rect {
    #[inline]
    pub fn new(x: ScreenSize, y: ScreenSize, width: ScreenSize, height: ScreenSize) -> Rect {
        Rect {
            position: Position { x, y },
            size: Size { width, height }
        }
    }

    #[inline]
    pub fn set_pos(&mut self, x: ScreenSize, y: ScreenSize) {
        self.position.x = x;
        self.position.y = y - self.size.height;
    }

    #[inline]
    pub fn set_size(&mut self, width: ScreenSize, height: ScreenSize) {
        self.size.width = width;
        self.size.height = height;
    }

    #[inline]
    pub fn contains(&self, x: ScreenSize, y: ScreenSize) -> bool {
        let dx = x - self.position.x;
        let dy = y - self.position.y;
        dx <= self.size.width && dy <= self.size.height && dx >= 0 && dy >= 0
    }

    #[inline]
    pub fn get_edge_proximity(&self, x: ScreenSize, y: ScreenSize, threshold: ScreenSize) -> u8 {
        let dx = x - self.position.x;
        let dy = y - self.position.y;

        let near_right = dx >= self.size.width - threshold &&
            dx <= self.size.width + threshold;
        let near_bottom = dy >= self.size.height - threshold &&
            dy <= self.size.height + threshold;
        let near_top = dy >= 0 && dy <= threshold;

        (near_right as u8) | ((near_bottom as u8) << 1) | ((near_top as u8) << 2)
    }

    // Convenience methods that use get_edge_proximity
    #[inline]
    pub fn is_near_bottom(&self, x: ScreenSize, y: ScreenSize, threshold: ScreenSize) -> bool {
        (self.get_edge_proximity(x, y, threshold) & 0b010) != 0
    }

    #[inline]
    pub fn is_near_right(&self, x: ScreenSize, y: ScreenSize, threshold: ScreenSize) -> bool {
        (self.get_edge_proximity(x, y, threshold) & 0b001) != 0
    }

    #[inline]
    pub fn is_near_bottom_right(&self, x: ScreenSize, y: ScreenSize, threshold: ScreenSize) -> bool {
        (self.get_edge_proximity(x, y, threshold) & 0b011) == 0b011
    }

    #[inline]
    pub fn is_near_top(&self, x: ScreenSize, y: ScreenSize, threshold: ScreenSize) -> bool {
        (self.get_edge_proximity(x, y, threshold) & 0b100) != 0
    }
}