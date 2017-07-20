use data::*;
use layout::Layout;
use eval::walker::*;

pub trait Evaluator {
    fn eval(&self, layout: &Layout) -> f64;
    fn walker<'e>(&'e self, driver: &mut WalkerDriver<'e>) -> Box<WalkableEval<'e> + 'e>;
}
