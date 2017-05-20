#![feature(plugin, custom_derive, conservative_impl_trait)]

mod model;
mod utils;
mod eval;
mod data;

mod layout;
mod operations;
mod heuristics;

extern crate rand;

#[macro_use]
extern crate error_chain;

#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;

#[macro_use]
extern crate nom;

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
        kb_def: &Path::new("kb_def.json"),
        corpus: &Path::new("corpus_file"),
        path_costs: &Path::new("path_costs")
    }
    // Analyze {
    //     kb_conf: &Path::new("data/kb_conf.json"),
    //     corpus: &Path::new("data/corpus.json"),
    //     score_tree: &Path::new("data/effort.json"),
    //     keymap: &Path::new("data/keymap.json"),
    //     results: &Path::new("results.json"),
    // }
}
