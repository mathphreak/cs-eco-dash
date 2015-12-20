extern crate nickel;
extern crate time;
extern crate crc;

use nickel::{Middleware, Request, Response, MiddlewareResult};
use rustc_serialize::json;
use std::fs;
use std::sync::{Arc, Mutex};
use std::io::Read;
use crc::crc32;

const CSGO_CFG_PATH: &'static str = "C:\\Program Files (x86)\\Steam\\steamapps\\common\\Counter-Strike Global Offensive\\csgo\\cfg";
const CFG_FILE_PREFIX: &'static str = "gamestate_integration_cs-eco-dash_";

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
                    if name.starts_with(CFG_FILE_PREFIX) {
                        println!("Found a config file: {}", name);
                        let result = name
                            .replace(CFG_FILE_PREFIX, "")
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
            let mut file = fs::File::open("config/gsi.cfg").unwrap();
            let mut buffer = Vec::new();
            file.read_to_end(&mut buffer).unwrap();
            let checksum = crc32::checksum_ieee(buffer.into_boxed_slice().as_ref());
            self.last_result = format!("{:x}", checksum);
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

pub struct Installer {
    #[allow(dead_code)]
    dummy: u8
}

impl<D> Middleware<D> for Installer {
    fn invoke<'a, 'server>(&'a self, _request: &mut Request<'a, 'server, D>, response: Response<'a, D>)
            -> MiddlewareResult<'a, D> {
        let inst = InstalledVersion::new().get();
        let target = TargetVersion::new().get();
        if inst != "NONE" {
            let mut del_path = CSGO_CFG_PATH.to_string().clone();
            del_path.push_str("/");
            del_path.push_str(&CFG_FILE_PREFIX);
            del_path.push_str(&inst);
            del_path.push_str(".cfg");
            println!("Deleting {}", del_path);
            fs::remove_file(del_path).unwrap();
        }
        let src_path = "config/gsi.cfg";
        let mut dst_path = CSGO_CFG_PATH.to_string().clone();
        dst_path.push_str("/");
        dst_path.push_str(&CFG_FILE_PREFIX);
        dst_path.push_str(&target);
        dst_path.push_str(".cfg");
        println!("Copying {} to {}", src_path, dst_path);
        fs::copy(src_path, dst_path).unwrap();
        return response.send("It worked");
    }
}

impl Installer {
    pub fn new() -> Installer {
        Installer {
            dummy: 0
        }
    }
}
