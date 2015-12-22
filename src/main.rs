#[macro_use] extern crate nickel;
extern crate rustc_serialize;
extern crate time;
extern crate crc;

mod gsi;
mod game;

use nickel::{Nickel, HttpRouter};
use nickel::StaticFilesHandler;
use std::sync::{Arc, Mutex};
use rustc_serialize::json::ToJson;
use self::gsi::message::{TakesUpdates, UpdateReason};

fn main() {
    let mut server = Nickel::new();
    let game_state = Arc::new(Mutex::new(game::State::default()));

    server.utilize(gsi::router(game_state.clone()));

    server.get("/data.json", middleware! { |_, response|
        let mut game_state = game_state.lock().unwrap();
        (*game_state).update(&UpdateReason::Fetch);
        return response.send((*game_state).to_json())
    });

    server.utilize(StaticFilesHandler::new("assets/"));

    server.listen("127.0.0.1:3000");
}
