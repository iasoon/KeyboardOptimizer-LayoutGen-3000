use layout::*;
use eval::Evaluator;
use utils::LookupTable;
use model::{GroupId, KeyId};

pub struct TabuSearcher<'a> {
    layout: Layout<'a>,
    evaluator: &'a Evaluator,
    group_component: LookupTable<GroupId, f64>,
}

#[derive(Debug)]
struct ScoredAlteration {
    pub alteration: Alteration,
    pub delta: f64,
}

fn init_group_components(layout: &Layout, evaluator: &Evaluator) -> LookupTable<GroupId, f64> {
    LookupTable::from_fn(layout.kb_def.groups.elem_count(),
                         |group_id| evaluator.eval_group(group_id, &layout.group_map))
}

impl<'a> TabuSearcher<'a> {
    pub fn new(layout: Layout<'a>, evaluator: &'a Evaluator) -> Self {
        TabuSearcher {
            group_component: init_group_components(&layout, evaluator),
            layout: layout,
            evaluator: evaluator,
        }
    }

    fn alter(&mut self, alteration: Alteration) {
        self.layout.do_move(&alteration);
        for group_id in self.layout.kb_def.groups.ids() {
            self.group_component[group_id] = self.evaluator
                .eval_group(group_id, &self.layout.group_map);
        }
    }


    pub fn best_move(&self) -> ScoredAlteration {
        let mut walker = GroupMapWalker::new(&self.layout.group_map, &self.layout.kb_def);
        self.layout
            .moves()
            .map(|alteration| {
                let deassign: f64 = alteration.groups(&self.layout.kb_def)
                    .map(|group_id| self.group_component[group_id])
                    .sum();

                walker.do_move(&alteration);

                let assign: f64 = alteration.groups(&self.layout.kb_def)
                    .map(|group_id| self.evaluator.eval_group(group_id, walker.pos()))
                    .sum();
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
            self.alter(mv.alteration);
            self.layout.print();
            println!("score: {}", self.evaluator.evaluate(&self.layout.group_map));
        }
    }
}
