use std::vec::Vec;
use std::collections::HashMap;

use parser::{Parser, KbParser};
use data::score_tree::{ScoreTree, Group, Elem, Loc as LocRepr};
use model::{Loc, LocData, Path, PathTable};

use errors::*;

use utils::score_tree_walker as walker;
use self::walker::ScoreTreeWalker;


impl<'a> Parser<PathTable> for KbParser<'a> {
    type Repr = ScoreTree;

    fn parse(&self, repr: &ScoreTree) -> Result<PathTable> {
        let mut reader = PathTableReader::new(self);
        try!(reader.read(repr));
        Ok(reader.mk_path_table())
    }
}

struct PathTableReader<'a> {
    parser: &'a KbParser<'a>,
    paths: HashMap<Vec<Loc>, f64>,
}

impl<'a> PathTableReader<'a> {
    fn new(parser: &'a KbParser<'a>) -> Self {
        PathTableReader {
            parser: parser,
            paths: HashMap::new(),
        }
    }

    fn read(&mut self, score_tree: &ScoreTree) -> Result<()> {
        let mut walker = Walker {
            parser: self.parser,
            weight: 1.0,
            paths: &mut self.paths,
        };
        walker.visit_group(score_tree)
    }

    fn extract_paths(&mut self) -> Vec<Path> {
        self.paths.drain().map(|(locs, weight)| Path {
            locs: locs,
            weight: weight,
        }).collect()
    }

    fn mk_path_table(mut self) -> PathTable {
        let loc_data = LocData {
            key_data: self.parser.kb_conf.keys.elem_count(),
            layer_data: self.parser.kb_conf.layers.elem_count(),
        };
        PathTable::new(self.extract_paths(), loc_data)
    }
}

impl<'a> Parser<Vec<Loc>> for KbParser<'a> {
    type Repr = Vec<LocRepr>;

    fn parse(&self, repr: &Vec<LocRepr>) -> Result<Vec<Loc>> {
        let mut vec = Vec::with_capacity(repr.len());
        for locdata in repr.iter() {
            vec.push(self.parse(locdata)?);
        }
        Ok(vec)
    }
}

struct Walker<'a> {
    parser: &'a KbParser<'a>,
    weight: f64,
    paths: &'a mut HashMap<Vec<Loc>, f64>,
}

impl<'a> ScoreTreeWalker<(), ()> for Walker<'a> {
    fn visit_elem(&mut self, elem: &Elem) -> Result<()> {
        let path = self.parser.parse(&elem.path)?;
        *self.paths.entry(path).or_insert(0.0) += self.weight * elem.weight;
        Ok(())
    }

    fn visit_group(&mut self, group: &Group) -> Result<()>{
        let mut walker = Walker {
            parser: self.parser,
            weight: self.weight * group.weight,
            paths: self.paths,
        };
        walker::visit_children(&mut walker, &group.children)
    }
}
