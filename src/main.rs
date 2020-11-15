use piston_window::*;

mod conf;
mod ship;
mod state;
mod stuff;
mod update;
use conf::*;
use ship::*;
use state::*;
use update::*;

fn main() {
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
        }
    }
    println!("Hello, world!");
}
