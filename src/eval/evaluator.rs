use model::{Language, FreqTable};
use model::{Path, PathList, PathTable};
use layout::Keymap;

pub struct Evaluator {
    language: Language,
    path_table: PathTable,
}

impl Evaluator {
    pub fn new(language: Language, path_table: PathTable) -> Self {
        Evaluator {
            language: language,
            path_table: path_table,
        }
    }

    pub fn score(&self, keymap: &Keymap) -> f64 {
        self.score_path_list(keymap, &self.path_table.all)
    }

    fn score_path_list(&self, keymap: &Keymap, path_list: &PathList) -> f64 {
        self.language.freqs.iter().enumerate().skip(1).flat_map(|(len, freqs)| {
            path_list.with_len(len).map(move |path| {
                Self::score_path(freqs, keymap, path)
            })
        }).sum()
    }

    fn score_path(freqs: &FreqTable, keymap: &Keymap, path: &Path) -> f64 {
        let path_tokens = path.iter().map(|&loc| keymap[loc].ok_or(Unassigned));
        match freqs.get(path_tokens) {
            Ok(score) => score * path.weight,
            Err(Unassigned) => 0.0,
        }
    }
}

struct Unassigned;
