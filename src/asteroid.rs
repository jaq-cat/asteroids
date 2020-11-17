use super::conf::*;
use super::randranges::Ranges;
use super::stuff::*;
use piston_window::*;
use rand::distributions::Uniform;

#[derive(Clone)]
pub struct Asteroid {
    pub x: f64,
    pub y: f64,
    pub radius: f64,
    pub xspd: f64,
    pub yspd: f64,
    pub shape: Shape,
}

impl Asteroid {
    pub fn new(r: &mut Ranges) -> Self {
        let radius = r.get(r.ast_r);
        let (x, y) = Self::get_x_y(r, radius);
        Self {
            x,
            y,
            radius,
            yspd: (r.get(r.ast_speed) * if y <= 0.1 { 1.0 } else { -1.0 }),
            xspd: (r.get(r.ast_speed) * if x <= 0.1 { 1.0 } else { -1.0 }),
            shape: Self::gen_points(0.0, 0.0, radius, r),
        }
    }

    pub fn get_x_y(r: &mut Ranges, radius: f64) -> (f64, f64) {
        let x;
        let y;
        if r.get(r.zero_one) == 0 {
            // spawn l/r
            x = if r.get(r.zero_one) == 0 {
                -radius
            } else {
                WIDTH as f64 + radius
            };
            y = r.get(r.height_half);
        } else {
            // spawn t/b
            x = r.get(r.width_half);
            y = if r.get(r.zero_one) == 0 {
                -radius
            } else {
                HEIGHT as f64 + radius
            };
        }
        (x, y)
    }

    fn gen_points(x: f64, y: f64, rad: f64, r: &mut Ranges) -> Shape {
        let mut v = Vec::new();
        let ast_round = Uniform::from(rad * AST_ROUND..rad);
        let edges = r.get(r.ast_edges);
        for i in 0..edges {
            let d = r.get(ast_round);
            let angle = (360.0 / edges as f64) * i as f64;
            let px = rotation_x(d, angle);
            let py = rotation_y(d, angle);
            v.push((x + px, y + py));
        }
        v
    }

    pub fn tick(&mut self) -> bool {
        self.x += self.xspd;
        self.y += self.yspd;
        !(self.x < -self.radius
            || self.x > WIDTH as f64 + self.radius
            || self.y < -self.radius
            || self.y > HEIGHT as f64 + self.radius)
    }

    pub fn draw(&self, c: &Context, g: &mut G2d) {
        for i in 0..self.shape.len() - 1 {
            let (x1, y1) = self.shape[i];
            let (x2, y2) = self.shape[i + 1];
            line(
                [1.0; 4],
                LINE_WIDTH,
                [x1, y1, x2, y2],
                c.transform.trans(self.x, self.y),
                g,
            );
        }
        let (x2, y2) = self.shape[self.shape.len() - 1];
        let (x1, y1) = self.shape[0];
        line(
            [1.0; 4],
            LINE_WIDTH,
            [x1, y1, x2, y2],
            c.transform.trans(self.x, self.y),
            g,
        );
    }
}
