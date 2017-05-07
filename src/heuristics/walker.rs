use layout::{Layout, GroupMap, Assignment};
use model::{KbDef, GroupId, KeyId};
use eval::Evaluator;


pub struct Walker<'a> {
    start: &'a GroupMap,
    pos: GroupMap,
    kb_def: &'a KbDef,
    evaluator: &'a Evaluator,
}

impl<'a> Walker<'a> {
    pub fn new(layout: &'a Layout, evaluator: &'a Evaluator) -> Self {
        Walker {
            start: &layout.group_map,
            pos: layout.group_map.clone(),
            kb_def: layout.kb_def,
            evaluator: evaluator,
        }
    }

    pub fn alteration_delta(&mut self, alteration: &[Assignment], assignments: &[Assignment]) -> f64 {
        let mut delta = 0.0;
        for &assignment in assignments.iter() {
            let before = self.calc_overlap(assignment, alteration);
            self.assign(assignment);
            let after = self.calc_overlap(assignment, alteration);
            delta += after - before;
        }
        self.reset_assignments(assignments);
        return delta;
    }

    fn calc_overlap(&self, assignment: Assignment, assignments: &[Assignment]) -> f64 {
        let group1 = assignment.group(self.kb_def);
        assignments.iter().map(|assignment| {
            let group2 = assignment.group(self.kb_def);
            self.evaluator.eval_overlap(group1, group2, &self.pos)
        }).sum()
    }


    pub fn delta(&mut self, assignments: &[Assignment]) -> f64 {
        let mut delta = 0.0;
        for &assignment in assignments.iter() {
            let group_id = assignment.group(self.kb_def);
            let before = self.evaluator.group_component(group_id, &self.pos);
            self.assign(assignment);
            let after = self.evaluator.group_component(group_id, &self.pos);
            delta += after - before;
        }
        self.reset_assignments(assignments);
        return delta;
    }

    pub fn assign(&mut self, assignment: Assignment) {
        match assignment {
            Assignment::Free { free_id, loc } => {
                let group_id = self.kb_def.free_group[free_id];
                self.assign_group(group_id, loc.key(&self.kb_def.loc_data()));
            },
            Assignment::Lock { lock_id, key_id } => {
                let group_id = self.kb_def.lock_group[lock_id];
                self.assign_group(group_id, key_id);
            }
        }
    }

    pub fn reset_assignments(&mut self, assignments: &[Assignment]) {
        for &assignment in assignments.iter() {
            self.reset_assignment(assignment);
        }
    }

    pub fn reset_assignment(&mut self, assignment: Assignment) {
        match assignment {
            Assignment::Free { free_id , loc: _ } => {
                let group_id = self.kb_def.free_group[free_id];
                self.reset_group(group_id);
            },
            Assignment::Lock { lock_id, key_id: _ } => {
                let group_id = self.kb_def.lock_group[lock_id];
                self.reset_group(group_id);
            }
        }
    }

    fn assign_group(&mut self, group_id: GroupId, key_id: KeyId) {
        self.pos[group_id] = key_id;
    }

    fn reset_group(&mut self, group_id: GroupId) {
        self.pos[group_id] = self.start[group_id];
    }
}
