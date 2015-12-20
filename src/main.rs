#[macro_use] extern crate nickel;
extern crate rustc_serialize;
extern crate time;

mod gsi;

use nickel::{Nickel, HttpRouter};
use std::collections::HashMap;
use nickel::StaticFilesHandler;
use std::sync::{Arc, Mutex};

fn main() {
    let gsi_installed = Arc::new(Mutex::new(gsi::InstalledVersion::new()));
    let gsi_target = Arc::new(Mutex::new(gsi::TargetVersion::new()));
    let mut server = Nickel::new();
    let current_player_state_write = Arc::new(Mutex::new(gsi::State::empty()));
    let current_player_stat_read = current_player_state_write.clone();
    let gsi_post_handler = gsi::PostHandler::new(current_player_state_write);

    server.post("/", gsi_post_handler);

    server.get("/", middleware! { |_, response|
        let mut data = HashMap::new();
        data.insert("dummy", "value");
        return response.render("assets/index.html.hbs", &data)
    });

    server.get("/data.json", middleware! { |_, response|
        let mut data = HashMap::new();
        let current_player_state = current_player_stat_read.lock().unwrap();
        data.insert("money", (*current_player_state).money.to_string());
        data.insert("gsi_installed", gsi_installed.lock().unwrap().get());
        data.insert("gsi_target", gsi_target.lock().unwrap().get());
        return response.render("assets/data.json.hbs", &data)
    });

    server.utilize(StaticFilesHandler::new("assets/vendor/"));
    server.utilize(StaticFilesHandler::new("assets/scripts/"));

    server.listen("127.0.0.1:3000");
}
