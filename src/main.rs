#[macro_use] extern crate nickel;
extern crate rustc_serialize;
extern crate time;
extern crate crc;

mod gsi;

use nickel::{Nickel, HttpRouter};
use std::collections::HashMap;
use nickel::StaticFilesHandler;
use std::sync::{Arc, Mutex};

fn main() {
    let gsi_installed = Arc::new(Mutex::new(gsi::InstalledVersion::new()));
    let gsi_target = Arc::new(Mutex::new(gsi::TargetVersion::new()));
    let mut server = Nickel::new();
    let current_player_state = Arc::new(Mutex::new(gsi::State::empty()));
    let gsi_post_handler = gsi::PostHandler::new(current_player_state.clone());
    let gsi_installer = gsi::Installer::new(gsi_installed.clone());

    server.post("/", gsi_post_handler);
    server.post("/install-gsi", gsi_installer);

    server.get("/", middleware! { |_, response|
        return response.send_file("assets/index.html")
    });

    server.get("/data.json", middleware! { |_, response|
        let mut data = HashMap::new();
        let current_player_state = current_player_state.lock().unwrap();
        data.insert("money", (*current_player_state).money.to_string());
        data.insert("gsi_installed", gsi_installed.lock().unwrap().get());
        data.insert("gsi_target", gsi_target.lock().unwrap().get());
        return response.render("assets/data.json.hbs", &data)
    });

    server.utilize(StaticFilesHandler::new("assets/vendor/"));
    server.utilize(StaticFilesHandler::new("assets/scripts/"));

    server.listen("127.0.0.1:3000");
}
