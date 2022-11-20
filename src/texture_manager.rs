use crate::CONSTS;
use macroquad::prelude::*;
use crate::entity::EntityType;
// const mario_amount: f32 = 7f32;y

const MARIO_START: Vec2  = Vec2::from_array([0f32, 0f32]);
const BRICK_START: Vec2  = Vec2::from_array([0f32, CONSTS.mario_size.y]);
const GROUND_START: Vec2 = Vec2::from_array([CONSTS.block_size as f32, CONSTS.mario_size.y]);

impl EntityType {
    pub fn get_start(&self) -> Vec2 {
        // let mario_frame: Rect = Rect::new(0f32, 0f32, CONSTS.mario_size.x * mario_amount, CONSTS.mario_size.y);
        // let block_frame: Rect = Rect::new(0f32, CONSTS.mario_size.x, CONSTS.block_size as f32, CONSTS.block_size as f32);
        
        match self {
            Self::Mario => return MARIO_START,
            Self::Brick => return BRICK_START,
            Self::Ground => return GROUND_START,
        }
    }
}