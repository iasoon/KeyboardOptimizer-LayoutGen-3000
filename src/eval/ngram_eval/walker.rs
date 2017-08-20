// TODO: some docs for this file

use cat::*;
use data::*;
use layout::*;

use eval::walker::*;
use eval::ngram_eval::types::*;
use eval::ngram_eval::eval::*;

/// A walker for the n-gram evaluator.
pub struct NGramWalker<'e, T: 'e, P: 'e> {
    eval: &'e NGramEval<T, P>,
    assignment_delta: AssignmentTable<'e, f64>,
}


impl<'e, T: 'e, P: 'e> Assignable for NGramWalker<'e, T, P> {
    fn assign(&mut self, _: &KbDef, _: Assignment) {
        // Do nothing
    }
}


pub trait HasMapping<T, P> {
    fn mapping<'m>(&'m self) -> &'m Table<T, Num<P>>;
}


impl<'a, 'e, T: 'e, P: 'e> Walker<'a, 'e, NGramWalker<'e, T, P>>
    where Self: HasMapping<T, P>,
{
    fn cost<'b>(&'b self) -> NGramCost<'b, T, P> {
        self.evaluator().ngram_cost(self.mapping())
    }

    fn evaluator<'b>(&'b self) -> &'b NGramEval<T, P> {
        self.eval.eval
    }

    fn eval_group(&self, group: Group) -> f64 {
        self.evaluator().group_ngrams(group).eval(self.cost())
    }

    fn eval_intersection(&self, ts: [Num<Group>; 2]) -> f64 {
        self.evaluator().intersection(ts).eval(self.cost())
    }

    fn recalc_group(&mut self, group_num: Num<Group>, delta: &[Assignment]) {
        let kb_def = self.driver.kb_def;
        self.excursion(|walker| {
            walker.assign_all(delta);
            for &assignment in kb_def.group_assignments[group_num].iter() {
                walker.recalc_delta(assignment);
            }
        })
    }

    fn recalc_delta(&mut self, assignment: Assignment) {
        let delta = self.measure_effect(
            |walker| walker.assign(assignment),
            |walker| walker.eval_group(assignment.group())
        );
        self.eval.assignment_delta[assignment] = delta;
    }

    fn update_group(&mut self, group_num: Num<Group>, delta: &[Assignment]) {
        self.excursion(|walker| {
            for &assignment in delta.iter() {
                walker.update_group_step(group_num, assignment);
                walker.assign(assignment);
            }
        })
    }

    fn update_group_step(&mut self, group_num: Num<Group>, change: Assignment) {
        let kb_def = self.driver.kb_def;
        let gs = [change.group_num(kb_def), group_num];
        let intersection = self.eval.eval.intersection(gs);

        let before = self.measure_effect(
            |walker| walker.assign(change),
            |walker| intersection.eval(walker.cost())
        );

        for &assignment in kb_def.group_assignments[group_num].iter() {
            self.excursion(|walker| {
                walker.assign(assignment);
                let after = walker.measure_effect(
                    |walker| walker.assign(change),
                    |walker| intersection.eval(walker.cost())
                );
                walker.eval.assignment_delta[assignment] += after - before;
            });
        }
    }

    fn eval_delta_delta(&mut self, assignment: Assignment, assignments: &[Assignment]) -> f64 {
        let kb_def = self.driver.kb_def;
        self.excursion(|walker| {
            assignments.iter().map(|&a| {
                walker.measure_effect_mut(
                    |walker| walker.assign(a),
                    |walker| walker.measure_effect(
                        |walker| walker.assign(assignment),
                        |walker| walker.eval_intersection([
                            assignment.group_num(kb_def),
                            a.group_num(kb_def)
                        ])
                    )
                )
            }).sum()
        })
    }
}

impl<'e, T: 'e, P: 'e> WalkableEval<'e> for NGramWalker<'e, T, P>
    where for<'w> Walker<'w, 'e, Self> : HasMapping<T, P>
{
    fn eval_delta<'w>(&'w mut self, driver: &'w mut WalkerDriver<'e>, assignments: &[Assignment]) -> f64 {
        assignments.iter().enumerate().map(|(idx, &assignment)| {
            let delta = self.assignment_delta[assignment];
            let delta_delta = driver.drive(self)
                .eval_delta_delta(assignments[idx], &assignments[0..idx]);
            return delta + delta_delta;
        }).sum()
    }

    fn update<'w>(&'w mut self, driver: &'w mut WalkerDriver<'e>, delta: &[Assignment]) {
        let kb_def = driver.kb_def;
        for group_num in kb_def.group_num().nums() {
            if delta.iter().any(|a| a.group_num(kb_def) == group_num) {
                driver.drive(self).recalc_group(group_num, delta);
            } else {
                driver.drive(self).update_group(group_num, delta);
            }
        }
    }
}

impl<'e> NGramWalker<'e, Group, Key> {
    pub fn new(eval: &'e NGramEval<Group, Key>, driver: &mut WalkerDriver<'e>) -> Self {
        let mut walker = NGramWalker {
            eval: eval,
            assignment_delta: AssignmentTable::new(driver.kb_def, |_| 0.0),
        };
        // init cache
        for (_, &assignment) in driver.kb_def.assignments.enumerate() {
            driver.drive(&mut walker).recalc_delta(assignment);
        }
        return walker;
    }
}
