use piston_window::*;
use std::collections::HashMap;

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
    let mut input: HashMap<char, bool> = HashMap::new();
    let mut state = State {
        ship: Ship::new(
            (DIM / 2) as f64,
            (DIM / 2) as f64,
            vec![
                (SHIP_HEIGHT - OFFSET, 0.0),
                (-OFFSET, -SHIP_WIDTH),
                (-OFFSET, SHIP_WIDTH),
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
    window.set_ups(60);
    let mut glyphs = window
        .load_font("fonts/november.ttf")
        .expect("error loading font!");
    while let Some(e) = window.next() {
        if let Some(_) = e.update_args() {
            // update
            update(&mut state, &input);
        } else if let Some(b) = e.button_args() {
            // process input
            if let Button::Keyboard(k) = b.button {
                match b.state {
                    ButtonState::Press => match k {
                        Key::W => {
                            input.insert('w', true);
                        }
                        Key::A => {
                            input.insert('a', true);
                        }
                        Key::D => {
                            input.insert('d', true);
                        }
                        Key::Space => {
                            input.insert('\n', true);
                        }
                        _ => {}
                    },
                    ButtonState::Release => match k {
                        Key::W => {
                            input.insert('w', false);
                        }
                        Key::A => {
                            input.insert('a', false);
                        }
                        Key::D => {
                            input.insert('d', false);
                        }
                        Key::Space => {
                            input.insert('\n', false);
                        }
                        _ => {}
                    },
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
