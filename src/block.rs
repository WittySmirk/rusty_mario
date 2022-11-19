use crate::entity::Render;
use crate::CONSTS;

struct Block {
    hitbox: Rect,
    reference_frame: Rect,
    texture: Texture2D,
}

impl Block {
    pub fn new(x: f32, y: f32, start: Vec2) -> Self {
        Self {
            hitbox: Rect::new(x, y, CONSTS.block_size, CONSTS.block_size),
            reference_frame: Rect::new(start.x, start.y, CONSTS.block_size, CONSTS.block_size),
        }
    }
}

impl Render for Block {
    fn draw(&self, texture: Texture2D) {
        draw_texture_ex(
            self.texture,
            self.hitbox.x,
            self.hitbox.y,
            WHITE,
            DrawTextureParams {
                dest_size: Some(CONSTS.block_size),
                ..Default::default()
            },
        );
    }
}