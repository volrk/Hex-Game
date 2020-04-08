use serde::{Serialize};

#[derive(Serialize, Clone)]
pub struct Tile {
    playeur: u8,
}

impl Tile {
    pub fn new(val: u8) -> Tile {
        Tile{
            playeur: val,
        }
    }
}