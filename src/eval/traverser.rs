use data::*;
use layout::{Layout, Assignable};
use std::marker::PhantomData;

use eval::walker::*;
use eval::evaluator::*;

pub struct Traverser<'e> {
    layout: Layout<'e>,
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

    pub fn assign_all(&mut self, assignments: &[Assignment]) {
        self.layout.assign_all(assignments);
        self.eval.update(assignments);
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
