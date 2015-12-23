extern crate time;

use super::super::gsi::{self, message};
use super::super::gsi::message::{TakesUpdates, UpdateReason};
use rustc_serialize::json::{ToJson, Json};
use std::collections::BTreeMap;
use super::equipment::Equipment;

pub struct State {
    last_up: time::Tm,
    in_game: bool,
    pub team: Option<message::Team>,
    pub money: u32,
    gsi: gsi::Versions,
    pub won_rounds: Vec<bool>,
    pub inventory: Vec<Equipment>,
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
            inventory: vec![],
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
            match player.state {
                Some(state) => {
                    self.in_game = true;
                    self.money = state.money;
                },
                None => {
                    self.reset();
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
        if let Ok(recs) = Equipment::recommended(self) {
            d.insert("recommendations".to_string(), recs.to_json());
        }
        d.insert("won_rounds".to_string(), self.won_rounds.to_json());
        d.insert("gsi".to_string(), self.gsi.to_json());
        d.insert("gamemode".to_string(), self.gamemode.to_json());
        d.insert("map".to_string(), self.map.to_json());
        Json::Object(d)
    }
}
