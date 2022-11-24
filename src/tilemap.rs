use std::fs;

use macroquad::prelude::*;

use crate::block::Block;
use crate::entity::{Entity, EntityT, EntityType};
use crate::mario::Player;
use crate::question::Question;
use crate::{World, CONSTS};

type MapF = Vec<Vec<char>>;

pub struct TileMapController {
    map: MapF,
}

impl TileMapController {
    pub fn new() -> Self {
        Self { map: Vec::new() }
    }

    //Parse file into 2d vector of chars and exclude spaces
    pub fn read_map(&mut self, file_name: &str) {
        let file = fs::read_to_string(file_name).expect("Unable to read file");
        let mut map: MapF = Vec::new();
        for line in file.lines() {
            let mut row: Vec<char> = Vec::new();
            for c in line.chars() {
                if c != ' ' {
                    row.push(c);
                }
            }
            map.push(row);
        }
        self.map = map;
    }

    pub async fn spawn_from_map(&self) -> World {
        let mut world: World = Vec::new();

        for i in 0..self.map.len() {
            for j in 0..self.map[i].len() {
                match self.map[i][j] {
                    'M' => {
                        //Spawn Mario
                        println!("Spawn Mario");
                        let n_mario: Player = Player::new(
                            j as f32 * CONSTS.block_size as f32,
                            i as f32 * CONSTS.block_size as f32,
                            EntityType::Mario,
                            None
                        );

                        let hitbox: Rect = n_mario.hitbox();

                        world.push(Entity::new(
                            Box::new(n_mario),
                            Some(hitbox),
                            EntityType::Mario,
                        ));
                    }
                    'G' => {
                        //Spawn Goomba
                    }
                    'K' => {
                        //Spawn Koopa
                    }
                    'B' => {
                        //Spawn Brick
                        let n_brick: Block = Block::new(
                            j as f32 * CONSTS.block_size as f32,
                            i as f32 * CONSTS.block_size as f32,
                            EntityType::Brick,
                            None
                        );
                        let hitbox: Rect = n_brick.hitbox();
                        world.push(Entity::new(
                            Box::new(n_brick),
                            Some(hitbox),
                            EntityType::Brick,
                        ));
                    }
                    'Q' => {
                        //Spawn Question
                        // println!("Spawn Question");
                        let n_question: Question = Question::new(
                            j as f32 * CONSTS.block_size as f32,
                            i as f32 * CONSTS.block_size as f32,
                            EntityType::Question,
                            None
                        );

                        world.push(Entity::new(
                            Box::new(n_question),
                            None,
                            EntityType::Question,
                        ));
                    }
                    'P' => {
                        //Spawn Pipe
                        // println!("Spawn Pipe");
                    }
                    'F' => {
                        //Spawn Flag
                        // println!("Spawn Flag");
                    }
                    'C' => {
                        //Spawn Coin
                        // println!("Spawn Coin");
                    }
                    'E' => {
                        //Spawn Empty
                        // println!("Spawn Empty");
                    }
                    '_' => {
                        //Fill Rest of row with Blocks
                        // println!("Fill rest of row with blocks");
                        let n_ground: Block = Block::new(
                            j as f32 * CONSTS.block_size as f32,
                            i as f32 * CONSTS.block_size as f32,
                            EntityType::Ground,
                            None,
                        );
                        let hitbox: Rect = n_ground.hitbox();
                        world.push(Entity::new(
                            Box::new(n_ground),
                            Some(hitbox),
                            EntityType::Ground,
                        ));
                    }
                    ' ' => {}
                    _ => {
                        //Spawn Empty
                        // println!("Spawn Empty");
                    }
                }
            }
        }
        return world;
    }
}
