use serde::{Serialize};

use crate::tile::Tile;

#[derive(Serialize, Clone)]
pub struct Game {
    board: Vec<Vec<Option<Tile>>>,
}

impl Game {
    pub fn new(val: u8) -> Game {
        Game{
            board: (0..val).map(|_| (0..val).map(|_| None).collect()).collect(),
        }
    }
}