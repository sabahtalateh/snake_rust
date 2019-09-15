use crate::coord::Coord;
use crate::game::Game;
use piston_window::*;
use std::process::exit;

mod coord;
mod draw;
mod error;
mod field;
mod game;
mod snake;

type MyResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> MyResult<()> {
    let mut game = match Game::create_form_level_file("level.lvl") {
        Ok(game) => game,
        Err(e) => {
            println!("{}", e);
            exit(1);
        }
    };

    let (width, height) = (
        Coord::new(game.field_width()),
        Coord::new(game.field_height()),
    );
    let size = [draw::to_coord_u32(width), draw::to_coord_u32(height)];

    let mut window: PistonWindow = WindowSettings::new("Snake", size)
        .exit_on_esc(true)
        .build()
        .unwrap();

    while let Some(event) = window.next() {
        if let Some(Button::Keyboard(key)) = event.press_args() {
            if game.is_over() {
                game = Game::create_form_level_file("level.lvl").unwrap();
            }
            game.key_pressed(key);
        }

        window.draw_2d(&event, |context, graphics, _| {
            clear(draw::EMPTY_COLOR, graphics);
            game.draw(&context, graphics);
        });

        event.update(|arg| {
            game.update(arg.dt);
        });
    }
    Ok(())
}
