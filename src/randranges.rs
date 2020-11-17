use rand::{
    distributions::{uniform::SampleUniform, Distribution, Uniform},
    rngs::ThreadRng,
    thread_rng,
};

use super::conf::*;

pub struct Ranges {
    pub rng: ThreadRng,
    pub zero_one: Uniform<u8>,

    pub width_half: Uniform<f64>,
    pub height_half: Uniform<f64>,

    pub ast_edges: Uniform<u8>,
    pub ast_r: Uniform<f64>,
    pub ast_speed: Uniform<f64>,

    pub p_speed: Uniform<f64>,
}

impl Ranges {
    pub fn new() -> Self {
        Self {
            rng: thread_rng(),
            zero_one: Uniform::from(0..=1),
            width_half: Uniform::from(0.0..=WIDTH as f64),
            height_half: Uniform::from(0.0..=WIDTH as f64),
            ast_edges: Uniform::from(AST_EDGES / 2..=AST_EDGES),
            ast_r: Uniform::from(AST_RAD * AST_SIZE_VAR..=AST_RAD),
            ast_speed: Uniform::from(ASTSPD * 0.5..=ASTSPD),
            p_speed: Uniform::from(-1.0..=1.0),
        }
    }

    pub fn get<T: SampleUniform>(&mut self, range: Uniform<T>) -> T {
        range.sample(&mut self.rng)
    }
}
