use super::state::*;
use piston_window::*;

pub fn render(state: &State, c: &Context, g: &mut G2d) {
    for i in (0..state.ship.shape.len() - 1).step_by(2) {
        let (x1, y1) = state.ship.shape[i];
        let (x2, y2) = state.ship.shape[i + 1];
        line([1.0; 4], 1.0, [x1, y1, x2, y2], c.transform, g);
    }
}
