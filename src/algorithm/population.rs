use eval::{Eval, Evaluator, Scored};
use layout::{Layout, TokenMap, Generator, LayoutPair, MoveGenState};
use rand::{thread_rng, sample, Rng};
use cat::*;
use data::*;
use algorithm::tabu_search::TabuParams;

use std::mem::swap;

pub struct GeneticAlgorithm<'e> {
    kb_def: &'e KbDef,
    eval: &'e Eval,
    tournament_size: usize,
    localsearch_intensity: usize,
    innovation_rate: f64,
    population_size: usize,
    num_generations: usize,
}

macro_rules! crossover {
    ($p:expr, $var:ident, $fst:ident, $snd:ident) => {
        if thread_rng().next_f64() < $p {
            swap(&mut $fst.$var, &mut $snd.$var);
        }
    }
}

impl<'e> GeneticAlgorithm<'e> {
    pub fn new(kb_def: &'e KbDef, eval: &'e Eval) -> Self {
        GeneticAlgorithm {
            kb_def: kb_def,
            eval: eval,
            tournament_size: 2,
            localsearch_intensity: 50,
            innovation_rate: 0.05,
            population_size: 200,
            num_generations: 20,
        }
    }

    pub fn run(&self) -> Scored<Layout<'e>> {
        let mut pop = self.gen_population();
        for i in 0..self.num_generations {
            let next = self.evolve_population(pop);
            pop = next;
            let min = pop.iter().min_by(|a, b| {
                a.score.partial_cmp(&b.score).unwrap()
            }).unwrap();
            println!("gen {}: {}", i, min.score);
        }
        return pop.into_iter().min_by(|a, b| {
            a.score.partial_cmp(&b.score).unwrap()
        }).unwrap().map(|ind| ind.layout);
    }

    fn gen_population(&self) -> Population<'e> {
        let mut gen = Generator::new(self.kb_def);
        let mut population = Vec::with_capacity(self.population_size);

        for _ in 0..self.population_size {
            let keymap = gen.generate().unwrap();
            let individual = Individual {
                layout: Layout::from_keymap(self.kb_def, keymap),
                behaviour: self.generate_behaviour(),
            };
            population.push(self.score(individual));
        }
        return population;
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

    fn evolve_population(&self, mut prev: Population<'e>) -> Population<'e> {
        prev.sort_by(|a, b| {
            a.score.partial_cmp(&b.score).unwrap()
        });

        let mut population = Vec::with_capacity(self.population_size);
        while population.len() < self.population_size {
            let maj = self.tournament(&prev);
            let min = self.tournament(&prev);
            let (mut a, mut b) = self.crossover(maj, min);
            self.mutate(&mut a);
            self.mutate(&mut b);

            population.push(a.improve(self));
            population.push(b.improve(self));
        }
        return population;
    }

    fn tournament<'a>(&self, population: &'a Population<'e>) -> &'a Individual<'e> {
        &sample(&mut thread_rng(), population, self.tournament_size)
            .iter()
            .min_by(|a, b| {
                a.score.partial_cmp(&b.score).unwrap()
            })
            .unwrap().value
    }

    fn crossover<'a>(&self, maj: &'a Individual<'e>, min: &'a Individual<'e>)
               -> (Individual<'e>, Individual<'e>)
    {
        let mut maj_child = maj.layout.token_map.clone();
        let mut min_child = min.layout.token_map.clone();

        let mut maj_behaviour = maj.behaviour.clone();
        let mut min_behaviour = min.behaviour.clone();

        for cycle in LayoutPair::new(&maj.layout, &min.layout).cycles() {
            if thread_rng().next_f64() < 0.5 {
                cycle.inject(&mut maj_child, &min.layout.token_map);
                cycle.inject(&mut min_child, &maj.layout.token_map);
            }
        }

        crossover!(0.5, mutation_intensity, maj_behaviour, min_behaviour);
        crossover!(0.5, tabu_duration,      maj_behaviour, min_behaviour);

        (self.mk_individual(maj_child, maj_behaviour),
         self.mk_individual(min_child, min_behaviour))
    }

    fn mutate(&self, individual: &mut Individual) {
        // mutation of layout is handled by individal improvement
        if thread_rng().next_f64() < self.innovation_rate {
            individual.behaviour = self.generate_behaviour();
        }
    }


    fn mk_individual(&self, token_map: TokenMap, behaviour: Behaviour) -> Individual<'e> {
        let layout = Layout::from_token_map(self.kb_def, token_map);
        return Individual { layout, behaviour };
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
        println!("mutation: {}\ttabu: {}\tscore: {}",
                 b.value.behaviour.mutation_intensity,
                 b.value.behaviour.tabu_duration,
                 b.score);
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

type Population<'e> = Vec<Scored<Individual<'e>>>;
