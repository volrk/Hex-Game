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
use rocket::http::Status;
use rocket::response::status::Custom;

mod game;
mod tile;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/hex/new/<row_number>")]
fn new(shared: State<Mutex<game::Game>>, row_number: u8) -> Result<Json<game::Game>, Custom<String>> {
    let mut game = shared.inner().lock().map_err(|_| Custom(Status::InternalServerError,"j'arrive pas à lock".to_string()))?;
    *game = game::Game::new(row_number);
    Ok(Json((*game).clone()))
}

#[get("/hex/get")]
fn get_current(shared: State<Mutex<game::Game>>) -> Result<Json<game::Game>, Custom<String>> {
    let game = shared.inner().lock().map_err(|_| Custom(Status::InternalServerError,"j'arrive pas à lock".to_string()))?;
    Ok(Json((*game).clone()))
}

#[post("/play", data = "<input>")]
fn play(shared: State<Mutex<game::Game>>, input: Json<tile::Tile>) -> Result<Json<game::Game>, Custom<String>> {
    let mut game = shared.inner().lock().map_err(|_| Custom(Status::InternalServerError,"j'arrive pas à lock".to_string()))?;
    game::check(&game, &input.0).map_err(|e| Custom(Status::BadRequest, e))?;
    *game = game::play(&game, & input.0);
    Ok(Json((*game).clone()))
}

fn main() {
    rocket::ignite()
    .manage(Mutex::new( game::Game::new(11u8)))
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