use model::Loc;
use utils::LocMap;

#[derive(Clone)]
pub struct Path {
    pub locs: Vec<Loc>,
    pub weight: f64,
}

impl Path {
    pub fn len(&self) -> usize {
        self.locs.len()
    }

    pub fn iter<'a>(&'a self) -> impl Iterator<Item = &'a Loc> {
        self.locs.iter()
    }

    pub fn unique_locs<'a>(&'a self) -> impl Iterator<Item = &'a Loc> + 'a {
        self.locs
            .iter()
            .enumerate()
            .filter(move |&(idx, loc)| self.locs[..idx].iter().all(|l| l != loc))
            .map(|(_, loc)| loc)
    }
}

pub struct PathList {
    paths: Vec<Path>,
    idx: Vec<usize>,
}

impl PathList {
    fn new(mut paths: Vec<Path>) -> Self {
        paths.sort_by_key(|path| path.len());
        PathList {
            idx: Self::mk_idx(&paths),
            paths: paths,
        }
    }

    fn mk_idx(sorted_paths: &Vec<Path>) -> Vec<usize> {
        let mut idx = Vec::new();
        for (i, path) in sorted_paths.iter().enumerate() {
            if path.len() > idx.len() {
                idx.push(i);
            }
        }
        return idx;
    }

    fn len_end(&self, len: usize) -> usize {
        if len < self.idx.len() {
            self.idx[len]
        } else {
            self.paths.len()
        }
    }

    pub fn with_len<'a>(&'a self, len: usize) -> impl Iterator<Item = &'a Path> {
        self.paths[self.len_end(len - 1)..self.len_end(len)].iter()
    }
}

pub struct PathTable {
    pub all: PathList,
    pub loc_paths: LocMap<PathList>,
}

impl PathTable {
    pub fn new(paths: Vec<Path>, num_layers: usize, num_keys: usize) -> Self {
        PathTable {
            loc_paths: Self::mk_loc_paths(&paths, num_layers, num_keys),
            all: PathList::new(paths),
        }
    }

    fn mk_loc_paths(paths: &Vec<Path>, num_layers: usize, num_keys: usize) -> LocMap<PathList> {
        let mut loc_paths = LocMap::from_fn(num_layers, num_keys, |_| Vec::new());
        for path in paths.iter() {
            for &loc in path.unique_locs() {
                loc_paths[loc].push(path.clone());
            }
        }
        return loc_paths.drain_map(|paths| PathList::new(paths));
    }
}
