use super::asteroid::Asteroid;
use super::ship::*;

pub struct State {
    pub ship: Ship,
    pub bullets: Vec<Bullet>,
    pub asteroids: Vec<Asteroid>,
}
