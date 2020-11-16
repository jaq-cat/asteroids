use super::asteroid::Asteroid;
use super::collision::*;
use super::randranges::Ranges;
use std::collections::HashMap;

use super::state::*;

pub fn update(state: &mut State, input: &HashMap<char, bool>, r: &mut Ranges) {
    if let Some(&v) = input.get(&'a') {
        if v {
            state.ship.rotate(-5.0);
        }
    }
    if let Some(&v) = input.get(&'d') {
        if v {
            state.ship.rotate(5.0);
        }
    }
    if let Some(&v) = input.get(&'w') {
        if v {
            state.ship.accelerate(0.05);
        }
    }
    state.ship.tick();
    for a in state.asteroids.iter_mut() {
        if !a.tick() {
            *a = Asteroid::new(r);
        }
    }
    for b in state.bullets.iter_mut() {
        b.x += b.xspd;
        b.y += b.yspd;
    }
    for a in state.asteroids.iter() {
        if ship_in_asteroid(&state.ship, &a) {
            println!("AAAA");
        }
    }
    //state.ship.rotate(1.0);
}
