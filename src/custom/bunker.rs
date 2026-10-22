use crate::ui::still_image::StillImage;
use macroquad::input::KeyCode;
use macroquad::prelude::*;

pub struct Bunker {
    image: StillImage,
    health: i32,

}

impl Bunker {
    pub async fn new (x: f32, y: f32, width: f32, height: f32, image_name: &str, health: i32) -> Bunker {
    let image = StillImage::new(image_name, width, height, x, y, true, 1.0,).await;
    
        Bunker{
        image,
        health: 5,
    }
    }
    pub fn get_image(&self) -> &StillImage {
        &self.image
    }

pub fn draw(&self) {
    self.image.draw();
}

pub fn bunk_hit(&self) {
self.health - 1;
}

pub fn bunk_change(&self) {


}

}