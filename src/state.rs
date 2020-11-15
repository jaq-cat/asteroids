use super::ship::*;
use super::stuff::*;

pub struct State {
    ship: Ship,
    bullets: Vec<Point>,
    asteroids: Vec<i32>,
}
