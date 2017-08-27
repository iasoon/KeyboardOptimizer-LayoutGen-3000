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
    tournament_size: usize,
    localsearch_intensity: usize,
    innovation_rate: f64,
    population_size: usize,
    // TODO: better name
    num_parents: usize,
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
        self.perform_selection();
        self.perform_reproduction();
    }

    fn perform_reproduction(&mut self) {
        println!("reproduction. len={}", self.individuals.len());
        while self.individuals.len() < self.params.population_size - 1 {
            let (mut a, mut b) = {
                let maj = self.random_parent();
                let min = self.random_parent();
                self.crossover(&maj.value, &min.value)
            };
            self.mutate(&mut a);
            self.mutate(&mut b);
            self.individuals.push(a.improve(self.params));
            self.individuals.push(b.improve(self.params));
        }
    }

    fn perform_selection(&mut self) {
        self.individuals.sort_by(|a, b| {
            a.score.partial_cmp(&b.score).unwrap()
        });
        let mut set = SpeciesSet::empty();
        for individual in self.individuals.drain(0..) {
            set.add_individual(individual);
        }
        set.truncate(self.params.num_parents);
        self.individuals.extend(set.drain());
    }

    fn random_parent<'a>(&'a self) -> &'a Scored<Individual<'e>> {
        let end = cmp::min(self.individuals.len(), self.params.num_parents);
        let parents = &self.individuals[0..end];
        let individual = rand::thread_rng().choose(parents).unwrap();
        return individual;
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

    fn mk_individual(&self, token_map: TokenMap, behaviour: Behaviour) -> Individual<'e> {
        let layout = Layout::from_token_map(self.params.kb_def, token_map);
        return Individual { layout, behaviour };
    }

    fn mutate(&self, individual: &mut Individual) {
        // mutation of layout is handled by individal improvement
        if thread_rng().next_f64() < self.params.innovation_rate {
            individual.behaviour = self.params.generate_behaviour();
        }
    }
}

struct SpeciesSet<'e> {
    species: Vec<Species<'e>>,
}


impl<'e> SpeciesSet<'e> {
    fn empty() -> Self {
        SpeciesSet {
            species: Vec::new(),
        }
    }

    fn add_individual(&mut self, individual: Scored<Individual<'e>>) {
        let radius = 25;

        if self.species.is_empty() {
            self.species.push(Species::new(individual));
        } else {
            let mut closest = 0;
            let mut smallest_dist = self.species[0].distance(&individual.value);

            for i in 1..self.species.len() {
                let dist = self.species[i].distance(&individual.value);
                if dist < smallest_dist {
                    smallest_dist = dist;
                    closest = i;
                }
            }

            if smallest_dist <= radius {
                self.species[closest].individuals.push(individual);
            } else {
                self.species.push(Species::new(individual));
            }
        }
    }

    fn truncate(&mut self, target_size: usize) {
        println!("SPECIES: ");
        let weights: Vec<f64> = self.species.iter()
            .map(|species| species.leader().score)
            .collect();
        let total_weight: f64 = weights.iter().sum();
        let avg_weight: f64 = total_weight / weights.len() as f64;
        let avg_size: f64 = target_size as f64 / self.species.len() as f64;
        let factor = avg_weight * avg_size;
        for (i, species) in self.species.iter_mut().enumerate() {
            let size: usize = (factor / weights[i]) as usize;
            println!(
                "score: {}\tsize: {}\tshare: {}",
                species.leader().score,
                species.size(),
                size,
            );
            species.truncate(size);
        }
        println!("\n");
    }

    fn drain(self) -> impl Iterator<Item = Scored<Individual<'e>>> {
        self.species.into_iter().flat_map(|species| species.drain())
    }
}

struct Species<'e> {
    individuals: Vec<Scored<Individual<'e>>>,
}

impl<'e> Species<'e> {
    fn new(leader: Scored<Individual<'e>>) -> Self {
        Species {
            individuals: vec![leader],
        }
    }

    fn size(&self) -> usize {
        self.individuals.len()
    }

    fn leader<'a>(&'a self) -> &'a Scored<Individual<'e>> {
        &self.individuals[0]
    }

    fn distance(&self, individual: &Individual<'e>) -> usize {
        self.leader().value.layout.distance(&individual.layout)
    }

    fn kill_one(&mut self, tournament_size: usize) {
        let mut rng = rand::thread_rng();
        let contestants = rand::sample(&mut rng, 0..self.size(), tournament_size);
        let loser_idx = contestants.iter().cloned().max_by(|&a, &b| {
            let a_score = self.individuals[a].score;
            let b_score = self.individuals[b].score;
            return a_score.partial_cmp(&b_score).unwrap();
        }).unwrap();
        self.individuals.swap_remove(loser_idx);
    }

    fn truncate(&mut self, target_size: usize) {
        while self.size() > target_size {
            let tournament_size = cmp::min(3, self.size());
            self.kill_one(tournament_size);
        }
    }

    fn drain(self) -> impl Iterator<Item = Scored<Individual<'e>>> {
        self.individuals.into_iter()
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
            population_size: 400,
            num_parents: 200,
            num_generations: 20,
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
