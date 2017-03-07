use std::ops::Index;

#[derive(Debug, Clone, Copy)]
pub struct TokenId(pub usize);

#[derive(Debug)]
pub struct Language {
    pub alphabet: Vec<String>,
    pub freqs: Vec<FreqTable>
}

impl Index<TokenId> for Language {
    type Output = str;

    fn index<'a>(&'a self, token_id: TokenId) -> &'a str {
        let TokenId(token_num) = token_id;
        return self.alphabet[token_num].as_str();
    }
}

#[derive(Debug)]
pub struct FreqTable {
    seq_len: usize,
    num_tokens: usize,
    elems: Vec<f64>,
}

impl FreqTable {
    pub fn empty(num_tokens: usize, seq_len: usize) -> Self {
        FreqTable {
            seq_len: seq_len,
            num_tokens: num_tokens,
            elems: vec![0.0; num_tokens.pow(seq_len as u32)],
        }
    }

    fn find_index<Iter, E>(&self, iter: Iter) -> Result<usize, E>
        where Iter: Iterator<Item = Result<TokenId, E>>
    {
        reduce_results(iter, 0, |acc, TokenId(token_num)| {
            acc * self.num_tokens + token_num
        })
    }

    pub fn freq<Iter, E>(&self, iter: Iter) -> Result<f64, E>
        where Iter: Iterator<Item = Result<TokenId, E>>
    {
        self.find_index(iter).map(|idx| self.elems[idx])

    }

    pub fn set_freq<Iter, E>(&mut self, iter: Iter, freq: f64) -> Result<(), E>
        where Iter: Iterator<Item = Result<TokenId, E>>
    {
        self.find_index(iter).map(|idx| self.elems[idx] = freq )
    }
}

fn reduce_results<A, B, E, Iter, F>(iter: Iter, init: B, fun: F) -> Result<B, E>
    where Iter: Iterator<Item = Result<A, E>>,
          F: Fn(B, A) -> B
{
    let mut acc = init;
    for item in iter {
        match item {
            Ok(a) => acc = fun(acc, a),
            Err(err) => return Err(err),
        }
    }
    return Ok(acc);
}
