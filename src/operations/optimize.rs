use std::path::Path;

use utils::*;
use utils::{Countable, LookupTable};
use model::*;
use layout::{Keymap};
use errors::*;
use heuristics::*;


use data::{KbDefData, KeymapReader, read_corpus, read_path_costs};

use operations::Operation;

use layout::combinator::*;
use layout::*;
use eval::Evaluator;
use layout::AssignmentResolver;
use utils::SubSequences;


pub struct Optimize<'a> {
    pub kb_def: &'a Path,
    pub corpus: &'a Path,
    pub path_costs: &'a Path,
}

impl<'a> Operation for Optimize<'a> {
    fn run(&self) -> Result<()> {
        let kb_def = KbDefData::read_from_path(self.kb_def)?;
        let corpus = read_corpus(&kb_def, self.corpus)?;
        let path_costs = read_path_costs(&kb_def, self.path_costs)?;
        let qwerty = KeymapReader::read(&kb_def, Path::new("qwerty.json"))?;

        let keymap = qwerty;
        let token_map = mk_token_map(&keymap, &kb_def);

        let eval = Evaluator::new(corpus, path_costs, &kb_def);

        let layout = Layout::from_token_map(token_map, &kb_def);
        let mut s = TabuSearcher::new(layout, &eval);
        //s.climb();
        s.bench();
        println!();
        //s.bench();
        // let assign = |lock, key| {
        //     Assignment::Lock {
        //         lock_id: LockId::from_num(&kb_def.locks.elem_count(), lock),
        //         key_id: KeyId::from_num(&kb_def.keys.elem_count(), key),
        //     }
        // };
        Ok(())
    }
}

impl<'a> Optimize<'a> {
}
