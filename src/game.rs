
use macroquad::prelude::*;
use crate::custom::enemy::{self, Enemy};
use crate::custom::player::Player;
use crate::custom::lazer::Lazer;
use crate::ui::grid::draw_grid;
use crate::ui::still_image::StillImage;
use crate::ui::label::Label;
use crate::utils::collision::check_collision;
use miniquad::date;




pub async fn run(mut score: i32, high_score: i32) -> (String, i32, i32) {

    rand::srand(date::now() as u64);

let mut player = Player::new(800.0, 1200.0, 50.0, 50.0, "assets/player.png" ,8.0).await;

let mut player_health = 3;

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

let mut lazer_ene_1 = Enemy::new(0.0, 3000.0, 70.0, 70.0, "assets/lazer_beam_ene.png" ,3.0, true).await;
let mut lazer_ene_2 = Enemy::new(0.0, 3000.0, 70.0, 70.0, "assets/lazer_beam_ene.png" ,3.0, true).await;

    let mut enemy_lazer: Vec<Enemy> = vec![
        
    ];


    let mut enemies: Vec<Enemy> = vec![
Enemy::new(0.0, 0.0, 70.0, 70.0, "assets/enemy_bl.png" ,3.0, true).await,
Enemy::new(100.0, 0.0, 70.0, 70.0, "assets/enemy_gr.png" ,3.0, true).await,
Enemy::new(200.0, 0.0, 70.0, 70.0, "assets/enemy_bl.png" ,3.0, true).await,
Enemy::new(300.0, 0.0, 70.0, 70.0, "assets/enemy_gr.png" ,3.0, true).await,
Enemy::new(400.0, 0.0, 70.0, 70.0, "assets/enemy_bl.png" ,3.0, true).await,
Enemy::new(500.0, 0.0, 70.0, 70.0, "assets/enemy_gr.png" ,3.0, true).await,
Enemy::new(600.0, 0.0, 70.0, 70.0, "assets/enemy_bl.png" ,3.0, true).await,
Enemy::new(700.0, 0.0, 70.0, 70.0, "assets/enemy_gr.png" ,3.0, true).await,
Enemy::new(800.0, 0.0, 70.0, 70.0, "assets/enemy_bl.png" ,3.0, true).await,
Enemy::new(0.0, 100.0, 70.0, 70.0, "assets/enemy_bl.png" ,3.0, true).await,
Enemy::new(110.0, 100.0, 70.0, 70.0, "assets/enemy_gr.png" ,3.0, true).await,
Enemy::new(220.0, 100.0, 70.0, 70.0, "assets/enemy_bl.png" ,3.0, true).await,
Enemy::new(330.0, 100.0, 70.0, 70.0, "assets/enemy_gr.png" ,3.0, true).await,
Enemy::new(440.0, 100.0, 70.0, 70.0, "assets/enemy_bl.png" ,3.0, true).await,
Enemy::new(550.0, 100.0, 70.0, 70.0, "assets/enemy_gr.png" ,3.0, true).await,
Enemy::new(660.0, 100.0, 70.0, 70.0, "assets/enemy_bl.png" ,3.0, true).await,
Enemy::new(770.0, 100.0, 70.0, 70.0, "assets/enemy_gr.png" ,3.0, true).await,
Enemy::new(880.0, 100.0, 70.0, 70.0, "assets/enemy_bl.png" ,3.0, true).await,
    
    ];



let mut lazer_amount = false;

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
lazer_ene_2.projectile_move_ene();

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

for i in 0..enemies.len(){
if check_collision(lazer_player.get_image(), enemies[i].get_image(), 1) {
    lazer_player.lazer_hit();
    enemies.remove(i);
    

    score = score + 100;
    lbl_score.set_text(format!("Score: {}", score));
    }

}


for i in 0..enemy_lazer.len(){
    enemy_lazer[i].draw();
    enemy_lazer[i].projectile_move_ene();

    let rng = rand::gen_range(0, enemies.len());

    if enemy_lazer[i].get_image().get_y() > 1200.0{
    enemy_lazer[i].set_position(enemies[rng].get_image().get_x(), enemies[rng].get_image().get_y());
    println!("Enemy lazer cleared");
}

}

for i in 0..enemies.len(){
let rng = rand::gen_range(0, enemies.len());
    if lazer_amount == false{
enemy_lazer.push(Enemy::new(enemies[rng].get_image().get_x(), enemies[rng].get_image().get_y(), 70.0, 70.0, "assets/lazer_beam_ene.png" ,3.0 , true).await);
enemy_lazer.last_mut().unwrap().lazer_shoot_ene(enemies[rng].get_image().get_x(), enemies[rng].get_image().get_y());

if enemy_lazer.len() > 3{
    lazer_amount = true;
}

if enemy_lazer.len() < 3 && lazer_amount == true{
    lazer_amount = false;
}
}
}

for i in 0..enemy_lazer.len(){
if check_collision(player.get_image(), enemy_lazer[i].get_image(), 1) {
player_health = player_health -1;
enemy_lazer[i].lazer_hit_enemy();
if player_health == 0{
return ("lose".to_string() , score , high_score);
}
}
}

draw_grid(50.0, BLACK);
        next_frame().await;
    }
}