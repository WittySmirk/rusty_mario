use crate::entity::{EntityT, EntityType};
use crate::CONSTS;
use macroquad::prelude::*;

pub struct Block {
    hitbox: Rect,
    reference_frame: Rect,
}

impl Block {
    pub fn hitbox(&self) -> Rect {
        self.hitbox
    }
}

impl EntityT for Block {
    fn new(x: f32, y: f32, e_type: EntityType, spawns: Option<EntityType>) -> Self {
        Self {
            hitbox: Rect::new(x, y, CONSTS.block_size as f32, CONSTS.block_size as f32),
            reference_frame: Rect::new(
                e_type.get_start().x,
                e_type.get_start().y,
                CONSTS.block_size as f32,
                CONSTS.block_size as f32,
            ),
        }
    }
    fn draw(&self, texture: Texture2D) {
        draw_texture_ex(
            texture,
            self.hitbox.x,
            self.hitbox.y,
            WHITE,
            DrawTextureParams {
                dest_size: Some(Vec2::new(
                    CONSTS.block_size as f32,
                    CONSTS.block_size as f32,
                )),
                source: Some(self.reference_frame),
                ..Default::default()
            },
        );
    }
}
