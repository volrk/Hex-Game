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
fn new(shared: State<Mutex<game::Game>>, row_number: u8) -> Result<Json<game::Game>, Custom<String>> {
    let mut game = shared.inner().lock().map_err(|_| Custom(Status::InternalServerError,"j'arrive pas à lock".to_string()))?;
    *game = game::Game::new(row_number);
    Ok(Json((*game).clone()))
}

#[get("/hex/new/<row_number>/<id>")]
fn new_by_id(shared: State<HashMap<u8, Mutex<game::Game>>>, row_number: u8, id: u8) -> Result<Json<game::Game>, Custom<String>> {
    let o_mu_game = shared.inner().get(&id);
    match o_mu_game {
        Some(mu_game) => {
            let mut game = mu_game.lock().map_err(|_| Custom(Status::InternalServerError,"j'arrive pas à lock".to_string()))?;
            *game = game::Game::new(row_number);
            return Ok(Json((*game).clone()));
        },
        _ => {
            return Err(Custom(Status::NotFound,"Partie non trouvée".to_string()));
        },
    }
}

#[get("/hex/get")]
fn get_current(shared: State<Mutex<game::Game>>) -> Result<Json<game::Game>, Custom<String>> {
    let game = shared.inner().lock().map_err(|_| Custom(Status::InternalServerError,"j'arrive pas à lock".to_string()))?;
    Ok(Json((*game).clone()))
}

#[get("/hex/get/<id>")]
fn get_current_by_id(shared: State<HashMap<u8, Mutex<game::Game>>>, id: u8) -> Result<Json<game::Game>, Custom<String>> {
    let o_mu_game = shared.inner().get(&id);
    match o_mu_game {
        Some(mu_game) => {
            let game = mu_game.lock().map_err(|_| Custom(Status::InternalServerError,"j'arrive pas à lock".to_string()))?;
            return Ok(Json((*game).clone()));
        },
        _ => {
            return Err(Custom(Status::NotFound,"Partie non trouvée".to_string()));
        },
    }
}

#[post("/hex/play", data = "<input>")]
fn play(shared: State<Mutex<game::Game>>, input: Json<tile::Tile>) -> Result<Json<game::Game>, Custom<String>> {
    let mut game = shared.inner().lock().map_err(|_| Custom(Status::InternalServerError,"j'arrive pas à lock".to_string()))?;
    game::check(&game, &input.0).map_err(|e| Custom(Status::BadRequest, e))?;
    *game = game::play(&game, & input.0);
    Ok(Json((*game).clone()))
}

#[post("/hex/play/<id>", data = "<input>")]
fn play_by_id(shared: State<HashMap<u8, Mutex<game::Game>>>, id: u8, input: Json<tile::Tile>) -> Result<Json<game::Game>, Custom<String>> {
    let o_mu_game = shared.inner().get(&id);
    match o_mu_game {
        Some(mu_game) => {
            let mut game = mu_game.lock().map_err(|_| Custom(Status::InternalServerError,"j'arrive pas à lock".to_string()))?;
            game::check(&game, &input.0).map_err(|e| Custom(Status::BadRequest, e))?;
            *game = game::play(&game, & input.0);
            Ok(Json((*game).clone()))
        },
        _ => {
            return Err(Custom(Status::NotFound,"Partie non trouvée".to_string()));
        },
    }
}

fn main() {
    rocket::ignite()
    .manage(Mutex::new( game::Game::new(11u8)))
    .manage(init_state_hashmap())
    .mount("/", routes![index, new, new_by_id, get_current, get_current_by_id, play])
    .attach(make_cors())
    .launch();
}

fn init_state_hashmap() -> HashMap<u8, Mutex<game::Game>>{
    let mut map: HashMap<u8, Mutex<game::Game>> = HashMap::new();
    for i in 0..10 {
        map.insert(i, Mutex::new(game::Game::new(11)));
    }
    map
}
 
fn make_cors() -> Cors {
    CorsOptions {
        ..Default::default()
    }
    .to_cors()
    .expect("error while building CORS")
}