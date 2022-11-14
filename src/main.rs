use macroquad::prelude::*;

// Constants for the game
const SCALE: i32 = 3;
const PLAYER_SIZE: Vec2 = Vec2::from_array([(13 * SCALE) as f32, (16 * SCALE) as f32]);
const PLAYER_SPEED: f32 = 100f32;
const PLAYER_FRAME_SPEED: f32 = 8f32;

// Bunch of window settings
fn window_conf() -> Conf {
    Conf {
        window_title: "Mario".to_owned(),
        window_width: 240 * SCALE,
        window_height: 240 * SCALE,
        window_resizable: false, // Disable this once we figure out how to do scaling based on resize
        ..Default::default()
    }
}

enum PlayerAnims {
    Idle,
    Right,
    // JumpRight,
    // Dead, //Will be removed later
    SwitchingDir,
}

struct Player {
    hitbox: Rect,           // Basically our physics object that our texture projects to
    texture_map: Texture2D, // All the sprites for mario, later will be a texture atlas for the whole game
    current_reference_frame: Rect, // Referance frame for where in the atlas the current sprite for mario is
    frame_min: f32,
    frame_max: f32,
    frame_counter: f32,
    x_move: f32,
    current_frame: f32,
    current_anim: PlayerAnims,
    current_anim_mirrored: bool,
}

impl Player {
    async fn new() -> Self {
        return Self {
            hitbox: Rect::new(
                screen_width() * 0.5,
                screen_height() * 0.5,
                PLAYER_SIZE.x,
                PLAYER_SIZE.y,
            ),
            texture_map: load_texture("res/mario_sprites.png").await.unwrap(),
            current_reference_frame: Rect::new(
                0f32,
                0f32,
                (13 * SCALE) as f32,
                (16 * SCALE) as f32,
            ),
            frame_min: 0f32,
            frame_max: 0f32,
            frame_counter: 0f32,
            current_frame: 0f32,
            current_anim: PlayerAnims::Idle,
            current_anim_mirrored: false,
            x_move: 0f32,
        };
    }

    pub fn update(&mut self, dt: f32) {
        //TODO: Create physics
        //TODO: Convert Movement into vector

        // const MIN_WALK:    f32 = 4.453125f32;
        // const MAX_WALK:    f32 = 93.75f32;
        // const ACC_WALK:    f32 = 133.5975f32;
        // const ACC_RUN:     f32 = 200.390625f32;
        // const DEC_REL:     f32 = 182.8135f32;
        // const DEC_SKID:    f32 = 365.625f32;
        // const MIN_SKID:    f32 = 33.75f32;

        // const STOP_FALL:   f32 = 1575f32;
        // const WALK_FALL:   f32 = 1800f32;
        // const RUN_FALL:    f32 = 2025f32;
        // const STOP_FALL_A: f32 = 450f32;
        // const WALK_FALL_A: f32 = 421.875f32;
        // const RUN_FALL_A:  f32 = 562.5f32;

        //Match tuple of moving left or right
        let p_xmove: f32 = self.x_move;
        match (
            is_key_down(KeyCode::A) || is_key_down(KeyCode::Left),
            is_key_down(KeyCode::D) || is_key_down(KeyCode::Right),
        ) {
            (true, false) => {
                self.x_move = -1f32;
                if let PlayerAnims::SwitchingDir = self.current_anim {
                } else {
                    self.current_anim = PlayerAnims::Right;
                    // self.current_frame = 4f32;
                }
                self.current_anim_mirrored = true;
            }
            (false, true) => {
                self.x_move = 1f32;
                if let PlayerAnims::SwitchingDir = self.current_anim {
                } else {
                    self.current_anim = PlayerAnims::Right;
                    // self.current_frame = 4f32;
                }
                self.current_anim_mirrored = false;
            }
            _ => {
                self.x_move = 0f32;
                self.current_anim = PlayerAnims::Idle;
                // self.current_frame = 0f32;
            }
        };

        //Move based on direction
        self.hitbox.x += self.x_move * dt * PLAYER_SPEED;

        if ((p_xmove == -1f32 || p_xmove == 0f32) && self.x_move == 1f32)
            || ((p_xmove == 1f32 || p_xmove == 0f32) && self.x_move == -1f32)
        {
            self.current_anim = PlayerAnims::SwitchingDir;
        }

        //Jump
        // if is_key_pressed(KeyCode::Space) || is_key_pressed(KeyCode::Z) {}
    }

    pub fn animate(&mut self) {
        match &(self.current_anim) {
            PlayerAnims::Right => {
                self.frame_min = 4f32;
                self.frame_max = 5f32;
            }
            PlayerAnims::SwitchingDir => {
                self.frame_min = 2f32;
                self.frame_max = 3f32;
            }
            // PLAYER_ANIMS::JumpRight => {
            // self.frame_min = 6f32;
            // self.frame_max = 6f32;
            // }
            // PLAYER_ANIMS::Dead => {
            //     self.frame_min = 1f32;
            //     self.frame_max = 1f32;
            // }
            _ => {
                self.frame_min = 0f32;
                self.frame_max = 0f32;
            }
        }

        self.frame_counter += 1f32;
        if self.current_frame < self.frame_min {
            self.current_frame = self.frame_min;
        }

        if self.frame_counter >= (get_fps() as f32 / PLAYER_FRAME_SPEED) {
            self.frame_counter = 0f32;

            if self.current_frame > self.frame_max {
                if self.frame_max == 3f32 {
                    self.current_anim = PlayerAnims::Right;
                }
                self.current_frame = self.frame_min;
            }

            self.current_reference_frame.x = self.current_frame * PLAYER_SIZE.x;
            self.current_frame += 1f32;
        }
    }

    pub fn draw(&self) {
        draw_texture_ex(
            self.texture_map,
            self.hitbox.x,
            self.hitbox.y,
            WHITE,
            DrawTextureParams {
                dest_size: Some(PLAYER_SIZE),
                source: Some(self.current_reference_frame),
                flip_x: self.current_anim_mirrored,
                ..Default::default()
            },
        );
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let screen_bg: Color = Color::from_rgba(90, 147, 245, 100);
    let mut mario: Player = Player::new().await;

    loop {
        //Update shit
        mario.update(get_frame_time());

        //Animate shit
        mario.animate();

        clear_background(screen_bg);

        //Draw shit
        mario.draw();

        next_frame().await;
    }
}
