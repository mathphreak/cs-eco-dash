
use nickel::{Middleware, Request, Response, MiddlewareResult};
use std::sync::{Arc, Mutex};
use rustc_serialize::json;
use super::version;
use super::paths;
use std::fs;
use std::io::Read;

#[derive(RustcEncodable, RustcDecodable)]
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

#[derive(RustcEncodable, RustcDecodable)]
struct Provider {
    steamid: String
}

impl Provider {
    fn empty() -> Provider {
        Provider{
            steamid: "".to_string()
        }
    }
}

#[derive(RustcEncodable, RustcDecodable)]
struct Player {
    steamid: String,
    state: State
}

impl Player {
    fn empty() -> Player {
        Player{
            steamid: "".to_string(),
            state: State::empty()
        }
    }
}

#[derive(RustcEncodable, RustcDecodable)]
struct Message {
    provider: Provider,
    player: Player
}

impl Message {
    fn empty() -> Message {
        Message{
            provider: Provider::empty(),
            player: Player::empty()
        }
    }
}

pub struct PostHandler {
    state_mutex: Arc<Mutex<State>>
}

impl<D> Middleware<D> for PostHandler {
    fn invoke<'a, 'server>(&'a self, request: &mut Request<'a, 'server, D>, response: Response<'a, D>)
            -> MiddlewareResult<'a, D> {
        let mut body = String::new();
        request.origin.read_to_string(&mut body).unwrap();
        println!("Got JSON: {}", body);
        let data: Message = match json::decode(&body) {
            Ok(data) => data,
            Err(_) => {
                println!("bad JSON: {}", body);
                Message::empty()
            },
        };
        if data.provider.steamid == data.player.steamid {
            let mut current_player_state = self.state_mutex.lock().unwrap();
            *current_player_state = data.player.state;
        } else {
            println!("Ignoring data from wrong player");
        }
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

pub struct Installer;

impl<D> Middleware<D> for Installer {
    fn invoke<'a, 'server>(&'a self, _request: &mut Request<'a, 'server, D>, response: Response<'a, D>)
            -> MiddlewareResult<'a, D> {
        let inst = version::Installed::new().get();
        let target = version::Target::new().get();
        if inst != "NONE" {
            let mut del_path = paths::CSGO_CFG.to_string().clone();
            del_path.push_str("/");
            del_path.push_str(&paths::CFG_PREFIX);
            del_path.push_str(&inst);
            del_path.push_str(".cfg");
            println!("Deleting {}", del_path);
            fs::remove_file(del_path).unwrap();
        }
        let src_path = "config/gsi.cfg";
        let mut dst_path = paths::CSGO_CFG.to_string().clone();
        dst_path.push_str("/");
        dst_path.push_str(&paths::CFG_PREFIX);
        dst_path.push_str(&target);
        dst_path.push_str(".cfg");
        println!("Copying {} to {}", src_path, dst_path);
        fs::copy(src_path, dst_path).unwrap();
        return response.send("It worked");
    }
}

impl Installer {
    pub fn new() -> Installer {
        Installer
    }
}
