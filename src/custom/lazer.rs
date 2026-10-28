use crate::ui::still_image::StillImage;
use macroquad::input::KeyCode;
use macroquad::input::MouseButton::Left;
use macroquad::prelude::*;
use crate::utils::collision::check_collision;

pub struct Lazer {
    image: StillImage,
    pub speed: f32,
    

}

impl Lazer {
    pub async fn new (x: f32, y: f32, width: f32, height: f32, speed: f32) -> Lazer {
    let image = StillImage::new("assets/lazer_beam.png", width, height, x, y, true, 1.0).await;
        Lazer{
        image,
        speed,
    }
}


pub fn get_image(&self) -> &StillImage {
        &self.image
    }

pub fn draw(&self) {
    self.image.draw();
    
}

pub fn projectile_move(&mut self) {
    self.image.set_y(self.image.get_y() - self.speed);

}


    pub fn get_speed(&self) -> f32 {
        self.speed
    }

pub fn get_y(&self) -> f32 {
    self.image.get_y()
}

pub fn set_x(&mut self, x: f32 ) {
    self.image.set_x(x);}

pub fn set_y(&mut self, y: f32) {
    self.image.set_y(y);}



    pub fn lazer_hit(&mut self) {
    self.image.set_y(-100.0);
    }

pub fn lazer_shoot(&mut self , y: f32 , x: f32) {
if self.image.get_y() < 0.0 {
    if is_mouse_button_down(Left) || is_key_down(KeyCode::Space){
     self.image.set_x(y);
     self.image.set_y(x);
     self.image.draw();
    }
}
}


}