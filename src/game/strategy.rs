use super::equipment::Equipment;
use rustc_serialize::json::{ToJson, Json};
use super::super::gsi::message;
use super::State;
use std::error::Error;
use std::fmt;
use std::convert;

#[derive(Debug, Default)]
pub struct NoTeamError;

impl fmt::Display for NoTeamError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}

impl Error for NoTeamError {
    fn description(&self) -> &str {
        "No team provided"
    }
}

impl<T> convert::From<Result<T, NoTeamError>> for NoTeamError {
    fn from(_: Result<T, NoTeamError>) -> Self {
        NoTeamError
    }
}

#[derive(Debug)]
#[derive(PartialEq)]
#[derive(Clone, Copy)]
pub enum Recommendation {
    Strong(Equipment),
    Weak(Equipment),
}

impl ToJson for Recommendation {
    fn to_json(&self) -> Json {
        use self::Recommendation::*;
        match *self {
            Strong(e) => e.to_json(),
            Weak(e) => {
                let orig = e.to_json();
                Json::String(match orig.as_string() {
                    Some(x) => x.to_string() + "?",
                    None => "?".to_string(),
                })
            }
        }
    }
}

#[derive(Debug)]
#[derive(PartialEq)]
#[derive(Clone, Copy)]
pub enum Strategy {
    FullBuy,
    ForceBuy,
    AntiEco,
    Eco,
    FullSave,
}

impl Strategy {
    pub fn exec(&self, state: &State, remaining_money: &mut i64) -> Result<Vec<Recommendation>, NoTeamError> {
        use super::equipment::Equipment::*;
        use self::Strategy::*;
        use self::Recommendation::*;
        let mut result: Vec<Recommendation> = vec![];
        let team = match state.team {
            Some(team) => team,
            None => return Err(Default::default())
        };
        let is_ct = team == message::Team::CT;
        let is_t = team == message::Team::T;
        let mut inventory = state.inventory.clone();

        macro_rules! check {
            ( $eqp:expr ) => {{
                let cost = $eqp.cost() as i64;
                let mut allowed = match $eqp.restriction() {
                    None => true,
                    Some(message::Team::CT) => is_ct,
                    Some(message::Team::T) => is_t,
                };
                if $eqp == Flash {
                    let count = inventory.count(Flash);
                    allowed = allowed && count <= 1;
                } else {
                    allowed = allowed && !inventory.contains($eqp);
                }
                let mut weak = false;
                let this_tier = $eqp.tier();
                if let Some(old) = inventory.replaced_item($eqp) {
                    let old_tier = old.tier();
                    allowed = allowed && this_tier >= old_tier;
                    weak = this_tier == old_tier;
                }
                if *remaining_money >= cost && allowed {
                    inventory.push($eqp);
                    if weak {
                        result.push(Weak($eqp));
                    } else {
                        result.push(Strong($eqp));
                    }
                    *remaining_money -= cost;
                }
            }};
        }

        match *self {
            FullBuy => {
                check!(M4A1S);
                check!(AK47);
                if is_t {
                    check!(Tec9);
                } else {
                    check!(P250);
                }
                check!(VestHelmet);
                check!(Defuse);
                check!(Smoke);
                check!(Flash);
                check!(Flash);
                check!(Incendiary);
                check!(Molotov);
            },
            ForceBuy => {
                check!(M4A1S);
                check!(AK47);
                check!(MP7);
                if *remaining_money >= 1150 && is_t {
                    check!(Vest);
                    check!(Tec9);
                }
                if *remaining_money >= 1000 {
                    check!(VestHelmet);
                    if *remaining_money >= 500 && is_t {
                        check!(Tec9);
                    }
                }
                check!(Vest);
                check!(Tec9);
                check!(P250);
            },
            AntiEco => {

            },
            Eco => {
                check!(MP7);
                check!(Vest);
                check!(Defuse);
                check!(Smoke);
                check!(Flash);
                check!(Flash);
                check!(Molotov);
                check!(Incendiary);
            },
            FullSave => {}
        }

        Ok(result)
    }

    pub fn recommended(state: &State) -> Result<Vec<Recommendation>, NoTeamError> {
        use self::Strategy::*;
        let mut remaining_money: i64 = state.money as i64;
        let team = match state.team {
            Some(team) => team,
            None => return Err(Default::default())
        };
        let is_ct = team == message::Team::CT;
        let is_t = team == message::Team::T;
        let next_round_loss_reward = state.min_next_reward() as i64;

        // full buy
        let strategy = {
            if is_ct && remaining_money >= 5000 {
                FullBuy
            } else if is_t && remaining_money >= 5000 {
                FullBuy
            } else {
                if remaining_money + next_round_loss_reward >= 5000 {
                    remaining_money -= 5000 - next_round_loss_reward;
                    ForceBuy
                } else {
                    Eco
                }
            }
        };

        strategy.exec(state, &mut remaining_money)
    }
}
