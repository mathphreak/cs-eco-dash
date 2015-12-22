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

impl State {
    pub fn empty() -> State {
        State{
            armor: 0,
            burning: 0,
            flashed: 0,
            health: 0,
            helmet: false,
            money: 0,
            round_killhs: 0,
            round_kills: 0,
            smoked: 0
        }
    }
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
pub struct Player {
    pub steamid: String,
    pub team: String,
    pub state: State
}

impl Player {
    pub fn empty() -> Player {
        Player{
            steamid: "".to_string(),
            team: "".to_string(),
            state: State::empty()
        }
    }
}

#[derive(RustcEncodable, RustcDecodable, Clone)]
pub struct Round {
    pub phase: String,
    pub bomb: Option<String>,
    pub win_team: Option<String>,
}

impl Round {
    pub fn empty() -> Round {
        Round {
            phase: "".to_string(),
            bomb: None,
            win_team: None,
        }
    }
}

#[derive(RustcEncodable, RustcDecodable, Clone)]
pub struct Message {
    pub provider: Provider,
    pub player: Player,
    pub round: Round,
}

impl Message {
    pub fn empty() -> Message {
        Message{
            provider: Provider::empty(),
            player: Player::empty(),
            round: Round::empty(),
        }
    }
}
