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

struct TokenSet {
    elems: Table<Token, String>,
    index: HashMapping<Token, Num<Token>>,
}

impl TokenSet {
    fn from_vec(vec: Vec<String>) -> Self {
        let elems: Table<Token, String> = Table::from_vec(vec);
        let mut index = HashMapping::empty();
        for (num, token) in elems.enumerate() {
            index.set(token.clone(), Some(num));
        }
        return TokenSet {
            elems: elems,
            index: index,
        }
    }
}

pub fn test<'t>() {
    let token_names: Vec<String> = vec!["hoi", "test", "hallo"]
        .iter()
        .map(|s| s.to_string())
        .collect();
    let tokens = TokenSet::from_vec(token_names);
    for (num, token) in tokens.elems.enumerate() {
        println!("{}: {}", num.as_usize(), token);
    }
}
