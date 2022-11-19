use macroquad::prelude::*;

// Systems that are inherinted by different entities

// Trait of physics application
pub trait Physics {
    fn update(&mut self, delta_time: f32); // Required for physics
    fn collision(&mut self) {} // Optional for physics
}

pub trait Render {
    fn draw(&self, texture: Texture2D); // Required for rendering
    fn animate(&mut self) {} // Optional for rendering
}