use rustc_serialize::json::{ToJson, Json};
use std::collections::BTreeMap;
use std::sync::{Arc, Mutex};
use super::gsi;

pub enum Equipment {
    USPS,
    M4,
    AWP,
    AK47,
    Glock,
    R8,
}

impl ToJson for Equipment {
    fn to_json(&self) -> Json {
        Json::String((match self {
            &Equipment::M4 => "M4",
            &Equipment::AWP => "AWP",
            &Equipment::USPS => "USP-S",
            &Equipment::AK47 => "AK-47",
            &Equipment::Glock => "Glock",
            &Equipment::R8 => "R8",
        }).to_string())
    }
}

pub struct State {
    pub team: String,
    pub money: u32,
    pub gsi: gsi::Versions,
    gsi_player: Arc<Mutex<gsi::Player>>
}

impl State {
    pub fn new(gsi_player: Arc<Mutex<gsi::Player>>) -> State {
        State {
            team: "CT".to_string(),
            money: 0,
            gsi: gsi::Versions::new(),
            gsi_player: gsi_player
        }
    }

    pub fn update(&mut self) {
        self.gsi.update();
        let player = self.gsi_player.lock().unwrap();
        self.money = player.clone().state.money;
        self.team = player.clone().team;
    }

    pub fn recommendations(&self) -> Vec<Equipment> {
        let mut result = vec![];
        let mut remaining_money = self.money;

        // primary weapon
        if remaining_money > 4700 {
            result.push(Equipment::AWP);
            remaining_money -= 4700;
        } else if remaining_money > 3100 && self.team == "CT" {
            result.push(Equipment::M4);
            remaining_money -= 3100;
        } else if remaining_money > 2700 && self.team == "T" {
            result.push(Equipment::AK47);
            remaining_money -= 2700;
        }

        // pistol
        if remaining_money > 850 {
            result.push(Equipment::R8);
        } else if self.team == "CT" {
            result.push(Equipment::USPS);
        } else if self.team == "T" {
            result.push(Equipment::Glock);
        }

        result
    }
}

impl ToJson for State {
    fn to_json(&self) -> Json {
        let mut d = BTreeMap::new();
        d.insert("money".to_string(), self.money.to_json());
        d.insert("team".to_string(), self.team.to_json());
        d.insert("recommendations".to_string(), self.recommendations().to_json());
        d.insert("gsi".to_string(), self.gsi.to_json());
        Json::Object(d)
    }
}
