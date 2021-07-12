use macroquad::prelude::*;

fn window_conf() -> Conf {
    Conf {
        window_title: "Tank Battle Ground".to_owned(),
        fullscreen: true,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let screen_width = screen_width();
    let screen_height = screen_height();
    let mut tank = Tank::new(screen_width, screen_height, Player::P1).await;

    loop {
        clear_background(BEIGE);

        tank.update();
        tank.draw();

        next_frame().await
    }
}

pub enum Player {
    P1,
    P2,
    P3,
    P4,
}

pub struct Tank {
    pub(crate) position: Vec2,
    direction: Vec2,
    rotation: f32,
    base_texture: Texture2D,
    top_texture: Texture2D,
}

impl Tank {
    async fn new(width: f32, height: f32, player: Player) -> Self {
        let tank_base: Texture2D = load_texture("../assets/TankBase.png")
            .await
            .unwrap_or(Texture2D::empty());
        let tank_top_img = load_image("../assets/TankTops.png")
            .await
            .unwrap_or(Image::empty());

        let tank_top = get_tank_top_texture(tank_top_img, &player);

        Self {
            position: Vec2::from((
                width / 2. - tank_base.width() / 2.,
                height / 2. - tank_base.height() / 2.,
            )),
            direction: Vec2::from((-1., 0.)),
            rotation: 0.,
            base_texture: tank_base,
            top_texture: tank_top,
        }
    }

    fn draw(&self) {
        draw_texture_ex(
            self.base_texture,
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

        draw_texture_ex(
            self.top_texture,
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

fn get_tank_top_texture(tank_top_img: Image, player: &Player) -> Texture2D {
    let img_height = tank_top_img.height as f32;
    let img_width = tank_top_img.width as f32;
    let draw_height = img_height / 2.;
    let draw_width = img_width / 2.;

    let rect = match player {
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
    };

    Texture2D::from_image(&tank_top_img.sub_image(rect))
}
