use macroquad::prelude::*;

pub async fn run() -> String {
    loop {
        clear_background(BLUE);
        draw_text("Game", 20.0, 40.0, 30.0, WHITE);

        if is_key_pressed(KeyCode::Space) {
            return "lose".to_string();
        }

        next_frame().await;
    }
}