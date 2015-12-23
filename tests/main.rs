extern crate cs_eco_dash;

use cs_eco_dash::gsi::message::Team::{self, CT, T};
use cs_eco_dash::game::State;
use cs_eco_dash::game::Equipment;
use cs_eco_dash::game::Equipment::*;

fn run(money: u32, team: Team, history: Vec<bool>, inventory: Vec<Equipment>, target: Vec<Equipment>) {
    let mut state: State = Default::default();
    state.money = money;
    state.team = Some(team);
    state.won_rounds = history;
    state.inventory = inventory.clone();
    let recommendation = Equipment::recommended(&state).unwrap();
    for owned in &inventory {
        assert!(!target.contains(owned));
        assert!(!recommendation.contains(owned));
    }
    assert_eq!(target, recommendation);
}

#[test]
fn recommends_full_buy_when_rich() {
    run(16000, CT,
        vec![],
        vec![],
        vec![M4A1S, P250, VestHelmet, Defuse, Smoke, Flash, Flash, Incendiary]);

    run(16000, T,
        vec![],
        vec![],
        vec![AK47, Tec9, VestHelmet, Smoke, Flash, Flash, Molotov]);
}

#[test]
fn ct_full_buy_valid() {
    run(5000, CT,
        vec![],
        vec![],
        vec![M4A1S, P250, VestHelmet, Defuse, Flash]);
}

#[test]
fn t_full_buy_valid() {
    run(5000, T,
        vec![],
        vec![],
        vec![AK47, Tec9, VestHelmet, Smoke, Flash, Flash]);
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
        vec![Vest, Tec9]);
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
        vec![MP7])
}

#[test]
fn recommends_nothing_when_fully_equipped() {
    run(16000, CT,
        vec![],
        vec![M4A1S, P250, VestHelmet, Defuse, Smoke, Flash, Flash, Incendiary],
        vec![])
}
