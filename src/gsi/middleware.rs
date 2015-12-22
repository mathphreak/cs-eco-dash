use nickel::{Middleware, Request, Response, MiddlewareResult};
use std::sync::{Arc, Mutex};
use rustc_serialize::json;
use super::version;
use super::paths;
use super::message;
use super::message::{TakesUpdates, UpdateReason};
use std::fs;
use std::io::Read;

pub struct PostHandler<T> where T: TakesUpdates {
    state_mutex: Arc<Mutex<T>>
}

impl<D, T: 'static> Middleware<D> for PostHandler<T> where T: TakesUpdates {
    fn invoke<'a, 'server>(&'a self, request: &mut Request<'a, 'server, D>, response: Response<'a, D>)
            -> MiddlewareResult<'a, D> {
        let mut body = String::new();
        request.origin.read_to_string(&mut body).unwrap();
        // println!("Got JSON: {}", body);
        let data: message::Message = match json::decode(&body) {
            Ok(data) => data,
            Err(err) => {
                println!("bad JSON: {}", body);
                println!("cause: {}", err);
                Default::default()
            },
        };
        let mut current_player = self.state_mutex.lock().unwrap();
        (*current_player).update(&UpdateReason::Data(data));
        return response.send("Thanks");
    }
}

impl<T> PostHandler<T> where T: TakesUpdates {
    pub fn new(state_mutex: Arc<Mutex<T>>) -> PostHandler<T> {
        PostHandler {
            state_mutex: state_mutex
        }
    }
}

pub struct Installer<T> where T: TakesUpdates {
    state_mutex: Arc<Mutex<T>>
}

impl<D, T: 'static> Middleware<D> for Installer<T> where T: TakesUpdates {
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
        let mut current_player = self.state_mutex.lock().unwrap();
        (*current_player).update(&UpdateReason::Update);
        return response.send("It worked");
    }
}

impl<T> Installer<T> where T: TakesUpdates {
    pub fn new(state_mutex: Arc<Mutex<T>>) -> Installer<T> {
        Installer {
            state_mutex: state_mutex
        }
    }
}
