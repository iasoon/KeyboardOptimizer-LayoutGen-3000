use std::vec::Vec;
use data::corpus::{Corpus, NGrams};
use model::{Language, FreqTable};
use errors::*;

use parser::{Parser, KbParser};

impl<'a> Parser<Language> for KbParser<'a> {
    type Repr = Corpus;

    fn parse(&self, repr: &Corpus) -> Result<Language> {
        LanguageReader::new(self).mk_language(repr)
    }
}

struct LanguageReader<'a> {
    parser: &'a KbParser<'a>,
}

impl<'a> LanguageReader<'a> {

    fn new(parser: &'a KbParser<'a>) -> Self {
        LanguageReader {
            parser: parser,
        }
    }

    fn mk_language(&self, corpus: &Corpus) -> Result<Language> {
        Ok(Language {
            alphabet: self.parser.kb_conf.tokens.clone(),
            freqs: self.mk_freqs(corpus)?,
        })
    }

    fn mk_freqs(&self, corpus: &Corpus) -> Result<Vec<FreqTable>> {
        let longest = corpus.iter().map(|ngrams| ngrams.ngram_length).max().unwrap_or(0);

        let mut freqs: Vec<FreqTable> = (0..longest + 1).map(|len| {
            FreqTable::empty(self.parser.kb_conf.tokens.len(), len)
        }).collect();

        for ngrams in corpus.iter() {
            try!(self.fill_freq_table(&mut freqs[ngrams.ngram_length], ngrams));
        }

        Ok(freqs)
    }

    fn fill_freq_table(&self, table: &mut FreqTable, ngrams: &NGrams) -> Result<()> {
        for ngram in ngrams.ngrams.iter() {
            try!(table.set_freq(
                ngram.tokens.iter().map(|t| self.parser.parse(t)),
                ngram.freq
            ));
        }
        Ok(())
    }
}
