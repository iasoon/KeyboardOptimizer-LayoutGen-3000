use utils::{SeqTable, SeqAssocList, LookupTable};
use model::{KbDef, Group, GroupId, KeyId};
use utils::ElemCount;
use layout::GroupMap;

type Paths = SeqAssocList<GroupId, f64>;
type GroupPaths = LookupTable<GroupId, Paths>;
type PairPaths = LookupTable<(GroupId, GroupId), Paths>;

pub struct Evaluator {
    seqs: SeqAssocList<GroupId, f64>,
    path_costs: SeqTable<KeyId, f64>,
    group_seqs: LookupTable<GroupId, Paths>,
    pair_paths: LookupTable<(GroupId, GroupId), Paths>,
}

fn seq_members<'a>(seq: &'a Vec<GroupId>) -> impl Iterator<Item = GroupId> + 'a {
    seq.iter()
        .cloned()
        .enumerate()
        .filter(move |&(idx, item)| !seq[0..idx].contains(&item))
        .map(|(_, item)| item)
}

struct SeqAssocData {
    items: Vec<GroupId>,
    values: Vec<f64>,
}

fn mk_group_paths(paths: &Paths, group_count: ElemCount<Group>) -> GroupPaths {
    let mut table = LookupTable::from_fn(group_count, |_| {
        SeqAssocData {
            items: Vec::new(),
            values: Vec::new(),
        }
    });
    for (seq, &cost) in paths.iter() {
        let seq_vec: Vec<GroupId> = seq.cloned().collect();
        for item in seq_members(&seq_vec) {
            table[item].items.extend(seq_vec.iter());
            table[item].values.push(cost);
        }
    }
    return table.drain_map(|data| {
        SeqAssocList::from_vecs(data.items, paths.seq_len(), data.values)
    });
}

fn pairs<'a, T>(vec: &'a Vec<T>) -> impl Iterator<Item = (T, T)> + 'a
    where T: Clone
{
    (0..vec.len()).flat_map(move |num| {
        (0..num).map(move |num2| (vec[num].clone(), vec[num2].clone()))
    })
}

fn mk_pair_paths(paths: &Paths, group_count: ElemCount<Group>) -> PairPaths {
    let mut table = LookupTable::from_fn((group_count.clone(), group_count), |_| {
        SeqAssocData {
            items: Vec::new(),
            values: Vec::new(),
        }
    });
    for (seq, &cost) in paths.iter() {
        let seq_vec: Vec<GroupId> = seq.cloned().collect();
        let members: Vec<GroupId> = seq_members(&seq_vec).collect();
        for (a, b) in pairs(&members) {
            table[(a, b)].items.extend(seq_vec.iter());
            table[(b, a)].items.extend(seq_vec.iter());
            table[(a, b)].values.push(cost);
            table[(b, a)].values.push(cost);
        }
    }
    return table.drain_map(|data| {
        SeqAssocList::from_vecs(data.items, paths.seq_len(), data.values)
    });
}

impl Evaluator {
    pub fn new(seqs: SeqAssocList<GroupId, f64>,
               path_costs: SeqTable<KeyId, f64>,
               kb_def: &KbDef)
               -> Self {
        Evaluator {
            path_costs: path_costs,
            group_seqs: mk_group_paths(&seqs, kb_def.groups.elem_count()),
            pair_paths: mk_pair_paths(&seqs, kb_def.groups.elem_count()),
            seqs: seqs,
        }
    }

    pub fn evaluate(&self, table: &GroupMap) -> f64 {
        self.eval_seqs(table, &self.seqs)
    }

    pub fn eval_group(&self, group_id: GroupId, table: &GroupMap) -> f64 {
        self.eval_seqs(table, &self.group_seqs[group_id])
    }

    pub fn eval_overlap(&self, group_a: GroupId, group_b: GroupId, table: &GroupMap) -> f64 {
        self.eval_seqs(table, &self.pair_paths[(group_a, group_b)])
    }

    fn eval_seqs(&self,
                 table: &GroupMap,
                 seqs: &SeqAssocList<GroupId, f64>)
                 -> f64 {
        seqs.iter()
            .map(|(group_seq, value)| {
                let key_seq = group_seq.cloned().map(|group_id| table[group_id]);
                return self.path_costs.get(key_seq) * value;
            })
            .sum()
    }
}
