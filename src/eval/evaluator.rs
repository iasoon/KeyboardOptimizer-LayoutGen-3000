use data::*;
use layout::Layout;
use eval::walker::*;

pub trait Evaluator<'e> {
    type Walker;

    fn eval(&self, layout: &Layout) -> f64;
    //fn walker(&'e self, kb_def: &KbDef, lt_walker: &mut LtWalker) -> Self::Walker;
}
