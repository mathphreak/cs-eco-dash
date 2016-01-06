use rustc_serialize::json::{ToJson, Json};
use super::super::gsi::message;
use std::fmt;
use std::string::ToString;

#[allow(dead_code)]
#[derive(Debug)]
#[derive(PartialEq)]
#[derive(Clone, Copy)]
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
    G3SG1,
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
    Knife,
    C4,
}

impl fmt::Display for Equipment {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::Equipment::*;
        write!(f, "{}", (match *self {
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
            G3SG1 => "G3SG1",
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
            Knife => "Knife",
            C4 => "C4",
        }).to_string())
    }
}

impl ToJson for Equipment {
    fn to_json(&self) -> Json {
        Json::String(self.to_string())
    }
}

impl From<String> for Equipment {
    fn from(s: String) -> Equipment {
        use self::Equipment::*;
        match &*s {
            "weapon_glock" => Glock,
            "weapon_hkp2000" => P2000,
            "weapon_usp_silencer" => USPS,
            "weapon_p250" => P250,
            "weapon_deagle" => Deagle,
            "weapon_elite" => Berettas,
            "weapon_tec9" => Tec9,
            "weapon_fiveseven" => FiveSeven,
            "weapon_cz75a" => CZ75,
            "weapon_revolver" => R8,
            "weapon_nova" => Nova,
            "weapon_xm1014" => XM1014,
            "weapon_sawedoff" => SawedOff,
            "weapon_mag7" => MAG7,
            "weapon_mac10" => MAC10,
            "weapon_mp9" => MP9,
            "weapon_mp7" => MP7,
            "weapon_ump45" => UMP45,
            "weapon_bizon" => PPBizon,
            "weapon_p90" => P90,
            "weapon_galilar" => GalilAR,
            "weapon_famas" => FAMAS,
            "weapon_ak47" => AK47,
            "weapon_m4a1" => M4A4,
            "weapon_m4a1_silencer" => M4A1S,
            "weapon_ssg08" => SSG08,
            "weapon_sg556" => SG553,
            "weapon_aug" => AUG,
            "weapon_awp" => AWP,
            "weapon_g3sg1" => G3SG1,
            "weapon_scar20" => SCAR20,
            "weapon_m249" => M249,
            "weapon_negev" => Negev,
            "weapon_taser" => Zeus,
            "weapon_molotov" => Molotov,
            "weapon_incgrenade" => Incendiary,
            "weapon_decoy" => Decoy,
            "weapon_hegrenade" => HENade,
            "weapon_flashbang" => Flash,
            "weapon_smokegrenade" => Smoke,
            "weapon_knife" => Knife,
            "weapon_knife_t" => Knife,
            "weapon_c4" => C4,
            _ => {
                println!("Unknown equipment: {}", s);
                Knife
            },
        }
    }
}

pub enum InvSlot {
    Primary,
    Secondary,
    Armor,
    Grenade,
    Misc,
    Trash,
}

pub enum Tier {
    Decent,
    Acceptable,
    EcoOnly,
    WhyEven,
}

impl Equipment {
    pub fn cost(&self) -> i32 {
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
            G3SG1 => 5000,
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
            Knife => 0,
            C4 => 0,
        }
    }

    pub fn restriction(&self) -> Option<message::Team> {
        use self::Equipment::*;
        use super::super::gsi::message::Team::*;
        match *self {
            Glock => Some(T),
            P2000 => Some(CT),
            USPS => Some(CT),
            Tec9 => Some(T),
            FiveSeven => Some(CT),
            SawedOff => Some(T),
            MAG7 => Some(CT),
            MAC10 => Some(T),
            MP9 => Some(CT),
            GalilAR => Some(T),
            FAMAS => Some(CT),
            AK47 => Some(T),
            M4A4 => Some(CT),
            M4A1S => Some(CT),
            SG553 => Some(T),
            AUG => Some(CT),
            G3SG1 => Some(T),
            SCAR20 => Some(CT),
            Defuse => Some(CT),
            Molotov => Some(T),
            Incendiary => Some(CT),
            C4 => Some(T),
            _ => None
        }
    }

    pub fn slot(&self) -> InvSlot {
        use self::Equipment::*;
        use self::InvSlot::*;
        match *self {
            Glock => Secondary,
            P2000 => Secondary,
            USPS => Secondary,
            P250 => Secondary,
            Deagle => Secondary,
            Berettas => Secondary,
            Tec9 => Secondary,
            FiveSeven => Secondary,
            CZ75 => Secondary,
            R8 => Secondary,
            Vest => Armor,
            VestHelmet => Armor,
            Zeus => Misc,
            Defuse => Misc,
            Molotov => Grenade,
            Incendiary => Grenade,
            Decoy => Grenade,
            HENade => Grenade,
            Flash => Grenade,
            Smoke => Grenade,
            Knife => Trash,
            C4 => Trash,
            _ => Primary,
        }
    }

    pub fn tier(&self) -> Tier {
        use self::Equipment::*;
        use self::Tier::*;
        match *self {
            Glock => Acceptable,
            P2000 => Decent,
            USPS => Decent,
            P250 => Decent,
            Deagle => Decent,
            Berettas => WhyEven,
            Tec9 => Decent,
            FiveSeven => Decent,
            CZ75 => Decent,
            R8 => Decent,
            Nova => EcoOnly,
            XM1014 => EcoOnly,
            SawedOff => WhyEven,
            MAG7 => EcoOnly,
            MAC10 => EcoOnly,
            MP9 => EcoOnly,
            MP7 => EcoOnly,
            UMP45 => EcoOnly,
            PPBizon => EcoOnly,
            P90 => Decent,
            GalilAR => Acceptable,
            FAMAS => Acceptable,
            AK47 => Decent,
            M4A4 => Decent,
            M4A1S => Decent,
            SSG08 => Decent,
            SG553 => Decent,
            AUG => Decent,
            AWP => Decent,
            G3SG1 => WhyEven,
            SCAR20 => WhyEven,
            M249 => WhyEven,
            Negev => WhyEven,
            Vest => Acceptable,
            VestHelmet => Decent,
            Zeus => WhyEven,
            Defuse => Decent,
            Molotov => Decent,
            Incendiary => Decent,
            Decoy => WhyEven,
            HENade => Decent,
            Flash => Decent,
            Smoke => Decent,
            Knife => Decent,
            C4 => Decent,
        }
    }
}
