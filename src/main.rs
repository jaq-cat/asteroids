use piston_window::*;

mod conf;
use conf::*;

fn main() {
    let mut window: PistonWindow = WindowSettings::new("Asteroids'", [DIM; 2])
        .exit_on_esc(true)
        .resizable(false)
        .decorated(true)
        .build()
        .unwrap();
    while let Some(e) = window.next() {
        if let Some(_) = e.update_args() {
            //
        } else if let Some(_) = e.render_args() {
            //
        } else if let Some(b) = e.button_args() {
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
        }
    }
    println!("Hello, world!");
}
