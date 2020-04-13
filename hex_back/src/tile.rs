use serde::{Serialize,Deserialize};

#[derive(Serialize, Deserialize , FromForm, Clone)]
pub struct Tile {
    playeur: u8,
    x: u8,
    y: u8,
}

impl Tile {
    pub fn playeur(&self) -> &u8 {
        &self.playeur
    }

    pub fn x(&self) -> &u8 {
        &self.x
    }

    pub fn y(&self) -> &u8 {
        &self.y
    }
}