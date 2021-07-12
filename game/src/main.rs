use macroquad::{prelude::*};

#[macroquad::main("Tank Battle Ground")]
async fn main() {
    loop {
        clear_background(BEIGE);

        draw_text("IT WORKS!", 20.0, 20.0, 30.0, DARKGRAY);

        next_frame().await
    }
}