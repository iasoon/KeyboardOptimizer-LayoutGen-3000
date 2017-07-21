use data::*;
use layout::{Layout, Assignable};
use std::marker::PhantomData;

use eval::walker::*;
use eval::evaluator::Evaluator;

pub struct Traverser<'e> {
    layout: Layout<'e>,
    eval: WalkingEval<'e>,
}

pub struct Delta {
    pub assignments: Vec<Assignment>,
    pub score: f64,
}

impl<'e> Traverser<'e> {
    pub fn new(evals: &'e [Box<Evaluator>], layout: Layout<'e>) -> Self {
        Traverser {
            eval: WalkingEval::new(&layout, evals),
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

struct WalkingEval<'e> {
    eval_walkers: Vec<Box<WalkableEval<'e> + 'e>>,
    driver: WalkerDriver<'e>,
}


impl<'e> WalkingEval<'e> {
    fn new(layout: &Layout<'e>, evals: &'e [Box<Evaluator>]) -> Self {
        let mut driver = WalkerDriver::new(&layout);
        let eval_walkers = evals.iter().map(|e| e.walker(&mut driver)).collect();
        WalkingEval { driver, eval_walkers }
    }

    fn eval_delta(&mut self, assignments: &[Assignment]) -> f64 {
        let driver = &mut self.driver;
        self.eval_walkers.iter_mut().map(|eval| {
            eval.eval_delta(driver, assignments)
        }).sum()
    }

    fn update(&mut self, assignments: &[Assignment]) {
        let driver = &mut self.driver;
        for eval in self.eval_walkers.iter_mut() {
            eval.update(driver, assignments);
        }
        driver.assign_all(assignments);
    }
}
