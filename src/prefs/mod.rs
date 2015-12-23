use std::path::Path;
use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use rustc_serialize::json::Json;

pub struct Prefs<'a> {
    data: HashMap<String, String>,
    path: &'a Path,
}

impl<'a> Prefs<'a> {
    pub fn new(path: &str) -> Prefs {
        let mut result = Prefs {
            data: HashMap::new(),
            path: Path::new(path),
        };
        result.load();
        result
    }

    fn load(&mut self) -> io::Result<()> {
        let mut file = try!(File::open(self.path));
        let mut contents = String::new();
        try!(file.read_to_string(&mut contents));
        let data = Json::from_str(&contents).unwrap();
        let data = data.as_object();
        let data = data.unwrap();
        self.data.clear();
        for (key, val) in data.iter() {
            self.data.insert(key.to_string(), val.as_string().unwrap().to_string());
        }
        Ok(())
    }

    pub fn read(&self, key: &str) -> Option<String> {
        self.data.get(key).cloned()
    }

    fn write(&mut self, key: &str, val: &str) {
        self.data.remove(key);
        self.data.insert(key.to_string(), val.to_string());
    }
}
