use utils::{SeqTable, SeqAssocList, SeqAssocListBuilder, LookupTable};
use model::{KbDef, Group, GroupId, KeyId};
use utils::{ElemCount, BagTable, BagData, SubSequences};
use layout::GroupMap;

type PathSet = SeqAssocList<GroupId, f64>;
type PathSetMap = BagTable<GroupId, PathSet>;

pub struct Evaluator {
    seqs: PathSet,
    path_costs: SeqTable<KeyId, f64>,
    seq_maps: Vec<PathSetMap>,
}

impl Evaluator {
    pub fn new(seqs: SeqAssocList<GroupId, f64>,
               path_costs: SeqTable<KeyId, f64>,
               kb_def: &KbDef)
               -> Self {
        Evaluator {
            path_costs: path_costs,
            seq_maps: mk_path_set_maps(kb_def.groups.elem_count(), &seqs),
            seqs: seqs,
        }
    }

    pub fn evaluate(&self, table: &GroupMap) -> f64 {
        self.eval_seqs(table, &self.seqs)
    }

    pub fn eval_component(&self, groups: &[GroupId], table: &GroupMap) -> f64 {
        let seqs = self.seq_maps[groups.len() - 1].get(groups.iter());
        return self.eval_seqs(table, seqs);
    }

    pub fn eval_seqs(&self,
                 table: &GroupMap,
                 seqs: &SeqAssocList<GroupId, f64>)
                 -> f64 {
        seqs.iter()
            .map(|(group_seq, value)| {
                let key_seq = group_seq.map(|&group_id| &table[group_id]);
                return self.path_costs.get(key_seq) * value;
            })
            .sum()
    }
}

#[derive(Clone)]
struct PathSetMapBuilderEntry {
    seqs: SeqAssocListBuilder<GroupId, f64>,
    last_pushed: usize,
}

impl PathSetMapBuilderEntry {
    fn push(&mut self, seq_num: usize, path: &Vec<GroupId>, weight: f64) {
        // prevent paths from being added twice
        if self.last_pushed < seq_num {
            self.seqs.push(path.iter().cloned(), weight);
            self.last_pushed = seq_num;
        }
    }
}

struct PathSetMapBuilder {
    table: BagTable<GroupId, PathSetMapBuilderEntry>,
    seq_num: usize,
}

impl PathSetMapBuilder {
    fn new(seq_len: usize, bag_data: BagData<GroupId>) -> Self {
        PathSetMapBuilder {
            seq_num: 0,
            table: BagTable::new(bag_data, PathSetMapBuilderEntry {
                seqs: SeqAssocListBuilder::new(seq_len),
                last_pushed: 0,
            }),
        }
    }

    fn register_path(&mut self, path: &Vec<GroupId>, weight: f64) {
        self.seq_num += 1;
        for seq in SubSequences::new(path.as_slice(), self.table.data().len) {
            self.table.get_mut(seq.into_iter()).push(self.seq_num, path, weight);
        }
    }

    fn build(self) -> PathSetMap {
        self.table.drain_map(|entry| entry.seqs.build())
    }
}

fn test(data: ElemCount<Group>, paths: &PathSet) {
    let (path_iter, &weight) = paths.iter().nth(0).unwrap();
    let path = path_iter.cloned().collect();

    let mut builder = PathSetMapBuilder::new(3, BagData {
        data: data.clone(),
        len: 1,
    });
    builder.register_path(&path, weight);
    let table = builder.build();
    for elem in path.iter().cloned() {
        let s = [elem];
        println!("{}", table.get(s.iter()).iter().count());
    }
}

fn mk_path_set_maps(data: ElemCount<Group>, paths: &PathSet) -> Vec<PathSetMap> {
    let mut vec = (1..paths.seq_len()+1).map(|len| {
        PathSetMapBuilder::new(3, BagData {
            data: data.clone(),
            len: len,
        })
    }).collect::<Vec<_>>();

    for (seq_iter, &weight) in paths.iter() {
        let seq = seq_iter.cloned().collect();
        for builder in vec.iter_mut() {
            builder.register_path(&seq, weight);
        }
    }
    return vec.into_iter().map(|builder| builder.build()).collect();
}

