extern crate nickel;
extern crate time;

use nickel::{Middleware, Request, Response, MiddlewareResult};
use rustc_serialize::json;
use std::fs;
use std::sync::{Arc, Mutex};
use std::io::Read;

static CSGO_CFG_PATH: &'static str = "C:\\Program Files (x86)\\Steam\\steamapps\\common\\Counter-Strike Global Offensive\\csgo\\cfg";

pub struct InstalledVersion {
    last_check: time::Tm,
    last_result: String,
}

impl InstalledVersion {
    pub fn new() -> InstalledVersion {
        InstalledVersion {
            last_check: time::now() - time::Duration::minutes(5),
            last_result: "".to_string()
        }
    }

    pub fn get(&mut self) -> String {
        if (self.last_check + time::Duration::minutes(1)).gt(&time::now()) {
            return self.last_result.clone();
        } else {
            self.last_check = time::now();
            if fs::metadata(CSGO_CFG_PATH).unwrap().is_dir() {
                for entry in fs::read_dir(CSGO_CFG_PATH).unwrap() {
                    let entry = entry.unwrap();
                    let name = entry.path();
                    let name = name.file_name().unwrap();
                    let name = name.to_str().unwrap();
                    if name.starts_with("gamestate_integration_cs-eco-dash_") {
                        println!("Found a config file: {}", name);
                        let result = name
                            .replace("gamestate_integration_cs-eco-dash_", "")
                            .replace(".cfg", "");
                        self.last_result = result;
                        return self.last_result.clone();
                    }
                }
            }
            self.last_result = "NONE".to_string();
            return self.last_result.clone();
        }
    }
}

pub struct TargetVersion {
    last_check: time::Tm,
    last_result: String,
}

impl TargetVersion {
    pub fn new() -> TargetVersion {
        TargetVersion {
            last_check: time::now() - time::Duration::minutes(5),
            last_result: "".to_string()
        }
    }

    pub fn get(&mut self) -> String {
        if (self.last_check + time::Duration::minutes(1)).gt(&time::now()) {
            return self.last_result.clone();
        } else {
            self.last_check = time::now();
            self.last_result = "1".to_string();
            return self.last_result.clone();
        }
    }
}

#[derive(RustcEncodable, RustcDecodable, Copy, Clone)]
pub struct State {
    armor: u32,
    burning: u32,
    flashed: u32,
    health: u32,
    helmet: bool,
    pub money: u32,
    round_killhs: u32,
    round_kills: u32,
    smoked: u32
}

impl State {
    pub fn empty() -> State {
        State{
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

#[derive(RustcEncodable, RustcDecodable, Copy, Clone)]
struct Player {
    state: State
}

#[derive(RustcEncodable, RustcDecodable, Copy, Clone)]
struct Message {
    player: Player
}

pub struct PostHandler {
    state_mutex: Arc<Mutex<State>>
}

impl<D> Middleware<D> for PostHandler {
    fn invoke<'a, 'server>(&'a self, request: &mut Request<'a, 'server, D>, response: Response<'a, D>)
            -> MiddlewareResult<'a, D> {
        let mut body = String::new();
        request.origin.read_to_string(&mut body).unwrap();
        let data: Message = match json::decode(&body) {
            Ok(data) => data,
            Err(_) => {
                println!("got bad JSON: {}", body);
                Message{
                    player: Player{
                        state: State::empty()
                    }
                }
            },
        };
        let mut current_player_state = self.state_mutex.lock().unwrap();
        *current_player_state = data.player.state;
        println!("You have ${}", data.player.state.money);
        return response.send("Thanks");
    }
}

impl PostHandler {
    pub fn new(state_mutex: Arc<Mutex<State>>) -> PostHandler {
        PostHandler {
            state_mutex: state_mutex
        }
    }
}
