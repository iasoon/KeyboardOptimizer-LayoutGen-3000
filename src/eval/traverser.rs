use data::*;
use layout::Layout;
use std::marker::PhantomData;

use eval::walker::*;
use eval::evaluator::Evaluator;

pub struct Traverser<'e> {
    layout: Layout<'e>,
    eval: WalkingEval<'e>,
}

impl<'e> Traverser<'e> {
    fn new(evals: &'e [Box<Evaluator>], layout: Layout<'e>) -> Self {
        Traverser {
            eval: WalkingEval::new(&layout, evals),
            layout: layout,
        }
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

    fn eval_delta(&'e mut self, assignments: &[Assignment]) -> f64 {
        let driver = &mut self.driver;
        self.eval_walkers.iter_mut().map(|eval| {
            eval.eval_delta(driver, assignments)
        }).sum()
    }

    fn update(&'e mut self, assignments: &[Assignment]) {
        let driver = &mut self.driver;
        for eval in self.eval_walkers.iter_mut() {
            eval.update(driver, assignments);
        }
    }
}
