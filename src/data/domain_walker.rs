use super::RestrictedRange;
use super::types::*;

use cat::*;

pub struct DomainWalker<'d> {
    domain: &'d Domain,
    ranges: Table<Key, RestrictedRange>,
}

impl<'d> DomainWalker<'d> {
    pub fn new(domain: &'d Domain) -> Self {
        let ranges = domain.keys.map_nums(|_| {
            RestrictedRange::new(domain.values.count())
        });

        DomainWalker {
            domain: domain,
            ranges: ranges,
        }
    }

    pub fn range<'a>(&'a self, key_num: Num<Key>) -> &'a RestrictedRange {
        &self.ranges[key_num]
    }

    pub fn range_for<'a>(&'a self, key_num: Num<Key>) -> &'a [Num<Value>] {
        self.ranges[key_num].accepted()
    }

    pub fn assign(&mut self, key_num: Num<Key>, value_num: Num<Value>) {
        for constraint in self.domain.constraints.iter() {
            if constraint.object == key_num {
                let restriction = &constraint.restrictor[value_num];
                self.ranges[constraint.subject].add_restriction(restriction);
            }
        }
    }

    pub fn unassign(&mut self, key_num: Num<Key>, value_num: Num<Value>) {
        for constraint in self.domain.constraints.iter() {
            if constraint.object == key_num {
                let restriction = &constraint.restrictor[value_num];
                self.ranges[constraint.subject].remove_restriction(restriction);
            }
        }
    }
}