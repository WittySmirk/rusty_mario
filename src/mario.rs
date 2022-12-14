use macroquad::prelude::*;

use crate::entity::EntityT;
use crate::entity::EntityType;
use crate::CONSTS;

// Constants that the Player Struct needs
const PLAYER_FRAME_SPEED: f32 = 8f32;

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

enum PlayerState {
    Idle,
    Walking,
    Running,
    Skidding,
    JumpFall,
    // Ducking,
    // PullingFlag
}

pub struct Player {
    hitbox: Rect, // Basically our physics object that our texture projects to
    current_reference_frame: Rect, // Referance frame for where in the atlas the current sprite for mario is
    frame_min: f32,
    frame_max: f32,
    frame_counter: f32,
    current_frame: f32,
    facing: bool, //false = right, true = left
    running: bool,
    velocity: Vec2,
    state: PlayerState,
    falling_accel: f32,
}

impl Player {
    pub fn hitbox(&self) -> Rect {
        self.hitbox
    }
}

impl EntityT for Player {
    fn new(x: f32, y: f32, e_type: EntityType, spawns: Option<EntityType>) -> Self {
        return Self {
            hitbox: Rect::new(x, y, CONSTS.mario_size.x, CONSTS.mario_size.y),
            current_reference_frame: Rect::new(
                e_type.get_start().x,
                e_type.get_start().y,
                (CONSTS.mario_size.x / CONSTS.settings.scale as f32) * 3f32,
                (CONSTS.mario_size.y / CONSTS.settings.scale as f32) * 3f32,
            ),
            frame_min: 0f32,
            frame_max: 0f32,
            frame_counter: 0f32,
            current_frame: 0f32,
            state: PlayerState::Idle,
            facing: false,
            running: false,
            velocity: Vec2::from_array([0f32, 0f32]),
            falling_accel: 0f32,
        };
    }

    fn update(&mut self, dt: f32, camera: Option<&mut Camera2D>) {
        //TODO: Add Skidding Timer Check

        match (
            is_key_pressed(KeyCode::Left),
            is_key_pressed(KeyCode::Right),
        ) {
            (true, false) => {
                self.facing = true;
            }
            (false, true) => {
                self.facing = false;
            }
            _ => {}
        }

        // Physics scope
        {
            if let PlayerState::JumpFall = self.state {
                //Jumping
                // Air physics

                //Verticle Physics
                if self.velocity.y < 0f32 && is_key_down(KeyCode::Z) {
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

                if is_key_down(KeyCode::Right) && !is_key_down(KeyCode::Left) {
                    if self.velocity.x.abs() > MAX_WALK {
                        self.velocity.x += ACC_RUN * dt;
                    } else {
                        self.velocity.x += ACC_WALK * dt;
                    }
                } else if is_key_down(KeyCode::Left) && !is_key_down(KeyCode::Right) {
                    if self.velocity.x.abs() > MAX_WALK {
                        self.velocity.x -= ACC_RUN * dt;
                    } else {
                        self.velocity.x -= ACC_WALK * dt;
                    }
                } else {
                    //Do nothing
                }
            } else {
                // Not jumping
                //Ground physics
                if self.velocity.x.abs() < MIN_WALK {
                    // slower than a walk // starting, stopping or turning around
                    self.velocity.x = 0f32;
                    self.state = PlayerState::Idle;
                    if is_key_down(KeyCode::Left) {
                        self.velocity.x -= MIN_WALK;
                        self.state = PlayerState::Walking;
                    }
                    if is_key_down(KeyCode::Right) {
                        self.velocity.x += MIN_WALK;
                        self.state = PlayerState::Walking;
                    }
                } else if self.velocity.x.abs() >= MIN_WALK {
                    //Faster than a walk // accelerating or decelarating
                    if self.facing == false {
                        if is_key_down(KeyCode::Right) && !is_key_down(KeyCode::Left) {
                            if is_key_down(KeyCode::X) {
                                self.velocity.x += ACC_RUN * dt;
                                self.state = PlayerState::Running;
                            } else {
                                self.velocity.x += ACC_WALK * dt;
                                self.state = PlayerState::Walking;
                            }
                        } else if is_key_down(KeyCode::Left) && !is_key_down(KeyCode::Right) {
                            self.velocity.x += DEC_SKID * dt;
                            self.state = PlayerState::Skidding;
                        } else {
                            self.velocity.x -= DEC_REL * dt;
                        }
                    }
                    if self.facing == true {
                        if is_key_down(KeyCode::Left) && !is_key_down(KeyCode::Right) {
                            if is_key_down(KeyCode::X) {
                                self.velocity.x -= ACC_RUN * dt;
                                self.state = PlayerState::Running;
                            } else {
                                self.velocity.x -= ACC_WALK * dt;
                                self.state = PlayerState::Walking;
                            }
                        } else if is_key_down(KeyCode::Right) && !is_key_down(KeyCode::Left) {
                            self.velocity.x += DEC_SKID * dt;
                            self.state = PlayerState::Skidding;
                        } else {
                            self.velocity.x += DEC_REL * dt;
                        }
                    }
                }

                self.velocity.y += self.falling_accel * dt;

                if is_key_pressed(KeyCode::Z) {
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
                    self.state = PlayerState::JumpFall;

                    //Play audio
                }
            }

            self.velocity.y += self.falling_accel * dt;

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
            self.hitbox.x += self.velocity.x * dt * (CONSTS.settings.scale as f32);
            self.hitbox.y += self.velocity.y * dt * (CONSTS.settings.scale as f32);
        }

        //Fake collision to prevent going off bottom of screen
        // if self.hitbox.y >= screen_height() - self.hitbox.h {
        //     self.hitbox.y = screen_height() - self.hitbox.h;
        //     if let PlayerState::JumpFall = self.state {
        //         self.state = PlayerState::Idle;
        //     }
        // }

        if self.hitbox.x <= 0f32 {
            self.hitbox.x = 0f32;
        }

        //TODO: make it so that the player can't go back
        if self.hitbox.x >= screen_width() / 2f32 {
            camera.expect("Camera not found").target.x = self.hitbox.x;
        }
    }

    //AABB check against all entities
    fn collision(&mut self, other: &Option<Rect>) {
        //Check if player is colliding with entity
        match other {
            Some(hitbox) => {
                if let Some(intersection) = self.hitbox.intersect(*hitbox) {
                    match intersection.w > intersection.h {
                        true => {
                            match self.hitbox.y > hitbox.y {
                                true => {
                                    if let PlayerState::JumpFall = self.state {
                                        self.state = PlayerState::Idle;
                                    }
                                    self.hitbox.y = hitbox.y - self.hitbox.h;
                                    self.velocity.y = 0f32;
                                    self.falling_accel = 0f32;
                                }
                                false => {
                                    // self.hitbox.y = hitbox.y + hitbox.h;
                                    // self.velocity.y = 0f32;
                                    // self.falling_accel = 0f32;
                                }
                            }
                        }
                        false => {}
                    }
                }
            }
            None => {}
        };
    }

    fn animate(&mut self) {
        match &(self.state) {
            PlayerState::Walking | PlayerState::Running => {
                self.frame_min = 4f32;
                self.frame_max = 5f32;
            }
            PlayerState::Skidding => {
                self.frame_min = 2f32;
                self.frame_max = 3f32;
            }
            PlayerState::JumpFall => {
                self.frame_min = 6f32;
                self.frame_max = 6f32;
            }
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
                self.current_frame = self.frame_min;
            }

            self.current_reference_frame.x =
                self.current_frame * (CONSTS.mario_size.x / CONSTS.settings.scale as f32) * 3f32;
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
                dest_size: Some(CONSTS.mario_size),
                source: Some(self.current_reference_frame),
                flip_x: self.facing,
                ..Default::default()
            },
        );
    }
}
