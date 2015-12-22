use rustc_serialize::json::{ToJson, Json};
use super::super::gsi::message;

#[allow(dead_code)]
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

    #[allow(unused_assignments)]
    pub fn recommended(money: u32, team: &message::Team) -> Vec<Equipment> {
        use self::Equipment::*;
        let mut result = vec![];
        let mut remaining_money = money;
        let is_ct = *team == message::Team::CT;
        let is_t = *team == message::Team::T;

        macro_rules! buy {
            ( $eqp:expr ) => {
                result.push($eqp);
                remaining_money -= $eqp.cost();
            }
        }

        // primary weapon
        if remaining_money > 4700 {
            buy!(AWP);
        } else if remaining_money > 3100 && is_ct {
            buy!(M4A1S);
        } else if remaining_money > 2700 && is_t {
            buy!(AK47);
        }

        // pistol
        if remaining_money > 850 {
            buy!(R8);
        } else if is_ct {
            buy!(USPS);
        } else if is_t {
            buy!(Glock);
        }

        // equipment
        if remaining_money > 1000 {
            buy!(VestHelmet);
        } else if remaining_money > 650 {
            buy!(Vest);
        }
        if remaining_money > 400 && is_ct {
            buy!(Defuse);
        }

        // grenades
        if remaining_money > 300 {
            buy!(Smoke);
        }
        if remaining_money > 200 {
            buy!(Flash);
        }
        if remaining_money > 200 {
            buy!(Flash);
        }
        if remaining_money > 400 && is_t {
            buy!(Molotov);
        } else if remaining_money > 600 && is_ct {
            buy!(Incendiary);
        }

        result
    }
}
