#[macro_use] extern crate nickel;
extern crate rustc_serialize;

use nickel::{Nickel, HttpRouter};
use std::io::Read;
use std::collections::HashMap;
use rustc_serialize::json;
use nickel::StaticFilesHandler;
use std::sync::{Arc, Mutex};

#[derive(RustcEncodable, RustcDecodable, Copy, Clone)]
struct State {
    armor: u32,
    burning: u32,
    flashed: u32,
    health: u32,
    helmet: bool,
    money: u32,
    round_killhs: u32,
    round_kills: u32,
    smoked: u32
}

#[derive(RustcEncodable, RustcDecodable, Copy, Clone)]
struct Player {
    state: State
}

#[derive(RustcEncodable, RustcDecodable, Copy, Clone)]
struct Message {
    player: Player
}

fn main() {
    let mut server = Nickel::new();
    let current_player_state = Arc::new(Mutex::new(State{
        armor: 0,
        burning: 0,
        flashed: 0,
        health: 0,
        helmet: false,
        money: 0,
        round_killhs: 0,
        round_kills: 0,
        smoked: 0
    }));
    let current_player_state2 = current_player_state.clone();

    server.post("/", middleware! { |request, response|
        let mut body = String::new();
        request.origin.read_to_string(&mut body).unwrap();
        let data: Message = match json::decode(&body) {
            Ok(data) => data,
            Err(_) => {
                println!("got bad JSON: {}", body);
                Message{
                    player: Player{
                        state: State{
                            armor: 0,
                            burning: 0,
                            flashed: 0,
                            health: 0,
                            helmet: false,
                            money: 0,
                            round_killhs: 0,
                            round_kills: 0,
                            smoked: 0
                        }
                    }
                }
            },
        };
        let mut current_player_state = current_player_state.lock().unwrap();
        *current_player_state = data.player.state;
        println!("You have ${}", data.player.state.money);
        "Thanks"
    });

    server.get("/", middleware! { |_, response|
        let mut data = HashMap::new();
        let current_player_state = current_player_state2.lock().unwrap();
        data.insert("money", (*current_player_state).money);
        return response.render("assets/index.html.hbs", &data)
    });

    server.utilize(StaticFilesHandler::new("assets/vendor/"));

    server.utilize(router! {
        get "**" => |_req, _res| {
            "Hello world!"
        }
    });

    server.listen("127.0.0.1:3000");
}
