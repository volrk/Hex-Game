#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] 
extern crate rocket;
extern crate rocket_cors;

use rocket_contrib::json::Json;
use std::sync::Mutex;
use std::collections::HashMap;
use rocket::State;
use rocket_cors::{
    Cors, CorsOptions
};
use rocket::http::Status;
use rocket::response::status::Custom;

mod game;
mod tile;

#[get("/")]
fn index() -> &'static str {
    "Hello, HEX!"
}

#[get("/hex/new/<row_number>")]
fn new(s_map: State<HashMap<u8, Mutex<game::Game>>>, s_vec: State<Mutex<Vec<u8>>>, row_number: u8) -> Result<Json<game::Game>, Custom<String>> {
    let mut vec = s_vec.inner().lock().map_err(|_| Custom(Status::InternalServerError,"j'arrive pas à lock".to_string()))?;
    let game_id = vec.remove(0);
    vec.push(game_id);
    match s_map.inner().get(&game_id) {
        Some(mu_game) => {
            let mut game = mu_game.lock().map_err(|_| Custom(Status::InternalServerError,"j'arrive pas à lock".to_string()))?;
            *game = game::Game::new(game_id, row_number);
            return Ok(Json((*game).clone()));
        },
        _ => {
            return Err(Custom(Status::NotFound,"Partie non trouvée".to_string()));
        },
    }
}

#[get("/hex/new/<row_number>/<id>")]
fn new_by_id(s_map: State<HashMap<u8, Mutex<game::Game>>>, s_vec: State<Mutex<Vec<u8>>>, row_number: u8, id: u8) -> Result<Json<game::Game>, Custom<String>> {
    let mut vec = s_vec.inner().lock().map_err(|_| Custom(Status::InternalServerError,"j'arrive pas à lock".to_string()))?;
    match s_map.inner().get(&id) {
        Some(mu_game) => {
            let mut game = mu_game.lock().map_err(|_| Custom(Status::InternalServerError,"j'arrive pas à lock".to_string()))?;
            *game = game::Game::new(id, row_number);
            if let Some(index) = vec.iter().position(|x| *x == id){
                vec.remove(index);
                vec.push(id);
            }
            return Ok(Json((*game).clone()));
        },
        _ => {
            return Err(Custom(Status::NotFound,"Partie non trouvée".to_string()));
        },
    }
}

#[get("/hex/get")]
fn get_current(s_map: State<HashMap<u8, Mutex<game::Game>>>, s_vec: State<Mutex<Vec<u8>>>) -> Result<Json<game::Game>, Custom<String>> {
    get_current_by_id(s_map, s_vec, 0)
}

#[get("/hex/get/<id>")]
fn get_current_by_id(s_map: State<HashMap<u8, Mutex<game::Game>>>, s_vec: State<Mutex<Vec<u8>>>, id: u8) -> Result<Json<game::Game>, Custom<String>> {
    let mut vec = s_vec.inner().lock().map_err(|_| Custom(Status::InternalServerError,"j'arrive pas à lock".to_string()))?;
    match s_map.inner().get(&id) {
        Some(mu_game) => {
            let game = mu_game.lock().map_err(|_| Custom(Status::InternalServerError,"j'arrive pas à lock".to_string()))?;
            if let Some(index) = vec.iter().position(|x| *x == id){
                vec.remove(index);
                vec.push(id);
            }
            return Ok(Json((*game).clone()));
        },
        _ => {
            return Err(Custom(Status::NotFound,"Partie non trouvée".to_string()));
        },
    }
}

#[post("/hex/play", data = "<input>")]
fn play(s_map: State<HashMap<u8, Mutex<game::Game>>>, s_vec: State<Mutex<Vec<u8>>>, input: Json<tile::Tile>) -> Result<Json<game::Game>, Custom<String>> {
    play_by_id(s_map, s_vec, 0, input)
}

#[post("/hex/play/<id>", data = "<input>")]
fn play_by_id(s_map: State<HashMap<u8, Mutex<game::Game>>>, s_vec: State<Mutex<Vec<u8>>>, id: u8, input: Json<tile::Tile>) -> Result<Json<game::Game>, Custom<String>> {
    let mut vec = s_vec.inner().lock().map_err(|_| Custom(Status::InternalServerError,"j'arrive pas à lock".to_string()))?;
    match s_map.inner().get(&id) {
        Some(mu_game) => {
            let mut game = mu_game.lock().map_err(|_| Custom(Status::InternalServerError,"j'arrive pas à lock".to_string()))?;
            game::check(&game, &input.0).map_err(|e| Custom(Status::BadRequest, e))?;
            *game = game::play(&game, & input.0);
            if let Some(index) = vec.iter().position(|x| *x == id){
                vec.remove(index);
                vec.push(id);
            }
            Ok(Json((*game).clone()))
        },
        _ => {
            return Err(Custom(Status::NotFound,"Partie non trouvée".to_string()));
        },
    }
}

fn main() {
    rocket::ignite()
    .manage(init_state_hashmap())
    .manage(init_state_vec())
    .mount("/", routes![index, new, new_by_id, get_current, get_current_by_id, play, play_by_id])
    .attach(make_cors())
    .launch();
}

fn init_state_hashmap() -> HashMap<u8, Mutex<game::Game>>{
    let mut map: HashMap<u8, Mutex<game::Game>> = HashMap::new();
    for i in 0..10 {
        map.insert(i, Mutex::new(game::Game::new(i, 11)));
    }
    map
}

fn init_state_vec() -> Mutex<Vec<u8>>{
    let mut vec : Vec<u8> = Vec::new();
    for i in 0..10 {
        vec.push(i);
    }
    Mutex::new(vec)
}
 
fn make_cors() -> Cors {
    CorsOptions {
        ..Default::default()
    }
    .to_cors()
    .expect("error while building CORS")
}