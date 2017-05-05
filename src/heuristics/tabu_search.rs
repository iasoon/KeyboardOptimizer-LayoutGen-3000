use layout::*;
use eval::Evaluator;
use utils::LookupTable;
use model::{GroupId, KeyId};

pub struct TabuSearcher<'a> {
    layout: Layout<'a>,
    evaluator: &'a Evaluator,
}

#[derive(Debug)]
struct ScoredAlteration {
    pub alteration: Alteration,
    pub delta: f64,
}

impl<'a> TabuSearcher<'a> {
    pub fn new(layout: Layout<'a>, evaluator: &'a Evaluator) -> Self {
        TabuSearcher {
            layout: layout,
            evaluator: evaluator,
        }
    }

    pub fn best_move(&self) -> ScoredAlteration {
        let mut walker = GroupMapWalker::new(&self.layout.group_map, &self.layout.kb_def);
        self.layout
            .moves()
            .map(|alteration| {
                let deassign: f64 = alteration.groups(&self.layout.kb_def).map(|group_id| {
                    self.evaluator.eval_group(group_id, walker.pos())
                }).sum();

                walker.do_move(&alteration);

                let assign: f64 = alteration.groups(&self.layout.kb_def).map(|group_id| {
                    self.evaluator.eval_group(group_id, walker.pos())
                }).sum();
                walker.reset_move(&alteration);
                ScoredAlteration {
                    alteration: alteration,
                    delta: assign - deassign,
                }
            })
            .min_by(|ref a, ref b| a.delta.partial_cmp(&b.delta).unwrap())
            .unwrap()
    }

    pub fn climb(&mut self) {
        let mut iter = 0;
        loop {
            iter += 1;
            println!("iteration {}", iter);
            let mv = self.best_move();
            if mv.delta >= 0.0 {
                println!("best move was {}", mv.delta);
                return;
            }
            self.layout.do_move(&mv.alteration);
            self.layout.print();
            println!("score: {}", self.evaluator.evaluate(&self.layout.group_map));
        }
    }
}
