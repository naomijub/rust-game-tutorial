use std::collections::HashSet;

use ggez;
use ggez::event;
use ggez::event::KeyCode;
use ggez::graphics::{self, Rect};
use ggez::nalgebra::{self as na, Point2};
use ggez::timer::delta;

use crate::state::Player;

pub struct Tank {
    pub(crate) position: na::Point2<f32>,
    pub(crate) tank_direction: na::Vector2<f32>,
    pub(crate) tank_rotation: f32,
    pub(crate) texture: Option<graphics::Image>,
    pub(crate) turret_texture: Option<graphics::Image>,
    pub(crate) turret_direction: na::Vector2<f32>,
    pub(crate) turret_rotation: f32,
    pub(crate) player: Player,
}

impl event::EventHandler for Tank {
    fn update(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult {
        let keys = ggez::input::keyboard::pressed_keys(ctx);
        let delta = delta(ctx).as_secs_f32();
        self.movement(keys);
        self.rotation(keys, delta);
        Ok(())
    }

    fn draw(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult {
        println!("{:?}", self.get_player_turret());
        let base_param = graphics::DrawParam::new()
            .dest(self.position)
            .offset(na::Point2::from([0.5, 0.5]))
            .rotation(self.tank_rotation);
        let turret_param = graphics::DrawParam::new()
            .dest(self.position)
            .offset(na::Point2::from([0.5, 0.5]))
            .src(self.get_player_turret())
            .rotation(self.tank_rotation);

        graphics::draw(ctx, self.texture.as_ref().unwrap(), base_param)?;
        graphics::draw(ctx, self.turret_texture.as_ref().unwrap(), turret_param)?;
        Ok(())
    }
}

impl Tank {
    fn movement(&mut self, keys: &HashSet<KeyCode>) {
        if keys.contains(&KeyCode::W) || keys.contains(&KeyCode::Up) {
            self.position = Point2::from([
                self.position.x + self.tank_direction.x,
                self.position.y + self.tank_direction.y,
            ]);
        }

        if keys.contains(&KeyCode::S) || keys.contains(&KeyCode::Down) {
            self.position = Point2::from([
                self.position.x - self.tank_direction.x,
                self.position.y - self.tank_direction.y,
            ]);
        }
    }

    fn rotation(&mut self, keys: &HashSet<KeyCode>, delta: f32) {
        if keys.contains(&KeyCode::D) || keys.contains(&KeyCode::Right) {
            self.tank_rotation += delta;
            self.update_direction();
        }

        if keys.contains(&KeyCode::A) || keys.contains(&KeyCode::Left) {
            self.tank_rotation -= delta;
            self.update_direction();
        }
    }

    fn update_direction(&mut self) {
        let (sin, cos) = self.tank_rotation.sin_cos();
        self.tank_direction = na::Vector2::from([-cos, -sin]);
    }

    fn get_player_turret(&self) -> Rect {
        let draw_height = 0.5;
        let draw_width = 0.5;

        match self.player {
            Player::P1 => Rect {
                x: 0.,
                y: 0.,
                h: draw_height,
                w: draw_width,
            },
            Player::P2 => Rect {
                x: draw_width,
                y: 0.,
                h: draw_height,
                w: draw_width,
            },
            Player::P3 => Rect {
                x: 0.,
                y: draw_height,
                h: draw_height,
                w: draw_width,
            },
            Player::P4 => Rect {
                x: draw_width,
                y: draw_height,
                h: draw_height,
                w: draw_width,
            },
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn initial_info() {
        let tank = tank();
        assert_eq!(tank.position, na::Point2::from([400., 300.]));
        assert_eq!(tank.tank_rotation, 0.);
        assert_eq!(tank.tank_direction, na::Vector2::from([-1., 0.]));
    }

    #[test]
    fn move_forward() {
        let mut tank = tank();
        let keys = vec![KeyCode::W].into_iter().collect();
        tank.movement(&keys);
        assert_eq!(tank.position, na::Point2::from([399., 300.]));
        let keys = vec![KeyCode::Up].into_iter().collect();
        tank.movement(&keys);
        assert_eq!(tank.position, na::Point2::from([398., 300.]));
    }

    #[test]
    fn move_backwards() {
        let mut tank = tank();
        let keys = vec![KeyCode::S].into_iter().collect();
        tank.movement(&keys);
        assert_eq!(tank.position, na::Point2::from([401., 300.]));
        let keys = vec![KeyCode::Down].into_iter().collect();
        tank.movement(&keys);
        assert_eq!(tank.position, na::Point2::from([402., 300.]));
    }

    #[test]
    fn turn_left() {
        let mut tank = tank();
        let keys = vec![KeyCode::A].into_iter().collect();
        tank.rotation(&keys, 0.3);
        let keys = vec![KeyCode::W].into_iter().collect();
        tank.movement(&keys);
        assert_eq!(tank.position, na::Point2::from([399.04468, 300.29553]));

        let keys = vec![KeyCode::Left].into_iter().collect();
        tank.rotation(&keys, 0.5);
        let keys = vec![KeyCode::W].into_iter().collect();
        tank.movement(&keys);
        assert_eq!(tank.position, na::Point2::from([398.34796, 301.01288]));
    }

    #[test]
    fn turn_right() {
        let mut tank = tank();
        let keys = vec![KeyCode::D].into_iter().collect();
        tank.rotation(&keys, 0.3);
        let keys = vec![KeyCode::S].into_iter().collect();
        tank.movement(&keys);
        assert_eq!(tank.position, na::Point2::from([400.95532, 300.29553]));

        let keys = vec![KeyCode::Right].into_iter().collect();
        tank.rotation(&keys, 0.5);
        let keys = vec![KeyCode::S].into_iter().collect();
        tank.movement(&keys);
        assert_eq!(tank.position, na::Point2::from([401.65204, 301.01288]));
    }

    fn tank() -> Tank {
        Tank {
            position: na::Point2::from([400., 300.]),
            tank_direction: na::Vector2::from([-1., 0.]),
            tank_rotation: 0.,
            texture: None,
            turret_texture: None,
            turret_direction: na::Vector2::from([-1., 0.]),
            turret_rotation: 0.,
            player: Player::P1,
        }
    }
}
