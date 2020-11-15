use super::conf::*;
use super::stuff::*;

pub struct Ship {
    pub x: f64,
    pub y: f64,
    pub angle: u16,
    pub cooldown: u8,
    pub shape: Shape,
}

impl Ship {
    pub fn new(x: f64, y: f64, shape: Shape) -> Self {
        Self {
            x,
            y,
            angle: 0,
            cooldown: COOLDOWN,
            shape,
        }
    }
}
