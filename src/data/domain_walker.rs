use super::RestrictedRange;
use super::types::*;

use cat::*;
use cat::ops::*;

pub struct DomainWalker<'d> {
    domain: &'d Domain,

    // Assigned values
    mapping: Table<Key, Option<Num<Value>>>,

    // Value domains remaining for each key
    ranges: Table<Key, RestrictedRange<Value>>,

    // For each (origin, target) pair, keep track of a range of values that
    // are not supported at target by any value at origin.
    // We track unsupported values instead of supported values because that
    // allows us to re-use the RestrictedRange for this purpose, by applying
    // De Morgan's law: if we add a rejection for each supporting value, the
    // value will only be present in the range once all the supporting values
    // are dropped.
    supports: Table<Key, Table<Key, RestrictedRange<Value>>>,

    // buffers for allowing and disallowing assignments
    // Never randomly push values into these buffers; they are queues for
    // processing assignments that _have been_ disallowed. That means that only
    // newly-rejected values should ever be added to this range.
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
                            // TODO: inspect this
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

    /// Get current walker position.
    pub fn mapping<'a>(&'a self) -> &'a Table<Key, Option<Num<Value>>> {
        &self.mapping
    }

    /// Get RestrictedRange for given key_num.
    pub fn range<'a>(&'a self, key_num: Num<Key>) -> &'a RestrictedRange<Value> {
        &self.ranges[key_num]
    }

    /// Get values that can be assigned to key_num without causing
    /// inconsistencies.
    pub fn range_for<'a>(&'a self, key_num: Num<Key>) -> &'a [Num<Value>] {
        self.ranges[key_num].accepted()
    }

    /// Assign a value to a key.
    pub fn assign(&mut self, key_num: Num<Key>, value_num: Num<Value>) {
        println!("assigning {:?} at {:?}", self.domain.values[value_num], key_num);
        
        // TODO
        // self.unassign(key_num);

        {
            let rejected = self.ranges[key_num].add_restriction(&vec![value_num]);

            println!("QUEUEING FOR REMOVAL AT {:?}: {:?}", key_num, value_names(&self.domain, rejected));

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

        for key_num in self.domain.keys.nums() {
            println!("domain for {:?} at {:?}", key_num, value_names(&self.domain, self.range_for(key_num)));
        }
    }

    /// Clear the value for a key.
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
                });
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
    }

    pub fn unassign_conflicts(&mut self, assignment: Assignment) {
        for key_num in self.domain.keys.nums() {
            let a = match self.mapping[key_num] {
                None => continue,
                Some(value_num) => Assignment { key_num, value_num },
            };

            if assignments_conflict(&self.domain, assignment, a) {
                self.unassign(a.key_num);
            }
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

        println!("QUEUEING FOR REMOVAL AT {:?}: {:?}", key_num, value_names(&self.domain, removed));

        for &value_num in removed {
            self.to_remove.push(Assignment { key_num, value_num });
        }
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

    fn remove_value(&mut self, key_num: Num<Key>, value_num: Num<Value>) {
        println!("removing {:?} at {:?}", self.domain.values[value_num], key_num);
        for origin_num in self.domain.keys.nums() {
            if origin_num == key_num {
                continue;
            }

            // println!("removing support {:?}: {:?} -> {:?}", origin_num, value_num, key_num);

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

            if lost_support.len() > 0 {
                // println!("no value at {:?} supports {:?} at {:?}", key_num, value_names(&self.domain, lost_support), origin_num);
            }

            let rejected = self.ranges[origin_num].add_rejection(lost_support);
            
            // for &value_num in rejected {
            //     self.to_remove.push(Assignment {
            //         key_num: origin_num,
            //         value_num,
            //     });
            // }
        }

    }

    fn add_value(&mut self, key_num: Num<Key>, value_num: Num<Value>) {
        if self.ranges[key_num].accepts(value_num) {
            return;
        }

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

            self.ranges[origin_num].remove_rejection(gained_support);
        }
    }
}

/// Whether given assignments conflict in the stated domain.
fn assignments_conflict(domain: &Domain, a: Assignment, b: Assignment) -> bool
{
    assignment_prohibits(domain, a, b) || assignment_prohibits(domain, b, a)
}

/// Whether assignment a prohibits assignment b from being made.
fn assignment_prohibits(domain: &Domain, a: Assignment, b: Assignment) -> bool {
    let cs = &domain.constraint_table;    
    match cs[a.key_num][b.key_num][a.value_num] {
        Restriction::Not(ref values) => {
            !values.contains(&b.value_num)
        }
        Restriction::Only(ref values) => {
            values.contains(&b.value_num)
        }
    }
}

fn value_names<'a>(domain: &'a Domain, values: &[Num<Value>]) -> Vec<&'a str> {
    values.iter().map(|&value_num| {
        domain.values[value_num].as_str()
    }).collect()
}