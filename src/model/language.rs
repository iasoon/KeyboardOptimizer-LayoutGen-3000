use model::TokenId;
use utils::SeqTable;

pub type FreqTable = SeqTable<TokenId, f64>;

pub struct Language {
    pub freqs: Vec<FreqTable>,
}
