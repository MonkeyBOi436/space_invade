use macroquad::prelude::*;
use crate::custom::enemy::Enemy;
use crate::custom::player::Player;
use crate::custom::lazer::Lazer;
use crate::ui::grid::draw_grid;
use crate::ui::text_button::TextButton;
use crate::ui::still_image::StillImage;

pub async fn run() -> String {
    let mut enemy = Enemy::new(300.0, 1000.0, 50.0, 50.0, "assets/bug.png" ,3.0).await;
let mut player = Player::new(300.0, 1000.0, 50.0, 50.0, "assets/player.png" ,3.0).await;

let background_image = StillImage::new(
        "assets/background.png",
        2620.0,  // width
        1900.0,  // height
        0.0,  // x position
        0.0,   // y position
        true,   // Enable stretching
        1.0,    // Normal zoom (100%)
    ).await;

let btn_exit_tutorial = TextButton::new(
        100.0,
        200.0,
        200.0,
        60.0,
        "Exit the tutorial",
        GREEN,
        GREEN,
        30
    );
    let mut lazer = Lazer::new(0.0, -19.0, 50.0, 50.0, 7.0).await;
    loop {

let lazer_x = player.get_image().get_x() + 3.0;
let lazer_y = player.get_image().get_y() - 55.0;
       //  player.increase_speed();
        clear_background(BLUE);

        
        draw_text("Tutorial", 20.0, 40.0, 30.0, WHITE);

player.keypress();



 player.move_player();
 
lazer.lazer_shoot(lazer_x, lazer_y);



lazer.projectile_move();

        if btn_exit_tutorial.click() {
            return "menu".to_string();
        }

        enemy.draw();
        player.draw();
        lazer.draw();
draw_grid(50.0, BLACK);
        next_frame().await;
    }
}