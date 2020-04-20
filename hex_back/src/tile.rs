use serde::{Serialize,Deserialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Tile {
    player: u8,
    x: u8,
    y: u8,
}

impl Tile {
    pub fn new(player: u8, x: u8, y: u8) -> Tile{
        Tile{
            player,x,y,
        }
    } 
    pub fn player(&self) -> &u8 {
        &self.player
    }

    pub fn x(&self) -> &u8 {
        &self.x
    }

    pub fn y(&self) -> &u8 {
        &self.y
    }
}