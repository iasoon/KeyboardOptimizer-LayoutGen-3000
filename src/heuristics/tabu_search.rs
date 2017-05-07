use layout::*;
use eval::Evaluator;
use utils::{LookupTable, Countable};
use model::{KbDef, GroupId, KeyId, LockId};

pub struct TabuSearcher<'a> {
    layout: Layout<'a>,
    evaluator: &'a Evaluator,
    cache: Cache<'a>,
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

fn init_assignment_delta(layout: &Layout, evaluator: &Evaluator) -> AssignmentMap<f64> {
    let mut walker = GroupMapWalker::new(&layout.group_map, layout.kb_def);
    AssignmentMap::from_fn(layout.kb_def, |assignment| {
        let group_id = assignment.group(layout.kb_def);
        let before = evaluator.eval_group(group_id, walker.pos());
        walker.assign(assignment);
        let after = evaluator.eval_group(group_id, walker.pos());
        walker.reset_assign(assignment);
        return after - before;
    })
}

impl<'a> TabuSearcher<'a> {
    pub fn new(layout: Layout<'a>, evaluator: &'a Evaluator) -> Self {
        TabuSearcher {
            cache: Cache::new(&layout, evaluator),
            layout: layout,
            evaluator: evaluator,
        }
    }

    fn alter(&mut self, alteration: Alteration) {
        self.layout.do_move(&alteration);
        self.cache.update(&self.layout);
    }

    fn scorer<'b>(&'b self) -> AlterationScorer<'b> {
        AlterationScorer {
            walker: GroupMapWalker::new(&self.layout.group_map, &self.layout.kb_def),
            evaluator: &self.evaluator,
            cache: &self.cache,
            kb_def: self.layout.kb_def,
        }
    }

    pub fn test(&mut self) {
        // construct a move
        let mut resolver = AssignmentResolver::new(
            &self.layout.keymap,
            &self.layout.token_map,
            self.layout.kb_def,
        );
        resolver.assign_lock(
            LockId::from_num(&self.layout.kb_def.locks.elem_count(), 0),
            KeyId::from_num(&self.layout.kb_def.keys.elem_count(), 0)
        );
        resolver.assign_lock(
            LockId::from_num(&self.layout.kb_def.locks.elem_count(), 2),
            KeyId::from_num(&self.layout.kb_def.keys.elem_count(), 2)
        );
        let alteration = resolver.resolve();
        let mut scorer = self.scorer();

        let assignments: Vec<Assignment> = alteration.assignments().collect();
        let calculated = scorer.score(assignments.as_slice());

        let mut walker = GroupMapWalker::new(&self.layout.group_map, &self.layout.kb_def);
        let before = self.evaluator.evaluate(walker.pos());
        walker.do_move(&alteration);
        let after = self.evaluator.evaluate(walker.pos());
        let delta = after - before;

        let tol = (10.0 as f64).powi(-12);
        if (calculated - delta).abs() > tol {
            println!("ERROR: expected {} but was {}, diff: {}", delta, calculated, calculated - delta);
        } else {
            println!("SUCCESS: expected {} and got {}", delta, calculated);
        }
    }

    pub fn best_move(&self) -> ScoredAlteration {
        let mut scorer = self.scorer();
        self.layout
            .moves()
            .map(|alteration| {
                let assignments: Vec<Assignment> = alteration.assignments().collect();
                let delta = scorer.score(assignments.as_slice());

                let score_before = self.evaluator.evaluate(scorer.walker.pos());
                scorer.walker.do_move(&alteration);
                let score_after = self.evaluator.evaluate(scorer.walker.pos());
                scorer.walker.reset_move(&alteration);


                let check = score_after - score_before;

                let tol = (10.0 as f64).powi(-12);
                if (delta - check).abs() > tol {
                    println!("ERROR: expected {} but was {}, diff: {}", check, delta, check - delta);
                }
                ScoredAlteration {
                    alteration: alteration,
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
            self.alter(mv.alteration);
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

    fn update(&mut self, layout: &Layout) {
        let mut walker = GroupMapWalker::new(&layout.group_map, &layout.kb_def);
        let eval = self.evaluator;
        self.assignment_delta.map_mut(|assignment, cost| {
            let group_id = assignment.group(layout.kb_def);
            let before = eval.eval_group(group_id, walker.pos());
            walker.assign(assignment);
            let after = eval.eval_group(group_id, walker.pos());
            walker.reset_assign(assignment);
            *cost = after - before;
        });
    }
}

struct AlterationScorer<'a> {
    evaluator: &'a Evaluator,
    walker: GroupMapWalker<'a>,
    cache: &'a Cache<'a>,
    kb_def: &'a KbDef,
}

impl<'a> AlterationScorer<'a> {
    fn calc_overlap(&self, assignment: Assignment, assignments: &[Assignment]) -> f64 {
        let group1 = assignment.group(self.kb_def);
        assignments.iter().map(|assignment| {
            let group2 = assignment.group(self.kb_def);
            self.evaluator.eval_overlap(group1, group2, self.walker.pos())
        }).sum()
    }

    fn calc_step(&mut self, assigned: &[Assignment], step: Assignment) -> f64 {
        let before = self.calc_overlap(step, assigned);
        self.walker.assign(step);
        let after = self.calc_overlap(step, assigned);
        return after - before;
    }

    fn calc_delta(&mut self, assigned: &[Assignment], to_assign: &[Assignment]) -> f64 {
        let mut delta = 0.0;
        for &step in to_assign.iter() {
            delta += self.calc_step(assigned, step);
        }

        for &assignment in to_assign.iter() {
            self.walker.reset_assign(assignment);
        }

        return delta;
    }

    fn score(&mut self, assignments: &[Assignment]) -> f64 {
        let mut delta = 0.0;
        for (num, &assignment) in assignments.iter().enumerate() {

            let d1 = self.calc_delta(&[assignment], &assignments[0..num]);
            self.walker.assign(assignment);
            let d2 = self.calc_delta(&[assignment], &assignments[0..num]);
            self.walker.reset_assign(assignment);

            let step = self.cache.assignment_delta[assignment] + d2 - d1;

            delta += step;
        }

        return delta;
    }
}
