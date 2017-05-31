use cat::universe::*;
use cat::mapping::*;
use cat::table::*;
use cat::hash_mapping::*;
use std::marker::PhantomData;

use std::collections::HashMap;

pub struct Token;

impl Domain for Token {
    type Type = String;
}

impl FiniteDomain for Token {}

type TokenSet<'t> = Subset<'t, Token, HashMapping<Token, Num<Token>>, Table<Token, String>>;

pub fn test<'t>() {
    let token_names: Vec<String> = vec!["hoi", "test", "hallo"]
        .iter()
        .map(|s| s.to_string())
        .collect();

    let set: TokenSet = Subset::from_elem_vec(token_names);

    if let Some(&num) = set.dict.map("hoi".to_string()) {
        println!("{}", set.elems[num]);
    } else {
        println!("None");
    }
}
