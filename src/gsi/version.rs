extern crate time;

use rustc_serialize::json::{ToJson, Json};
use std::collections::BTreeMap;
use std::fs;
use super::paths;
use crc::crc32;
use std::io::Read;

pub struct Installed {
    last_check: time::Tm,
    last_result: String,
}

impl Installed {
    pub fn new() -> Installed {
        Installed {
            last_check: time::now() - time::Duration::minutes(5),
            last_result: "".to_string()
        }
    }

    pub fn get(&mut self) -> String {
        if (self.last_check + time::Duration::minutes(1)).gt(&time::now()) {
            return self.last_result.clone();
        } else {
            self.last_check = time::now();
            if fs::metadata(paths::CSGO_CFG).unwrap().is_dir() {
                for entry in fs::read_dir(paths::CSGO_CFG).unwrap() {
                    let entry = entry.unwrap();
                    let name = entry.path();
                    let name = name.file_name().unwrap();
                    let name = name.to_str().unwrap();
                    if name.starts_with(paths::CFG_PREFIX) {
                        let result = name
                            .replace(paths::CFG_PREFIX, "")
                            .replace(".cfg", "");
                        self.last_result = result;
                        return self.last_result.clone();
                    }
                }
            }
            self.last_result = "NONE".to_string();
            return self.last_result.clone();
        }
    }
}

pub struct Target {
    last_check: time::Tm,
    last_result: String,
}

impl Target {
    pub fn new() -> Target {
        Target {
            last_check: time::now() - time::Duration::minutes(5),
            last_result: "".to_string()
        }
    }

    pub fn get(&mut self) -> String {
        if (self.last_check + time::Duration::minutes(1)).gt(&time::now()) {
            return self.last_result.clone();
        } else {
            self.last_check = time::now();
            let mut file = fs::File::open("config/gsi.cfg").unwrap();
            let mut buffer = Vec::new();
            file.read_to_end(&mut buffer).unwrap();
            let checksum = crc32::checksum_ieee(buffer.into_boxed_slice().as_ref());
            self.last_result = format!("{:x}", checksum);
            return self.last_result.clone();
        }
    }
}

pub struct Versions {
    pub installed: Installed,
    pub target: Target
}

impl Versions {
    pub fn new() -> Versions {
        Versions {
            installed: Installed::new(),
            target: Target::new()
        }
    }

    pub fn update(&mut self) {
        self.installed.get();
        self.target.get();
    }
}

impl ToJson for Versions {
    fn to_json(&self) -> Json {
        let mut d = BTreeMap::new();
        d.insert("installed".to_string(), self.installed.last_result.to_json());
        d.insert("target".to_string(), self.target.last_result.to_json());
        Json::Object(d)
    }
}
