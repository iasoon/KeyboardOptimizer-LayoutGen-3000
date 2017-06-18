use cat::{Domain, FiniteDomain, Num, Table};

/// A key on the keyboard.
pub struct Key;

impl Domain for Key {
    type Type = String;
}

impl FiniteDomain for Key {}


/// A layer on the keyboard, e.g. 'default', 'shift', 'alt', ...
pub struct Layer;

impl Domain for Layer {
    type Type = String;
}

impl FiniteDomain for Layer {}


/// A token that can be assigned to a location, e.g. letters of the alphabet.
pub struct Token;

impl Domain for Token {
    type Type = String;
}

impl FiniteDomain for Token {}

/// A location on the keyboard, determined by a key and a layer.
pub struct Loc {
    pub key_num: Num<Key>,
    pub layer_num: Num<Layer>,
}

impl Domain for Loc {
    type Type = Loc;
}

impl FiniteDomain for Loc {}


/// A token that can be moved freely.
/// A free token has two degrees of freedom: which key and which layer it is
/// placed on.
pub struct Free;

impl Domain for Free {
    type Type = Num<Token>;
}

impl FiniteDomain for Free {}


/// A group of tokens that is locked together.
/// Each token of the group is uniquely assigned to a layer, so that a 'locked'
/// token only has one degree of freedom: which key the group is assigned to.
/// The tokens in the locked group will always be assigned to the same key.
/// This is useful for requiring lowercase and uppercase letters to be on the
/// same key.
pub struct Lock;

impl Domain for Lock {
    type Type = Table<Layer, Option<Num<Token>>>;
}

impl FiniteDomain for Lock {}


/// An assignment either assigns a free token to a location, or a locked group
/// to a key.
pub enum Assignment {
    Free {
        free_num: Num<Free>,
        loc_num: Num<Loc>,
    },
    Lock {
        lock_num: Num<Lock>,
        key_num: Num<Key>,
    }
}

impl Domain for Assignment {
    type Type = Assignment;
}

impl FiniteDomain for Assignment {}
