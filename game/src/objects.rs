use std::collections::HashSet;

use ggez;
use ggez::event;
use ggez::event::KeyCode;
use ggez::graphics;
use ggez::graphics::Color;
use ggez::nalgebra::{self as na, Point2};

pub struct Tank {
    pub(crate) position: na::Point2<f32>,
    pub(crate) direction: na::Vector2<f32>,
    pub(crate) texture: graphics::Image,
}

impl event::EventHandler for Tank {
    fn update(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult {
        let keys = ggez::input::keyboard::pressed_keys(ctx);
        self.movement(keys);
        Ok(())
    }

    fn draw(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult {
        graphics::draw(
            ctx,
            &self.texture,
            (self.position, 0.0, Color::from_rgb(255, 255, 255)),
        )?;
        Ok(())
    }
}

impl Tank {
    fn movement(&mut self, keys: &HashSet<KeyCode>) {
        if keys.contains(&KeyCode::W) || keys.contains(&KeyCode::Up) {
            self.position = Point2::from([
                self.position.x + self.direction.x,
                self.position.y + self.direction.y,
            ]);
        }

        if keys.contains(&KeyCode::S) || keys.contains(&KeyCode::Down) {
            self.position = Point2::from([
                self.position.x - self.direction.x,
                self.position.y - self.direction.y,
            ]);
        }
    }
}
