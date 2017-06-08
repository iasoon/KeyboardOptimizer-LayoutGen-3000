use cat::domain::*;
use cat::mapping::*;
use cat::table::*;
use cat::hash_mapping::*;
use cat::seq::*;

use std::marker::PhantomData;
use std::collections::HashMap;


pub struct Token;

impl Domain for Token {
    type Type = String;
}

impl FiniteDomain for Token {}

pub fn test<'t>() {
    let token_names: Vec<String> = vec!["hoi", "test", "hallo"]
        .iter()
        .map(|s| s.to_string())
        .collect();
    let tokens: Table<Token, String> = Table::from_vec(token_names);
    for (num, token) in tokens.enumerate() {
        println!("{}: {}", num.as_usize(), token);
    }
}
