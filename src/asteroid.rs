use super::conf::*;
use super::randranges::Ranges;
use super::stuff::*;
use rand::distributions::Uniform;

pub struct Asteroid {
    pub x: f64,
    pub y: f64,
    pub xspd: f64,
    pub yspd: f64,
    pub shape: Shape,
}

impl Asteroid {
    pub fn new(r: &mut Ranges) -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            xspd: r.get(r.ast_speed),
            yspd: r.get(r.ast_speed),
            shape: Self::gen_points(0.0, 0.0, AST_RAD, r),
        }
    }

    pub fn get_x_y(r: &mut Ranges) -> (f64, f64) {
        let x;
        let y;
        if r.get(r.zero_one) == 0 {
            // spawn l/r
            x = if r.get(r.zero_one) == 0 {
                0.0
            } else {
                DIM as f64
            };
            y = r.get(r.dim_half);
        } else {
            // spawn t/b
            x = r.get(r.dim_half);
            y = if r.get(r.zero_one) == 0 {
                0.0
            } else {
                DIM as f64
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

    pub fn tick(&mut self) {
        self.x += self.xspd;
        self.y += self.yspd;
    }
}
