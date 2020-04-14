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

pub fn check(game: & Game, tile: & Tile) -> Result<(), String> {
    if game.player != *tile.player() {
        return Err("C'est pas le bon joueur".to_string());
    }
    let x = *tile.x() as usize;
    let y = *tile.y() as usize;
    if game.board[x][y].is_some() {
        return Err("Position déjà jouée".to_string());
    }
    Ok(())
}

pub fn play(game: & Game, tile: & Tile) -> Game {
    let mut curent_game = game.clone();
    let x = *tile.x() as usize;
    let y = *tile.y() as usize;
    curent_game.board[x][y] = Some(tile.clone());
    curent_game.player = 3 - game.player;
    curent_game
}


#[test]
fn test_change_player() {
    let play_game = play(&Game::new(5), &Tile::new(1, 0, 0));
    assert_eq!(2, play_game.player);
    let play_game = play(&play_game, &Tile::new(1, 0, 0));
    assert_eq!(1, play_game.player);
}

#[test]
fn test_play() {
    let new_game = Game::new(5);
    assert!(!new_game.board[0][0].is_some());
    let play_game = play(&new_game, & Tile::new(1, 0, 0));
    assert!(play_game.board[0][0].is_some());
}

#[test]
fn test_check_curent_player() {
    let mut new_game = Game::new(5);
    assert_eq!(1, new_game.player);
    assert!(check(&new_game, &Tile::new(1, 0, 0)).is_ok());
    assert!(check(&new_game, &Tile::new(2, 0, 0)).is_err());

    new_game.player = 2;
    assert!(check(&new_game, &Tile::new(2, 0, 0)).is_ok());
    assert!(check(&new_game, &Tile::new(1, 0, 0)).is_err());
}

#[test]
fn test_check_tile_played() {
    let mut new_game = Game::new(5);
    assert_eq!(1, new_game.player);
    assert_eq!(1, new_game.player);
    assert!(new_game.board[0][0].is_none());
    assert!(check(&new_game, &Tile::new(1, 0, 0)).is_ok());
    new_game.board[0][0] = Some(Tile::new(1, 0, 0));
    assert!(check(&new_game, &Tile::new(1, 0, 0)).is_err());
}