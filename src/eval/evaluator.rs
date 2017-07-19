use data::*;
use layout::Layout;
use eval::walker::*;

pub trait Evaluator<'e> {
    type Walker: EvalWalker<'e>;

    fn eval(&self, layout: &Layout) -> f64;
    fn walker(&'e self, lt_walker: &mut LtWalker) -> Self::Walker;
}
