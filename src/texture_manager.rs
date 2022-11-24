use crate::entity::EntityType;
use crate::CONSTS;
use macroquad::prelude::*;

const MARIO_START: Vec2 = Vec2::from_array([0f32, 0f32]);
const BRICK_START: Vec2 = Vec2::from_array([0f32, CONSTS.mario_size.y]);
const GROUND_START: Vec2 = Vec2::from_array([CONSTS.block_size as f32, CONSTS.mario_size.y]);
const QUESTION_START: Vec2 = Vec2::from_array([0f32, CONSTS.mario_size.y + CONSTS.block_size as f32]);

impl EntityType {
    pub fn get_start(&self) -> Vec2 {
        match self {
            Self::Mario => return MARIO_START,
            Self::Brick => return BRICK_START,
            Self::Ground => return GROUND_START,
            Self::Question => return QUESTION_START,
        }
    }
}
