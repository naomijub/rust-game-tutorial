use ggez;
use ggez::event;
use ggez::graphics::{self, Color, DrawMode, FillOptions, Rect};
use ggez::nalgebra as na;

use crate::state::MainState;

const WIDTH: f32 = 10.67;
const HEIGHT: f32 = 6.;
const SPEED: f32 = 3.;

#[derive(Clone)]
pub struct Bullet {
    pub position: na::Point2<f32>,
    pub direction: na::Vector2<f32>,
    pub rotation: f32,
    pub origin: na::Vector2<f32>,
}

impl event::EventHandler for Bullet {
    fn update(&mut self, _: &mut ggez::Context) -> ggez::GameResult {
        self.movement();
        Ok(())
    }

    fn draw(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult {
        let bounds = Rect::new(0., 0., WIDTH, HEIGHT);
        let param = graphics::DrawParam::new()
            .dest(self.position)
            .offset(na::Point2::from([0.5, 0.5]))
            .rotation(self.rotation);

        let bullet = graphics::Mesh::new_rectangle(
            ctx,
            DrawMode::Fill(FillOptions::DEFAULT),
            bounds,
            Color::from_rgba(30, 30, 30, 250),
        )?;
        graphics::draw(ctx, &bullet, param)?;
        Ok(())
    }
}

impl Bullet {
    pub(crate) fn movement(&mut self) {
        self.position += SPEED * self.direction;
    }
}

pub fn remove_bullet_if_outside_game_scren(
    bullet_position: na::Point2<f32>,
    main_state: &mut MainState,
    coordinates: Rect,
) {
    if !coordinates.contains(bullet_position) {
        main_state.bullet = None;
    }
}

#[cfg(test)]
mod tests {
    use ggez::graphics::Rect;
    use ggez::nalgebra as na;

    use crate::objects::Tank;
    use crate::state::MainState;

    use super::{remove_bullet_if_outside_game_scren, Bullet};

    #[test]
    fn bullet_is_outside_screen_x() {
        let mut main_state = main();
        let coord = coord();
        let bullet_position = na::Point2::from([901., 1000.]);

        remove_bullet_if_outside_game_scren(bullet_position, &mut main_state, coord);

        assert!(main_state.bullet.is_none())
    }

    #[test]
    fn bullet_is_outside_screen_y() {
        let mut main_state = main();
        let coord = coord();
        let bullet_position = na::Point2::from([800., 1201.]);

        remove_bullet_if_outside_game_scren(bullet_position, &mut main_state, coord);

        assert!(main_state.bullet.is_none())
    }

    #[test]
    fn bullet_is_inside_screen() {
        let mut main_state = main();
        let coord = coord();
        let bullet_position = na::Point2::from([800., 1000.]);

        remove_bullet_if_outside_game_scren(bullet_position, &mut main_state, coord);

        assert!(main_state.bullet.is_some())
    }

    #[test]
    fn update_movement() {
        let mut bullet = bullet();

        assert_eq!(bullet.position, na::Point2::from([300., 400.]),);

        bullet.movement();
        assert_eq!(bullet.position, na::Point2::from([303., 403.]),);
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
            bullet: Some(bullet()),
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

    fn bullet() -> Bullet {
        Bullet {
            position: na::Point2::from([300., 400.]),
            direction: na::Vector2::from([1., 1.]),
            rotation: 30.,
            origin: na::Vector2::from([1., 1.]),
        }
    }
}
