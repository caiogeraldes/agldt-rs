use crate::parser::{Body, Sentence, Token, Treebank};
use icu::collator::{Collator, CollatorOptions, Strength};
use icu::locid::{locale, Locale};

pub trait IterTokens {
    fn iter_tokens(&self) -> std::vec::IntoIter<Token>;
}

impl IterTokens for Treebank {
    fn iter_tokens(&self) -> std::vec::IntoIter<Token> {
        self.body().iter_tokens()
    }
}

impl IterTokens for Body {
    fn iter_tokens(&self) -> std::vec::IntoIter<Token> {
        self.clone()
            .into_iter()
            .flatten()
            .collect::<Vec<Token>>()
            .into_iter()
    }
}

impl IterTokens for Sentence {
    fn iter_tokens(&self) -> std::vec::IntoIter<Token> {
        self.clone().into_iter()
    }
}

pub trait Lexicon: IterTokens {
    fn get_lemmata(&self) -> Vec<String> {
        self.iter_tokens()
            .filter(Token::is_word)
            .filter_map(|t| t.lemma())
            .collect::<Vec<String>>()
    }
    fn build_lexicon(&self) -> Vec<String> {
        let mut lexicon = self.get_lemmata();
        let locale_el: Locale = locale!("el");
        let mut options = CollatorOptions::new();
        options.strength = Some(Strength::Primary);
        let collator_el: Collator =
            Collator::try_new_unstable(&icu_testdata::unstable(), &locale_el.into(), options)
                .unwrap();

        lexicon.sort();
        lexicon.dedup();
        lexicon.sort_by(|a, b| collator_el.compare(a, b));
        lexicon
    }
}

impl Lexicon for Sentence {}
impl Lexicon for Body {}
impl Lexicon for Treebank {}

#[derive(Debug)]
pub struct ConcordanceEntry {
    lemma: String,
    sent_ids: Vec<u32>,
}

impl ConcordanceEntry {
    #[must_use]
    ///
    /// # Panics
    /// If `sent_ids.sort_unstable()` fails.
    pub fn merge(&self, other: &ConcordanceEntry) -> ConcordanceEntry {
        assert_eq!(self.lemma, other.lemma);
        let lemma = self.lemma.clone();
        let mut sent_ids = self.sent_ids.clone();
        let mut other_sids = other.sent_ids.clone();
        sent_ids.append(&mut other_sids);
        sent_ids.sort_unstable();
        sent_ids.dedup();

        ConcordanceEntry { lemma, sent_ids }
    }

    #[must_use]
    pub fn lemma(&self) -> &str {
        self.lemma.as_ref()
    }
}

#[must_use]
pub fn s_ce(sentence: &Sentence) -> Vec<ConcordanceEntry> {
    let sent_ids = vec![sentence.id];
    let lemmata = sentence.build_lexicon();

    lemmata
        .iter()
        .map(|l| ConcordanceEntry {
            lemma: l.clone(),
            sent_ids: sent_ids.clone(),
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn conc() {
        use crate::parser::*;
        use serde_xml_rs::from_str;
        use std::fs::read_to_string;
        let src = read_to_string("./tests/tlg0007.tlg004.perseus-grc1.tb.xml").unwrap();
        let se = from_str::<Treebank>(&preprocess(&src)).unwrap().sentences()[0].clone();
        dbg!(s_ce(&se));
    }
}
