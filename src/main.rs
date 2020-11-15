use piston_window::*;

mod conf;
mod render;
mod ship;
mod state;
mod stuff;
mod update;
use conf::*;
use render::*;
use ship::*;
use state::*;
use stuff::*;
use update::*;

fn main() {
    let mut state = State {
        ship: Ship::new(
            (DIM / 2) as f64,
            (DIM / 2) as f64,
            vec![
                (0.0, SHIP_HEIGHT - OFFSET),
                (-SHIP_WIDTH, 0.0 - OFFSET),
                (SHIP_WIDTH, 0.0 - OFFSET),
            ],
        ),
        asteroids: Vec::new(),
        bullets: Vec::new(),
    };
    let mut window: PistonWindow = WindowSettings::new("Asteroids", [DIM; 2])
        .exit_on_esc(true)
        .resizable(false)
        .decorated(true)
        .build()
        .unwrap();
    let mut glyphs = window
        .load_font("fonts/november.ttf")
        .expect("error loading font!");
    while let Some(e) = window.next() {
        if let Some(_) = e.update_args() {
            // update
        } else if let Some(b) = e.button_args() {
            // process input
            if let Button::Keyboard(k) = b.button {
                match b.state {
                    ButtonState::Press => match k {
                        Key::W => {}
                        Key::A => {}
                        Key::D => {}
                        Key::Space => {}
                        _ => {}
                    },
                    ButtonState::Release => {}
                }
            }
        } else if let Some(_) = e.render_args() {
            // render
            window.draw_2d(&e, |c, g, dev| {
                render(&state, &c, g);
                glyphs.factory.encoder.flush(dev);
            });
        }
    }
    println!("Hello, world!");
}
