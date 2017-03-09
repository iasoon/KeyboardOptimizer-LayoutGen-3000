use data::score_tree::{Group, Elem, Loc as LocData};
use parser::{Parser, KbParser};
use model::{Language, Loc, TokenId};
use layout::Keymap;
use std::result;
use errors::*;

use utils::score_tree_walker as walker;
use self::walker::ScoreTreeWalker;

pub struct ScoreTreeEvaluator<'a> {
    pub parser: KbParser<'a>,
    pub language: Language,
    pub keymap: Keymap,
}

type LocValue = result::Result<TokenId, LocError>;

enum LocError {
    ParseError(Error),
    Unassigned,
}

use self::LocError::*;

impl<'a> ScoreTreeEvaluator<'a> {
    pub fn eval(&self, group: &Group) -> Result<Group> {
        Walker::new(self).visit_group(group)
    }

    fn path_weight(&self, path: &Vec<LocData>) -> Result<f64> {
        let table = &self.language.freqs[path.len()];
        match table.freq(self.read_path(path)) {
            Ok(score) => Ok(score),
            Err(Unassigned) => Ok(0.0),
            Err(ParseError(e)) => Err(e),
        }
    }

    fn read_path<'b>(&'b self, path: &'b Vec<LocData>) -> impl Iterator<Item = LocValue> + 'b {
        path.iter().map(move |locdata| {
            let loc: Loc = self.parser.parse(locdata).map_err(|e| ParseError(e))?;
            return self.keymap[loc].ok_or(Unassigned);
        })
    }
}

struct Walker<'a> {
    evaluator: &'a ScoreTreeEvaluator<'a>,
    weight: f64,
}

impl<'a> Walker<'a> {
    fn new(evaluator: &'a ScoreTreeEvaluator<'a>) -> Self {
        Walker {
            evaluator: evaluator,
            weight: 0.0,
        }
    }
}

impl<'a> ScoreTreeWalker<Elem, Group> for Walker<'a> {
    fn visit_elem(&mut self, elem: &Elem) -> Result<Elem> {
        let elem_weight = self.evaluator.path_weight(&elem.path)?;
        self.weight += elem.weight * elem_weight;
        Ok(Elem {
            path: elem.path.clone(),
            weight: elem_weight,
        })
    }

    fn visit_group(&mut self, group: &Group) -> Result<Group> {
        let mut walker = Walker::new(self.evaluator);
        let children = walker::map_children(&mut walker, &group.children)?;
        self.weight += group.weight * walker.weight;
        Ok(Group {
            label: group.label.clone(),
            weight: walker.weight,
            children: children,
        })
    }
}
