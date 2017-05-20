use model::{KbDef, GroupId, Group, Loc};
use utils::*;
use layout::{Layout, Assignment, AssignmentData};
use eval::Evaluator;
use heuristics::Walker;

fn group_assignments(group_id: GroupId, kb_def: &KbDef) -> Vec<Assignment> {
    match kb_def.groups[group_id] {
        Group::Free(free_id) => {
            Loc::enumerate(kb_def.loc_data()).map(|loc| {
                Assignment::Free {
                    free_id: free_id,
                    loc: loc,
                }
            }).collect()
        },
        Group::Locked(lock_id) => {
            kb_def.keys.ids().map(|key_id| {
                Assignment::Lock {
                    lock_id: lock_id,
                    key_id: key_id,
                }
            }).collect()
        }
    }
}

fn init_assignment_delta(layout: &Layout, evaluator: &Evaluator) -> LookupTable<Assignment, f64> {
    let mut walker = Walker::new(&layout, evaluator);
    let data = AssignmentData::new(layout.kb_def);
    LookupTable::from_fn(data, |assignment| walker.delta(&[assignment]))
}

pub struct Traverser<'a> {
    pub walker: Walker<'a>,
    evaluator: &'a Evaluator,
    assignment_delta: LookupTable<Assignment, f64>,
}

impl<'a> Traverser<'a> {
    pub fn new(layout: &Layout<'a>, evaluator: &'a Evaluator) -> Self {
        Traverser {
            assignment_delta: init_assignment_delta(layout, evaluator),
            evaluator: evaluator,
            walker: Walker::new(layout, evaluator),
        }
    }

    // for when alteration_group != group_id
    fn update_group(&mut self, alteration: Assignment, group_id: GroupId) {
        let kb_def = self.walker.kb_def;
        let alteration_group = alteration.group(kb_def);
        let assignment_delta = &mut self.assignment_delta;

        let before = self.walker.measure_effect(
            |walker| walker.assign(alteration),
            |walker| walker.eval_component(&[group_id, alteration_group]));

        for assignment in group_assignments(group_id, kb_def).into_iter() {
            self.walker.excursion(|walker| {
                walker.assign(assignment);
                let after = walker.measure_effect(
                    |walker| walker.assign(alteration),
                    |walker| walker.eval_component(&[group_id, alteration_group]));
                assignment_delta[assignment] += after - before;
            });
        }
    }

    fn update_changed_group(&mut self, group_id: GroupId) {
        let kb_def = self.walker.kb_def;

        let before = self.walker.eval_component(&[group_id]);
        for assignment in group_assignments(group_id, kb_def).into_iter() {
            let after = self.walker.excursion(|walker| {
                walker.assign(assignment);
                return walker.eval_component(&[group_id]);
            });
            self.assignment_delta[assignment] = after - before;
        }
    }

    pub fn update(&mut self, assignments: &[Assignment]) {
        let kb_def = self.walker.kb_def;

        let mut changed = LookupTable::new(kb_def.groups.elem_count(), false);
        for &assignment in assignments.iter() {
            changed[assignment.group(kb_def)] = true;
        }

        for &assignment in assignments.iter() {
            for group_id in kb_def.groups.ids() {
                if !changed[group_id] {
                    self.update_group(assignment, group_id);
                }
            }
            self.walker.assign(assignment);
        }

        for &assignment in assignments.iter() {
            self.update_changed_group(assignment.group(kb_def));
        }
    }

    pub fn score_assignments(&mut self, assignments: &[Assignment]) -> f64 {
        // assuming there are 2 assignments
        let a1 = assignments[0];
        let a2 = assignments[1];

        let group1 = a1.group(self.walker.kb_def);
        let group2 = a2.group(self.walker.kb_def);

        self.assignment_delta[a1] + self.assignment_delta[a2] + self.walker.excursion(|walker| {
            walker.measure_effect(
                |walker| walker.assign(a2),
                |walker| walker.measure_effect(
                    |walker| walker.assign(a1),
                    |walker| walker.eval_component(&[group1, group2])))
        })
    }
}
