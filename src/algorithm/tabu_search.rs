use layout::Layout;
use eval::*;
use data::*;

#[derive(Clone)]
pub struct TabuParams<'e> {
    pub num_iterations: usize,
    pub tabu_duration: usize,
    pub eval: &'e Eval,
}

impl<'e> TabuParams<'e> {
    pub fn search(&self, layout: Layout<'e>) -> Scored<Layout<'e>> {
        TabuSearch::new(layout, self.clone()).run()
    }
}

pub struct TabuSearch<'e> {
    traverser: Traverser<'e>,
    iteration: usize,
    best: Scored<Layout<'e>>,
    tabu: AssignmentTable<'e, usize>,
    params: TabuParams<'e>,
}

impl<'e> TabuSearch<'e> {
    pub fn new(start: Layout<'e>, params: TabuParams<'e>) -> Self {
        let kb_def = start.kb_def;
        let traverser = Traverser::new(params.eval, start);
        TabuSearch {
            iteration: 0,
            tabu: AssignmentTable::new(kb_def, |_| 0),
            params: params,
            best: traverser.position().clone(),
            traverser: traverser,
        }
    }

    pub fn run(mut self) -> Scored<Layout<'e>> {
        let rounding_error = 2f64.powi(-53); // 64-bit machine epsilon


        while self.iteration < self.params.num_iterations {

            let delta = self.pick_move();

            if delta.score >= -rounding_error {
                for &assignment in delta.assignments.iter() {
                    let inv = self.traverser.inverse(assignment);
                    self.tabu[inv] = self.iteration + self.params.tabu_duration;
                }
            }

            self.traverser.assign(&delta);

            if self.traverser.position().score + rounding_error < self.best.score {
                self.best = self.traverser.position().clone();
            }
            self.iteration += 1;
        }
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
