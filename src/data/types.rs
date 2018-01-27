use cat;
use cat::*;
use data::types::*;
use std::ops::{Index, IndexMut};

/// Indicator struct for a key
pub struct Key;

/// Indicator struct for a value
pub struct Value;

/// Simple key/value pair
#[derive(Debug, Copy, Clone)]
pub struct Assignment {
    key_num: Num<Key>,
    value_num: Num<Value>,
}

/// Defines a numbering on assignments
pub struct AssignmentNum {
    pub key_count: Count<Key>,
    pub value_count: Count<Value>,
}

impl AssignmentNum {
    fn product_num(&self) -> ProductNum<Key, Value> {
        ProductNum {
            major_count: self.key_count,
            minor_count: self.value_count,
        }
    }
}

impl HasCount<Assignment> for AssignmentNum {
    fn count(&self) -> Count<Assignment> {
        let count = self.product_num().count();
        return cat::internal::to_count(count.as_usize());
    }
}

impl Mapping<Assignment> for AssignmentNum {
    type Result = Num<Assignment>;

    fn apply(&self, assignment: Assignment) -> Num<Assignment> {
        let pair = (assignment.key_num, assignment.value_num);
        let num = self.product_num().apply(pair);
        return cat::internal::to_num(num.as_usize());
    }
}

/// Marker type for disambiguating between all inhabitants of the Assignment
/// type, and just the inhabitants that are permitted by the problem statement.
/// This type marks the latter.
pub struct AllowedAssignment;

type AssignmentTable<T> = Composed<AssignmentNum, Table<Assignment, T>>;

pub struct Problem {
    /// key names
    pub keys: Table<Key, String>,
    /// value names
    pub values: Table<Value, String>,

    /// A numbered list of all allowed assignments.
    pub assignments: Table<AllowedAssignment, Assignment>,

    /// A lookup table to check whether an assignment is allowed, and if so,
    /// which number it was assigned.
    pub assignment_map: AssignmentTable<Option<Num<AllowedAssignment>>>,
}

impl Problem {
    pub fn assignment_num(&self) -> AssignmentNum {
        AssignmentNum {
            key_count: self.keys.count(),
            value_count: self.values.count(),
        }
    }
}
