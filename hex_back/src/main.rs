#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

use rocket_contrib::json::Json;

mod game;
mod tile;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/hex/new/<row_number>")]
fn new(row_number: u8) -> Json<game::Game> {
    Json(game::Game::new(row_number))
}

fn main() {
    rocket::ignite().mount("/", routes![index, new]).launch();
}