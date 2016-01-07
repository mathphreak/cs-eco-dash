extern crate cs_eco_dash;

use cs_eco_dash::gsi::message::Team::{self, CT, T};
use cs_eco_dash::game::State;
use cs_eco_dash::game::Equipment;
use cs_eco_dash::game::Strategy;
use cs_eco_dash::game::Recommendation::{self, Strong, Weak};
use cs_eco_dash::game::Equipment::*;

fn run(money: i32, team: Team, history: Vec<bool>, inventory: Vec<Equipment>, target: Vec<Recommendation>) {
    let mut state: State = Default::default();
    state.money = money;
    state.team = Some(team);
    state.won_rounds = history;
    for eqp in inventory.clone() {
        state.inventory.push(eqp);
    }
    let recommendations = Strategy::recommended(&state).unwrap();
    for owned in &inventory {
        assert!(!target.contains(&Strong(*owned)));
        assert!(!target.contains(&Weak(*owned)));
        assert!(!recommendations.contains(&Strong(*owned)));
        assert!(!recommendations.contains(&Weak(*owned)));
    }
    assert_eq!(target, recommendations);
}

#[test]
fn recommends_full_buy_when_rich() {
    run(16000, CT,
        vec![],
        vec![],
        vec![Strong(M4A1S), Strong(P250), Strong(VestHelmet), Strong(Defuse), Strong(Smoke), Strong(Flash), Strong(Flash), Strong(Incendiary)]);

    run(16000, T,
        vec![],
        vec![],
        vec![Strong(AK47), Strong(Tec9), Strong(VestHelmet), Strong(Smoke), Strong(Flash), Strong(Flash), Strong(Molotov)]);
}

#[test]
fn ct_full_buy_valid() {
    run(5000, CT,
        vec![],
        vec![],
        vec![Strong(M4A1S), Strong(P250), Strong(VestHelmet), Strong(Defuse), Strong(Flash)]);
}

#[test]
fn t_full_buy_valid() {
    run(5000, T,
        vec![],
        vec![],
        vec![Strong(AK47), Strong(Tec9), Strong(VestHelmet), Strong(Smoke), Strong(Flash), Strong(Flash)]);
}

#[test]
fn recommends_full_save_when_close() {
    run(3650, T,
        vec![],
        vec![],
        vec![]);

    run(3650, CT,
        vec![],
        vec![],
        vec![]);
}

#[test]
fn recommends_t_armor_tec9_when_close() {
    run(4750, T,
        vec![],
        vec![],
        vec![Strong(Vest), Strong(Tec9)]);
}

#[test]
fn blue_shell_aware() {
    run(1600, CT,
        vec![false, false, false, false, false],
        vec![],
        vec![])
}

#[test]
fn recommends_eco_when_broke() {
    run(1800, CT,
        vec![],
        vec![],
        vec![Strong(MP7)])
}

#[test]
fn recommends_nothing_when_fully_equipped() {
    run(16000, CT,
        vec![],
        vec![M4A1S, P250, VestHelmet, Defuse, Smoke, Flash, Flash, Incendiary],
        vec![])
}

#[test]
fn recommends_ditching_negev() {
    run(16000, CT,
        vec![],
        vec![Negev, P250, VestHelmet, Defuse, Smoke, Flash, Flash, Incendiary],
        vec![Strong(M4A1S)])
}

#[test]
fn weakly_recommends_m4_when_holding_ak_as_ct() {
    run(16000, CT,
        vec![],
        vec![AK47, P250, VestHelmet, Defuse, Smoke, Flash, Flash, Incendiary],
        vec![Weak(M4A1S)])
}
