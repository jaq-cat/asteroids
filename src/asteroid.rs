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
            shape: Vec::new(),
        }
    }

    fn gen_points(x: f64, y: f64, rad: f64, r: &mut Ranges) -> Shape {
        let mut v = Vec::new();
        let ast_round = Uniform::from(rad * AST_ROUND..rad);
        let edges = r.get(r.ast_edges);
        for i in 0..edges {
            let d = ast_round.sample(&mut rr.rng);
            let angle = (360.0 / edges as f64) * i as f64;
            let px = cos_math(d, angle);
            let py = sin_math(d, angle);
            v.push([x + px, y + py]);
        }
        v
    }

    pub fn tick(&mut self) {
        self.x += self.xspd;
        self.y += self.yspd;
    }
}
