#![feature(plugin, custom_derive, conservative_impl_trait)]
#![allow(dead_code)]

mod algorithm;
mod cat;
mod data;
mod json;

//mod layout;

extern crate rand;

#[macro_use]
extern crate error_chain;

#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;

#[cfg(test)]
#[macro_use]
extern crate proptest;


use std::path::Path;

mod errors {
    error_chain! {
        links {
            // Parse(::json::errors::Error, ::json::errors::ErrorKind);
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
    let domain = json::read_config("abcABC.json")
        .chain_err(|| "Could not parse domain")?;
    let mut b = algorithm::Backtracker::new(&domain);
    try!(b.generate());
    Ok(())
}