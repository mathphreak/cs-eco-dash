use rustc_serialize::json::{ToJson, Json};
use std::collections::BTreeMap;
use std::sync::{Arc, Mutex};
use super::gsi;

pub enum Equipment {
    USPS,
    M4,
    AWP,
}

pub struct State {
    pub money: u32,
    pub gsi: gsi::Versions,
    gsi_state: Arc<Mutex<gsi::State>>
}

impl State {
    pub fn new(gsi_state: Arc<Mutex<gsi::State>>) -> State {
        State {
            money: 0,
            gsi: gsi::Versions::new(),
            gsi_state: gsi_state
        }
    }

    pub fn update(&mut self) {
        self.gsi.update();
        self.money = self.gsi_state.lock().unwrap().money;
    }

    pub fn recommendations(&self) -> Vec<Equipment> {
        if self.money > 4700 {
            vec![Equipment::AWP, Equipment::USPS]
        } else if self.money > 3100 {
            vec![Equipment::M4, Equipment::USPS]
        } else {
            vec![Equipment::USPS]
        }
    }
}

impl ToJson for State {
    fn to_json(&self) -> Json {
        let mut d = BTreeMap::new();
        d.insert("money".to_string(), self.money.to_json());
        let mut recs = vec![];
        for eqp in self.recommendations() {
            recs.push((match eqp {
                Equipment::M4 => "M4",
                Equipment::AWP => "AWP",
                Equipment::USPS => "USP-S"
            }).to_string())
        }
        d.insert("recommendations".to_string(), recs.to_json());
        d.insert("gsi".to_string(), self.gsi.to_json());
        Json::Object(d)
    }
}
