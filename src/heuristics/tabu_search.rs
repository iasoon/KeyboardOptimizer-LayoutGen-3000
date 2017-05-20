use layout::*;
use eval::Evaluator;
use utils::{LookupTable, Countable};
use model::{KbDef, GroupId, KeyId, LockId};
use heuristics::{Walker, Traverser};

use rand::{thread_rng, Rng};

pub struct TabuSearcher<'a> {
    layout: Layout<'a>,
    evaluator: &'a Evaluator,
    traverser: Traverser<'a>,
}

#[derive(Debug)]
struct ScoredAlteration {
    pub assignments: Vec<Assignment>,
    pub delta: f64,
}
impl<'a> TabuSearcher<'a> {
    pub fn new(layout: Layout<'a>, evaluator: &'a Evaluator) -> Self {
        TabuSearcher {
            traverser: Traverser::new(&layout, evaluator),
            layout: layout,
            evaluator: evaluator,
        }
    }

    fn alter(&mut self, assignments: &[Assignment]) {
        self.traverser.update(assignments);
        for &assignment in assignments.iter() {
            self.layout.assign(assignment);
        }
    }

    fn best_move(&mut self) -> ScoredAlteration {
        let traverser = &mut self.traverser;
        self.layout
            .moves()
            .map(|assignments| {
                let delta = traverser.score_assignments(assignments.as_slice());
                let check = traverser.walker.delta(assignments.as_slice());
                let tol = (10.0 as f64).powi(-12);
                if (delta - check).abs() > tol {
                    println!("ERROR: expected {} but was {}, diff: {}", check, delta, check - delta);
                }
                ScoredAlteration {
                    assignments: assignments,
                    delta: delta,
                }
            })
            .min_by(|ref a, ref b| a.delta.partial_cmp(&b.delta).unwrap())
            .unwrap()
    }

    pub fn bench(&mut self) {
        for _ in 0..100 {
            let moves = {
                let traverser = &mut self.traverser;
                self.layout.moves().map(|assignments| {
                    ScoredAlteration {
                        delta: traverser.score_assignments(assignments.as_slice()),
                        assignments: assignments,
                    }
                }).collect::<Vec<_>>()
            };

            let min = moves.iter().min_by(|ref a, ref b| a.delta.partial_cmp(&b.delta).unwrap()).unwrap();
            println!("best move: {}", min.delta);

            let mv = thread_rng().choose(moves.as_slice()).unwrap();
            self.alter(mv.assignments.as_slice());
            self.layout.print();
        }
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
            self.alter(mv.assignments.as_slice());
            self.layout.print();
            println!("score: {}", self.evaluator.evaluate(&self.layout.group_map));
        }
    }
}
