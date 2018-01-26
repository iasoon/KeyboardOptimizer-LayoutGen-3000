#![feature(plugin, custom_derive, conservative_impl_trait)]
#![allow(dead_code)]

mod cat;
mod eval;
mod data;
mod json;

mod layout;

extern crate rand;

#[macro_use]
extern crate error_chain;

#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;

use std::path::Path;

mod errors {
    error_chain! {
        links {
            Parse(::json::errors::Error, ::json::errors::ErrorKind);
        }
    }
}

use errors::*;

fn main() {
    if let Err(ref e) = run() {
        println!("error: {}", e);

        for e in e.iter().skip(1) {
            println!("caused by: {}", e);
        }

        if let Some(backtrace) = e.backtrace() {
            println!("backtrace: {:?}", backtrace);
        }

        ::std::process::exit(1);
    }
}

fn run() -> errors::Result<()> {
    let _config = json::read_config("config.json")
        .chain_err(|| "Could not parse config.json")?;
    Ok(())
}