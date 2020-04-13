use serde::{Serialize};

use crate::tile::Tile;

#[derive(Serialize, Clone)]
pub struct Game {
    board: Vec<Vec<Option<Tile>>>,
    player: u8,
}

impl Game {
    pub fn new(val: u8) -> Game {
        Game{
            board: (0..val).map(|_| (0..val).map(|_| None).collect()).collect(),
            player: 1,
        }
    }
}

pub fn play(mut game: Game, tile: Tile) -> Game {
    let x = *tile.x() as usize;
    let y = *tile.y() as usize;
    game.board[x][y] = Some(tile);
    game.player = 3 - game.player;
    game
}


#[test]
fn test_change_player() {
    let play_game = play(Game::new(5), Tile::new(1, 0, 0));
    assert_eq!(2, play_game.player);
    let play_game = play(play_game, Tile::new(1, 0, 0));
    assert_eq!(1, play_game.player);
}

#[test]
fn test_play() {
    let play_game = play(Game::new(5), Tile::new(1, 0, 0));
    assert!(play_game.board[0][0].is_some());
}