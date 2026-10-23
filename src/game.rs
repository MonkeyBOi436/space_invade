use macroquad::prelude::*;
use crate::custom::enemy::Enemy;
use crate::custom::player::Player;
use crate::custom::lazer::Lazer;
use crate::ui::grid::draw_grid;
use crate::ui::text_button::TextButton;
use crate::ui::still_image::StillImage;
use crate::ui::label::Label;

pub async fn run() -> String {
let mut player = Player::new(800.0, 1200.0, 50.0, 50.0, "assets/player.png" ,3.0).await;

let mut lbl_score = Label::new("Score: 0", 50.0, 100.0, 30);

let background_image = StillImage::new(
        "assets/background.png",
        2620.0,  // width
        1700.0,  // height
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

let mut eneny_right_or_left = true;
    let mut lazer_player = Lazer::new(0.0, -19.0, 50.0, 50.0, 7.0).await;

    let mut enemy_1 = Enemy::new(0.0, 100.0, 50.0, 50.0, "assets/enemy_bl.png" ,3.0).await;

    let enemies: Vec<Enemy> = vec![
        Enemy::new(0.0, 0.0, 50.0, 50.0, "assets/enemy_bl.png" ,3.0).await,
        Enemy::new(60.0, 0.0, 50.0, 50.0, "assets/enemy_gr.png" ,3.0).await,
        Enemy::new(120.0, 0.0, 50.0, 50.0, "assets/enemy_bl.png" ,3.0).await,
        Enemy::new(180.0, 0.0, 50.0, 50.0, "assets/enemy_gr.png" ,3.0).await,
        Enemy::new(240.0, 0.0, 50.0, 50.0, "assets/enemy_bl.png" ,3.0).await,
        Enemy::new(300.0, 0.0, 50.0, 50.0, "assets/enemy_gr.png" ,3.0).await,
        Enemy::new(360.0, 0.0, 50.0, 50.0, "assets/enemy_bl.png" ,3.0).await,
        Enemy::new(420.0, 0.0, 50.0, 50.0, "assets/enemy_gr.png" ,3.0).await,
        Enemy::new(480.0, 0.0, 50.0, 50.0, "assets/enemy_bl.png" ,3.0).await,
        Enemy::new(560.0, 0.0, 50.0, 50.0, "assets/enemy_gr.png" ,3.0).await
    ];

    loop {

let lazer_x = player.get_image().get_x() + 3.0;
let lazer_y = player.get_image().get_y() - 55.0;
       //  player.increase_speed();
        clear_background(BLUE);
                background_image.draw();
player.keypress();



 player.move_player();
 
lazer_player.lazer_shoot(lazer_x, lazer_y);



lazer_player.projectile_move();

        if btn_exit_tutorial.click() {
            return "menu".to_string();
        }

        if player.get_x() > 2500.0 || player.get_x() < 0.0{
            player.move_back_x();
        }

        if player.get_y() > 1330.0 || player.get_y() < 1100.0{
            player.move_back_y()
        }



        player.draw();
        lazer_player.draw();
        lbl_score.draw();
for (_i, enemy) in &mut enemies.iter().enumerate(){
enemy.draw();
}
enemy_1.draw();
if eneny_right_or_left == true{
enemy_1.move_enemy_x_right();
}

if eneny_right_or_left == false{
enemy_1.move_enemy_x_left();
}

if enemy_1.get_image().get_x() >= 2500.0 {
    eneny_right_or_left = false;
    enemy_1.move_enemy_y();
}

if enemy_1.get_image().get_x() <= 0.0 {
    eneny_right_or_left = true;
    enemy_1.move_enemy_y();
}
draw_grid(50.0, BLACK);
        next_frame().await;
    }
}