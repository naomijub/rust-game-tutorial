use ggez;
use ggez::conf::WindowMode;
use ggez::event;
use ggez::event::KeyCode;
use ggez::graphics;
use ggez::graphics::screen_coordinates;
use ggez::graphics::Rect;
use ggez::nalgebra as na;
use ggez::Context;

use crate::objects::bullet::remove_bullet_if_outside_game_scren;
use crate::objects::bullet::Bullet;
use crate::objects::Tank;

pub enum Player {
    P1,
    P2,
    P3,
    P4,
}

pub struct MainState {
    pub(crate) tank: Tank,
    pub bullet: Option<Bullet>,
    pub coordinate: Rect,
}

impl MainState {
    pub fn new(ctx: &mut Context, player: Player) -> ggez::GameResult<MainState> {
        let sc = screen_coordinates(ctx);
        let tank_base = graphics::Image::new(ctx, "/TankBase.png")?;
        let tank_dimensions = tank_base.dimensions();

        let tank = Tank {
            position: na::Point2::from([sc.w / 2., sc.h / 2.]),
            tank_direction: na::Vector2::from([-1., 0.]),
            tank_rotation: 0.,
            texture: Some(tank_base),
            turret_texture: Some(graphics::Image::new(ctx, "/TankTops.png")?),
            turret_direction: na::Vector2::from([-1., 0.]),
            turret_rotation: 0.,
            turret_rotation_origin: na::Vector2::from([
                tank_dimensions.w * 0.7,
                tank_dimensions.h / 2.,
            ]),
            player,
        };

        let s = MainState {
            tank,
            coordinate: sc,
            bullet: None,
        };
        Ok(s)
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult {
        self.tank.update(ctx)?;
        if let Some(bullet) = &mut self.bullet {
            let coord = self.coordinate.clone();
            bullet.update(ctx)?;
            remove_bullet_if_outside_game_scren(bullet.position, self, coord);
        } else {
            let keys = ggez::input::keyboard::pressed_keys(ctx);
            let left_mouse_button_pressed =
                ggez::input::mouse::button_pressed(ctx, event::MouseButton::Left);
            self.fire_bullet(keys, left_mouse_button_pressed);
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult {
        graphics::clear(ctx, [0.83, 0.69, 0.51, 1.0].into());
        self.tank.draw(ctx)?;
        if let Some(bullet) = &mut self.bullet {
            bullet.draw(ctx)?;
        }
        graphics::present(ctx)?;
        Ok(())
    }
}

impl MainState {
    fn fire_bullet(
        &mut self,
        keys: &std::collections::HashSet<KeyCode>,
        left_mouse_button_pressed: bool,
    ) {
        if keys.contains(&KeyCode::Space) || left_mouse_button_pressed {
            self.bullet = Some(Bullet {
                position: na::Point2::from(self.tank.position),
                direction: self.tank.turret_direction,
                rotation: self.tank.turret_rotation,
                origin: self.tank.turret_rotation_origin,
            });
        }
    }
}

pub fn window_state_mode() -> WindowMode {
    WindowMode {
        width: 1200.,
        height: 900.,
        maximized: false,
        fullscreen_type: ggez::conf::FullscreenType::Windowed,
        borderless: false,
        min_width: 800.,
        min_height: 600.,
        max_width: 1600.,
        max_height: 1200.,
        resizable: true,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn random_keys_dont_fire_bullet() {
        let mut main_state = main();
        let keys = &vec![KeyCode::W, KeyCode::A, KeyCode::K, KeyCode::V]
            .into_iter()
            .collect();

        assert!(main_state.bullet.is_none());

        main_state.fire_bullet(keys, false);

        assert!(main_state.bullet.is_none());
    }

    #[test]
    fn left_mouse_button_fires_bullet() {
        let mut main_state = main();
        let keys = &vec![KeyCode::Z].into_iter().collect();

        assert!(main_state.bullet.is_none());

        main_state.fire_bullet(keys, true);

        assert!(main_state.bullet.is_some());
    }

    #[test]
    fn spacebar_fires_bullet() {
        let mut main_state = main();
        let keys = &vec![KeyCode::Space].into_iter().collect();

        assert!(main_state.bullet.is_none());

        main_state.fire_bullet(keys, false);

        assert!(main_state.bullet.is_some());
    }

    fn main() -> MainState {
        MainState {
            tank: Tank {
                position: na::Point2::from([300., 300.]),
                tank_direction: na::Vector2::from([1., 1.]),
                tank_rotation: 30.,
                texture: None,
                turret_texture: None,
                turret_direction: na::Vector2::from([1., 1.]),
                turret_rotation: 30.,
                turret_rotation_origin: na::Vector2::from([1., 1.]),
                player: crate::state::Player::P1,
            },
            bullet: None,
            coordinate: coord(),
        }
    }

    fn coord() -> Rect {
        Rect {
            x: 0.,
            y: 0.,
            w: 900.,
            h: 1200.,
        }
    }
}
