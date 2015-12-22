#[derive(RustcEncodable, RustcDecodable, Clone)]
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

#[derive(RustcEncodable, RustcDecodable, Clone)]
pub struct Provider {
    pub steamid: String
}

impl Provider {
    fn empty() -> Provider {
        Provider{
            steamid: "".to_string()
        }
    }
}

#[derive(RustcEncodable, RustcDecodable, Clone)]
#[allow(non_camel_case_types)]
pub enum Activity {
    menu,
    playing,
    textinput,
}

#[derive(RustcEncodable, RustcDecodable, Clone)]
#[derive(PartialEq)]
#[allow(non_camel_case_types)]
pub enum Team {
    CT,
    T,
}

#[derive(RustcEncodable, RustcDecodable, Clone)]
pub struct Player {
    pub steamid: String,
    pub team: Option<Team>,
    pub activity: Option<Activity>,
    pub state: Option<State>,
}

impl Player {
    pub fn empty() -> Player {
        Player{
            steamid: "".to_string(),
            team: None,
            activity: None,
            state: None
        }
    }
}

#[derive(RustcEncodable, RustcDecodable, Clone)]
#[derive(PartialEq)]
#[allow(non_camel_case_types)]
pub enum Phase {
    over,
    live,
    freezetime,
}

#[derive(RustcEncodable, RustcDecodable, Clone)]
#[derive(PartialEq)]
#[allow(non_camel_case_types)]
pub enum Bomb {
    planted,
    exploded,
    defused,
}

#[derive(RustcEncodable, RustcDecodable, Clone)]
pub struct Round {
    pub phase: Phase,
    pub bomb: Option<Bomb>,
    pub win_team: Option<Team>,
}

#[derive(RustcEncodable, RustcDecodable, Clone)]
pub struct Message {
    pub provider: Provider,
    pub player: Player,
    pub round: Option<Round>,
}

impl Message {
    pub fn empty() -> Message {
        Message{
            provider: Provider::empty(),
            player: Player::empty(),
            round: None,
        }
    }
}
