use data::score_tree::{Group, Children, Elem, Loc as LocData};
use parser::{Parser, KbParser};
use model::{Language, Loc, TokenId};
use layout::Keymap;
use std::result;
use errors::*;

pub struct ScoreTreeEvaluator<'a> {
    pub parser: KbParser<'a>,
    pub language: Language,
    pub keymap: Keymap,
}

impl<'a> ScoreTreeEvaluator<'a> {
    pub fn eval(&self, tree: &Group) -> Result<Group> {
        self.eval_group(tree)
    }

    fn eval_group(&self, group: &Group) -> Result<Group> {
        match group.children {
            Children::Groups(ref groups) => self.mk_group(&group.label, groups),
            Children::Elems(ref elems) => self.mk_group(&group.label, elems),
        }
    }

    fn mk_group<C>(&self, label: &String, children: &Vec<C>) -> Result<Group>
        where C: Child
    {
        let mut weight = 0.0;
        let mut vec = Vec::with_capacity(children.len());

        for child in children.iter() {
            let child_ = child.eval(self)?;
            weight += child.weight() * child_.weight();
            vec.push(child_);
        }

        Ok(Group {
            label: label.clone(),
            weight: weight,
            children: C::wrap_vec(vec),
        })
    }

    fn eval_elem(&self, elem: &Elem) -> Result<Elem> {
        Ok(Elem {
            path: elem.path.clone(),
            weight: self.elem_score(&elem.path)?,
        })
    }

    fn elem_score(&self, path: &Vec<LocData>) -> Result<f64> {
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

type LocValue = result::Result<TokenId, LocError>;

enum LocError {
    ParseError(Error),
    Unassigned,
}

use self::LocError::*;

trait Child {
    fn weight(&self) -> f64;
    fn eval(&self, evaluator: &ScoreTreeEvaluator) -> Result<Self>
        where Self: Sized;
    fn wrap_vec(vec: Vec<Self>) -> Children where Self: Sized;
}

impl Child for Elem {
    fn weight(&self) -> f64 {
        self.weight
    }

    fn eval(&self, evaluator: &ScoreTreeEvaluator) -> Result<Elem> {
        evaluator.eval_elem(&self)
    }

    fn wrap_vec(vec: Vec<Elem>) -> Children {
        Children::Elems(vec)
    }
}

impl Child for Group {
    fn weight(&self) -> f64 {
        self.weight
    }

    fn eval(&self, evaluator: &ScoreTreeEvaluator) -> Result<Group> {
        evaluator.eval_group(&self)
    }

    fn wrap_vec(vec: Vec<Group>) -> Children {
        Children::Groups(vec)
    }
}
