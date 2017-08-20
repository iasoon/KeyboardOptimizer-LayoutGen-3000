use data::*;
use cat::*;
use layout::*;

type GroupMap = Table<Group, Num<Key>>;

pub struct WalkerDriver<'a> {
    pub kb_def: &'a KbDef,
    token_map: TokenMap,
    group_map: GroupMap,
    saved_locs: Vec<usize>,
    breadcrumbs: Vec<Assignment>,
}

impl<'a> Assignable for WalkerDriver<'a> {
    fn assign(&mut self, _: &KbDef, assignment: Assignment) {
        if self.saved_locs.len() > 0 {
            self.leave_breadcrumb(assignment);
        }
        self.assign_raw(assignment);
    }

    fn assign_token(&mut self, token_num: Num<Token>, loc_num: Num<Loc>) {
        self.token_map[token_num] = loc_num;
    }

    fn assign_group(&mut self, group_num: Num<Group>, key_num: Num<Key>) {
        self.group_map[group_num] = key_num;
    }
}

impl<'a> WalkerDriver<'a> {
    pub fn new(layout: &Layout<'a>) -> Self {
        WalkerDriver {
            kb_def: layout.kb_def,
            token_map: layout.token_map.clone(),
            group_map: layout.mk_group_map(),
            saved_locs: Vec::new(),
            breadcrumbs: Vec::new(),
        }
    }

    pub fn drive<'w, E>(&'w mut self, eval: &'w mut E) -> Walker<'w, 'a, E> {
        Walker {
            driver: self,
            eval: eval,
        }
    }

    pub fn assign_all(&mut self, assignments: &[Assignment]) {
        for &assignment in assignments.iter() {
            self.assign(self.kb_def, assignment);
        }
    }

    fn assign_raw(&mut self, assignment: Assignment) {
        self.dispatch_assignment(self.kb_def, assignment);
    }

    fn leave_breadcrumb(&mut self, assignment: Assignment) {
        let breadcrumb = self.inverse(assignment);
        self.breadcrumbs.push(breadcrumb);
    }

    fn save_loc(&mut self) {
        let pos = self.breadcrumbs.len();
        self.saved_locs.push(pos);
    }

    fn pop_loc<C>(&mut self, mut callback: C)
        where C: FnMut(Assignment)
    {
        let pos = self.saved_locs.pop().unwrap();
        for _ in 0..(self.breadcrumbs.len() - pos) {
            let assignment = self.breadcrumbs.pop().unwrap();
            self.assign_raw(assignment);
            callback(assignment);
        }
    }

    pub fn inverse(&self, assignment: Assignment) -> Assignment {
        match assignment {
            Assignment::Free { free_num, loc_num: _ } => {
                let token_num = self.kb_def.frees[free_num];
                let current_loc = self.token_map[token_num];
                Assignment::Free { free_num, loc_num: current_loc }
            },
            Assignment::Lock { lock_num, key_num: _ } => {
                let group = Group::Lock(lock_num);
                let group_num = self.kb_def.group_num().apply(group);
                let current_key = self.group_map[group_num];
                Assignment::Lock { lock_num, key_num: current_key }
            }
        }
    }

    pub fn group_map<'b>(&'b self) -> &'b GroupMap {
        &self.group_map
    }

    pub fn token_map<'b>(&'b self) -> &'b TokenMap {
        &self.token_map
    }
}

pub trait WalkableEval<'d> {
    fn eval_delta<'w>(&'w mut self, driver: &'w mut WalkerDriver<'d>, delta: &[Assignment]) -> f64;
    fn update<'w>(&'w mut self, driver: &'w mut WalkerDriver<'d>, delta: &[Assignment]);
}

pub struct Walker<'a, 'e: 'a, E>
    where E: 'e + ?Sized
{
    pub driver: &'a mut WalkerDriver<'e>,
    pub eval: &'a mut E,
}

impl<'a, 'e, E: 'e> Walker<'a, 'e, E>
    where E: Assignable
{
    fn save_loc(&mut self) {
        self.driver.save_loc();
    }

    fn pop_loc(&mut self) {
        let kb_def = self.driver.kb_def;
        let eval = &mut self.eval;
        self.driver.pop_loc(|assignment| {
            eval.assign(kb_def, assignment);
        });
    }

    pub fn assign(&mut self, assignment: Assignment) {
        self.driver.assign(self.driver.kb_def, assignment);
        self.eval.assign(self.driver.kb_def, assignment)
    }

    pub fn assign_all(&mut self, assignments: &[Assignment]) {
        for &assignment in assignments.iter() {
            self.assign(assignment);
        }
    }

    pub fn excursion<F, R>(&mut self, fun: F) -> R
        where F: FnOnce(&mut Walker<'a, 'e, E>) -> R
    {
        self.save_loc();
        let res = fun(self);
        self.pop_loc();

        return res;
    }

    pub fn measure_effect<F, M>(&mut self, mutation: M, eval: F) -> f64
        where F: Fn(&mut Walker<'a, 'e, E>) -> f64,
              M: FnOnce(&mut Walker<'a, 'e, E>)
    {
        self.excursion(|walker| {
            walker.measure_effect_mut(mutation, eval)
        })
    }

    pub fn measure_effect_mut<F, M>(&mut self, mutation: M, eval: F) -> f64
        where F: Fn(&mut Walker<'a, 'e, E>) -> f64,
              M: FnOnce(&mut Walker<'a, 'e, E>)
    {
        let before = eval(self);
        mutation(self);
        let after = eval(self);
        return after - before;
    }
}
