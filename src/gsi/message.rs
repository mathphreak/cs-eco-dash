use rustc_serialize::json::{Json, ToJson};

#[derive(RustcEncodable, RustcDecodable)]
#[derive(Default, Clone, Copy)]
pub struct State {
    armor: u32,
    burning: u32,
    flashed: u32,
    health: u32,
    helmet: bool,
    pub money: u32,
    round_killhs: u32,
    round_kills: u32,
    smoked: u32
}

#[derive(RustcEncodable, RustcDecodable)]
#[derive(Default, Clone)]
pub struct Provider {
    pub steamid: String,
    pub timestamp: u32,
}

#[derive(RustcEncodable, RustcDecodable)]
#[derive(Clone, Copy)]
#[allow(non_camel_case_types)]
pub enum Activity {
    menu,
    playing,
    textinput,
}

impl ToJson for Activity {
    fn to_json(&self) -> Json {
        Json::String((match *self {
            Activity::menu => "menu",
            Activity::playing => "playing",
            Activity::textinput => "textinput",
        }).to_string())
    }
}

#[derive(RustcEncodable, RustcDecodable)]
#[derive(Clone, Copy)]
#[derive(PartialEq)]
#[allow(non_camel_case_types)]
pub enum Team {
    CT,
    T,
}

impl ToJson for Team {
    fn to_json(&self) -> Json {
        Json::String((match *self {
            Team::CT => "CT",
            Team::T => "T",
        }).to_string())
    }
}

#[derive(RustcEncodable, RustcDecodable)]
#[derive(Default, Clone)]
pub struct Player {
    pub steamid: String,
    pub team: Option<Team>,
    pub activity: Option<Activity>,
    pub state: Option<State>,
}

#[derive(RustcEncodable, RustcDecodable)]
#[derive(Clone, Copy)]
#[derive(PartialEq)]
#[allow(non_camel_case_types)]
pub enum Phase {
    over,
    live,
    freezetime,
}

impl Default for Phase {
    fn default() -> Phase {
        Phase::over
    }
}

#[derive(RustcEncodable, RustcDecodable)]
#[derive(Clone, Copy)]
#[derive(PartialEq)]
#[allow(non_camel_case_types)]
pub enum Bomb {
    planted,
    exploded,
    defused,
}

#[derive(RustcEncodable, RustcDecodable)]
#[derive(Default, Clone, Copy)]
pub struct Round {
    pub phase: Phase,
    pub bomb: Option<Bomb>,
    pub win_team: Option<Team>,
}

#[derive(RustcEncodable, RustcDecodable)]
#[derive(Default, Clone)]
pub struct Message {
    pub provider: Provider,
    pub player: Player,
    pub round: Option<Round>,
}
