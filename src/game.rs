use macroquad::prelude::*;
use crate::custom::enemy::Enemy;
use crate::custom::player::Player;
use crate::custom::lazer::Lazer;
use crate::ui::grid::draw_grid;
use crate::ui::still_image::StillImage;
use crate::ui::label::Label;
use crate::utils::collision::check_collision;

pub async fn run(mut score: i32, high_score: i32) -> (String, i32, i32) {


let mut player = Player::new(800.0, 1200.0, 50.0, 50.0, "assets/player.png" ,8.0).await;

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

let mut eneny_right_or_left = true;
let mut enemy_down = false;
    let mut lazer_player = Lazer::new(0.0, -109.0, 50.0, 50.0, 9.0).await;

let mut lazer_ene_1 = Lazer::new(0.0, -109.0, 50.0, 50.0, 4.0).await;
let mut lazer_ene_2 = Lazer::new(0.0, -109.0, 50.0, 50.0, 9.0).await;

    let mut enemies: Vec<Enemy> = vec![
        Enemy::new(0.0, 0.0, 70.0, 70.0, "assets/enemy_bl.png" ,3.0).await,
        Enemy::new(60.0, 0.0, 70.0, 70.0, "assets/enemy_gr.png" ,3.0).await,
        Enemy::new(120.0, 0.0, 70.0, 70.0, "assets/enemy_bl.png" ,3.0).await,
        Enemy::new(180.0, 0.0, 70.0, 70.0, "assets/enemy_gr.png" ,3.0).await,
        Enemy::new(240.0, 0.0, 70.0, 70.0, "assets/enemy_bl.png" ,3.0).await,
        Enemy::new(300.0, 0.0, 70.0, 70.0, "assets/enemy_gr.png" ,3.0).await,
        Enemy::new(360.0, 0.0, 70.0, 70.0, "assets/enemy_bl.png" ,3.0).await,
        Enemy::new(420.0, 0.0, 70.0, 70.0, "assets/enemy_gr.png" ,3.0).await,
        Enemy::new(480.0, 0.0, 70.0, 70.0, "assets/enemy_bl.png" ,3.0).await,
        Enemy::new(560.0, 0.0, 70.0, 70.0, "assets/enemy_gr.png" ,3.0).await
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

lazer_ene_1.projectile_move_ene();

lazer_player.projectile_move();

        if is_key_pressed(KeyCode::Escape) {
            return ("menu".to_string() , score , high_score);
        }

        if player.get_x() > 2500.0 || player.get_x() < 0.0{
            player.move_back_x();
        }

        if player.get_y() > 1330.0 || player.get_y() < 1100.0{
            player.move_back_y()
        }



        player.draw();
        lazer_player.draw();
        lazer_ene_1.draw();
        lbl_score.draw();

for enemy in &mut enemies.iter_mut() {
lazer_ene_1.get_image();
let lazer_x = enemy.get_image().get_x();
let lazer_y = enemy.get_image().get_y();
lazer_ene_1.lazer_shoot_ene(lazer_x, lazer_y);
}


for enemy in &mut enemies {
enemy.draw();
}

for enemy in &mut enemies {
if eneny_right_or_left == true{
enemy.move_enemy_x_right();
}
}
for enemy in &mut enemies {
if eneny_right_or_left == false{
enemy.move_enemy_x_left();
}}

for enemy in &mut enemies {
if enemy.get_image().get_x() >= 2500.0 {
   enemy_down = true;
    eneny_right_or_left = false;
}
}
if enemy_down == true{
for enemy in &mut enemies.iter_mut() {
    enemy.move_enemy_y();
}
    enemy_down = false;
}
for enemy in &mut enemies {
if enemy.get_image().get_x() <= 0.0 {
    enemy_down = true;
    eneny_right_or_left = true;
}
}
for enemy in &mut enemies {
if check_collision(lazer_player.get_image(), enemy.get_image(), 1) {
    enemy.clear();
    lazer_player.lazer_hit();
    score = score + 100;
    lbl_score.set_text(format!("Score: {}", score));
    }
}


draw_grid(50.0, BLACK);
        next_frame().await;
    }
}