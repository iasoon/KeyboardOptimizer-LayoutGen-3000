use cat::domain::*;
use cat::mapping::*;
use cat::table::*;
use cat::hash_mapping::*;
use cat::seq::*;
use cat::composed;

use std::marker::PhantomData;
use std::borrow::Borrow;
use std::hash::Hash;


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

    // TODO: maybe return an option type here?
    fn num_seq<'q, I, Q: 'q + ?Sized>(&self, iterable: I) -> Vec<Num<Token>>
        where I: IntoIterator<Item = &'q Q>,
              String: Borrow<Q>,
              Q: Hash + Eq
    {
        iterable.into_iter().map(|token| self.index.get(token).unwrap()).cloned().collect()
    }
}

pub fn test<'t>() {
    let token_names: Vec<String> = vec!["hoi", "test", "hallo"]
        .iter()
        .map(|s| s.to_string())
        .collect();
    let tokens = TokenSet::from_vec(token_names);

    let table: Table<Seq<Token, Vec<String>>, Option<String>> = Table::from_vec(
        Seq::iter(tokens.elems.count(), 3).map(|_| None).collect()
    );
    let mut seq_table = composed::Pre::new(SeqNum::new(tokens.elems.count()), table);
    let seq = vec!["hoi", "hoi", "test"];

    *seq_table.get_mut(tokens.num_seq(seq.clone())) = Some("haha".to_string());
    println!("{:?}", seq_table.get(tokens.num_seq(seq)));
}
