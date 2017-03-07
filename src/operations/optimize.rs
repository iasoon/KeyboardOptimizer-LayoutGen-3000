use std::path::Path;
use parser::{Parser, KbParser};
use errors::*;

use operations::Operation;

pub struct Optimize<'a> {
    pub kb_conf: &'a Path,
    pub corpus: &'a Path,
    pub score_tree: &'a Path,
}

impl<'a> Operation for Optimize<'a> {
    fn run(&self) -> Result<()> {
        Ok(())
    }
}
