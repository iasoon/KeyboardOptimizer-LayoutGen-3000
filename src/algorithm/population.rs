use eval::{Eval, Evaluator, Scored};
use layout::{Layout, TokenMap, Generator, LayoutPair, MoveGenState};
use rand::{thread_rng, sample, Rng};
use rand;
use cat::*;
use data::*;
use algorithm::tabu_search::TabuParams;
use std::cmp;

use std::mem::swap;

macro_rules! crossover {
    ($p:expr, $var:ident, $fst:ident, $snd:ident) => {
        if thread_rng().next_f64() < $p {
            swap(&mut $fst.$var, &mut $snd.$var);
        }
    }
}

pub struct GeneticAlgorithm<'e> {
    kb_def: &'e KbDef,
    eval: &'e Eval,
    localsearch_intensity: usize,
    innovation_rate: f64,
    population_size: usize,
    num_generations: usize,
}

struct Population<'e> {
    individuals: Vec<Scored<Individual<'e>>>,
    params: &'e GeneticAlgorithm<'e>,
}

impl<'e> Population<'e> {
    fn generate(params: &'e GeneticAlgorithm<'e>) -> Self {
        let mut gen = Generator::new(params.kb_def);
        let mut individuals = Vec::with_capacity(params.population_size);

        for _ in 0..params.population_size {
            let keymap = gen.generate().unwrap();
            let individual = Individual {
                layout: Layout::from_keymap(params.kb_def, keymap),
                behaviour: params.generate_behaviour(),
            };
            individuals.push(params.score(individual));
        }
        return Population {
            individuals: individuals,
            params: params,
        };
    }

    fn evolve(&mut self) {
        let child = {
            let maj = self.tournament();
            let min = self.tournament();

            self.reproduce(&maj.value, &min.value)
        };

        let closest = self.closest_individual(&child.value.layout);
        if self.individuals[closest].score > child.score {
            println!("adding individual with score {}", child.score);
            self.individuals.swap_remove(closest);
            self.individuals.push(child);
        } else {
            println!("sucks, refused");
        }
    }

    fn tournament<'a>(&'a self) -> &'a Scored<Individual<'e>> {
        sample(&mut thread_rng(), &self.individuals, 3).iter().min_by(|a, b| {
            a.score.partial_cmp(&b.score).unwrap()
        }).unwrap()
    }

    fn closest_individual(&self, ind: &Layout<'e>) -> usize {
        let mut closest = 0;
        let mut best_dist = self.individuals[0].value.layout.distance(ind);
        for idx in 1..self.individuals.len() {
            let dist = self.individuals[idx].value.layout.distance(ind);
            if dist < best_dist {
                best_dist = dist;
                closest = idx;
            }
        }
        return closest;
    }

    fn reproduce<'a>(&self, maj: &'a Individual<'e>, min: &'a Individual<'e>)
                     -> Scored<Individual<'e>>
    {
        let mut child = maj.layout.token_map.clone();

        for cycle in LayoutPair::new(&maj.layout, &min.layout).cycles() {
            if thread_rng().next_f64() < 0.5 {
                cycle.inject(&mut child, &min.layout.token_map);
            }
        }

        return self.grow_individual(child, maj.behaviour.clone());
    }

    fn grow_individual(&self, token_map: TokenMap, behaviour: Behaviour)
                       -> Scored<Individual<'e>>
    {
        let layout = Layout::from_token_map(self.params.kb_def, token_map);
        let individual = Individual { layout, behaviour };
        return individual.improve(self.params);
    }

    fn mutate(&self, individual: &mut Individual) {
        // mutation of layout is handled by individal improvement
        if thread_rng().next_f64() < self.params.innovation_rate {
            individual.behaviour = self.params.generate_behaviour();
        }
    }
}


impl<'e> GeneticAlgorithm<'e> {
    pub fn new(kb_def: &'e KbDef, eval: &'e Eval) -> Self {
        GeneticAlgorithm {
            kb_def: kb_def,
            eval: eval,
            localsearch_intensity: 80,
            innovation_rate: 0.05,
            population_size: 250,
            num_generations: 2500,
        }
    }

    pub fn run(&'e self) -> Scored<Layout<'e>> {
        let mut population = Population::generate(self);
        for i in 0..self.num_generations {
            population.evolve();
            let min = population.individuals.iter().min_by(|a, b| {
                a.score.partial_cmp(&b.score).unwrap()
            }).unwrap();
            println!("gen {}: {}", i, min.score);
        }
        return population.individuals.into_iter().min_by(|a, b| {
            a.score.partial_cmp(&b.score).unwrap()
        }).unwrap().map(|ind| ind.layout);
    }

    fn generate_behaviour(&self) -> Behaviour {
        let mut rng = thread_rng();
        Behaviour {
            // TODO: ugh, magic numbers. I don't even know why this works.
            mutation_intensity: rng.gen_range(0, 5),
            tabu_duration: rng.gen_range(1, self.localsearch_intensity),
        }
    }

    fn score<'a>(&self, ind: Individual<'a>) -> Scored<Individual<'a>> {
        Scored {
            score: self.eval.eval(&ind.layout),
            value: ind,
        }
    }
}

struct Mutator<'l, 'a: 'l> {
    layout: &'l mut Layout<'a>,
    gen_state: MoveGenState,
    assignments: <Vec<Num<AllowedAssignment>> as IntoIterator>::IntoIter,
}

impl<'l, 'a: 'l> Mutator<'l, 'a> {
    fn new(layout: &'l mut Layout<'a>) -> Self {
        let mut assignments: Vec<_> = layout.kb_def.assignments.nums().collect();
        thread_rng().shuffle(assignments.as_mut_slice());
        Mutator {
            gen_state: MoveGenState::from_layout(layout),
            layout: layout,
            assignments: assignments.into_iter(),
        }
    }

    fn mutate(&mut self, num: usize) {
        for _ in 0..num {
            if let Some(assignments) = self.next_move() {
                self.layout.assign_all(assignments.as_slice());
            }
        }
    }

    // TODO: duplication with move_gen
    fn next_move(&mut self) -> Option<Vec<Assignment>> {
        while let Some(assignment) = self.next_assignment() {
            let value = self.gen_state.build_move(self.layout, assignment);
            if let Ok(assignments) = value {
                return Some(assignments);
            }
        }
        return None;
    }

    fn next_assignment(&mut self) -> Option<Assignment> {
        self.assignments.next().map(|num| self.layout.kb_def.assignments[num])
    }
}

#[derive(Clone)]
struct Individual<'a> {
    layout: Layout<'a>,
    behaviour: Behaviour,
}

impl<'a> Individual<'a> {
    fn new(layout: Layout<'a>, behaviour: Behaviour) -> Self {
        Individual {
            layout: layout,
            behaviour: behaviour,
        }
    }

    fn improve(mut self, algorithm: &GeneticAlgorithm<'a>) -> Scored<Individual<'a>> {
        self.mutate();
        let b = self.tabu_search(algorithm);
        // println!("mutation: {}\ttabu: {}\tscore: {}",
        //          b.value.behaviour.mutation_intensity,
        //          b.value.behaviour.tabu_duration,
        //          b.score);
        return b;
    }

    fn mutate(&mut self) {
        Mutator::new(&mut self.layout).mutate(self.behaviour.mutation_intensity);
    }

    fn tabu_search(self, algorithm: &GeneticAlgorithm<'a>) -> Scored<Individual<'a>> {
        let Individual { layout, behaviour } = self;
        let ps = TabuParams {
            num_iterations: algorithm.localsearch_intensity,
            tabu_duration: behaviour.tabu_duration,
            eval: algorithm.eval,
        };
        return ps.search(layout).map(|lt| {
            Individual::new(lt, behaviour)
        });
    }
}

#[derive(Clone)]
struct Behaviour {
    mutation_intensity: usize,
    tabu_duration: usize,
}
