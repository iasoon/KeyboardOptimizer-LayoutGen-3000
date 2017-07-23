use data::*;
use layout::Layout;
use eval::walker::*;

pub trait Evaluator {
    fn eval(&self, layout: &Layout) -> f64;
    fn walker<'e>(&'e self, driver: &mut WalkerDriver<'e>) -> Box<WalkableEval<'e> + 'e>;
}

pub struct Eval {
    pub evaluators: Vec<Box<Evaluator>>,
}

impl Evaluator for Eval {
    fn eval(&self, layout: &Layout) -> f64 {
        self.evaluators.iter().map(|eval| eval.eval(layout)).sum()
    }

    fn walker<'e>(&'e self, driver: &mut WalkerDriver<'e>) -> Box<WalkableEval<'e> + 'e> {
        Box::new(self.eval_walker(driver))
    }
}

impl Eval {
    pub fn eval_walker<'e>(&'e self, driver: &mut WalkerDriver<'e>) -> EvalWalker<'e> {
        EvalWalker {
            walkable_evals: self.evaluators
                .iter()
                .map(|eval| eval.walker(driver))
                .collect()
        }
    }
}

pub struct EvalWalker<'e> {
    walkable_evals: Vec<Box<WalkableEval<'e> + 'e>>
}

impl<'e> WalkableEval<'e> for EvalWalker<'e> {
    fn eval_delta<'w>(&'w mut self, driver: &'w mut WalkerDriver<'e>, delta: &[Assignment]) -> f64 {
        self.walkable_evals
            .iter_mut()
            .map(|eval| eval.eval_delta(driver, delta))
            .sum()
    }

    fn update<'w>(&'w mut self, driver: &'w mut WalkerDriver<'e>, delta: &[Assignment]) {
        for eval in self.walkable_evals.iter_mut() {
            eval.update(driver, delta);
        }
    }
}
