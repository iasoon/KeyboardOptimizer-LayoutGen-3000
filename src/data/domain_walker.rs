use super::RestrictedRange;
use super::types::*;

use cat::*;

pub struct DomainWalker<'d> {
    domain: &'d Domain,
    mapping: Table<Key, Option<Num<Value>>>,
    ranges: Table<Key, RestrictedRange>,
    // map key pairs to a range of values that have no support
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
                let restrictor = &domain.constraint_table[target][origin];
                for value_num in domain.values.nums() {
                    // inverse constraints to apply De Morgan's law
                    match restrictor[value_num] {
                        Restriction::Not(ref values) => {
                            if values.len() > 0 {
                                println!(
                                    "{:?} at {:?} not supported by {:?} at {:?}",
                                    values,
                                    target,
                                    value_num,
                                    origin);
                            }
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

        for &val in self.ranges[key_num].accepted() {
            if val != value_num {
                self.to_remove.push(Assignment {
                    key_num,
                    value_num: val,
                });
            }
        }

        self.ranges[key_num].add_restriction(&vec![value_num]);

        while let Some(assignment) = self.to_remove.pop() {
            self.remove_value(assignment.key_num, assignment.value_num);
        }
    
        self.mapping[key_num] = Some(value_num);
        let row = &self.domain.constraint_table[key_num];
        for target_num in self.domain.keys.nums() {
            let restriction = &row[target_num][value_num];
            self.restrict(target_num, restriction);
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
        if let Some(value_num) = self.mapping[key_num].take() {
            let row = &self.domain.constraint_table[key_num];
            for target_num in self.domain.keys.nums() {
                // TODO
                // let restriction = &row[target_num][value_num];
                // self.ranges[target_num].remove_restriction(restriction);
                unimplemented!()
            }
        }
    }

    fn restrict(&mut self, key_num: Num<Key>, restriction: &Restriction) {
        match restriction {
            &Restriction::Not(ref values) => {
                self.add_rejection(key_num, values);
            }
            &Restriction::Only(_) => {
                unimplemented!()
            }
        };

        if let Some(value_num) = self.mapping[key_num] {
            if !self.range(key_num).accepts(value_num) {
                println!("CONFLICT");
                self.unassign(key_num);
            }
        }
    }

    fn add_rejection(&mut self, key_num: Num<Key>, rejected: &[Num<Value>]) {
        println!(
            "rejecting {:?} at {:?}",
            value_names(&self.domain, rejected),
            self.domain.keys[key_num]
        );

        for &value_num in rejected{
            if self.ranges[key_num].accepts(value_num) {
                self.remove_value(key_num, value_num);
            }
        }

        self.ranges[key_num].add_rejection(rejected);


        for origin_num in self.domain.keys.nums() {
            let restrictor = &self.domain.constraint_table[origin_num][key_num];
            let support_set = &mut self.supports[key_num][origin_num];
            

            for &value_num in rejected {
                match restrictor[value_num] {
                    Restriction::Not(ref values) => {
                        // println!(
                        //     "REMOVING {:?} at {:?} not supported by {:?} at {:?}",
                        //     values,
                        //     origin_num,
                        //     value_num,
                        //     key_num,
                        // );
                        // support_set.remove_restriction(values);
                    }
                    Restriction::Only(ref values) => {
                        // support_set.remove_rejection(values);
                    }
                }
            }

            // for &value in support_set.accepted() {
            //     if self.ranges[target_num].accepts(value) {
            //         // this value has no support anymore; remove it
            //         println!("dropping value because of no support");
            //         self.ranges[target_num].reject(value);
            //         self.to_remove.push(Assignment {
            //             key_num: target_num,
            //             value_num: value,
            //         });
            //     }
            // }

            // println!(
            //     "{:?} now supports {:?} at {:?}",
            //     self.domain.keys[key_num],
            //     value_names(&self.domain, support_set.rejected()),
            //     self.domain.keys[origin_num],
            // );
        }

    }

    pub fn remove_value(&mut self, key_num: Num<Key>, value_num: Num<Value>) {
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

            match restrictor[value_num] {
                Restriction::Not(ref values) => {
                    support_set.remove_restriction(values);
                }
                Restriction::Only(_) => {
                    unimplemented!()
                }
            }
            for &unsupported in support_set.accepted() {
                if self.ranges[origin_num].accepts(unsupported) {
                    println!(
                        "{:?} lost support at {:?}",
                        self.domain.values[unsupported],
                        self.domain.keys[origin_num],
                    );

                    self.to_remove.push(Assignment {
                        key_num: origin_num,
                        value_num: unsupported,
                    });

                    self.ranges[origin_num].reject(unsupported);
                }
            }

            println!(
                "{:?} now supports {:?} at {:?}",
                self.domain.keys[key_num],
                value_names(&self.domain, support_set.rejected()),
                self.domain.keys[origin_num],
            );
        }

    }
}

fn value_names<'a>(domain: &'a Domain, values: &[Num<Value>]) -> Vec<&'a str> {
    values.iter().map(|&value_num| {
        domain.values[value_num].as_str()
    }).collect()
}