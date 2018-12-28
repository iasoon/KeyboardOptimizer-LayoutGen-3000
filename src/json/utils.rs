use serde_json;
use serde::Deserialize;
use std::fs::File;
use std::io::Read;

use Result;


pub struct JsonBuffer<'s> {
    path: &'s str,
    contents: String,
}

impl<'s> JsonBuffer<'s> {
    pub fn from_file(path: &'s str) -> Self {
        JsonBuffer {
            path: path,
            contents: String::new(),
        }
    }

    pub fn read_file(&mut self) -> Result<()> {
        let mut file = try!(File::open(self.path));
        try!(file.read_to_string(&mut self.contents));
        Ok(())
    }

    pub fn map<'de, D, R, F>(&'de mut self, fun: F) -> Result<R>
        where F: Fn(D) -> Result<R>,
              D: Deserialize<'de>
    {
        try!(self.read_file());
        let json = try!(serde_json::from_str(&self.contents));
        return fun(json);
    }
}
