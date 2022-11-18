use std::fs;

pub fn read_map(s: &str) -> String {
    return fs::read_to_string(s).expect("Error reading map");
}

pub fn spawn_from_map() {}