use super::*;
use cat::*;
use rand::Rng;

use std::ops::{Index, Range};

#[derive(Debug, Clone)]
struct Permutation<T> {
    items: Vec<Num<T>>,
    positions: Table<T, usize>,
}

impl<T> Permutation<T> {
    pub fn new(t_count: Count<T>) -> Self {
        Permutation {
            items: t_count.nums().collect(),
            positions: Table::from_vec((0..t_count.as_usize()).collect()),
        }
    }

    pub fn swap(&mut self, a: usize, b: usize) {
        self.items.swap(a, b);
        self.positions[self.items[a]] = a;
        self.positions[self.items[b]] = b;
    }

    pub fn pos(&self, value: Num<T>) -> usize {
        self.positions[value]
    }

    pub fn len(&self) -> usize {
        self.items.len()
    }

    pub fn shuffle<G>(&mut self, gen: &mut G)
        where G: Rng
    {
        gen.shuffle(&mut self.items);

        for i in 0..self.items.len() {
            self.update_pos(i);
        }
    }

    fn update_pos(&mut self, pos: usize) {
        self.positions[self.items[pos]] = pos;
    }
}

impl<T> Index<usize> for Permutation<T> {
    type Output = Num<T>;

    fn index<'a>(&'a self, idx: usize) -> &'a Num<T> {
        &self.items[idx]
    }
}

impl<T> Index<Range<usize>> for Permutation<T> {
    type Output = [Num<T>];

    fn index<'a>(&'a self, range: Range<usize>) -> &'a [Num<T>] {
        &self.items[range]
    }
}

#[derive(Debug, Clone)]
pub struct Segment {
    pub offset: usize,
    pub num_rejected: usize,
}

impl Segment {
    pub fn empty(offset: usize) -> Self {
        Segment {
            offset: offset,
            num_rejected: 0,
        }
    }

    pub fn frontier(&self) -> usize {
        self.offset + self.num_rejected
    }

    fn accepts(&self, pos: usize) -> bool {
        pos >= self.frontier()
    }
}

#[derive(Debug, Clone)]
pub struct SegmentedPermutation<T> {
    items: Permutation<T>,
    segments: Vec<Segment>,
    item_segment: Table<T, usize>,

}

impl<T> SegmentedPermutation<T> {
    pub fn new(t_count: Count<T>) -> Self {
        SegmentedPermutation {
            items: Permutation::new(t_count),
            segments: vec![Segment::empty(0)],
            item_segment: Table::from_vec(vec![0; t_count.as_usize()]),
        }
    }

    pub fn accept(&mut self, item_num: Num<T>) {
        let segment = self.item_segment[item_num];
        let pos = self.items.pos(item_num);
        self.accept_pos(segment, pos);
    }

    pub fn reject(&mut self, item_num: Num<T>) {
        let segment = self.item_segment[item_num];
        let pos = self.items.pos(item_num);
        self.reject_pos(segment, pos);
    }

    pub fn promote(&mut self, item_num: Num<T>) {
        let segment = self.item_segment[item_num];
        let pos = self.items.pos(item_num);
        self.promote_pos(segment, pos);
        self.item_segment[item_num] = segment + 1;
    }

    pub fn demote(&mut self, item_num: Num<T>) {
        let segment = self.item_segment[item_num];
        let pos = self.items.pos(item_num);
        self.demote_pos(segment, pos);
        self.item_segment[item_num] = segment - 1;
    }

    pub fn segments<'a>(&'a self) -> &'a [Segment] {
        &self.segments
    }

    pub fn push_segment(&mut self) {
        self.segments.push(Segment::empty(self.items.len()));
    }

    pub fn pop_segment(&mut self) {
        let popped_segment = self.segments.pop().unwrap();
        let num_segments = self.segments.len();
        let last_segment = &mut self.segments[num_segments - 1];

        // put all items in the previous segment
        for i in popped_segment.offset..self.items.len() {
            self.item_segment[self.items[i]] = num_segments - 1;
        }

        // move the rejected items to the rejected zone
        for i in 0..popped_segment.num_rejected {
            let pos = popped_segment.offset + i;
            let destination = last_segment.frontier() + i;
            self.items.swap(pos, destination);
        }
        last_segment.num_rejected += popped_segment.num_rejected;
    }

    pub fn pos(&self, item_num: Num<T>) -> usize {
        self.items.pos(item_num)
    }

    pub fn len(&self) -> usize {
        self.items.len()
    }

    fn promote_pos(&mut self, segment: usize, pos: usize) {
        let promoting_pos = self.segments[segment + 1].offset - 1;

        if self.segments[segment].accepts(pos) {
            self.items.swap(pos, promoting_pos);
            self.extend_rejected(segment + 1);
            self.accept_pos(segment + 1, promoting_pos);
        } else {
            let accepted_pos = self.accept_pos(segment, pos);
            self.items.swap(accepted_pos, promoting_pos);
            self.extend_rejected(segment + 1);
        }
    }

    fn demote_pos(&mut self, segment: usize, pos: usize) {
        let demoting_pos = self.segments[segment].offset;

        if self.segments[segment].accepts(pos) {
            let rejected_pos = self.reject_pos(segment, pos);
            self.items.swap(rejected_pos, demoting_pos);
            self.extend_accepted(segment - 1);
        } else {
            self.items.swap(pos, demoting_pos);
            self.extend_accepted(segment - 1);
            self.reject_pos(segment - 1, demoting_pos);
        }
    }

    fn extend_accepted(&mut self, segment: usize) {
        self.segments[segment + 1].offset += 1;
        self.segments[segment + 1].num_rejected -= 1;
    }

    fn extend_rejected(&mut self, segment: usize) {
        self.segments[segment].offset -= 1;
        self.segments[segment].num_rejected += 1;
    }

    fn accept_pos(&mut self, segment: usize, pos: usize) -> usize {
        self.segments[segment].num_rejected -= 1;
        let frontier = self.segments[segment].frontier();
        self.items.swap(pos, frontier);
        return frontier;
    }

    fn reject_pos(&mut self, segment: usize, pos: usize) -> usize {
        let frontier = self.segments[segment].frontier();
        self.items.swap(pos, frontier);
        self.segments[segment].num_rejected += 1;
        return frontier;
    }
}

impl<T> Index<usize> for SegmentedPermutation<T> {
    type Output = Num<T>;

    fn index<'a>(&'a self, idx: usize) -> &'a Num<T> {
        &self.items[idx]
    }
}

impl<T> Index<Range<usize>> for SegmentedPermutation<T> {
    type Output = [Num<T>];

    fn index<'a>(&'a self, range: Range<usize>) -> &'a [Num<T>] {
        &self.items[range]
    }
}

#[derive(Debug, Clone)]
pub struct RestrictedRange<T> {
    values: SegmentedPermutation<T>,
    times_rejected: Table<T, usize>,
}

impl<T> RestrictedRange<T> {
    pub fn new(value_count: Count<T>) -> Self {
        RestrictedRange {
            values: SegmentedPermutation::new(value_count),
            times_rejected: value_count.map_nums(|_| 0),
        }
    }

    pub fn accepted<'a>(&'a self) -> &'a [Num<T>] {
        &self.values[(self.frontier()..self.values.len())]
    }

    pub fn rejected<'a>(&'a self) -> &'a [Num<T>] {
        &self.values[(0..self.frontier())]
    }

    pub fn accepts(&self, value_num: Num<T>) -> bool {
        let segment = self.values.segments().last().unwrap();
        let pos = self.values.pos(value_num);
        return segment.accepts(pos);
    }

    // returns the values that were rejected by this operation
    pub fn add_rejection<'a>(&'a mut self, rejected: &[Num<T>])
        -> &'a [Num<T>]
    {

        let prev_frontier = self.frontier();
        for &value_num in rejected {
            self.reject(value_num);
        }

        // When a value is rejected, it is placed behind the frontier.
        return &self.values[(prev_frontier..self.frontier())];
    }

    pub fn remove_rejection<'a>(&'a mut self, rejected: &[Num<T>])
        -> &'a [Num<T>]
    {
        let prev_frontier = self.frontier();
        for &value_num in rejected {
            self.unreject(value_num);
        }
        
        // when a value is unrejected, it is places in front of the frontier.
        return &self.values[(self.frontier()..prev_frontier)];
    }

    // returns the values that were rejected by this operation
    pub fn add_restriction<'a>(&'a mut self, allowed: &[Num<T>])
        -> &'a [Num<T>]
    {
        // add a new segment for this restriction
        self.values.push_segment();
        for &value_num in allowed {
            self.values.promote(value_num);
        }
        let num_segments = self.values.segments.len();

        // this was the last segment before adding the restriction
        let prev_segment = &self.values.segments[num_segments - 2];
        let last_segment = &self.values.segments[num_segments - 1];

        // the values that were dropped in this operation are the values that
        // are accepted in the now one-but-last segment.
        return &self.values[(prev_segment.frontier()..last_segment.offset)];
    }

    // returns the values that are now allowed because of this operation
    pub fn remove_restriction<'a>(&'a mut self, allowed: &[Num<T>])
        -> &'a [Num<T>]
    {
        // first, we demote all values that are currently not in the last
        // segment.
        let num_segments = self.values.segments.len();
        for &value in allowed {
            if self.values.item_segment[value] < num_segments - 1 {
                // value is not in the last segment
                self.values.demote(value);
            }
        }

        // Merge the last two segments. This is done in a way that keeps the
        // previously accepted values and newly-accepted values separate,
        // so that a range of newly-accepted values can be returned.

        // all currently accepted items are past this frontier; they will not be
        // moved.
        let prev_frontier = self.frontier();
        self.values.pop_segment();
        return &self.values[self.frontier()..prev_frontier];
    }

    pub fn reject(&mut self, value_num: Num<T>) {
        if self.times_rejected[value_num] == 0 {
            self.values.reject(value_num);
        }
        self.times_rejected[value_num] += 1;
    }

    pub fn unreject(&mut self, value_num: Num<T>) {
        self.times_rejected[value_num] -= 1;
        if self.times_rejected[value_num] == 0 {
            self.values.accept(value_num);
        }
    }

    fn frontier(&self) -> usize {
        let last_segment = self.values.segments().last().unwrap();
        return last_segment.frontier();
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use Result;
    use failure::ResultExt;
    use rand::{Rng, RngCore};
    use rand::distributions::{Binomial, Distribution};
    use proptest::test_runner::{TestRunner, TestCaseError};
    use proptest::strategy::{Strategy, ValueTree, NewTree};
    use std::fmt::Debug;

    use cat::internal::{to_count, to_num};

    #[derive(Debug)]
    struct RestrictedRangeStrategy<T> {
        t_count: Count<T>,
    }

    impl<T> Strategy for RestrictedRangeStrategy<T>
        where T: Debug + Clone
    {
        type Tree = RestrictedRangeValueTree<T>;
        type Value = RestrictedRange<T>;

        fn new_tree(&self, runner: &mut TestRunner) -> NewTree<Self> {
            // first, choose the values for the range
            let mut permutation = Permutation::new(self.t_count);
            permutation.shuffle(runner.rng());

            // then draw the structure
            let max_segments = self.t_count.as_usize();
            let num_segments = runner.rng().gen_range(1, max_segments);

            let mut offsets = Vec::with_capacity(num_segments);
            offsets.push(0);
            for _ in 0..(num_segments - 1) {
                let offset = runner.rng().gen_range(0, self.t_count.as_usize());
                offsets.push(offset);
            }
            offsets.sort();


            let mut segments = Vec::with_capacity(num_segments);
            let mut segment_end = self.t_count.as_usize();
            for i in (0..num_segments).rev() {    
                let segment_len = segment_end - offsets[i];
                let distribution = Binomial::new(segment_len as u64, 0.2);
                let num_rejected = distribution.sample(runner.rng()) as usize;

                segments.push(Segment {
                    offset: offsets[i],
                    num_rejected,
                });
                
                segment_end = offsets[i];
            }
            segments.reverse();

            let mut item_segment = self.t_count.map_nums(|_| 0);
            let mut segment_end = self.t_count.as_usize();
            for segment_num in (0..num_segments).rev() {
                for i in offsets[segment_num]..segment_end {
                    item_segment[permutation[i]] = segment_num;
                }
                segment_end = offsets[segment_num];
            }

            let values = SegmentedPermutation {
                items: permutation,
                segments,
                item_segment,
            };

            let mut times_rejected = self.t_count.map_nums(|num| {
                let pos = values.pos(num);
                let segment_num = values.item_segment[num];

                if values.segments()[segment_num].accepts(pos) {
                    0
                } else {
                    const MAX_REJECTS: usize = 3;
                    runner.rng().gen_range(1, MAX_REJECTS)
                }
            });

            let rr = RestrictedRange {
                values,
                times_rejected,
            };

            return Ok(RestrictedRangeValueTree {
                value: rr,
            })
        }
    }

    #[derive(Debug)]
    struct RestrictedRangeValueTree<T> {
        value: RestrictedRange<T>,
    }

    impl<T> ValueTree for RestrictedRangeValueTree<T>
        where T: Debug + Clone
    {
        type Value = RestrictedRange<T>;

        fn current(&self) -> RestrictedRange<T> {
            self.value.clone()
        }

        fn simplify(&mut self) -> bool {
            false
        }

        fn complicate(&mut self) -> bool {
            false
        }
    }

    fn restricted_range(count: usize) -> RestrictedRangeStrategy<()> {
        RestrictedRangeStrategy { t_count: to_count(count) }
    }

    fn check_segments<T>(p: &SegmentedPermutation<T>)  {
        let mut segment_end = p.items.len();
        for segment_num in (0..p.segments().len()).rev() {
            let segment = &p.segments()[segment_num];

            for pos in segment.offset..segment_end {
                if p.item_segment[p.items[pos]] != segment_num {
                    panic!(
                        "{:?} in wrong segment: assigned {} but located in {}",
                        p.items[pos],
                        p.item_segment[p.items[pos]],
                        segment_num,
                    );
                }
            }

            if segment.offset + segment.num_rejected > segment_end {
                panic!("segment no. {} leaks into next segment", segment_num);
            }

            segment_end = p.segments()[segment_num].offset;
        }
    }

    fn check_rejects<T>(range: &RestrictedRange<T>) {
        let p = &range.values;
        for pos in 0..p.len() {
            let segment = &p.segments()[p.item_segment[p.items[pos]]];
            if segment.accepts(pos) {
                if range.times_rejected[p.items[pos]] > 0 {
                    panic!(
                        "{:?} is rejected but not in rejection zone",
                        p.items[pos],
                    )
                }
            } else {
                if range.times_rejected[p.items[pos]] == 0 {
                    panic!(
                        "{:?} is not rejected but in rejection zone",
                        p.items[pos],
                    )
                }
            }
        }
    }

    fn check_range_integrity<T>(range: &RestrictedRange<T>) {
        check_segments(&range.values);
        check_rejects(&range);
    }

    #[derive(Debug)]
    struct SubsetValueTree<T> {
        values: Vec<Num<T>>,
        mask: Vec<bool>,
    }

    impl<T> SubsetValueTree<T> {
        fn new(values: Vec<Num<T>>) -> Self {
            let mask = vec![true; values.len()];
            return SubsetValueTree {
                values,
                mask
            };
        }

        fn random<R: Rng>(mut values: Vec<Num<T>>, rng: &mut R) -> Self {
            rng.shuffle(&mut values);
            let num_values = rng.gen_range(0, values.len());
            return Self::new(values[0..num_values].to_vec());
        }
    }

    impl<T> ValueTree for SubsetValueTree<T>
        where T: Debug + Clone
    {
        type Value = Vec<Num<T>>;

        fn current(&self) -> Self::Value {
            let mut vec = Vec::with_capacity(self.values.len());
            for i in 0..self.values.len() {
                if self.mask[i] {
                    vec.push(self.values[i]);
                }
            }
            return vec;
        }

        fn simplify(&mut self) -> bool {
            false
        }

        fn complicate(&mut self) -> bool {
            false
        }

    }


    #[derive(Debug)]
    struct RangeSubsetStrategy<T> {
        t_count: Count<T>,
    }

    impl<T> Strategy for RangeSubsetStrategy<T>
        where T: Debug + Clone
    {
        type Tree = RangeSubsetValueTree<T>;
        type Value = (RestrictedRange<T>, Vec<Num<T>>);

        fn new_tree(&self, runner: &mut TestRunner) -> NewTree<Self> {
            let rr_strategy = RestrictedRangeStrategy { t_count: self.t_count };
            let values = self.t_count.nums().collect();
            let tree = RangeSubsetValueTree {
                range: rr_strategy.new_tree(runner)?,
                subset: SubsetValueTree::random(values, runner.rng()),
            };
            Ok(tree)
        }
    }

    #[derive(Debug)]
    struct RangeSubsetValueTree<T> {
        range: RestrictedRangeValueTree<T>,
        subset: SubsetValueTree<T>,
    }

    impl<T> ValueTree for RangeSubsetValueTree<T>
        where T: Debug + Clone
    {
        type Value = (RestrictedRange<T>, Vec<Num<T>>);

        fn current(&self) -> Self::Value {
            (self.range.current(), self.subset.current())
        }

        fn simplify(&mut self) -> bool {
            false
        }

        fn complicate(&mut self) -> bool {
            false
        }
    }

    fn range_subset(count: usize) -> RangeSubsetStrategy<()> {
        RangeSubsetStrategy {
            t_count: to_count(count),
        }
    }


    fn check_times_rejected<T, F>(range: &RestrictedRange<T>, expected: F)
        where F: Fn(Num<T>) -> usize
    {
        for (num, &reject_count) in range.times_rejected.enumerate() {
            assert_eq!(reject_count, expected(num));
        }
    }

    proptest! {
        #[test]
        fn test_generation(range in restricted_range(5)) {
            check_range_integrity(&range);
        }

        #[test]
        fn test_reject((range, to_reject) in range_subset(5)) {
            let before = range;

            let mut after = before.clone();
            after.add_rejection(&to_reject);

            check_range_integrity(&after);
            check_times_rejected(&after, |num| {
                if to_reject.contains(&num) {
                    before.times_rejected[num] + 1
                } else {
                    before.times_rejected[num]
                }
            });
        }
    }
}