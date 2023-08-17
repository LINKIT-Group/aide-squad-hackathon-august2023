use macroquad::prelude::*;

#[macroquad::main("BasicShapes")]
async fn main() {
    loop {
        clear_background(WHITE);
        draw_text("MyAPP!", 20.0, 20.0, 30.0, BLUE);

        next_frame().await
    }
}
