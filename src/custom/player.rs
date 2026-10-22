use crate::ui::still_image::StillImage;
use macroquad::input::KeyCode;
use macroquad::prelude::*;

pub struct Player {
    image: StillImage,
    pub speed: f32,
    pub move_dir: Vec2,
    pub old_pos: Vec2

}

impl Player {
    pub async fn new (x: f32, y: f32, width: f32, height: f32, image_name: &str , speed: f32) -> Player {
    let image = StillImage::new(image_name, width, height, x, y, true, 1.0).await;
    
        Player{
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

    pub fn keypress(&mut self) {
        self.move_dir = vec2(0.0, 0.0);
        self.old_pos = vec2 (self.image.get_x(), self.image.get_y());
    if is_key_down(KeyCode::D) {
        self.move_dir.x += self.speed;
    }
    if is_key_down(KeyCode::A) {
        self.move_dir.x -= self.speed;
    }
    if is_key_down(KeyCode::S) {
        self.move_dir.y += self.speed;
    }
    if is_key_down(KeyCode::W) {
        self.move_dir.y -=  self.speed;
    }
    if self.move_dir.length() > 0.0 {
        self.move_dir = self.move_dir.normalize() * 2.0;
    }
    }
    
    pub fn get_x(&self) -> f32 {
      self.image.get_x()
    }

        pub fn get_y(&self) -> f32 {
        self.image.get_y()
    }


pub fn move_player(&mut self) {
    self.image.set_x(self.image.get_x() + self.move_dir.x);
    self.image.set_y(self.image.get_y() + self.move_dir.y );
    
}
pub fn move_back_x(&mut self){
    self.image.set_x(self.old_pos.x);
    
}
pub fn move_back_y(&mut self){
    self.image.set_y(self.old_pos.y);
    
}


}

