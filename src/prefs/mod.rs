use std::fs::{self, File};
use std::io;
use std::io::prelude::*;
use rustc_serialize::json;
use rustc_serialize::json::{Json, ToJson};
use std::collections::BTreeMap;

#[derive(RustcEncodable, RustcDecodable)]
pub struct Prefs {
    pub csgo_cfg_path: String,
}

impl Prefs {
    pub fn get() -> io::Result<Prefs> {
        let mut file = try!(File::open("config/settings.json"));
        let mut contents = String::new();
        try!(file.read_to_string(&mut contents));
        let result: Prefs = json::decode(&contents).unwrap();
        Ok(result)
    }

    pub fn save(&self) -> io::Result<()> {
        let mut file = try!(File::create("config/settings.json"));
        let contents = json::as_pretty_json(self).indent(2);
        write!(file, "{}\n", contents)
    }

    pub fn is_valid(&self) -> bool {
        let metadata = fs::metadata(self.csgo_cfg_path.clone());
        match metadata {
            Ok(data) => data.is_dir(),
            Err(_) => false
        }
    }
}

impl ToJson for Prefs {
    fn to_json(&self) -> Json {
        let mut d = BTreeMap::new();
        d.insert("csgo_cfg_path".to_string(), self.csgo_cfg_path.to_json());
        Json::Object(d)
    }
}

impl Default for Prefs {
    fn default() -> Prefs {
        Prefs {
            csgo_cfg_path: "C:\\Program Files (x86)\\Steam\\steamapps\\common\\Counter-Strike Global Offensive\\csgo\\cfg".to_string(),
        }
    }
}
