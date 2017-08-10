use eval::{Eval, Evaluator};
use layout::{Layout, TokenMap, Generator, LayoutPair, MoveGenState};
use rand::{thread_rng, sample, Rng};
use cat::*;
use data::*;
use algorithm::TabuSearch;

pub struct GeneticAlgorithm<'e> {
    kb_def: &'e KbDef,
    eval: &'e Eval,
    tournament_size: usize,
}

const POP_SIZE: usize = 50;

impl<'e> GeneticAlgorithm<'e> {
    pub fn new(kb_def: &'e KbDef, eval: &'e Eval) -> Self {
        GeneticAlgorithm {
            kb_def: kb_def,
            eval: eval,
            tournament_size: 2,
        }
    }

    pub fn run(&self) -> Layout<'e> {
        let mut pop = self.gen_population();
        for i in 0..10 {
            let next = self.evolve_population(pop);
            pop = next;
            let min = pop.iter().min_by(|a, b| {
                a.score.partial_cmp(&b.score).unwrap()
            }).unwrap();
            println!("gen {}: {}", i, min.score);
        }
        return pop[0].layout.clone();
    }

    fn gen_population(&self) -> Population<'e> {
        let mut gen = Generator::new(self.kb_def);
        let mut population = Vec::with_capacity(POP_SIZE);
        for _ in 0..POP_SIZE {
            let keymap = gen.generate().unwrap();
            let layout = Layout::from_keymap(self.kb_def, keymap);
            population.push(Individual::from_layout(self.eval, layout));

        }
        return population;
    }

    fn evolve_population(&self, mut prev: Population<'e>) -> Population<'e> {
        prev.sort_by(|a, b| {
            a.score.partial_cmp(&b.score).unwrap()
        });

        let mut population = Vec::with_capacity(POP_SIZE);
        while population.len() < POP_SIZE {
            let maj = self.tournament(&prev);
            let min = self.tournament(&prev);
            let (a, b) = self.crossover(maj, min);
            population.push(a);
            population.push(b);
        }
        return population;
    }

    fn tournament<'a>(&self, population: &'a Population<'e>) -> &'a Individual<'e> {
        sample(&mut thread_rng(), population, self.tournament_size)
            .iter()
            .min_by(|a, b| {
                a.score.partial_cmp(&b.score).unwrap()
            })
            .unwrap()
    }

    fn crossover<'a>(&self, maj: &'a Individual<'e>, min: &'a Individual<'e>)
               -> (Individual<'e>, Individual<'e>)
    {
        let mut maj_child = maj.layout.token_map.clone();
        let mut min_child = min.layout.token_map.clone();

        for cycle in LayoutPair::new(&maj.layout, &min.layout).cycles() {
            if thread_rng().next_f64() < 0.5 {
                cycle.inject(&mut maj_child, &min.layout.token_map);
                cycle.inject(&mut min_child, &maj.layout.token_map);
            }
        }

        let mut lt1 = Layout::from_token_map(self.kb_def, maj_child);
        let mut lt2 = Layout::from_token_map(self.kb_def, min_child);

        self.mutate(&mut lt1);
        self.mutate(&mut lt2);

        let lt1_ = TabuSearch::new(lt1, self.eval).run();
        let lt2_ = TabuSearch::new(lt2, self.eval).run();

        ( Individual::from_layout(self.eval, lt1_),
          Individual::from_layout(self.eval, lt2_))
    }

    fn mutate<'l, 'a: 'l>(&self, layout: &'l mut Layout<'a>) {
        if thread_rng().next_f64() < 0.4 {
            let mut mutator = Mutator::new(layout);
            mutator.mutate(2);
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
  score: f64,
}

type Population<'e> = Vec<Individual<'e>>;

impl<'a> Individual<'a> {
    fn from_token_map(eval: &'a Eval, kb_def: &'a KbDef, map: TokenMap) -> Self {
        let layout = Layout::from_token_map(kb_def, map);
        return Individual::from_layout(eval, layout);
    }
    fn from_layout(eval: &'a Eval, layout: Layout<'a>) -> Self {
        Individual {
            score: eval.eval(&layout),
            layout: layout,
        }
    }
}
