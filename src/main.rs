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

fn main() {
    let mut server = Nickel::new();
    let current_gsi_state = Arc::new(Mutex::new(gsi::State::empty()));
    let current_player_state = Arc::new(Mutex::new(game::State::new(current_gsi_state.clone())));

    server.utilize(gsi::router(current_gsi_state.clone()));

    server.get("/", middleware! { |_, response|
        return response.send_file("assets/index.html")
    });

    server.get("/data.json", middleware! { |_, response|
        let mut current_player_state = current_player_state.lock().unwrap();
        current_player_state.update();
        return response.send((*current_player_state).to_json())
    });

    server.utilize(StaticFilesHandler::new("assets/vendor/"));
    server.utilize(StaticFilesHandler::new("assets/scripts/"));

    server.listen("127.0.0.1:3000");
}
