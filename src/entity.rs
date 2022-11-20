use macroquad::prelude::*;

pub enum EntityType {
    Mario,
    // Goomba,
    // Koopa,
    Ground,
    Brick,
    // Coin,
    // Flag,
}

//TODO figure out how to break systems into optional subtraits instead of having them all be defaults

// Systems that are inherinted by different entities 
pub trait Entity {
    fn new(x: f32, y: f32, e_type: EntityType) -> Self where Self: Sized;
    fn update(&mut self, _delta_time: f32) {}
    fn collision(&mut self) {} 
    fn draw(&self, _texture: Texture2D) {} 
    fn animate(&mut self) {} 
}

// Trait of physics application
// pub trait Physics: Entity {
//     fn update(&mut self, delta_time: f32); // Required for physics
//     fn collision(&mut self) {} // Optional for physics
// }

// pub trait Render: Entity {
//     fn draw(&self, texture: Texture2D); // Required for rendering
//     fn animate(&mut self) {} // Optional for rendering
// }
