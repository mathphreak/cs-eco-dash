extern crate time;

use super::super::gsi::{self, message};
use super::super::gsi::message::{TakesUpdates, UpdateReason};
use super::super::prefs::Prefs;
use rustc_serialize::json::{ToJson, Json};
use std::collections::BTreeMap;
use super::equipment::Equipment;
use super::inventory::Inventory;
use super::strategy::Strategy;

pub struct State {
    last_up: time::Tm,
    in_game: bool,
    pub team: Option<message::Team>,
    pub money: i32,
    gsi: gsi::Versions,
    pub won_rounds: Vec<bool>,
    pub inventory: Inventory,
    gamemode: message::Mode,
    map: String,
}

impl Default for State {
    fn default() -> State {
        State {
            last_up: time::at(time::Timespec::new(0, 0)),
            in_game: false,
            team: None,
            money: 0,
            gsi: gsi::Versions::new(),
            won_rounds: vec![],
            inventory: Default::default(),
            gamemode: Default::default(),
            map: "".to_string(),
        }
    }
}

impl State {
    fn reset(&mut self) {
        self.in_game = false;
        self.team = None;
        self.money = 0;
        self.won_rounds = vec![];
    }

    pub fn min_next_reward(&self) -> i32 {
        let mut reward = 1400;
        let mut past_rounds = self.won_rounds.clone();
        while let Some(x) = past_rounds.pop() {
            if x || reward >= 3400 {
                break;
            } else {
                reward += 500;
            }
        }
        reward
    }

    fn handle_message(&mut self, message: &message::Message) {
        self.gsi.update();
        if let Ok(last_up) = tm_from_unix_timestamp(message.provider.timestamp) {
            self.last_up = last_up;
        }
        if let Some(ref map) = message.map {
            self.gamemode = map.clone().mode;
            self.map = map.clone().name;
        }
        let ref provider = message.provider;
        let ref player = message.player;
        if provider.steamid == player.steamid {
            self.inventory.clear();
            match player.clone().state {
                Some(state) => {
                    self.in_game = true;
                    self.money = state.money;
                    if state.helmet && state.armor >= 50 {
                        self.inventory.push(Equipment::VestHelmet);
                    } else if state.armor >= 50 {
                        self.inventory.push(Equipment::Vest);
                    }
                },
                None => {
                    self.reset();
                }
            }
            if let Some(weapons) = player.clone().weapons {
                if let Some(weapon_0) = weapons.weapon_0 {
                    self.inventory.push(Equipment::from(weapon_0.name))
                }
                if let Some(weapon_1) = weapons.weapon_1 {
                    self.inventory.push(Equipment::from(weapon_1.name))
                }
                if let Some(weapon_2) = weapons.weapon_2 {
                    self.inventory.push(Equipment::from(weapon_2.name))
                }
                if let Some(weapon_3) = weapons.weapon_3 {
                    self.inventory.push(Equipment::from(weapon_3.name))
                }
                if let Some(weapon_4) = weapons.weapon_4 {
                    self.inventory.push(Equipment::from(weapon_4.name))
                }
                if let Some(weapon_5) = weapons.weapon_5 {
                    self.inventory.push(Equipment::from(weapon_5.name))
                }
                if let Some(weapon_6) = weapons.weapon_6 {
                    self.inventory.push(Equipment::from(weapon_6.name))
                }
                if let Some(weapon_7) = weapons.weapon_7 {
                    self.inventory.push(Equipment::from(weapon_7.name))
                }
            }
            if let Some(team) = player.team {
                self.team = Some(team);
            }
        }
        let added_win_team = message.added;
        let added_win_team = added_win_team.and_then(|x| x.round);
        let added_win_team = added_win_team.and_then(|x| x.win_team);
        let added_win_team = added_win_team.unwrap_or(false);
        if added_win_team {
            if let Some(round) = message.round {
                if let Some(win_team) = round.win_team {
                    if let Some(team) = self.team {
                        self.won_rounds.push(win_team == team);
                    }
                }
            }
        }
    }
}

fn tm_from_unix_timestamp(timestamp: u32) -> Result<time::Tm, time::ParseError> {
    let timestamp_as_string = timestamp.to_string();
    time::strptime(&timestamp_as_string, "%s")
}

impl TakesUpdates for State {
    fn update(&mut self, reason: &UpdateReason) {
        match *reason {
            UpdateReason::Fetch => {
                self.gsi.update();
            },
            UpdateReason::Update => {
                self.gsi.invalidate();
            },
            UpdateReason::Data(ref message) => {
                self.handle_message(message);
            }
        }
    }
}

impl ToJson for State {
    fn to_json(&self) -> Json {
        let mut d = BTreeMap::new();
        let twenty_seconds = time::Duration::seconds(20);
        let twenty_seconds_ago = time::now() - twenty_seconds;
        let is_up = self.last_up > twenty_seconds_ago;
        d.insert("up".to_string(), is_up.to_json());
        d.insert("in_game".to_string(), self.in_game.to_json());
        d.insert("money".to_string(), self.money.to_json());
        if let Some(ref team) = self.team {
            d.insert("team".to_string(), team.to_json());
        }
        if let Ok(recs) = Strategy::recommended(self) {
            d.insert("recommendations".to_string(), recs.to_json());
        }
        d.insert("inventory".to_string(), self.inventory.to_json());
        d.insert("won_rounds".to_string(), self.won_rounds.to_json());
        d.insert("gsi".to_string(), self.gsi.to_json());
        d.insert("gamemode".to_string(), self.gamemode.to_json());
        d.insert("map".to_string(), self.map.to_json());
        if let Ok(prefs) = Prefs::get() {
            d.insert("settings".to_string(), prefs.to_json());
        }
        Json::Object(d)
    }
}
