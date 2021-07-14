use ggez;
use ggez::conf::WindowMode;
use ggez::event;
use ggez::graphics;
use ggez::graphics::screen_coordinates;
use ggez::nalgebra as na;
use ggez::Context;

use crate::objects::Tank;

pub enum Player {
    P1,
    P2,
    P3,
    P4,
}

pub struct MainState {
    tank: Tank,
}

impl MainState {
    pub fn new(ctx: &mut Context, player: Player) -> ggez::GameResult<MainState> {
        let sc = screen_coordinates(ctx);
        let tank = Tank {
            position: na::Point2::from([sc.w / 2., sc.h / 2.]),
            tank_direction: na::Vector2::from([-1., 0.]),
            tank_rotation: 0.,
            texture: Some(graphics::Image::new(ctx, "/TankBase.png")?),
            turret_texture: Some(graphics::Image::new(ctx, "/TankTops.png")?),
            turret_direction: na::Vector2::from([-1., 0.]),
            turret_rotation: 0.,
            player,
        };
        let s = MainState { tank };
        Ok(s)
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult {
        self.tank.update(ctx)?;
        Ok(())
    }

    fn draw(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult {
        graphics::clear(ctx, [0.83, 0.69, 0.51, 1.0].into());
        self.tank.draw(ctx)?;
        graphics::present(ctx)?;
        Ok(())
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
