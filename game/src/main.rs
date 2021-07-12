use macroquad::prelude::*;

#[macroquad::main("Tank Battle Ground")]
async fn main() {
    let tank_base: Texture2D = load_texture("../assets/TankBase.png").await.unwrap();
    
    loop {
        clear_background(BEIGE);

        draw_texture(
            tank_base,
            screen_width() / 2. - tank_base.width() / 2.,
            screen_height() / 2. - tank_base.height() / 2.,
            WHITE,
        );

        next_frame().await
    }
}
