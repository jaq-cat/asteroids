use super::conf::*;
use std::f64::consts::PI;

pub enum States {
    Nothing,
    Dead,
}

pub type Point = (f64, f64);
pub type Shape = Vec<Point>;

pub const OFFSET: f64 = SHIP_HEIGHT / 2.0;

const RADTODEG: f64 = 2.0 * (PI / 360.0);

// cosine math
pub fn rotation_x(m: f64, rot: f64) -> f64 {
    m * (rot * RADTODEG).cos()
}

// sine math
pub fn rotation_y(m: f64, rot: f64) -> f64 {
    m * (rot * RADTODEG).sin()
}
