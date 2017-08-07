use eval::{Eval, Evaluator};
use layout::{Layout, TokenMap, Generator, LayoutPair};
use rand::{thread_rng, sample, Rng};
use data::{KbDef, Assignment};

pub struct GeneticAlgorithm<'e> {
    kb_def: &'e KbDef,
    eval: &'e Eval,
    tournament_size: usize,
}

const POP_SIZE: usize = 5000;


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
        for i in 0..100 {
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
        population.extend_from_slice(&prev[0..0]);
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

        ( Individual::from_layout(self.eval, lt1),
          Individual::from_layout(self.eval, lt2))
    }

    fn mutate(&self, layout: &mut Layout) {
        if thread_rng().next_f64() < 0.4 {
            let moves: Vec<Vec<Assignment>> = layout.gen_moves().collect();
            let mv = thread_rng().choose(moves.as_slice()).unwrap();
            layout.assign_all(mv.as_slice());
        }
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
