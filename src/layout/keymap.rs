use model::{TokenId, Loc};
use utils::LookupTable;

pub type Keymap = LookupTable<Loc, Option<TokenId>>;
