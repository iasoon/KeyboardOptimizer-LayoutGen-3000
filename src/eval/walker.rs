use data::*;
use cat::*;
use layout::*;

type GroupMap = Table<Group, Num<Key>>;

pub struct LtWalker<'a> {
    pub kb_def: &'a KbDef,
    keymap: Keymap,
    group_map: GroupMap,
    token_map: TokenMap,
    saved_locs: Vec<usize>,
    breadcrumbs: Vec<Assignment>,
}

impl<'a> Assignable for LtWalker<'a> {
    fn assign(&mut self, kb_def: &KbDef, assignment: Assignment) {
        if self.saved_locs.len() > 0 {
            self.leave_breadcrumb(assignment);
        }
        self.assign_raw(assignment);
    }

    fn assign_token(&mut self, token_num: Num<Token>, loc_num: Num<Loc>) {
        *self.token_map.get_mut(token_num) = loc_num;
    }
}

impl<'a> LtWalker<'a> {
    fn assign_raw(&mut self, assignment: Assignment) {
        self.dispatch_assignment(self.kb_def, assignment);
    }

    fn leave_breadcrumb(&mut self, assignment: Assignment) {
        let breadcrumb = self.inverse(assignment);
        self.breadcrumbs.push(breadcrumb);
    }

    pub fn save_loc(&mut self) {
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
                let token_num = *self.kb_def.frees.get(free_num);
                let current_loc = *self.token_map.get(token_num);
                Assignment::Free { free_num, loc_num: current_loc }
            },
            Assignment::Lock { lock_num, key_num: _ } => {
                let group = Group::Lock(lock_num);
                let group_num = self.kb_def.group_num().apply(group);
                let current_key = *self.group_map.get(group_num);
                Assignment::Lock { lock_num, key_num: current_key }
            }
        }
    }

    pub fn with_eval<E: 'a>(&'a mut self, eval: &'a mut E) -> Walker<'a, E> {
        Walker {
            layout_walker: self,
            eval_walker: eval,
        }
    }
}

pub struct Walker<'e, E: 'e> {
    layout_walker: &'e mut LtWalker<'e>,
    pub eval_walker: &'e mut E,
}

pub trait EvalWalker : Assignable {
    fn eval_delta<'a>(&'a mut self, walker: &'a mut LtWalker<'a>, delta: &[Assignment]) -> f64;
    fn update<'a>(&'a mut self, walker: &'a mut LtWalker<'a>, delta: &[Assignment]);
}

impl<'e, E: 'e> Walker<'e, E>
    where E: EvalWalker
{
    fn save_loc(&mut self) {
        self.layout_walker.save_loc();
    }

    fn pop_loc(&mut self) {
        let kb_def = self.layout_walker.kb_def;
        let eval_walker = &mut self.eval_walker;
        self.layout_walker.pop_loc(|assignment| {
            eval_walker.assign(kb_def, assignment);
        });
    }

    pub fn assign(&mut self, assignment: Assignment) {
        let kb_def = self.layout_walker.kb_def;
        self.layout_walker.assign(kb_def, assignment);
        self.eval_walker.assign(kb_def, assignment)
    }

    pub fn assign_all(&mut self, assignments: &[Assignment]) {
        for &assignment in assignments.iter() {
            self.assign(assignment);
        }
    }

    pub fn excursion<F, R>(&mut self, fun: F) -> R
        where F: FnOnce(&mut Walker<'e, E>) -> R
    {
        self.save_loc();
        let res = fun(self);
        self.pop_loc();

        return res;
    }

    pub fn measure_effect<F, M>(&mut self, mutation: M, eval: F) -> f64
        where F: Fn(&mut Walker<'e, E>) -> f64,
              M: FnOnce(&mut Walker<'e, E>)
    {
        self.excursion(|walker| {
            walker.measure_effect_mut(mutation, eval)
        })
    }

    pub fn measure_effect_mut<F, M>(&mut self, mutation: M, eval: F) -> f64
        where F: Fn(&mut Walker<'e, E>) -> f64,
              M: FnOnce(&mut Walker<'e, E>)
    {
        let before = eval(self);
        mutation(self);
        let after = eval(self);
        return after - before;
    }
}