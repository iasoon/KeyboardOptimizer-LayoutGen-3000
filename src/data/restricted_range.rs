use cat::*;
use rand::Rng;
use rand::seq::SliceRandom;

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
        self.items.shuffle(gen);

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

impl<T> HasCount<T> for Permutation<T> {
    fn count(&self) -> Count<T> {
        self.positions.count()
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
    use rand::Rng;
    use rand_distr::{Binomial, Distribution};
    use rand::seq::SliceRandom;

    use proptest::test_runner::TestRunner;
    use proptest::strategy::{Strategy, BoxedStrategy, ValueTree, NewTree};
    use std::mem;
    use std::fmt;
    use std::fmt::Debug;
    use std::marker::PhantomData;

    use cat::internal::{to_count, to_num};
    use cat::ops::*;

    fn segment_permutation<T>(items: Permutation<T>, segments: Vec<Segment>)
        -> SegmentedPermutation<T>
    {
        let mut item_segment = items.map_nums(|_| 0);

        let mut segment_end = items.count().as_usize();
        for segment_num in (0..segments.len()).rev() {
            for i in segments[segment_num].offset..segment_end {
                item_segment[items[i]] = segment_num;
            }
            segment_end = segments[segment_num].offset;
        }

        return SegmentedPermutation {
            items,
            segments,
            item_segment,
        };
    }

    // TODO: oh please break this function up
    fn generate_range<T, G: Rng>(rng: &mut G, t_count: Count<T>)
        -> RestrictedRange<T>
    {
        // choose the values for the range
        let mut permutation = Permutation::new(t_count);
        permutation.shuffle(rng);

        // then draw the structure
        let max_segments = t_count.as_usize();
        let num_segments = 1 + rng.gen_range(0, max_segments);

        let mut offsets = Vec::with_capacity(num_segments);
        offsets.push(0);
        for _ in 0..(num_segments - 1) {
            let offset = rng.gen_range(0, t_count.as_usize());
            offsets.push(offset);
        }
        offsets.sort();


        let mut segments = Vec::with_capacity(num_segments);
        let mut segment_end = t_count.as_usize();
        for i in (0..num_segments).rev() {    
            let segment_len = segment_end - offsets[i];
            let distribution = Binomial::new(segment_len as u64, 0.2).unwrap();
            let num_rejected = distribution.sample(rng) as usize;

            segments.push(Segment {
                offset: offsets[i],
                num_rejected,
            });
            
            segment_end = offsets[i];
        }
        segments.reverse();

        let values = segment_permutation(permutation, segments);

        let times_rejected = t_count.map_nums(|num| {
            let pos = values.pos(num);
            let segment_num = values.item_segment[num];

            if values.segments()[segment_num].accepts(pos) {
                0
            } else {
                const MAX_REJECTS: usize = 3;
                rng.gen_range(1, MAX_REJECTS)
            }
        });

        return RestrictedRange { values, times_rejected };
    }

    fn remove_segment<T: Clone>(p: &SegmentedPermutation<T>, segment_num: usize)
        -> SegmentedPermutation<T>
    {
        let mut segments = p.segments.clone();
        let mut items = p.items.clone();
        
        {
            // TODO: this part is exact duplication from the implementation.
            // maybe extract the shared code into a helper?
            let segment = segments.remove(segment_num);
            let prev_segment = &mut segments[segment_num - 1];

            for i in 0..segment.num_rejected {
                let pos = segment.offset + i;
                let dest = prev_segment.frontier() + i;
                items.swap(pos, dest);
            }

            prev_segment.num_rejected += segment.num_rejected;
        }

        return segment_permutation(items, segments);
    }


    #[derive(Debug)]
    struct RestrictedRangeStrategy<T> {
        phantom_t: PhantomData<T>,
        max_size: usize,
    }

    impl<T> RestrictedRangeStrategy<T> {
        fn new(max_size: usize) -> Self {
            RestrictedRangeStrategy {
                phantom_t: PhantomData,
                max_size,
            }
        }
    }

    impl<T> Strategy for RestrictedRangeStrategy<T>
        where T: Debug + Clone
    {
        type Tree = DomainShrinkerTree<T, 
            SimpleValueTree<RestrictedRangeSubtree<T>>
        >;
        type Value = RestrictedRange<T>;

        fn new_tree(&self, runner: &mut TestRunner) -> NewTree<Self> {
            let count = to_count(runner.rng().gen_range(1, self.max_size));
            let range = generate_range(runner.rng(), count);
            return Ok(
                DomainShrinkerTree::new(
                    count,
                    SimpleValueTree::new(RestrictedRangeSubtree::new(range)),
                )
            );
        }
    }

    #[derive(Debug, Clone)]
    struct RemoveSegment(usize);

    #[derive(Debug, Clone)]
    struct RestrictedRangeSubtree<T>
        where T: Debug + Clone
    {
        root: RestrictedRange<T>,
        next_shrink: RemoveSegment,
    }

    impl<T> RestrictedRangeSubtree<T>
        where T: Debug + Clone
    {
        fn new(root: RestrictedRange<T>) -> Self {
            RestrictedRangeSubtree {
                root,
                // cannot remove first segment
                next_shrink: RemoveSegment(1),
            }
        }
    }

    impl<T> ValueSubtree for RestrictedRangeSubtree<T>
        where T: Debug + Clone
    {
        type Value = RestrictedRange<T>;

        fn root(&self) -> RestrictedRange<T> {
            self.root.clone()
        }

        fn next_child(&mut self) -> Option<Self> {
            let RemoveSegment(to_remove) = self.next_shrink;

            if to_remove >= self.root.values.segments.len() {
                return None;
            }

            let values = remove_segment(&self.root.values, to_remove);
            let times_rejected = self.root.times_rejected.clone();
            let child = RestrictedRange { values, times_rejected };

            self.next_shrink = RemoveSegment(to_remove + 1);
            return Some(RestrictedRangeSubtree::new(child));
        }

    }

    impl<T> ShrinkDomain<T> for RestrictedRangeSubtree<T>
        where T: Debug + Clone
    {
        fn shrink_remove(&self, to_remove: Num<T>) -> Self {
            RestrictedRangeSubtree {
                root: self.root.shrink_remove(to_remove),
                next_shrink: self.next_shrink.clone(),
            }
        }
    }

    trait ValueSubtree: Sized {
        type Value: Debug;

        fn root(&self) -> Self::Value;

        /// Yield the next unvisited child
        fn next_child(&mut self) -> Option<Self>;
    }


    #[derive(Debug, Clone)]
    struct SimpleValueTree<T> {
        pos: T,
        parent: Option<T>,
    }


    impl<T> SimpleValueTree<T> {
        pub fn new(subtree: T) -> Self {
            SimpleValueTree {
                pos: subtree,
                parent: None,
            }
        }
    }

    impl<T> ValueTree for SimpleValueTree<T>
        where T: Debug + ValueSubtree
    {
        type Value = T::Value;

        fn current(&self) -> T::Value {
            self.pos.root()
        }

        fn simplify(&mut self) -> bool {
            match self.pos.next_child() {
                None => false,
                Some(child) => {
                    let parent = mem::replace(&mut self.pos, child);
                    self.parent = Some(parent);
                    true
                }
            }
        }

        fn complicate(&mut self) -> bool {
            match self.parent.take() {
                None => false,
                Some(parent) => {
                    self.pos = parent;
                    true
                }
            }
        }
    }

    impl<D, T> ShrinkDomain<D> for SimpleValueTree<T>
        where T: ShrinkDomain<D>
    {
        fn shrink_remove(&self, to_remove: Num<D>) -> Self {
            SimpleValueTree {
                pos: self.pos.shrink_remove(to_remove),
                parent: self.parent.as_ref().map(|parent| {
                    parent.shrink_remove(to_remove)
                }),
            }
        }
    }

    #[derive(Debug)]
    struct DomainShrinkerTree<D, T> {
        pos: DomainShrinkerSubtree<D, T>,
        parent: Option<DomainShrinkerSubtree<D, T>>,
        shrinking_inner: bool,
    }

    impl<D, T> DomainShrinkerTree<D, T>
        where T: Debug + Clone + ValueTree + ShrinkDomain<D>
    {
        fn new(count: Count<D>, value: T) -> Self {
            DomainShrinkerTree {
                pos: DomainShrinkerSubtree::new(count, value),
                parent: None,
                shrinking_inner: false,
            }
        }
    }

    impl<D, T> ValueTree for DomainShrinkerTree<D, T>
        where T: Debug + Clone + ValueTree + ShrinkDomain<D>
    {
        type Value = T::Value;

        fn current(&self) -> T::Value {
            self.pos.value.current()
        }

        fn simplify(&mut self) -> bool {
            loop {
                if self.shrinking_inner {
                    return self.pos.value.simplify();
                }

                match self.pos.next_child() {
                    Some(child) => {
                        let parent = mem::replace(&mut self.pos, child);
                        self.parent = Some(parent);
                        return true;
                    }
                    None => {
                        self.shrinking_inner = true;
                    }
                }
            }
        }

        fn complicate(&mut self) -> bool {
            if self.shrinking_inner {
                return self.pos.value.complicate()
            }

            match self.parent.take() {
                None => false,
                Some(parent) => {
                    self.pos = parent;
                    true
                }
            }
        }
    }

    trait ShrinkDomain<T> {
        /// shrink the value by removing an element from the domain.
        /// this is done in a swap-remove fashion: the current last element
        /// assumes the number of the removed element.
        fn shrink_remove(&self, to_remove: Num<T>) -> Self;
    }

    impl<T, A, B> ShrinkDomain<T> for (A, B)
        where A: ShrinkDomain<T>,
              B: ShrinkDomain<T>
    {
        fn shrink_remove(&self, to_remove: Num<T>) -> Self {
            let (ref a, ref b) = self;
            (a.shrink_remove(to_remove), b.shrink_remove(to_remove))
        }
    }

    impl<T, V> ShrinkDomain<T> for Table<T, V>
        where V: Clone
    {
        fn shrink_remove(&self, to_remove: Num<T>) -> Self {
            // create a new table that is one element shorter
            let new_count = to_count(self.count().as_usize() - 1);
            let mut table = new_count.map_nums(|num| self[num].clone());

            if to_remove.as_usize() < new_count.as_usize() {
                // swap role of last element and removed element
                table[to_remove] = self[to_num(new_count.as_usize())].clone();
            }
            return table;
        }
    }

    fn permutation_from_vec<T>(items: Vec<Num<T>>) -> Permutation<T> {
        let mut positions = to_count(items.len()).map_nums(|_| 0);
        for (pos, &num) in items.iter().enumerate() {
            positions[num] = pos;
        }
        return Permutation { items, positions };
    }

    impl<T> ShrinkDomain<T> for Permutation<T> {
        fn shrink_remove(&self, to_remove: Num<T>) -> Self {
            let last = to_num(self.items.len() - 1);
            let mut items = self.items.clone();

            items[self.positions[last]] = to_remove;
            items.remove(self.positions[to_remove]);

            return permutation_from_vec(items);
        }
    }

    impl<T> ShrinkDomain<T> for SegmentedPermutation<T> {
        fn shrink_remove(&self, to_remove: Num<T>) -> Self {
            let segment_num = self.item_segment[to_remove];
            let mut segments = self.segments.clone();
            if !segments[segment_num].accepts(self.items.pos(to_remove)) {
                segments[segment_num].num_rejected -= 1;
            }
            // shift all following segments one place to the left
            for i in (segment_num+1)..segments.len() {
                segments[i].offset -= 1;
            }
            return SegmentedPermutation {
                items: self.items.shrink_remove(to_remove),
                item_segment: self.item_segment.shrink_remove(to_remove),
                segments,
            };
        }
    }

    impl<T> ShrinkDomain<T> for RestrictedRange<T> {
        fn shrink_remove(&self, to_remove: Num<T>) -> Self {
            RestrictedRange {
                values: self.values.shrink_remove(to_remove),
                times_rejected: self.times_rejected.shrink_remove(to_remove),
            }
        }
    }

    type ShrinkDomainTree<D, T> = SimpleValueTree<DomainShrinkerSubtree<D, T>>;

    #[derive(Debug)]
    struct DomainShrinkerSubtree<D, T> {
        value: T,
        count: Count<D>,
        next_step: Enumerator<D>,
    }

    impl<D, T> DomainShrinkerSubtree<D, T> {
        fn new(count: Count<D>, value: T) -> Self {
            DomainShrinkerSubtree {
                value,
                next_step: count.nums(),
                count,
            }
        }
    }

    impl<D, T> ValueSubtree for DomainShrinkerSubtree<D, T>
        where T: Debug + Clone + ShrinkDomain<D>
    {
        type Value = T;

        fn root(&self) -> T {
            self.value.clone()
        }

        fn next_child(&mut self) -> Option<Self> {
            self.next_step.next().map(|to_remove| {
                let count = to_count(self.count.as_usize() - 1);
                let value = self.value.shrink_remove(to_remove);
                return Self::new(count, value);
            })
        }
    }

    fn restricted_range(max_size: usize) -> RestrictedRangeStrategy<()> {
        RestrictedRangeStrategy::new(max_size)
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

    #[derive(Clone)]
    struct Subset<T> {
        included: Table<T, bool>,
    }

    impl<T> Subset<T> {
        fn from_items(count: Count<T>, items: &[Num<T>]) -> Self {
            let mut included = count.map_nums(|_| false);
            for &num in items {
                included[num] = true;
            }
            return Subset { included };
        }

        fn iter<'a>(&'a self) -> impl Iterator<Item = Num<T>> + 'a {
            self.included.nums().filter(move |&num| self.included[num] )
        }

        fn to_vec(&self) -> Vec<Num<T>> {
            self.iter().collect()
        }
    }

    impl<T> Debug for Subset<T> {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            f.debug_list().entries(self.iter()).finish()
        }
    }
    
    impl<T> ShrinkDomain<T> for Subset<T> {
        fn shrink_remove(&self, to_remove: Num<T>) -> Self {
            Subset { included: self.included.shrink_remove(to_remove) }
        } 
    }

    fn generate_subset<T, G: Rng>(
        rng: &mut G,
        count: Count<T>,
        mut items: Vec<Num<T>>
    ) -> Subset<T>
    {
        if items.is_empty() {
            return Subset::from_items(count, &[]);
        }

        items.shuffle(rng);
        let num_items = rng.gen_range(0, items.len());
        return Subset::from_items(count, &items[0..num_items]);
    }


    struct RangeSubsetStrategy<T, F> {
        phantom_t: PhantomData<T>,
        domain_func: F,
        max_size: usize,
    }

    impl<T, F> RangeSubsetStrategy<T, F>
        where F: Fn(&RestrictedRange<T>) -> Vec<Num<T>>
    {
        fn new(max_size: usize, domain_func: F) -> Self {
            RangeSubsetStrategy {
                phantom_t: PhantomData,
                domain_func,
                max_size,
            }
        }
    }


    impl<T, F> Debug for RangeSubsetStrategy<T, F>
        where F: Fn(&RestrictedRange<T>) -> Vec<Num<T>>
    {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            f.debug_struct("RangeSubsetStrategy")
                .field("max_size", &self.max_size)
                .field("domain_func", &"<function>")
                .finish()
        }
    }

    impl<T, F> Strategy for RangeSubsetStrategy<T, F>
        where T: Debug + Clone,
              F: Copy + Fn(&RestrictedRange<T>) -> Vec<Num<T>>,
    {
        type Tree = DomainShrinkerTree<T, SimpleValueTree<RangeSubsetSubtree<T, F>>>;
        type Value = (RestrictedRange<T>, Subset<T>);

        fn new_tree(&self, runner: &mut TestRunner) -> NewTree<Self> {
            let t_count = to_count(runner.rng().gen_range(1, self.max_size));
            let range = generate_range(runner.rng(), t_count);

            let subset_domain = (self.domain_func)(&range);
            let subset = generate_subset(runner.rng(), t_count, subset_domain);

            Ok(
                DomainShrinkerTree::new(
                    t_count,
                    SimpleValueTree::new(
                    RangeSubsetSubtree {
                        range: RestrictedRangeSubtree::new(range),
                        subset,
                        domain_func: self.domain_func,
                    }
                ))
            )
        }
    }

    #[derive(Clone)]
    struct RangeSubsetSubtree<D, F>
        where D: Debug + Clone
    {
        range: RestrictedRangeSubtree<D>,
        subset: Subset<D>,
        domain_func: F,
    }

    impl<D, F> Debug for RangeSubsetSubtree<D, F>
        where F: Fn(&RestrictedRange<D>) -> Vec<Num<D>>,
              D: Debug + Clone,
    {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            f.debug_struct("RangeSubsetSubtree")
                .field("range", &self.range)
                .field("subset", &self.subset)
                .field("domain_func", &"<function>")
                .finish()
        }
    }


    impl<D, F> ValueSubtree for RangeSubsetSubtree<D, F>
        where D: Debug + Clone,
              F: Copy + Fn(&RestrictedRange<D>) -> Vec<Num<D>>,
    {
        type Value = (RestrictedRange<D>, Subset<D>);

        fn root(&self) -> Self::Value {
            (self.range.root(), self.subset.clone())
        }

        fn next_child(&mut self) -> Option<Self> {
            self.range.next_child().map(|range| {
                let allowed_values = (self.domain_func)(&range.root);

                let mut included = self.subset.included.map(|_| false);
                for num in allowed_values {
                    included[num] = self.subset.included[num];
                }
                
                RangeSubsetSubtree {
                    domain_func: self.domain_func,
                    subset: Subset { included },
                    range,
                }
            })
        }
    }

    impl<D, F> ShrinkDomain<D> for RangeSubsetSubtree<D, F>
        where D: Debug + Clone,
              F: Copy
    {
        fn shrink_remove(&self, to_remove: Num<D>) -> Self {
            RangeSubsetSubtree {
                range: self.range.shrink_remove(to_remove),
                subset: self.subset.shrink_remove(to_remove),
                domain_func: self.domain_func,
            }
        }
    }

    fn range_and_subset(max_size: usize)
        -> BoxedStrategy<(RestrictedRange<()>, Subset<()>)>
    {
        RangeSubsetStrategy::new(max_size, |range| {
            range.times_rejected.count().nums().collect()
        }).boxed()
    }

    fn range_and_rejected(max_size: usize)
        -> BoxedStrategy<(RestrictedRange<()>, Subset<()>)>
    {
        RangeSubsetStrategy::new(max_size, |range| {
            range.times_rejected.nums().filter(|&num| {
                range.times_rejected[num] > 0
            }).collect()
        }).boxed()
    }

    fn range_and_restricted(max_size: usize)
        -> BoxedStrategy<(RestrictedRange<()>, Subset<()>)>
    {
        RangeSubsetStrategy::new(max_size, |range| {
            range.times_rejected.nums().filter(|&num| {
                range.values.item_segment[num] > 0
            }).collect()
        }).boxed()
    }


    fn check_times_rejected<T, F>(range: &RestrictedRange<T>, expected: F)
        where F: Fn(Num<T>) -> usize
    {
        for (num, &reject_count) in range.times_rejected.enumerate() {
            let expected_reject_count = expected(num);
            if reject_count != expected_reject_count {
                panic!{
                    "expected {} to be rejected {} times, but was {} times",
                    num.as_usize(),
                    expected_reject_count,
                    reject_count,
                }
            }
        }
    }

    fn check_segment<T, F>(range: &RestrictedRange<T>, expected: F)
        where F: Fn(Num<T>) -> usize
    {
        for (num, &segment_num) in range.values.item_segment.enumerate() {
            let expected_segment = expected(num);
            if segment_num != expected_segment {
                panic!{
                    "expected {} to be in segment {}, but was in segment {}",
                    num.as_usize(),
                    expected_segment,
                    segment_num,
                }
            }
        }
    }

    fn sorted<T>(values: &[T]) -> Vec<T>
        where T: Ord + Clone
    {
        let mut vec = values.to_vec();
        vec.sort();
        return vec;
    }

    fn diff<T>(fst: &RestrictedRange<T>, snd: &RestrictedRange<T>)
        -> Vec<Num<T>>
    {
        let mut vec = sorted(fst.accepted());
        vec.retain(|&num| !snd.accepts(num));
        return vec;
    }

    proptest! {
        #[test]
        fn test_generation(range in restricted_range(10)) {
            check_range_integrity(&range);
        }

        #[test]
        fn test_reject((range, subset) in range_and_subset(10)) {
            let before = range;
            let to_reject = subset.to_vec();

            let mut after = before.clone();
            let removed = sorted(after.add_rejection(&to_reject));

            check_range_integrity(&after);
            check_times_rejected(&after, |num| {
                if to_reject.contains(&num) {
                    before.times_rejected[num] + 1
                } else {
                    before.times_rejected[num]
                }
            });

            assert_eq!(diff(&before, &after), removed);
        }

        #[test]
        fn test_unreject((range, subset) in range_and_rejected(10)) {
            let before = range;
            let to_unreject = subset.to_vec();

            let mut after = before.clone();
            let added = sorted(after.remove_rejection(&to_unreject));

            check_range_integrity(&after);
            check_times_rejected(&after, |num| {
                if to_unreject.contains(&num) {
                    before.times_rejected[num] - 1
                } else {
                    before.times_rejected[num]
                }
            });

            assert_eq!(diff(&after, &before), added);
        }

        #[test]
        fn test_restrict((range, subset) in range_and_subset(10)) {
            let before = range;
            let to_restrict = subset.to_vec();

            let mut after = before.clone();
            let removed = sorted(after.add_restriction(&to_restrict));

            check_range_integrity(&after);
            check_segment(&after, |num| {
                if to_restrict.contains(&num) {
                    before.values.item_segment[num] + 1
                } else {
                    before.values.item_segment[num]
                }
            });

            assert_eq!(diff(&before, &after), removed);
        }

        #[test]
        fn test_unrestrict((range, subset) in range_and_restricted(10)) {
            let before = range;
            let to_unrestrict = subset.to_vec();

            let mut after = before.clone();
            let added = sorted(after.remove_restriction(&to_unrestrict));

            check_range_integrity(&after);
            check_segment(&after, |num| {
                if to_unrestrict.contains(&num) {
                    before.values.item_segment[num] - 1
                } else {
                    before.values.item_segment[num]
                }
            });

            assert_eq!(diff(&after, &before), added);
        }

        // TODO: how to properly test this?
        #[test]
        fn test_shrink(range in restricted_range(10), n in 0..10usize) {
            prop_assume!(n < range.times_rejected.count().as_usize());
            let to_remove = to_num(n);
            let last = to_num(range.times_rejected.count().as_usize() - 1);
            let after = range.shrink_remove(to_remove);

            check_range_integrity(&after);
            check_times_rejected(&after, |num| {
                if num == to_remove {
                    range.times_rejected[last]
                } else {
                    range.times_rejected[num]
                }
            });
            check_segment(&after, |num| {
                if num == to_remove {
                    range.values.item_segment[last]
                } else {
                    range.values.item_segment[num]
                }
            });
        }
    }
}