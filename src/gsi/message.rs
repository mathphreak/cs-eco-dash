use rustc_serialize::json::{Json, ToJson};

pub enum UpdateReason {
    Fetch,
    Update,
    Data(Message),
}

pub trait TakesUpdates : Send {
    fn update(&mut self, data: &UpdateReason);
}

#[derive(RustcEncodable, RustcDecodable)]
#[derive(Default, Clone, Copy)]
pub struct State {
    pub armor: u8,
    burning: u8,
    flashed: u8,
    health: u8,
    pub helmet: bool,
    pub money: i32,
    round_killhs: u8,
    round_kills: u8,
    smoked: u8
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

mod round {
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
        pub win_team: Option<super::Team>,
    }
}

#[derive(RustcEncodable, RustcDecodable)]
#[derive(Clone, Copy)]
#[derive(PartialEq)]
#[allow(non_camel_case_types)]
pub enum Mode {
    casual,
    competitive,
    gungametrbomb,
    gungameprogressive,
    deathmatch,
    custom,
}

impl Default for Mode {
    fn default() -> Mode {
        Mode::custom
    }
}

impl ToJson for Mode {
    fn to_json(&self) -> Json {
        use self::Mode::*;
        Json::String((match *self {
            casual => "Casual",
            competitive => "Competitive",
            gungametrbomb => "Demolition",
            gungameprogressive => "Arms Race",
            deathmatch => "Deathmatch",
            custom => "Custom",
        }).to_string())
    }
}

mod map {
    #[derive(RustcEncodable, RustcDecodable)]
    #[derive(Clone, Copy)]
    #[derive(PartialEq)]
    #[allow(non_camel_case_types)]
    pub enum Phase {
        live,
        gameover,
        warmup,
        intermission,
    }

    impl Default for Phase {
        fn default() -> Phase {
            Phase::gameover
        }
    }

    #[derive(RustcEncodable, RustcDecodable)]
    #[derive(Default, Clone)]
    pub struct Map {
        pub mode: super::Mode,
        pub name: String,
        pub phase: Phase,
    }
}

mod added {
    #[derive(RustcEncodable, RustcDecodable)]
    #[derive(Default, Clone, Copy)]
    pub struct Round {
        pub win_team: Option<bool>,
    }

    #[derive(RustcEncodable, RustcDecodable)]
    #[derive(Default, Clone, Copy)]
    pub struct Added {
        pub round: Option<Round>,
    }
}

mod previous {
    #[derive(RustcEncodable, RustcDecodable)]
    #[derive(Default, Clone, Copy)]
    pub struct State {
        pub armor: Option<u8>,
        burning: Option<u8>,
        flashed: Option<u8>,
        health: Option<u8>,
        pub helmet: Option<bool>,
        pub money: Option<i32>,
        round_killhs: Option<u8>,
        round_kills: Option<u8>,
        smoked: Option<u8>
    }

    #[derive(RustcEncodable, RustcDecodable)]
    #[derive(Default, Clone)]
    pub struct Provider {
        pub steamid: Option<String>,
        pub timestamp: Option<u32>,
    }

    #[derive(RustcEncodable, RustcDecodable)]
    #[derive(Clone, Copy)]
    #[allow(non_camel_case_types)]
    pub enum Activity {
        menu,
        playing,
        textinput,
    }

    #[derive(RustcEncodable, RustcDecodable)]
    #[derive(Clone, Copy)]
    #[derive(PartialEq)]
    #[allow(non_camel_case_types)]
    pub enum Team {
        CT,
        T,
    }

    #[derive(RustcEncodable, RustcDecodable)]
    #[derive(Default, Clone)]
    pub struct Player {
        pub steamid: Option<String>,
        pub team: Option<Team>,
        pub activity: Option<Activity>,
        pub state: Option<State>,
    }

    #[derive(RustcEncodable, RustcDecodable)]
    #[derive(Default, Clone)]
    pub struct Message {
        pub provider: Option<Provider>,
        pub player: Option<Player>,
    }
}

#[derive(RustcEncodable, RustcDecodable)]
#[derive(Default, Clone)]
pub struct Message {
    pub provider: Provider,
    pub map: Option<map::Map>,
    pub player: Player,
    pub round: Option<round::Round>,
    pub added: Option<added::Added>,
    pub previously: Option<previous::Message>,
}
