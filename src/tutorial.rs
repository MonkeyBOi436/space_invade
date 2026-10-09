use macroquad::prelude::*;
use crate::custom::player::Player;
use crate::ui::grid::draw_grid;
use crate::ui::text_button::TextButton;

pub async fn run() -> String {
    let mut player = Player::new(100.0, 100.0, 50.0, 50.0, "assets/bug.png", 5.0).await;
    loop {
        clear_background(BLUE);
        draw_text("Tutorial", 20.0, 40.0, 30.0, WHITE);

player.keypress();
 player.move_player();



        if is_key_pressed(KeyCode::Space) {
            return "break".to_string();
        }

        player.draw();
draw_grid(50.0, BLACK);
        next_frame().await;
    }
}