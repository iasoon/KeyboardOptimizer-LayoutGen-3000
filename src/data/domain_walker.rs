use super::RestrictedRange;
use super::types::*;

use cat::*;

pub struct DomainWalker<'d> {
    domain: &'d Domain,
    mapping: Table<Key, Option<Num<Value>>>,
    ranges: Table<Key, RestrictedRange>,
}

impl<'d> DomainWalker<'d> {
    pub fn new(domain: &'d Domain) -> Self {
        let ranges = domain.keys.map_nums(|_| {
            RestrictedRange::new(domain.values.count())
        });

        DomainWalker {
            mapping: domain.keys.map_nums(|_| None),
            ranges: ranges,
            domain: domain,
        }
    }

    pub fn mapping<'a>(&'a self) -> &'a Table<Key, Option<Num<Value>>> {
        &self.mapping
    }

    pub fn range<'a>(&'a self, key_num: Num<Key>) -> &'a RestrictedRange {
        &self.ranges[key_num]
    }

    pub fn range_for<'a>(&'a self, key_num: Num<Key>) -> &'a [Num<Value>] {
        self.ranges[key_num].accepted()
    }

    pub fn assign(&mut self, key_num: Num<Key>, value_num: Num<Value>) {
        self.unassign(key_num);
        self.mapping[key_num] = Some(value_num);

        for constraint in self.domain.constraints.iter() {
            if constraint.origin == key_num {
                let restriction = &constraint.restrictor[value_num];
                self.restrict(constraint.target, restriction);
            }
        }
    }

    pub fn unassign(&mut self, key_num: Num<Key>) {
        if let Some(value_num) = self.mapping[key_num].take() {
            for constraint in self.domain.constraints.iter() {
                if constraint.origin == key_num {
                    let restriction = &constraint.restrictor[value_num];
                    self.ranges[constraint.target].remove_restriction(restriction);
                }
            }
        }
    }

    fn restrict(&mut self, key_num: Num<Key>, restriction: &Restriction) {
        self.ranges[key_num].add_restriction(restriction);
        if let Some(value_num) = self.mapping[key_num] {
            if !self.range(key_num).accepts(value_num) {
                self.unassign(key_num);
            }
        }
    }
}