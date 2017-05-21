use layout::*;
use eval::Evaluator;
use utils::{LookupTable, Countable};
use model::{KbDef, GroupId, KeyId, LockId};
use heuristics::{Walker, Traverser};

use rand::{thread_rng, Rng};

pub struct TabuSearcher<'a> {
    traverser: Traverser<'a>,
    evaluator: &'a Evaluator,
    tabu: LookupTable<Assignment, usize>,
    iteration: usize,

    layout: Layout<'a>,
    score: f64,

    best: Layout<'a>,
    best_score: f64,
}

#[derive(Debug)]
struct ScoredAlteration {
    pub assignments: Vec<Assignment>,
    pub delta: f64,
}
impl<'a> TabuSearcher<'a> {
    pub fn new(layout: Layout<'a>, evaluator: &'a Evaluator) -> Self {
        let score = evaluator.evaluate(&layout.group_map);
        TabuSearcher {
            tabu: LookupTable::new(AssignmentData::new(layout.kb_def), 0),
            iteration: 0,
            traverser: Traverser::new(&layout, evaluator),
            evaluator: evaluator,

            best: layout.clone(),
            best_score: score,

            layout: layout,
            score: score,

        }
    }

    fn do_move(&mut self, alteration: ScoredAlteration) {
        self.score += alteration.delta;
        self.traverser.update(alteration.assignments.as_slice());
        for &assignment in alteration.assignments.iter() {
            self.layout.assign(assignment);
        }
    }

    fn best_move(&mut self) -> ScoredAlteration {
        let iteration = self.iteration;
        let tabu = &self.tabu;
        let traverser = &mut self.traverser;

        self.layout
            .moves()
            .filter_map(|assignments| {
                if assignments.iter().any(|&a| tabu[a] >= iteration) {
                    // move is tabu
                    None
                } else {
                    Some(ScoredAlteration {
                        delta: traverser.score_assignments(assignments.as_slice()),
                        assignments: assignments,
                    })
                }
            })
            .min_by(|ref a, ref b| a.delta.partial_cmp(&b.delta).unwrap())
            .unwrap()
    }

    pub fn search(&mut self) {
        let tol = 10f64.powi(-16);
        while self.iteration < 50_000 {
            self.iteration += 1;
            let mv = self.best_move();
            //println!("{}: {:?}", self.iteration, mv);
            if mv.delta >= -tol {
                //println!("iteration {}", self.iteration);
                //println!("OPTIMUM: {}", self.evaluator.evaluate(&self.layout.group_map));
                //self.layout.print();
                for &assignment in mv.assignments.iter() {
                    let inv = self.traverser.walker.inverse(assignment);
                    self.tabu[inv] = self.iteration + 25;
                }
            }

            self.do_move(mv);
            if self.score + tol < self.best_score {
                self.best_score = self.score;
                self.best = self.layout.clone();
                println!("iteration {}: {}", self.iteration, self.best_score);
                self.best.print();
            }

        }

        println!("best: {}", self.best_score);
        self.best.print();
    }
}
