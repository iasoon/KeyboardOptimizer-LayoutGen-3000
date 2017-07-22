pub mod errors {
    error_chain! {
        foreign_links {
            Io(::std::io::Error);
            Json(::serde_json::Error);
        }
    }
}

mod config;
mod reader;
mod elements;
mod groups;
mod keymap;
mod eval;

use std::fs::File;
use std::io::Read;

use data::KbDef;
use layout::Keymap;

use serde_json;

pub use self::keymap::KeymapData;

use serde::Deserialize;

use self::config::{Config, ConfigData};
use self::reader::{Reader, EvalReader};

use self::errors::*;

pub fn parse_config(path: &str) -> Result<Config> {
    JsonBuffer::from_file(path).parse(|json: ConfigData| {
        json.read()
    })
}

pub fn parse_keymap(path: &str, kb_def: &KbDef) -> Result<Keymap> {
    JsonBuffer::from_file(path).parse(|json: KeymapData| {
        let reader = EvalReader::new(kb_def);
        reader.read(&json)
    })
}

struct JsonBuffer<'s> {
    path: &'s str,
    contents: String,
}

impl<'s> JsonBuffer<'s> {
    fn from_file(path: &'s str) -> Self {
        JsonBuffer {
            path: path,
            contents: String::new(),
        }
    }

    fn read_file(&mut self) -> Result<()> {
        let mut file = try!(File::open(self.path));
        try!(file.read_to_string(&mut self.contents));
        Ok(())
    }

    fn parse<'de, D, R, F>(&'de mut self, fun: F) -> Result<R>
        where F: Fn(D) -> Result<R>,
              D: Deserialize<'de>
    {
        try!(self.read_file());
        let json = try!(serde_json::from_str(&self.contents));
        return fun(json);
    }
}
