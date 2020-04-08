#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] 
extern crate rocket;

use rocket_contrib::json::Json;
use std::sync::Mutex;
use rocket::State;

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

fn main() {
    rocket::ignite()
    .manage(SharedData { game: Mutex::new(ToMutex{game : game::Game::new(11u8)}) })
    .mount("/", routes![index, new, get_current]).launch();
}