use nickel::{Middleware, Request, Response, MiddlewareResult};
use std::sync::{Arc, Mutex};
use rustc_serialize::json;
use super::version;
use super::paths;
use super::message;
use std::fs;
use std::io::Read;

pub struct PostHandler {
    player_mutex: Arc<Mutex<message::Player>>
}

impl<D> Middleware<D> for PostHandler {
    fn invoke<'a, 'server>(&'a self, request: &mut Request<'a, 'server, D>, response: Response<'a, D>)
            -> MiddlewareResult<'a, D> {
        let mut body = String::new();
        request.origin.read_to_string(&mut body).unwrap();
        println!("Got JSON: {}", body);
        let data: message::Message = match json::decode(&body) {
            Ok(data) => data,
            Err(_) => {
                println!("bad JSON: {}", body);
                message::Message::empty()
            },
        };
        if data.provider.steamid == data.player.steamid {
            let mut current_player = self.player_mutex.lock().unwrap();
            *current_player = data.player;
        } else {
            println!("Ignoring data from wrong player");
        }
        return response.send("Thanks");
    }
}

impl PostHandler {
    pub fn new(player_mutex: Arc<Mutex<message::Player>>) -> PostHandler {
        PostHandler {
            player_mutex: player_mutex
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
