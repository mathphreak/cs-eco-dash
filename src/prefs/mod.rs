use std::fs::File;
use std::io;
use std::io::prelude::*;
use rustc_serialize::json;

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
        let contents = json::encode(self).unwrap();
        try!(file.write_all(contents.as_bytes()));
        file.sync_data()
    }
}
