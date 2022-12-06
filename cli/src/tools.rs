use agldt::parser::*;
use anyhow::Result;
use icu::collator::*;
use icu::locid::{locale, Locale};
use std::collections::HashMap;
use std::fs::{read_to_string, write};
use unicode_normalization::{is_nfkc, UnicodeNormalization};

fn normalize_unicode<T: Into<String>>(input: T) -> String {
    let input: &str = &input.into();
    input.nfkc().collect::<String>()
}

fn order_greek(tokens: &mut Vec<String>) {
    const LOCALE: Locale = locale!("el");
    let mut options = CollatorOptions::new();
    options.strength = Some(Strength::Primary);
    let collator =
        Collator::try_new_unstable(&icu_testdata::unstable(), &LOCALE.into(), options).unwrap();
    tokens.sort_by(|a, b| collator.compare(a, b));
}

pub(crate) fn build_lexicon_lemmata(treebank: &Treebank, output: &str, count: bool) -> Result<()> {
    let mut tokens: Vec<String> = vec![];
    let lexicon_string: String;

    for sentence in treebank.sentences() {
        for word in sentence.words() {
            if let Some(form_string) = word.lemma() {
                if word.is_word() {
                    if form_string.is_empty() {
                        tokens.push(format!("UNKNOWN: {}", normalize_unicode(word.form())));
                    } else {
                        tokens.push(normalize_unicode(form_string));
                    }
                } else {
                    continue;
                }
            } else {
                continue;
            }
        }
    }

    if count {
        log::info!("Writing list of lemmata with counts in {}", &output);
        let mut lemma_count: HashMap<String, usize> = HashMap::new();
        for token in tokens.iter() {
            *lemma_count.entry(token.trim().to_string()).or_default() += 1;
        }

        let mut hash_vec: Vec<(&String, &usize)> = lemma_count.iter().collect();

        hash_vec.sort_by(|a, b| b.1.cmp(a.1));

        let mut tokens: Vec<String> = vec![];
        for i in hash_vec {
            tokens.push(format!("\"{}\",{}", normalize_unicode(i.0), i.1));
        }
        lexicon_string = tokens.join("\n");
    } else {
        log::info!("Writing list of lemmata in {}", &output);
        for token in tokens.iter_mut() {
            if token.starts_with(" ") {
                return Err(anyhow::anyhow!("{}", token));
            };
            *token = token.trim().to_string();
        }

        order_greek(&mut tokens);
        tokens.dedup();
        lexicon_string = tokens.join("\n");
    }

    write(output, lexicon_string)?;
    Ok(())
}

pub(crate) fn build_lexicon_forms(treebank: &Treebank, output: &str, count: bool) -> Result<()> {
    let mut tokens: Vec<String> = vec![];
    let lexicon_string: String;
    for sentence in treebank.sentences() {
        for word in sentence.words() {
            let form_string = word.form().to_string();
            if word.is_word() {
                tokens.push(normalize_unicode(form_string))
            }
        }
    }

    if count {
        log::info!("Writing list of tokens with counts in {}", &output);
        let mut forms_count: HashMap<String, usize> = HashMap::new();
        for token in tokens.iter() {
            *forms_count.entry(token.trim().to_string()).or_default() += 1;
        }

        let mut hash_vec: Vec<(&String, &usize)> = forms_count.iter().collect();
        hash_vec.sort_by(|a, b| b.1.cmp(a.1));

        let mut tokens: Vec<String> = vec![];
        for i in hash_vec {
            tokens.push(format!("\"{}\",{}", normalize_unicode(i.0), i.1));
        }
        lexicon_string = tokens.join("\n");
    } else {
        log::info!("Writing list of tokens in {}", &output);
        for token in tokens.iter_mut() {
            *token = token.trim().to_string();
        }

        order_greek(&mut tokens);
        tokens.dedup();
        lexicon_string = tokens.join("\n");
    }
    write(output, lexicon_string)?;
    Ok(())
}

pub(crate) fn print_info(treebank: &Treebank) -> Result<()> {
    println!("{}", treebank);
    Ok(())
}

pub(crate) fn pick_treebank_file(treebank_file: &String) -> Result<Treebank> {
    let src: String;
    if let Ok(agldt_path) = dotenv::var("AGLDT_PATH") {
        let agldt_file = std::path::Path::new(&agldt_path).join(treebank_file);
        src = match read_to_string(&agldt_file) {
            Ok(string) => {
                log::info!("Using path {:?}", agldt_file);
                string
            }
            Err(_) => {
                log::info!("Using path {}", treebank_file);
                read_to_string(treebank_file)?
            }
        };
    } else {
        src = read_to_string(treebank_file)?;
    }

    Ok(Treebank::from_str(&src)?)
}

pub(crate) fn check_unicode(treebank: &Treebank) -> Result<()> {
    let mut tokens: Vec<String> = vec![];
    let mut report_string: String;
    for sentence in treebank.sentences() {
        for word in sentence.words() {
            let form_string = word.form().to_string();
            if word.is_word() {
                tokens.push(form_string)
            }
        }
    }

    tokens.dedup();

    let mut nfkc: HashMap<bool, usize> = HashMap::new();
    let mut non_nfkc: Vec<String> = vec![];

    for x in tokens.iter() {
        *nfkc.entry(is_nfkc(x)).or_default() += 1;
        if !is_nfkc(x) && !non_nfkc.contains(x) {
            non_nfkc.push(x.to_string());
        }
    }

    let mut hash_vec: Vec<(&bool, &usize)> = nfkc.iter().collect();
    hash_vec.sort_by(|a, b| b.1.cmp(a.1));

    let mut tokens: Vec<String> = vec![];

    tokens.push(treebank.to_string());
    tokens.push("NFKC frequency (tokens):".to_string());

    for i in hash_vec {
        tokens.push(format!("\"{}\",{}", i.0, i.1));
    }
    report_string = tokens.join("\n");
    report_string.push_str(&format!("\n\nNon-NFKC unique tokens: {:?}\n", non_nfkc));

    write("report.txt", report_string)?;

    Ok(())
}
