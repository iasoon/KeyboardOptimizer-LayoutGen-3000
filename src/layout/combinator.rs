use data::*;
use cat::*;

use layout::utils::Subset;
use layout::layout::*;

pub struct Cycle {
    tokens: Vec<Num<Token>>,
}

impl Cycle {
    pub fn inject(&self, target: &mut TokenMap, other: &TokenMap) {
        for &token_num in self.tokens.iter() {
            target[token_num] = other[token_num];
        }
    }
}

pub struct LayoutPair<'a> {
    keymaps: [&'a Keymap; 2],
    token_maps: [&'a TokenMap; 2],
    kb_def: &'a KbDef,
}

impl<'a> LayoutPair<'a> {
    pub fn new(fst: &'a Layout<'a>, snd: &'a Layout<'a>) -> Self {
        LayoutPair {
            keymaps: [&fst.keymap, &snd.keymap],
            token_maps: [&fst.token_map, &snd.token_map],
            kb_def: fst.kb_def,
        }
    }

    fn differing_tokens(&self) -> Subset<Token> {
        let mut token_set = Subset::complete(self.kb_def.tokens.count());
        for token_num in self.kb_def.tokens.nums() {
            let fst = self.token_maps[0][token_num];
            let snd = self.token_maps[1][token_num];
            if fst == snd {
                token_set.remove(token_num);
            }
        }
        return token_set;
    }

    fn cycle_next(&self, token_num: Num<Token>) -> Option<Num<Token>> {
        let loc_num = self.token_maps[0][token_num];
        return self.keymaps[1][loc_num];
    }

    fn cycle_prev(&self, token_num: Num<Token>) -> Option<Num<Token>> {
        let loc_num = self.token_maps[1][token_num];
        return self.keymaps[0][loc_num];
    }

    pub fn cycles<'b: 'a>(&'b self) -> Cycles<'b> {
        Cycles {
            unvisited: self.differing_tokens(),
            layout_pair: self,
        }
    }
}

pub struct Cycles<'a> {
    layout_pair: &'a LayoutPair<'a>,
    unvisited: Subset<Token>,
}

impl<'a> Iterator for Cycles<'a> {
    type Item = Cycle;

    fn next(&mut self) -> Option<Cycle> {
        self.unvisited.next().map(|start| {
            CycleBuilder::new(&self.layout_pair, &mut self.unvisited).mk_cycle(start)
        })
    }
}

struct CycleBuilder<'a> {
    layout_pair: &'a LayoutPair<'a>,
    unvisited: &'a mut Subset<Token>,
    cycle: Vec<Num<Token>>,
    pos: usize,
}

impl<'a> CycleBuilder<'a> {
    fn new(layout_pair: &'a LayoutPair, unvisited: &'a mut Subset<Token>) -> Self {
        CycleBuilder {
            layout_pair: layout_pair,
            unvisited: unvisited,
            cycle: Vec::new(),
            pos: 0,
        }
    }

    fn mk_cycle(mut self, start: Num<Token>) -> Cycle {
        self.visit_group(start);
        self.build();
        return Cycle { tokens: self.cycle };
    }

    fn build(&mut self) {
        while self.pos < self.cycle.len() {
            let token_num = self.cycle[self.pos];
            self.visit_neighbours(token_num);
            self.pos += 1;
        }
    }

    fn visit_neighbours(&mut self, token_num: Num<Token>) {
        if let Some(t) = self.layout_pair.cycle_next(token_num) {
            self.visit_group(t);
        }
        if let Some(t) = self.layout_pair.cycle_prev(token_num) {
            self.visit_group(t);
        }
    }

    fn visit_group(&mut self, token_num: Num<Token>) {
        match self.layout_pair.kb_def.token_group[token_num] {
            Group::Free(_) => self.visit_token(token_num),
            Group::Lock(lock_num) => self.visit_lock(lock_num),
        }
    }

    fn visit_lock(&mut self, lock_num: Num<Lock>) {
        let lock = &self.layout_pair.kb_def.locks[lock_num];
        for (_, &value) in lock.enumerate() {
            if let Some(token_num) = value {
                self.visit_token(token_num);
            }
        }
    }

    fn visit_token(&mut self, token_num: Num<Token>) {
        if self.unvisited.contains(token_num) {
            self.cycle.push(token_num);
            self.unvisited.remove(token_num);
        }
    }
}
