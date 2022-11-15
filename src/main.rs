use macroquad::prelude::*;

// Constants for the game
const SCALE: i32 = 3;
const PLAYER_SIZE: Vec2 = Vec2::from_array([(13 * SCALE) as f32, (16 * SCALE) as f32]);
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

enum PlayerState {
    Idle,
    Walking,
    Jumping,
    Running,
    Skidding,
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
    facing: bool, //false = right, true = left
    running: bool,
    velocity: Vec2,
    state: PlayerState,
    falling_accel: f32,
    jumping: bool,
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
            state: PlayerState::Idle,
            facing: false,
            running: false,
            velocity: Vec2::from_array([0f32, 0f32]),
            x_move: 0f32,
            falling_accel: 0f32,
            jumping: false,
        };
    }

    pub fn update(&mut self, dt: f32) {
        //TODO: Fix no deceleration bug
        //TODO: Refactor physics (Enums)

        // Match tuple of moving left or right
        // let p_xmove: f32 = self.x_move;
        match (is_key_down(KeyCode::Left), is_key_down(KeyCode::Right)) {
            (true, false) => {
                self.x_move = -1f32;
                if let PlayerAnims::SwitchingDir = self.current_anim {
                } else {
                    self.current_anim = PlayerAnims::Right;
                    // self.current_frame = 4f32;
                }
                self.facing = true;
            }
            (false, true) => {
                self.x_move = 1f32;
                if let PlayerAnims::SwitchingDir = self.current_anim {
                } else {
                    self.current_anim = PlayerAnims::Right;
                    // self.current_frame = 4f32;
                }
                self.facing = false;
            }
            (false, false) => {
                self.x_move = 0f32;
            }
            _ => {}
        };

        match is_key_down(KeyCode::X) {
            true => {
                self.running = true;
            }
            false => {
                self.running = false;
            }
        }

        match is_key_down(KeyCode::Z) {
            true => {
                self.jumping = true;
            }
            false => {
                self.jumping = false;
            }
        }

        //Move based on direction
        // self.hitbox.x += self.x_move * dt * PLAYER_SPEED;

        // if ((p_xmove == -1f32 || p_xmove == 0f32) && self.x_move == 1f32)
        // || ((p_xmove == 1f32 || p_xmove == 0f32) && self.x_move == -1f32)
        // {
        // self.current_anim = PlayerAnims::SwitchingDir;
        // }
        const MIN_WALK: f32 = 4.453125f32;
        const MAX_WALK: f32 = 93.75f32;
        const MAX_RUN: f32 = 153.75f32;
        const ACC_WALK: f32 = 133.5975f32;
        const ACC_RUN: f32 = 200.390625f32;
        const DEC_REL: f32 = 182.8135f32;
        const DEC_SKID: f32 = 365.625f32;
        // const MIN_SKID: f32 = 33.75f32;

        const STOP_FALL: f32 = 1575f32;
        const WALK_FALL: f32 = 1800f32;
        const RUN_FALL: f32 = 2025f32;
        const STOP_FALL_A: f32 = 450f32;
        const WALK_FALL_A: f32 = 421.875f32;
        const RUN_FALL_A: f32 = 562.5f32;
        const MAX_FALL: f32 = 270f32;

        if let PlayerState::Jumping = self.state {
            //Jumping
            if self.velocity.x.abs() < MIN_WALK {
                // slower than a walk // starting, stopping or turning around
                self.velocity.x = 0f32;
                self.state = PlayerState::Idle;
                if self.x_move == -1f32 {
                    self.velocity.x -= MIN_WALK;
                }
                if self.x_move == 1f32 {
                    self.velocity.x += MIN_WALK;
                }
            } else if self.velocity.x.abs() >= MIN_WALK {
                //Faster than a walk // accelerating or decelarating
                if self.facing == false {
                    if self.x_move == 1f32 {
                        if self.running {
                            self.velocity.x += ACC_RUN * dt;
                        } else {
                            self.velocity.x += ACC_WALK * dt;
                        }
                    } else if self.x_move == -1f32 {
                        self.velocity.x += DEC_SKID * dt;
                        self.state = PlayerState::Skidding;
                    } else {
                        self.velocity.x -= DEC_REL * dt;
                    }
                } else {
                    if self.x_move == 1f32 {
                        if self.running {
                            self.velocity.x -= ACC_RUN * dt;
                        }
                    }
                }
            }

            self.velocity.y += self.falling_accel * dt;

            if self.jumping {
                if self.velocity.x.abs() < 16f32 {
                    self.velocity.y = -240f32;
                    self.falling_accel = STOP_FALL;
                } else if self.velocity.x.abs() < 40f32 {
                    self.velocity.y = -240f32;
                    self.falling_accel = WALK_FALL;
                } else {
                    self.velocity.y = -300f32;
                    self.falling_accel = RUN_FALL;
                }
                self.state = PlayerState::Jumping;
            }
        } else {
            //Not jumping
            if self.velocity.y < 0f32 && self.jumping {
                if self.falling_accel == STOP_FALL {
                    self.velocity.y -= (STOP_FALL - STOP_FALL_A) * dt;
                }
                if self.falling_accel == WALK_FALL {
                    self.velocity.y -= (WALK_FALL - WALK_FALL_A) * dt;
                }
                if self.falling_accel == RUN_FALL {
                    self.velocity.y -= (RUN_FALL - RUN_FALL_A) * dt;
                }
            }

            self.velocity.y += self.falling_accel * dt;

            if self.x_move == 1f32 {
                if self.velocity.x.abs() > MAX_WALK {
                    self.velocity.x += ACC_RUN * dt;
                } else {
                    self.velocity.x += ACC_WALK * dt;
                }
            } else if self.x_move == -1f32 {
                if self.velocity.x.abs() > MAX_WALK {
                    self.velocity.x -= ACC_RUN * dt;
                } else {
                    self.velocity.x -= ACC_WALK * dt;
                }
            } else {
                //Do nothing
            }
        }

        //Cap max speeds
        if self.velocity.y >= MAX_FALL {
            self.velocity.y = MAX_FALL;
        }
        if self.velocity.y <= -MAX_FALL {
            self.velocity.y = -MAX_FALL;
        }

        if self.velocity.x >= MAX_RUN {
            self.velocity.x = MAX_RUN;
        }
        if self.velocity.x <= -MAX_RUN {
            self.velocity.x = -MAX_RUN;
        }
        if self.velocity.x >= MAX_RUN && !self.running {
            self.velocity.x = MAX_WALK;
        }
        if self.velocity.x <= -MAX_RUN && !self.running {
            self.velocity.x = -MAX_WALK;
        }

        //Update position
        self.hitbox.x += self.velocity.x * dt * (SCALE as f32);
        self.hitbox.y += self.velocity.y * dt * (SCALE as f32);

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
                flip_x: self.facing,
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
