use super::state::*;
use piston_window::*;

pub fn render(state: &State, c: &Context, g: &mut G2d) {
    clear([0.0, 0.0, 0.0, 1.0], g);
    state.ship.draw(c, g);
    for a in state.asteroids.iter() {
        a.draw(c, g);
    }
}
