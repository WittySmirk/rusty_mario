use crate::entity::EntityT;
use crate::entity::EntityType;
use crate::CONSTS;
use macroquad::prelude::*;

const QUESTION_FRAME_SPEED: f32 = 8f32;

pub struct Question {
    hitbox: Rect,
    reference_frame: Rect,
    spawns: Option<EntityType>,
    frame_counter: f32,
    frame_min: f32,
    current_frame: f32,
    frame_max: f32,
}

impl EntityT for Question {
    fn new(x: f32, y: f32, e_type: EntityType, spawns: Option<EntityType>) -> Self {
        Self {
            hitbox: Rect::new(x, y, CONSTS.block_size as f32, CONSTS.block_size as f32),
            reference_frame: Rect::new(
                e_type.get_start().x,
                e_type.get_start().y,
                CONSTS.block_size as f32,
                CONSTS.block_size as f32,
            ),
            frame_counter: 0f32,
            frame_min: 0f32,
            frame_max: 2f32,
            current_frame: 0f32,
            spawns,
        }
    }

    fn animate(&mut self) {
        self.frame_counter += 1f32;
        if self.current_frame < self.frame_min {
            self.current_frame = self.frame_min;
        }

        if self.frame_counter >= (get_fps() as f32 / QUESTION_FRAME_SPEED) {
            self.frame_counter = 0f32;

            if self.current_frame > self.frame_max {
                self.current_frame = self.frame_min;
            }

            self.reference_frame.x = self.current_frame
                * (CONSTS.block_size as f32 / CONSTS.settings.scale as f32)
                * 3f32;
            self.current_frame += 1f32;
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
        )
    }
}
