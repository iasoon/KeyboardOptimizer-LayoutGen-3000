use data::*;
use layout::Layout;
use eval::walker::*;

pub trait Evaluator<'e> {
    type Walker;

    fn eval(&self, layout: &Layout) -> f64;
    fn walker(&'e self, driver: &'e mut WalkerDriver<'e>) -> Self::Walker;
}
