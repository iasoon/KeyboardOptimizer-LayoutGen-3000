use model::{GroupId, Group};
use utils::*;
use layout::{Layout, Assignment, AssignmentMap};
use eval::Evaluator;
use heuristics::Walker;

type PathSet = SeqAssocList<GroupId, f64>;
type PathSetMap = BagTable<GroupId, PathSet>;

struct PathSetMapBuilder {
    table: BagTable<GroupId, SeqAssocListBuilder<GroupId, f64>>,
}

impl PathSetMapBuilder {
    fn new(seq_len: usize, bag_data: BagData<GroupId>) -> Self {
        PathSetMapBuilder {
            table: BagTable::new(bag_data, SeqAssocListBuilder::new(seq_len)),
        }
    }

    fn register_path(&mut self, path: &Vec<GroupId>, weight: f64) {
        for seq in SubSequences::new(path.as_slice(), self.table.data().len) {
            self.table.get_mut(seq.into_iter()).push(path.iter().cloned(), weight);
        }
    }

    fn build(self) -> PathSetMap {
        self.table.drain_map(|builder| builder.build())
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

fn init_assignment_delta(layout: &Layout, evaluator: &Evaluator) -> AssignmentMap<f64> {
    let mut walker = Walker::new(&layout, evaluator);
    AssignmentMap::from_fn(layout.kb_def, |assignment| walker.delta(&[assignment]))
}

pub struct Cache<'a> {
    layout: Layout<'a>,
    evaluator: &'a Evaluator,
    seqs: Vec<PathSetMap>,
    components: Vec<LookupTable<BagId<GroupId>, f64>>,
    assignment_delta: AssignmentMap<f64>,
}


impl<'a> Cache<'a> {
    pub fn new(layout: Layout<'a>, evaluator: &'a Evaluator) -> Self {
        let seqs = mk_path_set_maps(layout.kb_def.groups.elem_count(), &evaluator.seqs);
        let components = (0..seqs.len()-1).map(|i| {
            seqs[i].table().map(|seqs| {
                evaluator.eval_seqs(&layout.group_map, seqs)
            })
        }).collect();
        Cache {
            assignment_delta: init_assignment_delta(&layout, evaluator),
            layout: layout,
            evaluator: evaluator,
            seqs: seqs,
            components: components,
        }
    }

    pub fn score_assignments(&self, assignments: &[Assignment]) -> f64 {
        let mut walker = Walker::new(&self.layout, self.evaluator);
        let mut delta = 0.0;
        for (num, &assignment) in assignments.iter().enumerate() {
            let d = walker.alteration_delta(&[assignment], &assignments[0..num]);
            delta += self.assignment_delta[assignment] + d;
        }
        return delta;
    }

    pub fn update(&mut self, assignments: &[Assignment]) {
        {
            let mut walker = Walker::new(&self.layout, self.evaluator);

            let mut changed = LookupTable::new(self.layout.kb_def.groups.elem_count(), false);
            for &assignment in assignments.iter() {
                let group_id = assignment.group(self.layout.kb_def);
                changed[group_id] = true;
            }

            let kb_def = self.layout.kb_def;
            self.assignment_delta.map_mut(|assignment, delta| {
                let group_id = assignment.group(kb_def);
                if changed[group_id] {
                    walker.assign_all(assignments);
                    *delta = walker.delta(&[assignment]);
                    walker.reset_all(assignments);
                } else {
                    *delta += walker.alteration_delta(&[assignment], assignments);
                }
            });
        }

        for &assignment in assignments.iter() {
            self.layout.assign(assignment);
        }
    }
}
