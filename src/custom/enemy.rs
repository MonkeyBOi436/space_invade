use crate::ui::still_image::StillImage;
use macroquad::prelude::*;

pub struct Enemy {
    image: StillImage,
    speed: f32,
    pub move_dir: Vec2,
    pub old_pos: Vec2,

}

impl Enemy {
    pub async fn new (x: f32, y: f32, width: f32, height: f32, image_name: &str, speed: f32) -> Enemy {
    let image = StillImage::new(image_name, width, height, x, y, true, 1.0,).await;
    
        Enemy{
        image,
        speed,
        move_dir: vec2(0.0, 0.0),
        old_pos: vec2(0.0, 0.0),
    }
    }
    pub fn get_image(&self) -> &StillImage {
        &self.image
    }
    
pub fn draw(&self) {
    self.image.draw();
}

    pub fn get_speed(&self) -> f32 {
        self.speed
    }

pub fn move_enemy_y(&mut self){
self.image.set_y(self.image.get_y() + 20.0);

}

pub fn move_enemy_x_right(&mut self) {
    self.image.set_x(self.image.get_x() + 2.0);

}


pub fn move_enemy_x_left(&mut self) {
    self.image.set_x(self.image.get_x() - 2.0);

}

pub fn move_back_x(&mut self){
    self.image.set_x(self.old_pos.x);
    
}

pub fn move_back_y(&mut self){
    self.image.set_y(self.old_pos.y);
    
}


}