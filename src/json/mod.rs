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

use data::KbDef;
use std::fs::File;
use std::io::Read;
use serde_json;

pub use self::keymap::Keymap;
use self::config::{Config, ConfigData};

pub fn parse_config(path: &str) -> errors::Result<Config> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let json: ConfigData = serde_json::from_str(&contents)?;
    Ok(json.read()?)
}
