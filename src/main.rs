#![feature(plugin, custom_derive, conservative_impl_trait)]
#![allow(dead_code)]

mod algorithm;
mod cat;
mod data;
mod json;


extern crate rand;

#[macro_use]
extern crate failure;

#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;

#[cfg(test)]
#[macro_use]
extern crate proptest;


use std::path::Path;
use std::result;
use failure::ResultExt;

type Result<T> = result::Result<T, failure::Error>;

fn main() {
    if let Err(ref e) = run() {
        eprintln!("{}", pretty_error(&e));

        let backtrace = e.backtrace().to_string();
        if !backtrace.trim().is_empty() {
            eprintln!("{}", backtrace);
        }

        ::std::process::exit(1);
    }
}

fn pretty_error(err: &failure::Error) -> String {
    let mut pretty = err.to_string();
    let mut prev = err.as_fail();
    while let Some(next) = prev.cause() {
        pretty.push_str(": ");
        pretty.push_str(&next.to_string());
        prev = next;
    }
    pretty
}

fn run() -> Result<()> {
    let domain = json::read_config("abcABC.json")
        .context("Could not parse domain")?;
    let mut b = algorithm::Backtracker::new(&domain);
    try!(b.generate());
    Ok(())
}