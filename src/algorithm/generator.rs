use data::*;
use cat::*;

use Result;

use std::collections::HashSet;
use std::iter::FromIterator;

pub struct Backtracker<'d> {
    domain_walker: DomainWalker<'d>,
    stack: Vec<Step>,
    unassigned: HashSet<Num<Key>>,
}

struct Step {
    key_num: Num<Key>,
    values: Vec<Num<Value>>,
    pos: usize,
}

impl Step {
    fn new(key_num: Num<Key>, values: Vec<Num<Value>>) -> Self {
        Step {
            key_num: key_num,
            values: values,
            pos: 0,
        }
    }

    fn advance(&mut self) {
        self.pos += 1;
    }

    fn has_next(&mut self) -> bool {
        self.pos < self.values.len() - 1
    }

    fn assignment(&self) -> Assignment {
        Assignment {
            key_num: self.key_num,
            value_num: self.values[self.pos],
        }
    }
}

impl<'d> Backtracker<'d> {
    pub fn new(domain: &'d Domain) -> Self {
        Backtracker {
            domain_walker: DomainWalker::new(domain),
            stack: Vec::with_capacity(domain.keys.count().as_usize()),
            unassigned: HashSet::from_iter(domain.keys.nums()),
        }
    }

    pub fn generate(&mut self) -> Result<()> {
        loop {
            while !self.domain_walker.valid_state() {
                // reached invalid state; backtrack
                println!("{:?}", self.domain_walker.mapping());
                println!("INVALID");

                self.next()?;
            }

            if let Some(key_num) = self.next_key() {
                self.descend(key_num);
            } else {
                // all keys assigned! We are done.
                println!("{:?}", self.domain_walker.mapping());
                return Ok(());
            }
        }
    }

    fn next_key(&self) -> Option<Num<Key>> {
        // Select most constrained key first for fail-first strategy
        self.unassigned.iter().cloned().min_by_key(|&key_num| {
            self.domain_walker.range(key_num).accepted().len()
        })
        // self.unassigned.iter().cloned().min()
    }

    fn next(&mut self) -> Result<()> {
        while !self.current_step().has_next() {
            self.ascend();
        }

        if !self.stack.is_empty() {
            self.goto_next_sibling();
            return Ok(());
        }

        bail!("no more candidates")
    }

    fn descend(&mut self, key_num: Num<Key>) {
        let range = self.domain_walker.range_for(key_num).iter().cloned().collect();
        let step = Step::new(key_num, range);
        self.stack.push(step);
        self.assign_pos();
    }   

    fn ascend(&mut self) {
        self.unassign_pos();
        self.stack.pop();
    }

    // assumes there is an unvisited sibling at the current level
    fn goto_next_sibling(&mut self) {
        self.unassign_pos();
        self.current_step().advance();
        self.assign_pos();
    }

    fn assign_pos(&mut self) {
        let a = self.current_step().assignment();
        self.assign(a);
    }

    fn unassign_pos(&mut self) {
        let a = self.current_step().assignment();
        self.unassign(a);
    }

    fn current_step<'a>(&'a mut self) -> &'a mut Step {
        let idx = self.stack.len() - 1;
        &mut self.stack[idx]
    }

    fn assign(&mut self, assignment: Assignment) {
        self.domain_walker.assign(assignment.key_num, assignment.value_num);
        self.unassigned.remove(&assignment.key_num);
    }

    fn unassign(&mut self, assignment: Assignment) {
        self.domain_walker.unassign(assignment.key_num);
        self.unassigned.insert(assignment.key_num);
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use failure::ResultExt;
    use cat::ops::*;
    use json;

    fn mapping_valid(mapping: &Table<Key, Num<Value>>, domain: &Domain) -> bool {
        // TODO: maybe give a reason
        for (key_num, &value_num) in mapping.enumerate() {
            if !domain.key_restrictions[key_num].allows(value_num) {
                return false;
            }

            for (other_key, &other_value) in mapping.enumerate() {
                let restrictor = &domain.constraint_table[key_num][other_key];
                if !restrictor[value_num].allows(other_value) {
                    return false;
                }
            }
        }

        return true;
    }


    #[test]
    fn test_sudoku() {
        // TODO: clean suite of files to test on, maybe
        let domain = json::read_config("sudoku.json")
            .context("Could not parse domain").unwrap();
        
        let mut g = Backtracker::new(&domain);
        g.generate().unwrap();
        // TODO: implement mapmaybe?
        let mapping = g.domain_walker.mapping().map(|e| e.unwrap());

        assert!(mapping_valid(&mapping, &domain))
    }
}