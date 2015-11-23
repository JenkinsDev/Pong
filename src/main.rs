extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;

mod game;

use piston::window::WindowSettings;
use glutin_window::GlutinWindow as Window;
use piston::event_loop::*; // This import will add the events() method to our Window object.
use piston::input::*;
use opengl_graphics::{ GlGraphics, OpenGL };
use game::Game;

fn main() {
    const OPENGL: OpenGL = OpenGL::V4_5;
    const WIDTH: u32 = 1024;
    const HEIGHT: u32 = 700;

    let window: Window = WindowSettings::new(
            "PONG!",
            [WIDTH, HEIGHT]
        )
        .opengl(OPENGL).exit_on_esc(true)
        .build().unwrap();
    let mut gl: GlGraphics = GlGraphics::new(OPENGL);
    let mut game: Game = Game::new(WIDTH, HEIGHT);
    game.loading = true;

    for event in window.events().ups(60).max_fps(60) {
        match event {
            Event::Input(Input::Press(button)) => {
                game.input_update(button);
            },
            Event::Input(Input::Release(_)) => {
                game.input_update(Button::Keyboard(keyboard::Key::Unknown));
            },
            Event::Render(args) => {
                gl.draw(args.viewport(), |context, graphics| {
                    game.render(context, graphics)
                });
            },
            Event::Update(args) => {
                game.update(args.dt);
            },
            _ => { }
        }
    }
}
