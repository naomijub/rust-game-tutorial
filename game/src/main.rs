use std::path;

use ggez;
use ggez::conf::WindowMode;
use ggez::conf::WindowSetup;
use ggez::event;
use ggez::graphics;
use ggez::graphics::screen_coordinates;
use ggez::graphics::Color;
use ggez::nalgebra as na;
use ggez::Context;

pub struct Tank {
    position: na::Point2<f32>,
    texture: graphics::Image,
}

impl event::EventHandler for Tank {
    fn update(&mut self, _ctx: &mut ggez::Context) -> ggez::GameResult {
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

struct MainState {
    tank: Tank,
}

impl MainState {
    fn new(ctx: &mut Context) -> ggez::GameResult<MainState> {
        let sc = screen_coordinates(ctx);
        let tank = Tank {
            position: na::Point2::from([sc.w / 2., sc.h / 2.]),
            texture: graphics::Image::new(ctx, "/TankBase.png")?,
        };
        let s = MainState { tank };
        Ok(s)
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, _ctx: &mut ggez::Context) -> ggez::GameResult {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult {
        graphics::clear(ctx, [0.83, 0.69, 0.51, 1.0].into());
        self.tank.draw(ctx)?;
        graphics::present(ctx)?;
        Ok(())
    }
}

pub fn main() -> ggez::GameResult {
    let resources_dir = path::PathBuf::from("../resources");
    let cb = ggez::ContextBuilder::new("tank_battle", "naomijub")
        .add_resource_path(resources_dir)
        .window_setup(WindowSetup {
            title: "Tank Battle Ground".to_owned(),
            samples: ggez::conf::NumSamples::Zero,
            vsync: true,
            icon: String::new(),
            srgb: true,
        })
        .window_mode(WindowMode {
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
        });

    let (ctx, event_loop) = &mut cb.build()?;
    let mut state = MainState::new(ctx)?;
    event::run(ctx, event_loop, &mut state)
}
