use super::asteroid::*;
use super::ship::*;
use super::stuff::*;

fn distance(p1: Point, p2: Point) -> f64 {
    ((p1.0 - p2.0).powf(2.0) + (p1.1 - p2.1).powf(2.0)).powf(0.5)
}

pub fn ship_in_asteroid(ship: &Ship, a: &Asteroid) -> bool {
    distance((ship.x, ship.y), (a.x, a.y)) <= a.radius
}
