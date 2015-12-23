use super::super::prefs::Prefs;

pub const CFG_PREFIX: &'static str = "gamestate_integration_cs-eco-dash_";

pub fn get_csgo_cfg() -> String {
    let prefs = Prefs::new("config/settings.json");
    let path = prefs.read("csgo_cfg_path").unwrap();
    path
}
