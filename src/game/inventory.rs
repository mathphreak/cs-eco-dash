use super::equipment::Equipment;
use rustc_serialize::json::{ToJson, Json};

#[derive(Clone)]
pub struct Inventory {
    primary: Option<Equipment>,
    secondary: Option<Equipment>,
    armor: Option<Equipment>,
    grenades: Vec<Equipment>,
    misc: Vec<Equipment>,
}

impl ToJson for Inventory {
    fn to_json(&self) -> Json {
        let mut result = vec![];
        if let Some(primary) = self.primary {
            result.push(primary);
        }
        if let Some(secondary) = self.secondary {
            result.push(secondary);
        }
        if let Some(armor) = self.armor {
            result.push(armor);
        }
        result.extend(self.grenades.iter().cloned());
        result.extend(self.misc.iter().cloned());
        result.to_json()
    }
}

impl Default for Inventory {
    fn default() -> Inventory {
        Inventory {
            primary: None,
            secondary: None,
            armor: None,
            grenades: vec![],
            misc: vec![],
        }
    }
}

impl Inventory {
    pub fn push(&mut self, eqp: Equipment) {
        use super::equipment::InvSlot::*;
        match eqp.slot() {
            Primary => self.primary = Some(eqp),
            Secondary => self.secondary = Some(eqp),
            Armor => self.armor = Some(eqp),
            Grenade => self.grenades.push(eqp),
            Misc => self.misc.push(eqp),
        }
    }
    
    pub fn clear(&mut self) {
        self.primary = None;
        self.secondary = None;
        self.armor = None;
        self.grenades.clear();
        self.misc.clear();
    }
    
    pub fn count(&self, eqp: Equipment) -> i32 {
        let grenade_count = self.grenades.iter().filter(|x| **x == eqp).count();
        let misc_count = self.misc.iter().filter(|x| **x == eqp).count();
        return (grenade_count + misc_count) as i32;
    }
    
    pub fn contains(&self, eqp: Equipment) -> bool {
        use super::equipment::InvSlot::*;
        match eqp.slot() {
            Primary => self.primary == Some(eqp),
            Secondary => self.secondary == Some(eqp),
            Armor => self.armor == Some(eqp),
            Grenade => self.grenades.contains(&eqp),
            Misc => self.misc.contains(&eqp),
        }
    }
}
