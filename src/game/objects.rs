use opengl_graphics::GlGraphics;
use graphics::{Context, Rectangle, Ellipse, color};
use game::collision::*;

pub trait GameObject {
    fn render(&mut self, context: Context, gl: &mut GlGraphics);
    fn update(&mut self, dt: f64, screen_width: u32, screen_height: u32);
    fn reset(&mut self);
}

pub struct Speed {
    pub x: f64,
    pub y: f64
}

#[derive(Copy, Clone)]
pub struct Position {
    pub x: f64,
    pub y: f64
}

pub struct Movement {
    pub up: bool,
    pub down: bool
}

pub struct Pong {
    pub score: u32,
    pub position: Position,
    pub width: u32,
    pub height: u32,
    pub bounding_box: BoundingBox,
    speed: Speed,
    movement: Movement,
    initial_position: Position
}

impl Pong {
    pub fn new(position: Position, speed: Speed, width: u32, height: u32) -> Pong {
        Pong {
            score: 0,
            position: position,
            initial_position: position,
            width: width,
            height: height,
            speed: speed,
            movement: Movement { up: false, down: false },
            bounding_box: BoundingBox {
                top_left: Vec2 { x: position.x, y: position.y },
                top_right: Vec2 { x: position.x + width as f64, y: position.y },
                bottom_left: Vec2 { x: position.x, y: position.y + height as f64 },
                bottom_right: Vec2 { x: position.x + width as f64, y: position.y + height as f64 }
            }
        }
    }

    pub fn move_position(&mut self, up: bool, down: bool) {
        self.movement.up = up;
        self.movement.down = down;
    }

    pub fn scored(&mut self) {
        self.score += 1;
    }
}

impl GameObject for Pong {
    fn render(&mut self, context: Context, gl: &mut GlGraphics) {
        Rectangle::new(color::WHITE).draw(
            [self.position.x as f64, self.position.y as f64, self.width as f64, self.height as f64],
            &context.draw_state, context.transform, gl);
    }

    fn update(&mut self, dt: f64, screen_width: u32, screen_height: u32) {
        let movement_value: f64 = self.speed.y * dt;

        if self.position.y > 0.0 && self.movement.up {
            self.position.y -= movement_value;
            self.bounding_box.translate_by(0.0, -movement_value);
        }
        else if (self.position.y + self.height as f64) < (screen_height as f64)
          && self.movement.down {
            self.position.y += movement_value;
            self.bounding_box.translate_by(0.0, movement_value);
        }
    }

    fn reset(&mut self) {
        self.position = self.initial_position;
        self.bounding_box.translate_to(self.position.x, self.position.y);
        self.score = 0;
    }
}

pub struct Ball {
    pub position: Position,
    pub radius: u32,
    pub bounding_box: BoundingBox,
    speed: Speed,
    initial_position: Position
}

impl Ball {
    pub fn new(position: Position, radius: u32) -> Ball {
        let diameter: f64 = (radius * 2) as f64;

        Ball {
            position: position,
            initial_position: position,
            speed: Speed { x: 500.0, y: 100.0 },
            radius: radius,
            bounding_box: BoundingBox {
                top_left: Vec2 { x: position.x, y: position.y },
                top_right: Vec2 { x: position.x + diameter as f64, y: position.y },
                bottom_left: Vec2 { x: position.x, y: position.y + diameter as f64 },
                bottom_right: Vec2 { x: position.x + diameter as f64, y: position.y + diameter as f64 }
            }
        }
    }

    pub fn pong_collision(&mut self, pong: &Pong) {
        self.speed.x = (self.speed.x * -1.0) - 1.0;
        self.speed.y = ((self.bounding_box.top_left.y + self.bounding_box.bottom_left.y) / 2.0 -
          (pong.bounding_box.top_left.y + pong.bounding_box.bottom_left.y) / 2.0);
    }

    pub fn bounce(&mut self) {
        self.speed.y *= -1.0;
    }
}

impl GameObject for Ball {
    fn render(&mut self, context: Context, gl: &mut GlGraphics) {
        let diameter: u32 = self.radius * 2;
        Ellipse::new(color::WHITE).draw(
            [self.position.x as f64, self.position.y as f64, diameter as f64, diameter as f64],
            &context.draw_state, context.transform, gl);
    }

    fn update(&mut self, dt: f64, screen_width: u32, screen_height: u32) {
        let x_movement = self.speed.x * dt;
        let y_movement = self.speed.y * dt;

        self.position.x -= x_movement;
        self.position.y += y_movement;

        self.bounding_box.translate_by(-x_movement, y_movement);
    }

    fn reset(&mut self) {
        self.position = self.initial_position;
        self.bounding_box.translate_to(self.position.x, self.position.y);
        self.speed.y *= -1.0;
        self.speed.x *= -1.0;
    }
}
