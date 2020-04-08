#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] 
extern crate rocket;
extern crate rocket_cors;

use rocket_contrib::json::Json;
use std::sync::Mutex;
use rocket::State;
use rocket::http::Method;
use rocket_cors::{
    AllowedHeaders, AllowedOrigins,
    Cors, CorsOptions
};

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
    .mount("/", routes![index, new, get_current])
    .attach(make_cors())
    .launch();
}
 
fn make_cors() -> Cors {
    let allowed_origins = AllowedOrigins::some_exact(&[
        "http://localhost:3000",
    ]);

    CorsOptions {
        allowed_origins,
        allowed_methods: vec![Method::Get].into_iter().map(From::from).collect(),
        allowed_headers: AllowedHeaders::some(&[
            "Authorization",
            "Accept",
            "Access-Control-Allow-Origin",  
        ]),
        allow_credentials: true,
        ..Default::default()
    }
    .to_cors()
    .expect("error while building CORS")
}