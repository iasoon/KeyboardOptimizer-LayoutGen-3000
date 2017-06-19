mod domain;
mod mapping;
mod has_count;
mod composed;

mod seq;

mod seq_table;
mod table;

pub mod ops;

pub use self::domain::{Domain, FiniteDomain, Num};
pub use self::has_count::{HasCount, Count, ElemEnumerator};
pub use self::mapping::{Mapping, Dict};
pub use self::composed::ComposedDict;
pub use self::seq::{Seq, SeqIter, SeqNum};

pub use self::table::Table;
pub use self::seq_table::SeqTable;

// export functions that should usually not be used
pub mod internal {
    pub use cat::domain::to_num;
    pub use cat::has_count::to_count;
}
