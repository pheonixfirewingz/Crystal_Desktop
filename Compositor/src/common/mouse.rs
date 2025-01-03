use libprotocol::ScreenSize;

#[derive(Debug)]
pub struct Mouse {
    x: ScreenSize,
    y: ScreenSize,
    rel_x: ScreenSize,
    rel_y: ScreenSize,
    wheel_delta_x: f32,
    wheel_delta_y: f32,
    button_left: bool,
    button_middle: bool,
    button_right: bool,
}

impl Mouse {
    pub fn new() -> Mouse {
        Mouse {
            x: 0,
            y: 0,
            rel_x: 0,
            rel_y: 0,
            wheel_delta_x: 0.0,
            wheel_delta_y: 0.0,
            button_left: false,
            button_middle: false,
            button_right: false,
        }
    }
    pub fn is_left_button_pressed(&self) -> bool {
        self.button_left
    }
    pub fn is_right_button_pressed(&self) -> bool {
        self.button_right
    }
    pub fn is_middle_button_pressed(&self) -> bool {
        self.button_middle
    }
    pub fn get_wheel_delta_x(&self) -> f32 {
        self.wheel_delta_x
    }
    pub fn get_wheel_delta_y(&self) -> f32 {
        self.wheel_delta_y
    }
    pub fn get_x(&self) -> ScreenSize {
        self.x
    }
    pub fn get_y(&self) -> ScreenSize {
        self.y
    }
    pub fn get_rel_x(&self) -> ScreenSize {
        self.rel_x
    }
    pub fn get_rel_y(&self) -> ScreenSize {
        self.rel_y
    }
    
    pub fn add_wheel_delta(&mut self,x:f32, y:f32) {
        self.wheel_delta_x += x;
        self.wheel_delta_y += y;
    }
    
    pub fn add_position(&mut self, x: ScreenSize,y: ScreenSize) {
        self.rel_x = x;
        self.x += x;
        self.rel_y = y;
        self.y += y;
    }
    
    pub fn set_left_button(&mut self, button: bool) {
        self.button_left = button;
    }
    pub fn set_right_button(&mut self, button: bool) {
        self.button_right = button;
    }
    pub fn set_middle_button(&mut self, button: bool) {
        self.button_middle = button;
    }
}
