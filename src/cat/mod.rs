mod types;
mod num;
mod has_count;
mod composed;
mod product;

mod seq;

mod seq_table;
mod table;

pub mod ops;

pub use self::types::*;
pub use self::has_count::{HasCount, Count, Enumerator, ElemEnumerator};
pub use self::num::Num;
pub use self::composed::Composed;
pub use self::seq::{Seq, SeqIter, SeqNum};
pub use self::product::{Product, ProductNum};


pub use self::table::Table;
pub use self::seq_table::SeqTable;
pub type ElemTable<D, M, T> = Composed<M, Table<D, T>>;

// export functions that should usually not be used
pub mod internal {
    pub use cat::num::to_num;
    pub use cat::has_count::to_count;
}
