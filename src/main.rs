extern crate piston_window;
extern crate gfx_device_gl;
extern crate find_folder;
extern crate gfx_graphics;
extern crate gfx;
extern crate rand;

mod game;
mod object;
mod dungeon;
mod sprite;
mod dijkstra_map;

//use piston_window::graphics::*;
use piston_window::*;
use game::Game;

fn main() {
    //use find_folder::Search;
    let window: PistonWindow = WindowSettings::new(
        "Spellbound",
        [600, 600]
    )
    .exit_on_esc(true)
    .build()
    .unwrap();
    let mut game = Game::new(&window);
    game.on_load();

    for e in window {
        match e.event {
            Some(Event::Update(_)) => {
                game.on_update();
            }
            Some(Event::Render(ren)) => {
                game.on_draw(ren, e);//, &mut glyphs);
            }
            Some(Event::Input(inp)) => {
                game.on_input(inp);
            }
            _ => {

            }
        }
    }
}
