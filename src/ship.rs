use super::conf::*;
use super::stuff::*;
use piston_window::*;

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

    pub fn shoot(&self) -> Bullet {
        Bullet {
            x: self.x + rotation_x(SHIP_HEIGHT, self.angle),
            y: self.y + rotation_y(SHIP_HEIGHT, self.angle),
            xspd: rotation_x(5.0, self.angle),
            yspd: rotation_y(5.0, self.angle),
        }
    }

    pub fn tick(&mut self) {
        self.x += self.xspd;
        self.y += self.yspd;
    }

    pub fn draw(&self, c: &Context, g: &mut G2d) {
        for i in 0..self.shape.len() - 1 {
            let (x1, y1) = self.shape[i];
            let (x2, y2) = self.shape[i + 1];
            line(
                [1.0; 4],
                1.0,
                [x1, y1, x2, y2],
                c.transform.trans(self.x, self.y).rot_deg(self.angle),
                g,
            );
        }
        let (x2, y2) = self.shape[self.shape.len() - 1];
        let (x1, y1) = self.shape[0];
        line(
            [1.0; 4],
            1.0,
            [x1, y1, x2, y2],
            c.transform.trans(self.x, self.y).rot_deg(self.angle),
            g,
        );
    }
}

#[derive(Debug)]
pub struct Bullet {
    pub x: f64,
    pub y: f64,
    pub xspd: f64,
    pub yspd: f64,
}
