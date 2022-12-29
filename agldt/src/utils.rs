use crate::parser::{Body, Sentence, Token, Treebank};
use icu::collator::*;
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
            .filter(|t| t.is_word())
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
