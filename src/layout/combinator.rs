use model::*;
use utils::{LookupTable, BoundedSubset};
use layout::mk_keymap;

#[derive(Debug)]
pub struct Cycle {
    tokens: Vec<TokenId>,
}

impl Cycle {
    // pub fn inject(&self, maj: &mut TokenMap, min: &TokenMap) {
    //     unimplemented!()
    // }

    // pub fn swap(&self, maj: &mut Layout, min: &mut Layout) {
    //     unimplemented!()
    // }
}

// fn swap_cycle(&mut self, cycle: &Vec<usize>) {
//     for &token_id in cycle.iter() {
//         self.swap_token(token_id);
//     }
// }

// fn swap_token(&mut self, token_id: usize) {
//     let loc_a = self.keymaps[0][token_id];
//     let loc_b = self.keymaps[1][token_id];
//     self.keymaps[0].cycle_swap(loc_b, token_id);
//     self.keymaps[1].cycle_swap(loc_a, token_id);
// }

type Keymap = LookupTable<Loc, Option<TokenId>>;
pub type TokenMap = LookupTable<TokenId, Loc>;

type TokenSet = BoundedSubset<TokenId>;

pub struct LayoutPair<'a> {
    keymaps: [Keymap; 2],
    token_maps: [&'a TokenMap; 2],
    kb_def: &'a KbDef,
}

impl<'a> LayoutPair<'a> {
    pub fn new(fst_map: &'a TokenMap, snd_map: &'a TokenMap, kb_def: &'a KbDef) -> Self {
        LayoutPair {
            keymaps: [mk_keymap(fst_map, kb_def), mk_keymap(snd_map, kb_def)],
            token_maps: [fst_map, snd_map],
            kb_def: kb_def,
        }
    }

    fn differing_tokens(&self) -> TokenSet {
        let mut token_set = self.kb_def.tokens.subset();
        for token_id in self.kb_def.tokens.ids() {
            if self.token_maps[0][token_id] == self.token_maps[1][token_id] {
                token_set.remove(token_id);
            }
        }
        return token_set;
    }

    fn cycle_next(&self, token_id: TokenId) -> Option<TokenId> {
        self.keymaps[1][self.token_maps[0][token_id]]
    }

    fn cycle_prev(&self, token_id: TokenId) -> Option<TokenId> {
        self.keymaps[0][self.token_maps[1][token_id]]
    }

    pub fn cycles<'b: 'a>(&'b self) -> Cycles<'b> {
        Cycles {
            unvisited: self.differing_tokens(),
            layout_pair: self,
        }
    }
}

struct Cycles<'a> {
    layout_pair: &'a LayoutPair<'a>,
    unvisited: TokenSet,
}

impl<'a> Iterator for Cycles<'a> {
    type Item = Cycle;

    fn next(&mut self) -> Option<Cycle> {
        self.unvisited.first().map(|start| {
            CycleBuilder::new(&self.layout_pair, &mut self.unvisited).mk_cycle(start)
        })
    }
}

struct CycleBuilder<'a> {
    layout_pair: &'a LayoutPair<'a>,
    unvisited: &'a mut TokenSet,
    cycle: Vec<TokenId>,
    pos: usize,
}

impl<'a> CycleBuilder<'a> {
    fn new(layout_pair: &'a LayoutPair, unvisited: &'a mut TokenSet) -> Self {
        CycleBuilder {
            layout_pair: layout_pair,
            unvisited: unvisited,
            cycle: Vec::new(),
            pos: 0,
        }
    }

    fn mk_cycle(mut self, start: TokenId) -> Cycle {
        self.visit_group(start);
        self.build();
        return Cycle { tokens: self.cycle };
    }

    fn build(&mut self) {
        while self.pos < self.cycle.len() {
            let token_id = self.cycle[self.pos];
            self.visit_neighbours(token_id);
            self.pos += 1;
        }
    }

    fn visit_neighbours(&mut self, token_id: TokenId) {
        if let Some(t) = self.layout_pair.cycle_next(token_id) {
            self.visit_group(t);
        }
        if let Some(t) = self.layout_pair.cycle_prev(token_id) {
            self.visit_group(t);
        }
    }

    fn visit_group(&mut self, token_id: TokenId) {
        let group_id = self.layout_pair.kb_def.token_group[token_id];
        match self.layout_pair.kb_def.groups[group_id] {
            Group::Free(_) => self.visit_token(token_id),
            Group::Locked(lock_id) => self.visit_lock(lock_id),
        }
    }

    fn visit_lock(&mut self, lock_id: LockId) {
        for token_id in self.layout_pair.kb_def.locks[lock_id].members() {
            self.visit_token(token_id);
        }
    }

    fn visit_token(&mut self, token_id: TokenId) {
        if self.unvisited.contains(token_id) {
            self.cycle.push(token_id);
            self.unvisited.remove(token_id);
        }
    }
}
