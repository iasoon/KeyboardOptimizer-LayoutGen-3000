use data::*;
use layout::Layout;
use std::marker::PhantomData;

use eval::walker::*;

struct Traverser<'e> {
    layout: Layout<'e>,
    eval: WalkingEval<'e>,
}

struct WalkingEval<'e> {
    eval_walkers: Vec<Box<WalkableEval<'e>>>,
    driver: WalkerDriver<'e>,
}


impl<'e> WalkingEval<'e> {
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
