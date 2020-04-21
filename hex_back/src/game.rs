use serde::{Serialize};
use std::collections::HashMap;
use crate::tile::Tile;

#[derive(Serialize, Clone, Debug)]
pub struct Game {
    board: Vec<Vec<Option<Tile>>>,
    player: u8,
    winner: Option<u8>,
}

impl Game {
    pub fn new(val: u8) -> Game {
        Game{
            board: (0..val).map(|_| (0..val).map(|_| None).collect()).collect(),
            player: 1,
            winner: None,
        }
    }
}

pub fn check(game: & Game, tile: & Tile) -> Result<(), String> {
    if game.winner.is_some() {
        return Err("Il y a déjà un vainqueur".to_string());
    }
    if game.player != *tile.player() {
        return Err("C'est pas le bon joueur".to_string());
    }
    let x = *tile.x() as usize;
    let y = *tile.y() as usize;
    if game.board.len() <= x || game.board[x].len() <= y {
        return Err(format!("la position ({}, {}) n'est pas jouable", x, y));
    }
    if game.board[x][y].is_some() {
        return Err("Position déjà jouée".to_string());
    }

    Ok(())
}

pub fn play(game: &Game, tile: &Tile) -> Game {
    let mut curent_game = game.clone();
    let x = *tile.x() as usize;
    let y = *tile.y() as usize;
    curent_game.board[x][y] = Some(tile.clone());
    if is_winner(&curent_game, game.player){
        curent_game.winner = Some(game.player);
    }
    curent_game.player = 3 - game.player;
    curent_game
}

fn is_winner(game: &Game, playeur: u8) -> bool {
    let mut map: HashMap<(u8,u8), ()> = HashMap::new();
    let first_vex: Vec<&Tile>;
    if playeur == 1 {
        first_vex = get_firsts_tiles_player_1(game);
    } else {
        first_vex = get_firsts_tiles_player_2(game);
    }
    for tile in first_vex {
        if map.get(&(*tile.x(), *tile.y())).is_none(){
            map.insert((*tile.x(), *tile.y()), ());
            let val = check_other_tile(&mut map, game, &tile);
            if val {return val}
        }
    }
    false
}

fn get_firsts_tiles_player_1(game: & Game) -> Vec<& Tile>{
    let mut list: Vec<&Tile> = Vec::new();
    for x in 0..(game.board.len()) {
        match &game.board[x][0]{
            Some(tile) if *tile.player() == 1  => {list.push(tile)},
            _ => {},
        }
    }
    list
}

fn get_firsts_tiles_player_2(game: & Game) -> Vec<& Tile>{
    let mut list: Vec<&Tile> = Vec::new();
    for o_tile in &(game.board[0]) {
        match o_tile {
            Some(tile) if *tile.player() == 2 => {list.push(tile)}
            _ => {}
        }
    }
    list
}

fn check_other_tile(map: &mut HashMap<(u8,u8), ()>, game: & Game, tile: & Tile) -> bool {
    let list = get_tile_around(game, tile, map);
    for next_tile in list {
        if *next_tile.player() == 1 && (next_tile.y() + 1) as usize == game.board[*next_tile.x() as usize].len() {return true};
        if *next_tile.player() == 2 && (next_tile.x() + 1) as usize == game.board.len() {return true};
        map.insert((*next_tile.x(), *next_tile.y()), ());
        check_other_tile(map, game, next_tile);
    }
    false
}

fn get_tile_around<'a>(game: &'a Game, tile: &Tile, map: &HashMap<(u8,u8), ()>) -> Vec<&'a Tile>{
    let mut coordonate_list: Vec<(u8, u8)> = Vec::new();
    let (x,y) = (tile.x(), tile.y());
    if *y as i8 - 1 >= 0 {coordonate_list.push((*x, y - 1))};
    if *x as i8 - 1 >= 0 {coordonate_list.push((x - 1, *y))};
    if *y as i8 - 1 >= 0 && x + 1 < game.board.len() as u8  {coordonate_list.push((x + 1, y - 1))};
    if x + 1 < game.board.len() as u8 {coordonate_list.push((x + 1, *y))};
    if y + 1 < game.board[*x as usize].len() as u8 {coordonate_list.push((*x, y + 1))};
    if *x as i8 - 1 >= 0 && y + 1 < game.board[*x as usize].len() as u8 {coordonate_list.push((x - 1, y + 1))};

    let mut list: Vec<&Tile> = Vec::new();
    for (x, y) in coordonate_list {
        match &(game.board[x as usize][y as usize]) {
            Some(new_tile) if *new_tile.player() == *tile.player() && map.get(&(x, y)).is_none() => list.push(new_tile),
            _ => {},
        }
    }
    list
}

#[test]
fn test_is_winner() {
    let mut game = Game::new(2);
    game.board[0][0] = Some(Tile::new(1, 0, 0));
    game.board[0][1] = Some(Tile::new(1, 0, 1));
    // 1 0
    // 1 0
    assert!(is_winner(&game, 1));

    game.board[0][0] = Some(Tile::new(2, 0, 0));
    game.board[1][0] = Some(Tile::new(2, 1, 0));
    // 2 2
    // 1 0
    assert!(is_winner(&game, 2));

}

#[test]
fn test_get_firsts_tiles_player_1() {
    let mut game = Game::new(5);
    game.board[0][0] = Some(Tile::new(1, 0, 0));
    game.board[1][0] = Some(Tile::new(1, 1, 0));
    game.board[3][0] = Some(Tile::new(2, 3, 0));
    game.board[4][0] = Some(Tile::new(1, 4, 0));
    assert_eq!(get_firsts_tiles_player_1(&game).len(), 3);
}

#[test]
fn test_get_firsts_tiles_player_2() {
    let mut game = Game::new(5);
    game.board[0][0] = Some(Tile::new(2, 0, 0));
    game.board[0][1] = Some(Tile::new(2, 0, 1));
    game.board[0][3] = Some(Tile::new(1, 0, 3));
    game.board[0][4] = Some(Tile::new(2, 0, 4));
    assert_eq!(get_firsts_tiles_player_2(&game).len(), 3);
}

#[test]
fn test_visit_other_tile() {
    let mut game = Game::new(2);
    game.board[0][0] = Some(Tile::new(1, 0, 0));
    game.board[1][0] = Some(Tile::new(1, 1, 0));
    // 1 1
    // 0 0
    assert!(!check_other_tile(&mut HashMap::new(), &game, &Tile::new(1, 0, 0)));

    game.board[0][1] = Some(Tile::new(1, 0, 1));
    // 1 1
    // 1 0
    assert!(check_other_tile(&mut HashMap::new(), &game, &Tile::new(1, 0, 0)));

    game.board[0][1] = Some(Tile::new(2, 0, 1));
    // 1 1
    // 2 0
    assert!(!check_other_tile(&mut HashMap::new(), &game, &Tile::new(1, 0, 0)));
    assert!(!check_other_tile(&mut HashMap::new(), &game, &Tile::new(2, 0, 1)));

    game.board[1][0] = Some(Tile::new(2, 1, 0));
    // 1 2
    // 2 0
    assert!(check_other_tile(&mut HashMap::new(), &game, &Tile::new(2, 0, 1)));
}

#[test]
fn test_get_tile_around() {
    let mut game = Game::new(5);
    game.board[2][1] = Some(Tile::new(1, 2, 1));
    game.board[3][1] = Some(Tile::new(1, 3, 1));
    game.board[1][2] = Some(Tile::new(1, 1, 2));
    game.board[3][2] = Some(Tile::new(1, 3, 2));
    game.board[1][3] = Some(Tile::new(1, 1, 3));
    game.board[2][3] = Some(Tile::new(1, 2, 3));
    let result = get_tile_around(&game, &Tile::new(1, 2, 2), &HashMap::new());
    assert_eq!(result.len(), 6);
    let result = get_tile_around(&game, &Tile::new(2, 2, 2), &HashMap::new());
    assert_eq!(result.len(), 0);

    let mut map: HashMap<(u8,u8), ()> = HashMap::new();
    map.insert((2,1), ());
    let result = get_tile_around(&game, &Tile::new(1, 2, 2), &map);
    assert_eq!(result.len(), 5);

    game.board[1][3] = None;
    let result = get_tile_around(&game, &Tile::new(1, 2, 2), &map);
    assert_eq!(result.len(), 4);

    let result = get_tile_around(&game, &Tile::new(1, 0, 0), &HashMap::new());
    assert_eq!(result.len(), 0);

    let result = get_tile_around(&game, &Tile::new(1, 4, 4), &HashMap::new());
    assert_eq!(result.len(), 0);

    let mut game = Game::new(2);
    game.board[0][1] = Some(Tile::new(2, 0, 1));
    game.board[1][0] = Some(Tile::new(2, 1, 0));
    let result = get_tile_around(&game, &Tile::new(2, 0, 1), &HashMap::new());
    assert_eq!(result.len(), 1);
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
    assert!(new_game.board[0][0].is_none());
    assert!(check(&new_game, &Tile::new(1, 0, 0)).is_ok());
    new_game.board[0][0] = Some(Tile::new(1, 0, 0));
    assert!(check(&new_game, &Tile::new(1, 0, 0)).is_err());
}

#[test]
fn test_play_in_board() {
    let new_game = Game::new(5);
    assert!(check(&new_game, &Tile::new(1, 0, 0)).is_ok());
    assert!(check(&new_game, &Tile::new(1, 5, 0)).is_err());
    assert!(check(&new_game, &Tile::new(1, 0, 5)).is_err());
}