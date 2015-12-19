#[macro_use] extern crate nickel;
extern crate rustc_serialize;

use nickel::{Nickel, HttpRouter};
use std::io::Read;
use rustc_serialize::json;

#[derive(RustcEncodable, RustcDecodable)]
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

#[derive(RustcEncodable, RustcDecodable)]
struct Player {
    state: State
}

#[derive(RustcEncodable, RustcDecodable)]
struct Message {
    player: Player
}

fn main() {
    let mut server = Nickel::new();

    server.post("/", middleware! { |request, response|
        let mut body = String::new();
        request.origin.read_to_string(&mut body).unwrap();
        let data: Message = match json::decode(&body) {
            Ok(num) => num,
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
        println!("You have ${}", data.player.state.money);
        "Thanks"
    });

    server.utilize(router! {
        get "**" => |_req, _res| {
            "Hello world!"
        }
    });

    server.listen("127.0.0.1:3000");
}
