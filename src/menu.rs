use macroquad::prelude::*;
use crate::ui::grid::draw_grid;
use crate::ui::text_button::TextButton;
use crate::ui::label::Label;
use crate::ui::still_image::StillImage;

pub async fn run(mut score: i32 , mut high_score: i32) -> (String, i32 , i32) {

    let btn_exit: TextButton = TextButton::new(
        1100.0,
        800.0,
        300.0,
        80.0,
        "Exit",
        RED,
        YELLOW,
        30
    );

        let btn_start = TextButton::new(
        1100.0,
        600.0,
        300.0,
        80.0,
        "Start the game",
        BLUE,
        GREEN,
        40
    );

let img_back = StillImage::new(
        "assets/back.png",     // Empty string creates a transparent image
        3000.0,  // width
        1400.0,  // height
        0.0,  // x position
        0.0,   // y position
        true,   // Enable stretching
        1.0,    // Normal zoom (100%)
    ).await;

 let mut lbl_score = Label::new("High Score: 0", 50.0, 100.0, 60);

    loop {
        clear_background(WHITE);
        img_back.draw();

if btn_exit.click() {
            return ("exit".to_string() , score , high_score);
        }

if score > high_score {
high_score = score;
score = 0;
}
else {
    score = 0;
}


if btn_start.click() {
            return ("game".to_string() , score , high_score);
        }

draw_text("Space invaders", 1020.0, 180.0, 60.0, BLACK);
draw_text("Controls WASD for movement", 1800.0, 400.0, 35.0, BLACK);
draw_text("left click or space to shoot", 1800.0, 450.0, 35.0, BLACK);
draw_text("The goal is to distroy the aliens", 1800.0, 500.0, 35.0, BLACK);

    lbl_score.set_text(format!("Score: {}", high_score));

lbl_score.draw();
draw_grid(50.0, BLACK);
        next_frame().await;
    }
}