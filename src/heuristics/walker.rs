use layout::{Layout, GroupMap, TokenMap, Assignment, AssignmentAcceptor};
use model::{KbDef, GroupId, TokenId, KeyId, Loc};
use eval::Evaluator;

pub struct Walker<'a> {
    group_map: GroupMap,
    token_map: TokenMap,
    pub kb_def: &'a KbDef,
    evaluator: &'a Evaluator,
    saved_locs: Vec<usize>,
    breadcrumbs: Vec<Assignment>,
}

impl<'a> AssignmentAcceptor for Walker<'a> {
    fn assign_group(&mut self, group_id: GroupId, key_id: KeyId) {
        self.group_map[group_id] = key_id;
    }

    fn assign_token(&mut self, token_id: TokenId, loc: Loc) {
        self.token_map[token_id] = loc;
    }
}

impl<'a> Walker<'a> {
    pub fn new(layout: &Layout<'a>, evaluator: &'a Evaluator) -> Self {
        Walker {
            group_map: layout.group_map.clone(),
            token_map: layout.token_map.clone(),
            kb_def: layout.kb_def,
            evaluator: evaluator,
            breadcrumbs: Vec::new(),
            saved_locs: Vec::new(),
        }
    }

    pub fn assign(&mut self, assignment: Assignment) {
        if self.saved_locs.len() > 0 {
            self.leave_breadcrumb(assignment);
        }
        self.assign_raw(assignment);
    }

    fn assign_raw(&mut self, assignment: Assignment) {
        let kb_def = self.kb_def;
        assignment.perform(self, kb_def);
    }

    fn leave_breadcrumb(&mut self, assignment: Assignment) {
        let breadcrumb = self.inverse(assignment);
        self.breadcrumbs.push(breadcrumb);
    }

    pub fn save_loc(&mut self) {
        let pos = self.breadcrumbs.len();
        self.saved_locs.push(pos);
    }

    pub fn restore_loc(&mut self) {
        let pos = self.saved_locs.pop().unwrap();
        for _ in 0..(self.breadcrumbs.len() - pos) {
            let assignment = self.breadcrumbs.pop().unwrap();
            self.assign_raw(assignment);
        }
    }

    pub fn inverse(&self, assignment: Assignment) -> Assignment {
        match assignment {
            Assignment::Free { free_id, loc } => {
                let token_id = self.kb_def.frees[free_id].token_id;
                let current_loc = self.token_map[token_id];
                Assignment::Free { free_id, loc: current_loc }
            },
            Assignment::Lock { lock_id, key_id } => {
                let group_id = self.kb_def.lock_group[lock_id];
                let current_key = self.group_map[group_id];
                Assignment::Lock { lock_id, key_id: current_key }
            }
        }
    }

    pub fn excursion<F, R>(&mut self, fun: F) -> R
        where F: FnOnce(&mut Walker<'a>) -> R
    {
        self.save_loc();
        let res = fun(self);
        self.restore_loc();
        return res;
    }

    pub fn eval_component(&self, component: &[GroupId]) -> f64 {
        self.evaluator.eval_component(component, &self.group_map)
    }

    pub fn measure_effect<E, M>(&mut self, mut mutation: M, eval: E) -> f64
        where E: Fn(&mut Walker<'a>) -> f64,
        M: FnOnce(&mut Walker<'a>)
    {
        self.excursion(|walker| {
            walker.measure_effect_mut(mutation, eval)
        })
    }

    pub fn measure_effect_mut<E, M>(&mut self, mut mutation: M, eval: E) -> f64
        where E: Fn(&mut Walker<'a>) -> f64,
              M: FnOnce(&mut Walker<'a>)
    {
        let before = eval(self);
        mutation(self);
        let after = eval(self);
        return after - before;
    }


    pub fn measure_effect_<F>(&mut self, assignments: &[Assignment], mut fun: F) -> f64
        where F: FnMut(&mut Walker<'a>) -> f64
    {
        self.excursion(|walker| {
            let before = fun(walker);
            for &assignment in assignments.iter() {
                walker.assign(assignment);
            }
            let after = fun(walker);
            return after - before;
        })
    }

    pub fn delta(&mut self, assignments: &[Assignment]) -> f64 {
        let kb_def = self.kb_def;

        self.excursion(|walker| {
            assignments.iter().map(|&assignment| {
                let group = assignment.group(kb_def);
                walker.measure_effect_mut(
                    |walker| walker.assign(assignment),
                    |walker| walker.eval_component(&[group]))
            }).sum()
        })
    }
}
