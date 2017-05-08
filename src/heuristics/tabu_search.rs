use layout::*;
use eval::Evaluator;
use utils::{LookupTable, Countable};
use model::{KbDef, GroupId, KeyId, LockId};
use heuristics::Walker;

pub struct TabuSearcher<'a> {
    layout: Layout<'a>,
    evaluator: &'a Evaluator,
    cache: Cache<'a>,
}

#[derive(Debug)]
struct ScoredAlteration {
    pub assignments: Vec<Assignment>,
    pub delta: f64,
}

fn init_group_components(layout: &Layout, evaluator: &Evaluator) -> LookupTable<GroupId, f64> {
    LookupTable::from_fn(layout.kb_def.groups.elem_count(),
                         |group_id| evaluator.group_component(group_id, &layout.group_map))
}

fn init_assignment_delta(layout: &Layout, evaluator: &Evaluator) -> AssignmentMap<f64> {
    let mut walker = Walker::new(&layout, evaluator);
    AssignmentMap::from_fn(layout.kb_def, |assignment| walker.delta(&[assignment]))
}

impl<'a> TabuSearcher<'a> {
    pub fn new(layout: Layout<'a>, evaluator: &'a Evaluator) -> Self {
        TabuSearcher {
            cache: Cache::new(&layout, evaluator),
            layout: layout,
            evaluator: evaluator,
        }
    }

    fn alter(&mut self, assignments: &[Assignment]) {
        self.cache.update(&self.layout, assignments);
        for &assignment in assignments.iter() {
            self.layout.assign(assignment);
        }
    }

    fn scorer<'b>(&'b self) -> AlterationScorer<'b> {
        AlterationScorer {
            walker: Walker::new(&self.layout, self.evaluator),
            evaluator: &self.evaluator,
            cache: &self.cache,
            kb_def: self.layout.kb_def,
        }
    }

    pub fn best_move(&self) -> ScoredAlteration {
        let mut scorer = self.scorer();
        self.layout
            .moves()
            .map(|assignments| {
                let delta = scorer.score(assignments.as_slice());
                let check = scorer.walker.delta(assignments.as_slice());
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

struct Cache<'a> {
    //pub group_component: LookupTable<GroupId, f64>,
    pub assignment_delta: AssignmentMap<f64>,
    evaluator: &'a Evaluator,
}

impl<'a> Cache<'a> {
    fn new(layout: &Layout, evaluator: &'a Evaluator) -> Self {
        Cache {
            //group_component: init_group_components(layout, evaluator),
            assignment_delta: init_assignment_delta(layout, evaluator),
            evaluator: evaluator,
        }
    }

    fn update(&mut self, layout: &Layout, assignments: &[Assignment]) {
        let mut walker = Walker::new(layout, self.evaluator);
        let mut changed = LookupTable::new(layout.kb_def.groups.elem_count(), false);
        for &assignment in assignments.iter() {
            changed[assignment.group(layout.kb_def)] = true;
        }

        self.assignment_delta.map_mut(|assignment, cost| {
            if changed[assignment.group(layout.kb_def)] {
                walker.assign_all(assignments);
                *cost = walker.delta(&[assignment]);
                walker.reset_all(assignments);
            } else {
                *cost += walker.alteration_delta(&[assignment], assignments);
            }
        });
    }
}

struct AlterationScorer<'a> {
    evaluator: &'a Evaluator,
    walker: Walker<'a>,
    cache: &'a Cache<'a>,
    kb_def: &'a KbDef,
}

impl<'a> AlterationScorer<'a> {
    fn score(&mut self, assignments: &[Assignment]) -> f64 {
        let mut delta = 0.0;
        for (num, &assignment) in assignments.iter().enumerate() {
            let d = self.walker.alteration_delta(&[assignment], &assignments[0..num]);
            delta += self.cache.assignment_delta[assignment] + d;
        }

        return delta;
    }
}
