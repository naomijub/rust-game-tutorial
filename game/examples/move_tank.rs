use std::collections::HashSet;
use std::path;

use ggez::conf::WindowSetup;
use ggez::event::KeyCode;
use ggez::graphics::Rect;
use ggez::nalgebra as na;
use ggez::{
    self, event,
    graphics::{self, screen_coordinates},
};

use game::{objects::Tank, state::window_state_mode};

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
        .window_mode(window_state_mode());

    let (ctx, events_loop) = &mut cb.build()?;
    let sc = screen_coordinates(ctx);

    let tank_base = graphics::Image::new(ctx, "/TankBase.png")?;
    let tank_dimensions = tank_base.dimensions();

    let mut tank = Tank {
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
        player: game::state::Player::P1,
        turret_width: tank_dimensions.w * 0.7,
    };
    let dim = Rect::new(0., 0., 10., 10.);

    tank.movement(
        &vec![KeyCode::W, KeyCode::P]
            .into_iter()
            .collect::<HashSet<KeyCode>>(),
        sc,
        dim,
        &Rect::new(0., 0., 0., 0.),
    );
    tank.rotation(
        &vec![KeyCode::Up, KeyCode::D]
            .into_iter()
            .collect::<HashSet<KeyCode>>(),
        0.3,
    );
    tank.movement(
        &vec![KeyCode::Up, KeyCode::D]
            .into_iter()
            .collect::<HashSet<KeyCode>>(),
        sc,
        dim,
        &Rect::new(0., 0., 0., 0.),
    );
    tank.rotation(
        &vec![KeyCode::Up, KeyCode::Right]
            .into_iter()
            .collect::<HashSet<KeyCode>>(),
        0.3,
    );
    tank.movement(
        &vec![KeyCode::W].into_iter().collect::<HashSet<KeyCode>>(),
        sc,
        dim,
        &Rect::new(0., 0., 0., 0.),
    );
    tank.movement(
        &vec![KeyCode::S].into_iter().collect::<HashSet<KeyCode>>(),
        sc,
        dim,
        &Rect::new(0., 0., 0., 0.),
    );
    tank.rotation(
        &vec![KeyCode::Up, KeyCode::Left]
            .into_iter()
            .collect::<HashSet<KeyCode>>(),
        0.3,
    );

    let mut test_state = test_ggez::TestState::new(tank, "move_tank");

    let _ = event::run(ctx, events_loop, &mut test_state);
    Ok(())
}
