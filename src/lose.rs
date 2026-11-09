use macroquad::prelude::*;
use crate::ui::still_image::StillImage;

pub async fn run(score: i32 , high_score: i32) -> (String, i32 , i32) {
    let mut current_score = score;
    let mut high_score = high_score;

let background_image = StillImage::new(
        "assets/background.png",
        2620.0,  // width
        1700.0,  // height
        0.0,  // x position
        0.0,   // y position
        true,   // Enable stretching
        1.0,    // Normal zoom (100%)
    ).await;

let fire = StillImage::new(
        "assets/fire.png",
        1900.0,  // width
        700.0,  // height
        700.0,  // x position
        600.0,   // y position
        true,   // Enable stretching
        1.0,    // Normal zoom (100%)
    ).await;

    loop {
        clear_background(BLUE);
        background_image.draw();
        fire.draw();
        draw_text("YOU LOST", 600.0, 200.0, 70.0, WHITE);
        draw_text("Press SPACE to return to the menu", 600.0, 300.0, 70.0, WHITE);
        draw_text(&format!("You're score was {}", current_score), 600.0, 400.0, 70.0, WHITE);

        if is_key_pressed(KeyCode::Space) {
            return ("menu".to_string() , current_score , high_score);
        }

        next_frame().await;
    }
}