use super::conf::*;
use super::stuff::*;

pub struct Ship {
    pub x: f64,
    pub y: f64,
    pub xspd: f64,
    pub yspd: f64,
    pub angle: f64,
    pub cooldown: u8,
    pub shape: Shape,
}

impl Ship {
    pub fn new(x: f64, y: f64, shape: Shape) -> Self {
        Self {
            x,
            y,
            xspd: 0.0,
            yspd: 0.0,
            angle: 0.0,
            cooldown: COOLDOWN,
            shape,
        }
    }

    pub fn rotate(&mut self, angle: f64) {
        self.angle += angle;
    }

    pub fn accelerate(&mut self, acc: f64) {
        self.xspd += rotation_x(acc, self.angle as f64);
        self.yspd += rotation_y(acc, self.angle as f64);
        if self.xspd >= MAXSPEED {
            self.xspd = MAXSPEED;
        } else if self.xspd <= -MAXSPEED {
            self.xspd = -MAXSPEED;
        }
        if self.yspd >= MAXSPEED {
            self.yspd = MAXSPEED;
        } else if self.yspd <= -MAXSPEED {
            self.yspd = -MAXSPEED;
        }
    }
}
