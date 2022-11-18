use std::fs;

type Map = Vec<Vec<char>>;

pub struct TileMapController {
    map: Map,
}

enum Entities {
    Mario,
    Goomba,
    Koopa,
    Brick,
    Question,
    Pipe,
    Flag,
    Coin,
    Empty,
}

impl TileMapController {
    pub fn new() -> Self {
        Self {
            map: Vec::new(),
        }
    }

    pub fn read_map(&mut self, path: &str) {
        let map = fs::read_to_string(path).expect("Unable to read file");
        let map: Map = map
            .lines()
            .map(|line| line.chars().collect())
            .collect();
        self.map = map;
    }

    pub fn spawn_from_map(&self) {
        //Parse map and spawn objects
        for i in 0..self.map.len() {
            for j in 0..self.map[i].len() {
                match self.map[i][j] {
                    'M' => {
                        //Spawn Mario
                        println!("Spawn Mario");
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
                    _ => {
                        //Spawn Empty
                        println!("Spawn Empty");
                    }
                }
            }
        }
    }
}