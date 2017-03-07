#![feature(plugin, custom_derive, conservative_impl_trait)]

mod model;
mod data;
mod parser;
mod utils;
mod eval;

mod layout;
mod operations;

extern crate rand;

#[macro_use]
extern crate error_chain;

#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;

use std::path::Path;
use operations::*;

mod errors {
    error_chain! { }
}

fn main() {
    if let Err(ref e) = get_operation().run() {
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

fn get_operation<'a>() -> impl Operation + 'a {
    Optimize {
        kb_conf: &Path::new("data/kb_conf.json"),
        corpus: &Path::new("data/corpus.json"),
        score_tree: &Path::new("data/score_tree.json"),
    }
}
