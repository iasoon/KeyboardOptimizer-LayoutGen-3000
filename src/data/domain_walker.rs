use super::RestrictedRange;
use super::types::*;

use cat::*;

pub struct DomainWalker<'d> {
    domain: &'d Domain,
    mapping: Table<Key, Option<Num<Value>>>,
    ranges: Table<Key, RestrictedRange>,
    supports: Table<Key, Table<Key, RestrictedRange>>,

    to_remove: Vec<Assignment>,
}

impl<'d> DomainWalker<'d> {
    pub fn new(domain: &'d Domain) -> Self {
        let ranges = domain.keys.map_nums(|_| {
            RestrictedRange::new(domain.values.count())
        });

        // init supports
        let supports = domain.keys.map_nums(|origin| {
            domain.keys.map_nums(|target| {
                let mut range = RestrictedRange::new(domain.values.count());
                let restrictor = &domain.constraint_table[origin][target];
                for value_num in domain.values.nums() {
                    let restriction = restrictor[value_num].inverse();
                    range.add_restriction(&restriction);
                }
                return range;
            })
        });

        DomainWalker {
            mapping: domain.keys.map_nums(|_| None),
            ranges,
            domain,
            supports,

            to_remove: Vec::new(),
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
        let row = &self.domain.constraint_table[key_num];
        for target_num in self.domain.keys.nums() {
            let restriction = &row[target_num][value_num];
            self.restrict(target_num, restriction);
        }
    }

    pub fn unassign(&mut self, key_num: Num<Key>) {
        if let Some(value_num) = self.mapping[key_num].take() {
            let row = &self.domain.constraint_table[key_num];
            for target_num in self.domain.keys.nums() {
                let restriction = &row[target_num][value_num];
                self.ranges[target_num].remove_restriction(restriction);
            }
        }
    }

    fn restrict(&mut self, key_num: Num<Key>, restriction: &Restriction) {
        match restriction {
            &Restriction::Not(ref values) => {
                for target_num in self.domain.keys.nums() {
                    let restrictor = &self.domain.constraint_table[key_num][target_num];
                    for &value_num in values {
                        let support_set = &mut self.supports[key_num][target_num];
                        let restriction = restrictor[value_num].inverse();
                        support_set.remove_restriction(&restriction);
                        for &value in support_set.accepted() {
                            if self.ranges[target_num].accepts(value) {
                                // this value has no support anymore; remove it
                                self.ranges[target_num].reject(value);
                                self.to_remove.push(Assignment {
                                    key_num: target_num,
                                    value_num: value,
                                });
                            }
                        }
                    }
                }
            }
            &Restriction::Only(_) => {
                unimplemented!()
            }
        };
        self.ranges[key_num].add_restriction(restriction);

        if let Some(value_num) = self.mapping[key_num] {
            if !self.range(key_num).accepts(value_num) {
                println!("CONFLICT");
                self.unassign(key_num);
            }
        }
    }
}

struct Assigner<'a> {
    queue: Vec<()>,
    domain: &'a Domain,
    ranges: &'a mut Table<Key, RestrictedRange>,
    supports: &'a mut Table<Key, Table<Key, RestrictedRange>>,
}