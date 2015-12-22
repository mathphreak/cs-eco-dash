use rustc_serialize::json::{self, ToJson, Json};
use std::collections::BTreeMap;
use super::gsi;
use super::gsi::message;

mod equipment;

pub struct State {
    pub team: Option<message::Team>,
    pub money: u32,
    pub gsi: gsi::Versions,
    pub won_rounds: Vec<bool>,
}

impl State {
    pub fn empty() -> State {
        State {
            team: None,
            money: 0,
            gsi: gsi::Versions::new(),
            won_rounds: vec![]
        }
    }

    pub fn update(&mut self, message: message::Message) {
        self.gsi.update();
        let player = message.clone().player;
        if let Some(state) = player.clone().state {
            self.money = state.money;
        }
        if let Some(team) = player.clone().team {
            self.team = Some(team);
        }
        if let Some(round) = message.clone().round {
            if round.phase == message::Phase::over {
                if let Some(win_team) = round.win_team {
                    if let Some(ref team) = self.team {
                        self.won_rounds.push(win_team == *team);
                    }
                }
            }
        }
    }
}

impl ToJson for State {
    fn to_json(&self) -> Json {
        let mut d = BTreeMap::new();
        d.insert("money".to_string(), self.money.to_json());
        if let Some(ref team) = self.team {
            d.insert("team".to_string(), json::encode(&team).unwrap().to_json());
            let recommendations = equipment::Equipment::recommended(self.money, team);
            d.insert("recommendations".to_string(), recommendations.to_json());
        }
        d.insert("won_rounds".to_string(), self.won_rounds.to_json());
        d.insert("gsi".to_string(), self.gsi.to_json());
        Json::Object(d)
    }
}
