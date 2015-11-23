use std::time::Duration;
use std::thread;
use std::path::Path;
use opengl_graphics::GlGraphics;
use opengl_graphics::glyph_cache::GlyphCache;
use graphics::{Context, Transformed, Text, color, clear};
use piston::input::*;
use game::objects::{Pong, Ball, Position, GameObject, Speed};
use game::collision::{CollisionDetection, AABB};

pub struct Game {
    pub loading: bool,
    size: (u32, u32),
    winning_score: u8,
    win_screen: bool,
    pong_1: Pong,
    pong_2: Pong,
    ball: Ball,
    font: GlyphCache<'static>
}

impl Game {
    pub fn new(width: u32, height: u32) -> Game {
        const PONG_OFFSET_X: u32 = 20;
        const PONG_WIDTH: u32 = 30;
        const PONG_HEIGHT: u32 = 150;
        const BALL_RADIUS: u32 = 7;

        Game {
            size: (width, height),
            winning_score: 10,
            win_screen: true,
            loading: true,
            pong_1: Pong::new(
                Position {
                    x: (PONG_OFFSET_X) as f64,
                    y: ((height/2) - (PONG_HEIGHT/2)) as f64
                },
                Speed {
                    x: 0.0,
                    y: 200.0
                },
                PONG_WIDTH,
                PONG_HEIGHT
            ),
            pong_2: Pong::new(
                Position {
                    x: ((width - PONG_OFFSET_X) - PONG_WIDTH) as f64,
                    y: ((height/2) - (PONG_HEIGHT/2)) as f64
                },
                Speed {
                    x: 0.0,
                    y: 50.0
                },
                PONG_WIDTH,
                PONG_HEIGHT
            ),
            ball: Ball::new(
                Position {
                    x: (width/2) as f64,
                    y: (height/2 - BALL_RADIUS) as f64
                },
                BALL_RADIUS
            ),
            font: GlyphCache::new(&Path::new("resources/fonts/atarifull.ttf")).unwrap()
        }
    }

    fn scored(&mut self, player: u8) {
        let mut score: u32 = 0;

        if player == 1 {
            self.pong_1.scored();
            score = self.pong_1.score;
        }
        else {
            self.pong_2.scored();
            score = self.pong_2.score;
        }

        if score >= 5 {
            self.win_screen = true;
        }
        else {
            self.reset_objects();
        }
    }

    fn reset_objects(&mut self) {
        self.ball.reset();
    }

    fn reset_game(&mut self) {
        self.win_screen = false;
        self.loading = false;
        self.pong_1.reset();
        self.pong_2.reset();
    }

    // Used to render objects/text/assets onto the screen.
    pub fn render(&mut self, context: Context, gl: &mut GlGraphics) {
        // Reset the screen
        clear(color::BLACK, gl);

        if self.loading {
            let mut start_game_text = Text::new(30);
            start_game_text.color = color::WHITE;
            start_game_text.draw(
                &format!("Press SPACE To Play PONG!"),
                &mut self.font,
                &context.draw_state,
                context.trans((self.size.0 as f64 / 2.0) - 330.0, (self.size.1 as f64 / 2.0) - 10.0).transform,
                gl
            );

            // End the rendering phase here if we are still "loading," or waiting for the player to
            // start playing the game.
            return;
        }

        if self.win_screen {
            let winner = if self.pong_1.score >= 5 { 1 } else { 2 };

            let mut won_text = Text::new(30);
            won_text.color = color::WHITE;
            won_text.draw(
                &format!("Player {} Has Won!", winner),
                &mut self.font,
                &context.draw_state,
                context.trans((self.size.0 as f64 / 2.0) - 230.0, (self.size.1 as f64 / 2.0) - 10.0).transform,
                gl
            );

            let mut play_again_text = Text::new(17);
            play_again_text.color = color::WHITE;
            play_again_text.draw(
                &format!("Press SPACE To Play Again"),
                &mut self.font,
                &context.draw_state,
                context.trans((self.size.0 as f64 / 2.0) - 205.0, (self.size.1 as f64 / 2.0) + 40.0).transform,
                gl
            );
        }
        else {
            self.pong_1.render(context, gl);
            self.pong_2.render(context, gl);
            self.ball.render(context, gl);
        }

        // Text::new(font_size);
        let mut score_text = Text::new(18);
        score_text.color = color::WHITE;
        score_text.draw(
            &format!("Player 1: {}", self.pong_1.score),
            &mut self.font,
            &context.draw_state,
            context.trans(20.0, 30.0).transform,
            gl
        );

        score_text.draw(
            &format!("Player 2: {}", self.pong_2.score),
            &mut self.font,
            &context.draw_state,
            context.trans(self.size.0 as f64 - 200.0, 30.0).transform,
            gl
        );
    }

    // Used to update paddle position, ball position, handle AI, etc.
    pub fn update(&mut self, dt: f64) {
        if self.win_screen {
            return;
        }

        let ball_diameter: f64 = self.ball.radius as f64 * 2.0;

        if AABB::is_colliding(&self.pong_1.bounding_box, &self.ball.bounding_box) {
            self.ball.pong_collision(&self.pong_1);
        }
        else if AABB::is_colliding(&self.ball.bounding_box, &self.pong_2.bounding_box) {
            self.ball.pong_collision(&self.pong_2);
        }

        // Checking for Collision Against Walls
        if self.ball.position.y <= 0.0 || self.ball.position.y + ball_diameter >= self.size.1 as f64 {
            self.ball.bounce();
        }

        if self.ball.position.x <= 0.0 {
            self.scored(2);
        }
        else if self.ball.position.x + ball_diameter >= self.size.0 as f64 {
            self.scored(1);
        }
        // End Wall Collision Detection

        // Really Quick and Dirty AI
        //
        // TODO: Implement an AI Struct that is smatter and a bit more abstracted.
        let ai_center = (self.pong_2.bounding_box.top_left.y + self.pong_2.bounding_box.bottom_left.y) / 2.0;
        if self.ball.bounding_box.top_left.y > ai_center - 80.0 {
            self.pong_2.move_position(false, true);
        }
        else if self.ball.bounding_box.top_left.y < ai_center + 80.0 {
            self.pong_2.move_position(true, false);
        }
        else {
            self.pong_2.move_position(false, false);
        }
        // End Quick and Dirty AI

        self.pong_1.update(dt, self.size.0, self.size.1);
        self.pong_2.update(dt, self.size.0, self.size.1);
        self.ball.update(dt, self.size.0, self.size.1);
    }

    pub fn input_update(&mut self, button: Button) {
        match button {
            Button::Keyboard(keyboard::Key::W) => {
                self.pong_1.move_position(true, false);
            },
            Button::Keyboard(keyboard::Key::S) => {
                self.pong_1.move_position(false, true);
            },
            Button::Keyboard(keyboard::Key::Space) => {
                if self.win_screen || self.loading {
                    self.reset_game();
                }
            },
            _ => {
                self.pong_1.move_position(false, false);
            }
        }
    }

}
