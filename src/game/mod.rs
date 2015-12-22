use rustc_serialize::json::{ToJson, Json};
use std::collections::BTreeMap;
use super::gsi;

mod equipment;

pub struct State {
    pub team: String,
    pub money: u32,
    pub gsi: gsi::Versions,
    pub won_rounds: Vec<bool>,
}

impl State {
    pub fn empty() -> State {
        State {
            team: "CT".to_string(),
            money: 0,
            gsi: gsi::Versions::new(),
            won_rounds: vec![]
        }
    }

    pub fn update(&mut self, message: gsi::Message) {
        self.gsi.update();
        let player = message.clone().player;
        self.money = player.clone().state.money;
        self.team = player.clone().team;
        if message.clone().round.phase == "over" {
            if let Some(win_team) = message.clone().round.win_team {
                self.won_rounds.push(win_team == self.team);
            }
        }
    }
}

impl ToJson for State {
    fn to_json(&self) -> Json {
        let mut d = BTreeMap::new();
        d.insert("money".to_string(), self.money.to_json());
        d.insert("team".to_string(), self.team.to_json());
        let recommendations = equipment::Equipment::recommended(self.money, &self.team);
        d.insert("recommendations".to_string(), recommendations.to_json());
        d.insert("won_rounds".to_string(), self.won_rounds.to_json());
        d.insert("gsi".to_string(), self.gsi.to_json());
        Json::Object(d)
    }
}
