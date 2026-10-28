use macroquad::prelude::*;
use crate::ui::still_image::StillImage;

pub async fn run(score: i32 , high_score: i32) -> (String, i32 , i32) {
    let mut current_score = score;
    let mut high_score = high_score;

    loop {
        clear_background(BLUE);
        draw_text("lose", 20.0, 40.0, 30.0, WHITE);

        if is_key_pressed(KeyCode::Space) {
            return ("menu".to_string() , current_score , high_score);
        }

        next_frame().await;
    }
}