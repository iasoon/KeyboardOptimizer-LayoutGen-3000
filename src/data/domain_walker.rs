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

        DomainWalker {
            mapping: domain.keys.map_nums(|_| None),
            ranges,
            domain,
        }
    }

    /// Get current walker position.
    pub fn mapping<'a>(&'a self) -> &'a Table<Key, Option<Num<Value>>> {
        &self.mapping
    }

    /// Get values that can be assigned to key_num without causing
    /// inconsistencies.
    pub fn range_for<'a>(&'a self, key_num: Num<Key>) -> &'a [Num<Value>] {
        self.ranges[key_num].accepted()
    }

    /// Assign a value to a key.
    pub fn assign(&mut self, key_num: Num<Key>, value_num: Num<Value>) {
        println!("assigning {:?} at {:?}", self.domain.values[value_num], self.domain.keys[key_num]);
        
        self.ranges[key_num].add_restriction(&vec![value_num]);

        self.mapping[key_num] = Some(value_num);
        let row = &self.domain.constraint_table[key_num];

        for target_num in self.domain.keys.nums() {
            let restriction = &row[target_num][value_num];
            self.restrict(target_num, restriction);
        }

    }

    /// Clear the value for a key.
    pub fn unassign(&mut self, key_num: Num<Key>) {
        let value_num = match self.mapping[key_num].take() {
            Some(value_num) => value_num,
            None => return,
        };

        {
            self.ranges[key_num]
                .remove_restriction(&vec![value_num]);

        }

        let row = &self.domain.constraint_table[key_num];
        for target_num in self.domain.keys.nums() {
            let restriction = &row[target_num][value_num];
            self.unrestrict(target_num, restriction);
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
        match restriction {
            &Restriction::Not(ref values) => {
                self.ranges[key_num].add_rejection(values)
            }
            &Restriction::Only(ref values) => {
                self.ranges[key_num].add_restriction(values)
            }
        };

    }

    fn unrestrict(&mut self, key_num: Num<Key>, restriction: &Restriction) {
        match restriction {
            &Restriction::Not(ref values) => {
                self.ranges[key_num].remove_rejection(values)
            }
            &Restriction::Only(ref values) => {
                self.ranges[key_num].remove_restriction(values)
            }
        };

    }

    pub fn valid_state(&self) -> bool {
        self.mapping.enumerate().all(|(key_num, value)| {
            let valid = match value {
                None => true,
                &Some(value_num) => {
                    self.ranges[key_num].accepts(value_num)
                }
            };

            let satisfiable = self.ranges[key_num].accepted().len() > 0;

            valid && satisfiable
        })
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


#[cfg(test)]
mod test {
    use super::*;
    use failure::ResultExt;
    use json;

    enum Op {
        Assign(Assignment),
    }

    impl Op {
        fn apply(&self, w: &mut DomainWalker) {
            match self {
                &Op::Assign(Assignment { key_num, value_num }) => {
                    w.assign(key_num, value_num)
                }
            }
        }
    }

    #[test]
    fn test_domains() {
        let domain = json::read_config("abcABC.json")
            .context("Could not parse domain").unwrap();
        // TODO
    }
}