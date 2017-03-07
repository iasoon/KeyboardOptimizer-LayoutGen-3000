use std::path::Path;
use utils::json;
use parser::{Parser, KbParser};
use errors::*;

use data::{KbConf, ScoreTree};
use eval::ScoreTreeEvaluator;

use operations::Operation;

pub struct Analyze<'a> {
    pub kb_conf: &'a Path,
    pub corpus: &'a Path,
    pub score_tree: &'a Path,
    pub keymap: &'a Path,
    pub results: &'a Path,
}

struct Data {
    kb_conf: KbConf,
    score_tree: ScoreTree,
}

impl<'a> Operation for Analyze<'a> {
    fn run(&self) -> Result<()> {
        json::write(self.results, &self.calc_results()?)
            .chain_err(|| "could not write results")?;
        Ok(())
    }
}

impl<'a> Analyze<'a> {
    fn calc_results(&self) -> Result<ScoreTree> {
        let data = self.mk_data()?;
        let evaluator = self.mk_evaluator(&data.kb_conf)?;
        Ok(evaluator.eval(&data.score_tree)?)
    }

    fn mk_evaluator(&self, kb_conf: &'a KbConf) -> Result<ScoreTreeEvaluator>
    {
        let parser = KbParser::new(kb_conf);
        Ok(ScoreTreeEvaluator {
            language: parser.read_json(self.corpus)
                .chain_err(|| "could not read corpus")?,
            keymap: parser.read_json(self.keymap)
                .chain_err(|| "could not read keymap")?,
            parser: parser,
        })
    }

    fn mk_data(&self) -> Result<Data> {
        Ok(Data {
            kb_conf: json::read(self.kb_conf)
                .chain_err(|| "could not read keyboard configuration")?,
            score_tree: json::read(self.score_tree)
                .chain_err(|| "could not read score tree")?,
        })
    }
}
