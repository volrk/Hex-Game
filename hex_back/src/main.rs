#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] 
extern crate rocket;
extern crate rocket_cors;

use rocket_contrib::json::Json;
use std::sync::Mutex;
use rocket::State;
use rocket_cors::{
    Cors, CorsOptions
};
use rocket::response::status::BadRequest;

mod game;
mod tile;

struct SharedData { game: Mutex<ToMutex> }

struct ToMutex { game: game::Game }

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/hex/new/<row_number>")]
fn new(shared: State<SharedData>, row_number: u8) -> Json<game::Game> {
    shared.inner().game.lock().unwrap().game = game::Game::new(row_number);
    Json(shared.inner().game.lock().unwrap().game.clone())
}

#[get("/hex/get")]
fn get_current(shared: State<SharedData>) -> Json<game::Game> {
    Json(shared.inner().game.lock().unwrap().game.clone())
}

#[post("/play", data = "<input>")]
fn play(shared: State<SharedData>, input: Json<tile::Tile>) -> Result<Json<game::Game>, BadRequest<String>> {
    let game = shared.inner().game.lock().unwrap().game.clone();
    game::check(&game, & input.0).map_err(|e| BadRequest(Some(e)))?;
    let played_game = game::play(& game, & input.0);
    shared.inner().game.lock().unwrap().game = played_game.clone();
    Ok(Json(played_game.clone())) 
}


fn main() {
    rocket::ignite()
    .manage(SharedData { game: Mutex::new(ToMutex{game : game::Game::new(11u8)}) })
    .mount("/", routes![index, new, get_current, play])
    .attach(make_cors())
    .launch();
}
 
fn make_cors() -> Cors {
    CorsOptions {
        ..Default::default()
    }
    .to_cors()
    .expect("error while building CORS")
}