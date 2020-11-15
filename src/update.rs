use super::state::*;

pub fn update(state: &mut State) {
    state.ship.rotate(1.0);
}
