use std::vec::Vec;

pub type Corpus = Vec<NGrams>;

#[derive(Debug, Serialize, Deserialize)]
pub struct NGrams {
    pub ngram_length: usize,
    pub ngrams: Vec<NGram>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NGram {
    pub tokens: Vec<String>,
    pub freq: f64,
}
