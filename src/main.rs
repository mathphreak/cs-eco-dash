#[macro_use] extern crate nickel;
extern crate rustc_serialize;
extern crate time;
extern crate crc;
extern crate cs_eco_dash;

use nickel::{Nickel, HttpRouter};
use nickel::StaticFilesHandler;
use std::sync::{Arc, Mutex};
use rustc_serialize::json::ToJson;
use cs_eco_dash::{game, gsi};
use cs_eco_dash::gsi::message::{TakesUpdates, UpdateReason};
use std::process::Command;

// lifted from https://github.com/euclio/aurelius/blob/master/src/browser.rs,
fn launch_ui() {
    let (browser, args) = if cfg!(target_os = "linux") {
        ("xdg-open", vec![])
    } else if cfg!(target_os = "macos") {
        ("open", vec!["-g"])
    } else if cfg!(target_os = "windows") {
        // originally this was ("start", vec![""])
        // but that caused weird issues, since start is a cmd builtin
        
        // `start` requires an empty string as its first parameter.
        ("cmd", vec!["/c", "start", ""])
    } else {
        println!("What even is your OS");
        return;
    };

    let process = Command::new(browser)
        .args(&args)
        .arg("http://localhost:3000".to_string())
        .status();

    if let Err(e) = process {
        println!("Couldn't launch browser {}: {}", browser, e);
    }
}

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

    launch_ui();

    server.listen("127.0.0.1:3000");
}
