use eval::*;
use layout::*;
use cat::*;
use data::*;
use algorithm::tabu_search::TabuParams;
use rand::{thread_rng, Rng};


pub struct ScatterSearch<'e> {
    pub kb_def: &'e KbDef,
    pub eval: &'e Eval,
    pub population_size: usize,
}

impl<'e> ScatterSearch<'e> {
    pub fn run(&self) {
        let mut pop = Population::new(self);
        pop.build();
        for i in 0..1000 {
            println!("iteration {}", i);
            pop.evolve();
        }

        pop.solutions.sort_by(|a, b| {
            a.score.partial_cmp(&b.score).unwrap().reverse()
        });
        pop.print();
    }
}

struct Population<'e> {
    solutions: Vec<Scored<Layout<'e>>>,
    params: &'e ScatterSearch<'e>,
}

impl<'e> Population<'e> {
    pub fn new(params: &'e ScatterSearch<'e>) -> Self {
        Population {
            solutions: Vec::with_capacity(params.population_size),
            params: params,
        }
    }

    fn tabu_search(&self, lt: Layout<'e>) -> Scored<Layout<'e>> {
        let params = TabuParams {
            num_iterations: 250,
            tabu_duration: 25,
            eval: self.params.eval,
        };
        return params.search(lt);
    }

    pub fn build(&mut self) {
        let mut gen = Generator::new(self.params.kb_def);

        while self.solutions.len() < self.params.population_size {
            let keymap = gen.generate().unwrap();
            let lt = self.tabu_search(
                Layout::from_keymap(self.params.kb_def, keymap)
            );
            if self.solutions.iter().all(|solution| solution != &lt) {
                self.solutions.push(lt);
            } else {
                println!("miss");
            }
        }
    }

    fn worst_solution_index(&self) -> usize {
        let mut worst = 0;
        for idx in 1..self.solutions.len() {
            if self.solutions[worst].score < self.solutions[idx].score {
                worst = idx;
            }
        }
        return worst;
    }

    fn try_add(&mut self, lt: Scored<Layout<'e>>) -> bool {
        let worst = self.worst_solution_index();

        if lt.score > self.solutions[worst].score {
            println!("not good enough");
            return false;
        }

        if self.solutions.iter().any(|solution| solution == &lt) {
            println!("duplicate");
            return false;
        }

        self.solutions.swap_remove(worst);
        self.solutions.push(lt);
        return true;
    }

    fn evolve(&mut self) -> bool {
        let start = {
            let parent1 = &thread_rng().choose(&self.solutions).unwrap().value;
            let parent2 = &thread_rng().choose(&self.solutions).unwrap().value;

            let mut token_map = parent1.token_map.clone();

            for cycle in LayoutPair::new(parent1, parent2).cycles() {
                if thread_rng().next_f64() < 0.5 {
                    cycle.inject(&mut token_map, &parent2.token_map);
                }
            }

            Layout::from_token_map(self.params.kb_def, token_map)
        };

        let lt = self.tabu_search(start);

        println!("solution with score {}", lt.score);

        return self.try_add(lt);
    }

    pub fn print(&self) {
        for solution in self.solutions.iter() {
            println!("{}", solution.score);
        }
    }
}
