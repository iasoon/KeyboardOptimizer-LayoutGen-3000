use layout::Layout;
use eval::*;
use cat::*;
use data::*;

const ROUNDING_ERROR: f64 = 2f64.powi(-53); // 64-bit machine epsilon

#[derive(Clone)]
pub struct TabuParams<'e> {
    num_iterations: usize,
    tabu_duration: usize,
    eval: &'e Eval,
}

impl<'e> TabuParams<'e> {
    pub fn apply(&self, layout: Layout<'e>) -> Layout<'e> {
        TabuSearch::new(layout, self.clone()).run()
    }
}

pub struct TabuSearch<'e> {
    traverser: Traverser<'e>,
    iteration: usize,
    best: Layout<'e>,
    best_score: f64,
    tabu: AssignmentTable<'e, usize>,
    params: TabuParams<'e>,
}

impl<'e> TabuSearch<'e> {
    pub fn new(start: Layout<'e>, params: TabuParams<'e>) -> Self {
        let traverser = Traverser::new(params.eval, start.clone());
        TabuSearch {
            iteration: 0,
            tabu: AssignmentTable::new(start.kb_def, |_| 0),
            best_score: traverser.position_score(),
            traverser: traverser,
            best: start,
            params: params,
        }
    }

    pub fn run(mut self) -> Layout<'e> {

        while self.iteration < self.params.num_iterations {

            let delta = self.pick_move();

            if delta.score >= -ROUNDING_ERROR {
                for &assignment in delta.assignments.iter() {
                    let inv = self.traverser.inverse(assignment);
                    self.tabu[inv] = self.iteration + self.params.tabu_duration;
                }
            }

            self.traverser.assign(&delta);

            if self.traverser.position_score() + ROUNDING_ERROR < self.best_score {
                self.best = self.traverser.position().clone();
                self.best_score = self.traverser.position_score();
            }
            self.iteration += 1;
        }
        println!("search finished. best score found: {}", self.best_score);
        return self.best;
    }

    fn available_moves<'a>(&'a mut self) -> impl Iterator<Item = Delta> + 'a {
        let tabu = &self.tabu;
        let current_iteration = self.iteration;
        self.traverser.deltas().filter(move |delta| {
            delta.assignments.iter().all(|&assignment| {
                tabu[assignment] <= current_iteration
            })
        })
    }

    fn pick_move(&mut self) -> Delta {
        self.available_moves().min_by(|a, b| {
            a.score.partial_cmp(&b.score).unwrap()
        }).unwrap()
    }
}
