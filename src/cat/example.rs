use cat::universe::*;
use cat::table::*;
use std::marker::PhantomData;

use std::collections::HashMap;

pub struct Token<'a> {
    phantom: PhantomData<&'a ()>,
}

impl<'a> ElemType for Token<'a> {
    type Type = &'a str;
}

type Tokens<'a> = Universe<'a,
                           Token<'a>,
                           HashMap<&'a str, Num<Token<'a>>>,
                           Table<Token<'a>, &'a str>>;

fn test() {
    let token_names: Vec<&str> = vec!["hoi", "test", "hallo"];

    let table: Table<Token, &str> = Table::from_vec(token_names);

    let map: HashMap<&str, Num<Token>> = table.iter().map(|(num, &value)| (value, num)).collect();

    let universe: Tokens = Universe {
        mapping: map,
        set: table,
        phantom: PhantomData,
    };
}
