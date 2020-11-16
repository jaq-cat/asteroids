use super::asteroid::Asteroid;
use super::collision::*;
use super::randranges::Ranges;
use super::stuff::*;
use std::collections::HashMap;

use super::state::*;

pub fn update(state: &mut State, input: &HashMap<char, bool>, r: &mut Ranges) -> States {
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
    state.bullets = state
        .bullets
        .iter_mut()
        .filter_map(|b| if b.tick() { Some(b.clone()) } else { None })
        .collect();
    for a in state.asteroids.iter_mut() {
        if ship_in_asteroid(&state.ship, &a) {
            return States::Dead;
        }
        for b in state.bullets.iter() {
            if bullet_in_asteroid(&b, &a) {
                *a = Asteroid::new(r);
            }
        }
    }
    States::Nothing
}
