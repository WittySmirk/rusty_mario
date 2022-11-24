use macroquad::prelude::*;

pub enum EntityType {
    Mario,
    // Goomba,
    // Koopa,
    Ground,
    Brick,
    Question,
    // Coin,
    // Flag,
}

pub struct Entity {
    pub entity: Box<dyn EntityT>,
    pub e_type: EntityType,
    pub hitbox: Option<Rect>,
}

impl Entity {
    pub fn new(entity: Box<dyn EntityT>, hitbox: Option<Rect>, e_type: EntityType) -> Self {
        Self {
            entity,
            hitbox,
            e_type,
        }
    }
}

// Systems that are inherinted by different entities
pub trait EntityT {
    //Systems
    fn new(x: f32, y: f32, e_type: EntityType, spawns: Option<EntityType>) -> Self
    where
        Self: Sized;
    fn collision(&mut self, _other: &Option<Rect>) {}
    fn update(&mut self, _delta_time: f32, _camera: Option<&mut Camera2D>) {}
    fn draw(&self, _texture: Texture2D) {}
    fn animate(&mut self) {}
}
