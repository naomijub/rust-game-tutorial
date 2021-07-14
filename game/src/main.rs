use ggez;
use ggez::conf::WindowMode;
use ggez::conf::WindowSetup;
use ggez::event;
use ggez::graphics;

struct MainState {}

impl MainState {
    fn new() -> MainState {
        let s = MainState {};
        s
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, _ctx: &mut ggez::Context) -> ggez::GameResult {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult {
        graphics::clear(ctx, [0.83, 0.69, 0.51, 1.0].into());

        graphics::present(ctx)?;
        Ok(())
    }
}

pub fn main() -> ggez::GameResult {
    let cb = ggez::ContextBuilder::new("tank_battle", "naomijub")
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
    let mut state = MainState::new();
    event::run(ctx, event_loop, &mut state)
}
