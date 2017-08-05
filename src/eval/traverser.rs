use data::*;
use layout::{Layout, Assignable};
use std::marker::PhantomData;

use eval::walker::*;
use eval::evaluator::*;

pub struct Traverser<'e> {
    layout: Layout<'e>,
    score: f64,
    eval: WalkingEval<'e>,
}

pub struct Delta {
    pub assignments: Vec<Assignment>,
    pub score: f64,
}

impl<'e> Traverser<'e> {
    pub fn new(eval: &'e Eval, layout: Layout<'e>) -> Self {
        Traverser {
            eval: WalkingEval::new(&layout, eval),
            score: eval.eval(&layout),
            layout: layout,
        }
    }

    pub fn deltas<'a>(&'a mut self) -> impl Iterator<Item = Delta> + 'a {
        let eval = &mut self.eval;
        self.layout.gen_moves().map(move |assignments| {
            Delta {
                score: eval.eval_delta(assignments.as_slice()),
                assignments: assignments,
            }
        })
    }

    pub fn assign(&mut self, delta: &Delta) {
        self.layout.assign_all(&delta.assignments);
        self.eval.update(&delta.assignments);
        self.score += delta.score;
    }

    pub fn inverse(&self, assignment: Assignment) -> Assignment {
        self.eval.driver.inverse(assignment)
    }

    pub fn position<'a>(&'a self) -> &'a Layout<'e> {
        &self.layout
    }

    pub fn position_score(&self) -> f64 {
        self.score
    }
}

pub struct WalkingEval<'e> {
    eval_walker: EvalWalker<'e>,
    driver: WalkerDriver<'e>,
}

impl<'e> WalkingEval<'e> {
    pub fn new(layout: &Layout<'e>, eval: &'e Eval) -> Self {
        let mut driver = WalkerDriver::new(&layout);
        let eval_walker = eval.eval_walker(&mut driver);
        return WalkingEval { driver, eval_walker };
    }

    pub fn eval_delta(&mut self, assignments: &[Assignment]) -> f64 {
        self.eval_walker.eval_delta(&mut self.driver, assignments)
    }

    pub fn update(&mut self, assignments: &[Assignment]) {
        self.eval_walker.update(&mut self.driver, assignments);
        self.driver.assign_all(assignments);
    }
}
