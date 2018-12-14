use super::RestrictedRange;
use super::types::*;

use cat::*;
use cat::ops::*;

pub struct DomainWalker<'d> {
    domain: &'d Domain,
    mapping: Table<Key, Option<Num<Value>>>,
    ranges: Table<Key, RestrictedRange>,
    // map key pairs to a range of values that have no support
    supports: Table<Key, Table<Key, RestrictedRange>>,

    to_remove: Vec<Assignment>,
    to_add: Vec<Assignment>,
}

impl<'d> DomainWalker<'d> {
    pub fn new(domain: &'d Domain) -> Self {
        // init domains
        let ranges = domain.key_restrictions.map(|restriction| {
            let mut range = RestrictedRange::new(domain.values.count());
            match restriction {
                Restriction::Not(ref values) => {
                    range.add_rejection(values);
                }
                Restriction::Only(ref values) => {
                    range.add_restriction(values);
                }
            }
            return range;
        });

        // init supports
        let supports = domain.keys.map_nums(|origin| {
            domain.keys.map_nums(|target| {
                let mut range = RestrictedRange::new(domain.values.count());
                let restrictor = &domain.constraint_table[target][origin];
                for value_num in domain.values.nums() {
                    // inverse constraints to apply De Morgan's law
                    match restrictor[value_num] {
                        Restriction::Not(ref values) => {
                            range.add_restriction(values);
                        }
                        Restriction::Only(ref values) => {
                            range.add_rejection(values);
                        }
                    }
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
            to_add: Vec::new(),
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
        println!("assigning {:?} to {:?}",
            self.domain.values[value_num],
            self.domain.keys[key_num]
        );
        // TODO
        // self.unassign(key_num);

        {
            let rejected = self.ranges[key_num].add_restriction(&vec![value_num]);

            for &val in rejected {
                self.to_remove.push(Assignment {
                    key_num,
                    value_num: val,
                });
            }
        }

        self.mapping[key_num] = Some(value_num);
        let row = &self.domain.constraint_table[key_num];
        for target_num in self.domain.keys.nums() {
            let restriction = &row[target_num][value_num];
            self.restrict(target_num, restriction);
        }

        while let Some(assignment) = self.to_remove.pop() {
            self.remove_value(assignment.key_num, assignment.value_num);
        }

        // print ranges
        for key in self.domain.keys.nums() {
            println!(
                "domain for {:?} is {:?}",
                self.domain.keys[key],
                value_names(self.domain, self.ranges[key].accepted()),
            )
        }
    }

    pub fn unassign(&mut self, key_num: Num<Key>) {
        let value_num = match self.mapping[key_num].take() {
            Some(value_num) => value_num,
            None => return,
        };

        {
            let unrejected = self.ranges[key_num]
                .remove_restriction(&vec![value_num]);

            for &val in unrejected {
                self.to_add.push(Assignment {
                    key_num,
                    value_num: val,
                })
            }
        }

        let row = &self.domain.constraint_table[key_num];
        for target_num in self.domain.keys.nums() {
            let restriction = &row[target_num][value_num];
            self.unrestrict(target_num, restriction);
        }

        while let Some(assignment) = self.to_add.pop() {
            self.add_value(assignment.key_num, assignment.value_num);
        }


        // print ranges
        for key in self.domain.keys.nums() {
            println!(
                "domain for {:?} is {:?}",
                self.domain.keys[key],
                value_names(self.domain, self.ranges[key].accepted()),
            )
        }

    }

    fn restrict(&mut self, key_num: Num<Key>, restriction: &Restriction) {
        let removed = match restriction {
            &Restriction::Not(ref values) => {
                self.ranges[key_num].add_rejection(values)
            }
            &Restriction::Only(ref values) => {
                self.ranges[key_num].add_restriction(values)
            }
        };

        for &value_num in removed {
            self.to_remove.push(Assignment { key_num, value_num });
        }

        // if let Some(value_num) = self.mapping[key_num] {
        //     if !self.range(key_num).accepts(value_num) {
        //         println!("CONFLICT");
        //         self.unassign(key_num);
        //     }
        // }
    }

    fn unrestrict(&mut self, key_num: Num<Key>, restriction: &Restriction) {
        let added = match restriction {
            &Restriction::Not(ref values) => {
                self.ranges[key_num].remove_rejection(values)
            }
            &Restriction::Only(ref values) => {
                self.ranges[key_num].remove_restriction(values)
            }
        };

        for &value_num in added {
            self.to_add.push(Assignment { key_num, value_num })
        }
    }

    pub fn remove_value(&mut self, key_num: Num<Key>, value_num: Num<Value>) {
        if !self.ranges[key_num].accepts(value_num) {
            return;
        }

        println!(
            "removing value {:?} at {:?}",
            self.domain.values[value_num],
            self.domain.keys[key_num],
        );
        for origin_num in self.domain.keys.nums() {
            if origin_num == key_num {
                continue;
            }

            let restrictor = &self.domain.constraint_table[origin_num][key_num];
            let support_set = &mut self.supports[key_num][origin_num];

            let lost_support = match restrictor[value_num] {
                Restriction::Not(ref values) => {
                    support_set.remove_restriction(values)
                }
                Restriction::Only(ref values) => {
                    support_set.remove_rejection(values)
                }
            };
            
            for &value_num in lost_support {
                self.to_remove.push(Assignment {
                    key_num: origin_num,
                    value_num,
                });
            }
            
            if lost_support.len() > 0 {
                println!(
                    "lost support at {:?}: {:?}",
                    self.domain.keys[origin_num],
                    value_names(&self.domain, lost_support),
                );
            }
            self.ranges[origin_num].add_rejection(lost_support);
        }

    }

    pub fn add_value(&mut self, key_num: Num<Key>, value_num: Num<Value>) {
        if self.ranges[key_num].accepts(value_num) {
            return;
        }
        println!(
            "adding value {:?} at {:?}",
            self.domain.values[value_num],
            self.domain.keys[key_num],
        );

        for origin_num in self.domain.keys.nums() {
            if origin_num == key_num {
                continue;
            }

            let restrictor = &self.domain.constraint_table[origin_num][key_num];
            let support_set = &mut self.supports[key_num][origin_num];
            
            let gained_support = match restrictor[value_num] {
                Restriction::Not(ref values) => {
                    support_set.add_restriction(values)
                }
                Restriction::Only(ref values) => {
                    support_set.add_rejection(values)
                }
            };

            for &value_num in gained_support {
                self.to_add.push(Assignment {
                    key_num: origin_num,
                    value_num,
                });
            }

            println!(
                "unrejecting at {:?}: {:?}",
                self.domain.keys[origin_num],
                value_names(&self.domain, gained_support),
            );
            self.ranges[origin_num].remove_rejection(gained_support);
            println!("unrejected");

            // for &value_num in gained_support {
            //     println!(
            //         "{:?} gained support at {:?}",
            //         self.domain.values[value_num],
            //         self.domain.keys[origin_num],
            //     );
            // }
        }
        println!("removed value");

    }
}

fn value_names<'a>(domain: &'a Domain, values: &[Num<Value>]) -> Vec<&'a str> {
    values.iter().map(|&value_num| {
        domain.values[value_num].as_str()
    }).collect()
}