use macroquad::prelude::*;

#[macroquad::main("Tank Battle Ground")]
async fn main() {
    let mut tank = TankBase::new(screen_width(), screen_height()).await;
    loop {
        clear_background(BEIGE);

        tank.update();
        tank.draw();

        next_frame().await
    }
}

pub struct TankBase {
    pub(crate) position: Vec2,
    direction: Vec2,
    rotation: f32,
    texture: Texture2D,
}

impl TankBase {
    async fn new(width: f32, height: f32) -> Self {
        let tank_base: Texture2D = load_texture("../assets/TankBase.png")
            .await
            .unwrap_or(Texture2D::empty());
        Self {
            position: Vec2::from((
                width / 2. - tank_base.width() / 2.,
                height / 2. - tank_base.height() / 2.,
            )),
            direction: Vec2::from((-1., 0.)),
            rotation: 0.,
            texture: tank_base,
        }
    }

    fn draw(&self) {
        draw_texture_ex(
            self.texture,
            self.position.x,
            self.position.y,
            WHITE,
            DrawTextureParams {
                dest_size: None,
                source: None,
                rotation: self.rotation,
                flip_x: false,
                flip_y: false,
                pivot: None,
            },
        );
    }

    fn update(&mut self) {
        if is_key_down(KeyCode::D) || is_key_down(KeyCode::Right) {
            self.turn_right();
        }

        if is_key_down(KeyCode::A) || is_key_down(KeyCode::Left) {
            self.turn_left();
        }

        if is_key_down(KeyCode::W) || is_key_down(KeyCode::Up) {
            self.forward();
        }

        if is_key_down(KeyCode::S) || is_key_down(KeyCode::Down) {
            self.back();
        }
    }

    fn forward(&mut self) {
        self.position += self.direction;
    }

    fn back(&mut self) {
        self.position -= self.direction;
    }

    fn turn_right(&mut self) {
        self.rotation += get_frame_time();
        self.reset_direction();
    }

    fn turn_left(&mut self) {
        self.rotation -= get_frame_time();
        self.reset_direction();
    }

    fn reset_direction(&mut self) {
        let (sin, cos) = self.rotation.sin_cos();
        self.direction = Vec2::from((-cos, -sin));
    }
}
