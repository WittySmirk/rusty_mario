use std::fs;

use crate::entity::{EntityType, Entity};
use crate::CONSTS;
use crate::mario::Player;
use crate::block::Block;

type Map = Vec<Vec<char>>;

pub struct TileMapController {
    map: Map,
}

impl TileMapController {
    pub fn new() -> Self {
        Self { map: Vec::new() }
    }

    //Parse file into 2d vector of chars and exclude spaces
    pub fn read_map(&mut self, file_name: &str) {
        let file = fs::read_to_string(file_name).expect("Unable to read file");
        let mut map: Map = Vec::new();
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
    
    // pub fn read_map(&mut self, path: &str) {
    //     //TODO Layers
    //     let map = fs::read_to_string(path).expect("Unable to read file");
    //     let map: Map = map.lines().map(|line| line.chars().collect()).collect();
    //     self.map = map;
    // }

    pub fn spawn_from_map(&self, entities: &mut Vec<Box<dyn Entity>>) {
        //Parse map and spawn objects
        for i in 0..self.map.len() {
            for j in 0..self.map[i].len() {
                match self.map[i][j] {
                    'M' => {
                        //Spawn Mario
                        println!("Spawn Mario");
                        entities.push(Box::new(Player::new(j as f32 * CONSTS.block_size as f32, i as f32 * CONSTS.block_size as f32, EntityType::Mario)));
                    }
                    'G' => {
                        //Spawn Goomba
                        println!("Spawn Goomba");
                    }
                    'K' => {
                        //Spawn Koopa
                        println!("Spawn Koopa");
                    }
                    'B' => {
                        //Spawn Brick
                        println!("Spawn Brick");
                        entities.push(Box::new(Block::new(j as f32 * CONSTS.block_size as f32, i as f32 * CONSTS.block_size as f32, EntityType::Brick)));
                    }
                    'Q' => {
                        //Spawn Question
                        println!("Spawn Question");
                    }
                    'P' => {
                        //Spawn Pipe
                        println!("Spawn Pipe");
                    }
                    'F' => {
                        //Spawn Flag
                        println!("Spawn Flag");
                    }
                    'C' => {
                        //Spawn Coin
                        println!("Spawn Coin");
                    }
                    'E' => {
                        //Spawn Empty
                        println!("Spawn Empty");
                    }
                    '_' => {
                        //Fill Rest of row with Blocks
                        println!("Fill rest of row with blocks");
                        entities.push(Box::new(Block::new(j as f32 * CONSTS.block_size as f32, i as f32 * CONSTS.block_size as f32, EntityType::Ground)));
                    }
                    ' ' => {}
                    _ => {
                        //Spawn Empty
                        println!("Spawn Empty");
                    }
                }
            }
        }
    }
}
