extern crate time;

use super::super::gsi::{self, message};
use super::super::common::TakesUpdates;
use rustc_serialize::json::{ToJson, Json};
use std::collections::BTreeMap;
use super::equipment;

pub struct State {
    last_up: time::Tm,
    in_game: bool,
    team: Option<message::Team>,
    money: u32,
    gsi: gsi::Versions,
    won_rounds: Vec<bool>,
}

impl State {
    pub fn empty() -> State {
        State {
            last_up: time::at(time::Timespec::new(0, 0)),
            in_game: false,
            team: None,
            money: 0,
            gsi: gsi::Versions::new(),
            won_rounds: vec![]
        }
    }

    fn reset(&mut self) {
        self.in_game = false;
        self.team = None;
        self.money = 0;
        self.won_rounds = vec![];
    }
}

impl TakesUpdates<()> for State {
    fn update(&mut self, _: &()) {
        self.gsi.update();
    }
}

fn tm_from_unix_timestamp(timestamp: u32) -> Result<time::Tm, time::ParseError> {
    let timestamp_as_string = timestamp.to_string();
    time::strptime(&timestamp_as_string, "%s")
}

impl TakesUpdates<message::Message> for State {
    fn update(&mut self, message: &message::Message) {
        self.gsi.update();
        if let Ok(last_up) = tm_from_unix_timestamp(message.provider.timestamp) {
            self.last_up = last_up;
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
            d.insert("team".to_string(), self.team.to_json());
            let recommendations = equipment::Equipment::recommended(self.money, team);
            d.insert("recommendations".to_string(), recommendations.to_json());
        }
        d.insert("won_rounds".to_string(), self.won_rounds.to_json());
        d.insert("gsi".to_string(), self.gsi.to_json());
        Json::Object(d)
    }
}
