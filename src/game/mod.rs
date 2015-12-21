use rustc_serialize::json::{ToJson, Json};
use std::collections::BTreeMap;
use std::sync::{Arc, Mutex};
use super::gsi;

pub enum Equipment {
    Glock,
    P2000,
    USPS,
    P250,
    Deagle,
    Berettas,
    Tec9,
    FiveSeven,
    CZ75,
    R8,
    Nova,
    XM1014,
    SawedOff,
    MAG7,
    MAC10,
    MP9,
    MP7,
    UMP45,
    PPBizon,
    P90,
    GalilAR,
    FAMAS,
    AK47,
    M4A4,
    M4A1S,
    SSG08,
    SG553,
    AUG,
    AWP,
    G3GS1,
    SCAR20,
    M249,
    Negev,
    Vest,
    VestHelmet,
    Zeus,
    Defuse,
    Molotov,
    Incendiary,
    Decoy,
    HENade,
    Flash,
    Smoke,
}

impl ToJson for Equipment {
    fn to_json(&self) -> Json {
        use self::Equipment::*;
        Json::String((match *self {
            Glock => "Glock",
            P2000 => "P2000",
            USPS => "USP-S",
            P250 => "P250",
            Deagle => "Desert Eagle",
            Berettas => "Dual Berettas",
            Tec9 => "Tec-9",
            FiveSeven => "Five-SeveN",
            CZ75 => "CZ75 Auto",
            R8 => "R8 Revolver",
            Nova => "Nova",
            XM1014 => "XM1014",
            SawedOff => "Sawed-Off",
            MAG7 => "MAG-7",
            MAC10 => "MAC-10",
            MP9 => "MP9",
            MP7 => "MP7",
            UMP45 => "UMP-45",
            PPBizon => "PP-Bizon",
            P90 => "P90",
            GalilAR => "Galil AR",
            FAMAS => "FAMAS",
            AK47 => "AK-47",
            M4A4 => "M4A4",
            M4A1S => "M4A1-S",
            SSG08 => "SSG 08",
            SG553 => "SG 553",
            AUG => "AUG",
            AWP => "AWP",
            G3GS1 => "G3GS1",
            SCAR20 => "SCAR-20",
            M249 => "M249",
            Negev => "Negev",
            Vest => "Vest",
            VestHelmet => "Vest + Helmet",
            Zeus => "Zeus x27",
            Defuse => "Defuse kit",
            Molotov => "Molotov",
            Incendiary => "Incendiary",
            Decoy => "Decoy",
            HENade => "HE grenade",
            Flash => "Flashbang",
            Smoke => "Smoke",
        }).to_string())
    }
}

impl Equipment {
    fn cost(&self) -> u32 {
        use self::Equipment::*;
        match *self {
            Glock => 0,
            P2000 => 0,
            USPS => 0,
            P250 => 300,
            Deagle => 700,
            Berettas => 500,
            Tec9 => 500,
            FiveSeven => 500,
            CZ75 => 500,
            R8 => 850,
            Nova => 1200,
            XM1014 => 2000,
            SawedOff => 1200,
            MAG7 => 1800,
            MAC10 => 1050,
            MP9 => 1250,
            MP7 => 1700,
            UMP45 => 1200,
            PPBizon => 1400,
            P90 => 2350,
            GalilAR => 2000,
            FAMAS => 2250,
            AK47 => 2700,
            M4A4 => 3100,
            M4A1S => 3100,
            SSG08 => 1700,
            SG553 => 3000,
            AUG => 3300,
            AWP => 4750,
            G3GS1 => 5000,
            SCAR20 => 5000,
            M249 => 5200,
            Negev => 5700,
            Vest => 650,
            VestHelmet => 1000,
            Zeus => 200,
            Defuse => 400,
            Molotov => 400,
            Incendiary => 600,
            Decoy => 50,
            HENade => 300,
            Flash => 200,
            Smoke => 300,
        }
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

    #[allow(unused_assignments)]
    pub fn recommendations(&self) -> Vec<Equipment> {
        use self::Equipment::*;
        let mut result = vec![];
        let mut remaining_money = self.money;

        macro_rules! buy {
            ( $eqp:expr ) => {
                {
                    result.push($eqp);
                    remaining_money -= $eqp.cost();
                }
            }
        }

        // primary weapon
        if remaining_money > 4700 {
            buy!(AWP);
        } else if remaining_money > 3100 && self.team == "CT" {
            buy!(M4A1S);
        } else if remaining_money > 2700 && self.team == "T" {
            buy!(AK47);
        }

        // pistol
        if remaining_money > 850 {
            buy!(R8);
        } else if self.team == "CT" {
            buy!(USPS);
        } else if self.team == "T" {
            buy!(Glock);
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