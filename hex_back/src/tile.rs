use serde::{Serialize};

#[derive(Serialize)]
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