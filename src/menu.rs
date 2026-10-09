use macroquad::prelude::*;
use crate::ui::grid::draw_grid;
use crate::ui::text_button::TextButton;

pub async fn run() -> String {
    let btn_exit = TextButton::new(
        100.0,
        200.0,
        200.0,
        60.0,
        "Exit",
        BLUE,
        GREEN,
        30
    );

        let btn_start = TextButton::new(
        100.0,
        200.0,
        200.0,
        60.0,
        "Start the game",
        BLUE,
        GREEN,
        30
    );

        let btn_tutorial = TextButton::new(
        100.0,
        200.0,
        200.0,
        60.0,
        "Tutorial",
        BLUE,
        GREEN,
        30
    );

        let btn_colour = TextButton::new(
        100.0,
        200.0,
        200.0,
        60.0,
        "Change Color of ship",
        BLUE,
        GREEN,
        30
    );


    loop {
        clear_background(WHITE);
        draw_text("Menu", 20.0, 40.0, 30.0, WHITE);

if btn_exit.click() {
            return "exit".to_string();
        }

        if is_key_pressed(KeyCode::Space) {
            return "tutorial".to_string();
        }
draw_grid(50.0, BLACK);
        next_frame().await;
    }
}