use agldt::parser::*;
use anyhow::Result;
use std::collections::HashMap;
use std::fs::{read_to_string, write};
use unicode_normalization::is_nfkc;

pub(crate) fn build_lexicon_lemmata(treebank: &Treebank, output: &str, count: bool) -> Result<()> {
    let mut tokens: Vec<String> = vec![];
    let tokens_string: String;

    for sentence in treebank.sentences() {
        for word in sentence.words() {
            if let Some(form_string) = word.lemma() {
                if word.is_word() {
                    if form_string.is_empty() {
                        tokens.push(format!("UNKNOWN: {}", word.form()));
                    } else {
                        tokens.push(form_string.into());
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
        let mut m: HashMap<String, usize> = HashMap::new();
        for x in tokens.iter() {
            *m.entry(x.trim().to_string()).or_default() += 1;
        }

        let mut hash_vec: Vec<(&String, &usize)> = m.iter().collect();
        hash_vec.sort_by(|a, b| b.1.cmp(a.1));

        let mut tokens: Vec<String> = vec![];
        for i in hash_vec {
            tokens.push(format!("\"{}\",{}", i.0, i.1));
        }
        tokens_string = tokens.join("\n");
    } else {
        log::info!("Writing list of lemmata in {}", &output);
        for token in tokens.iter_mut() {
            if token.starts_with(" ") {
                return Err(anyhow::anyhow!("{}", token));
            };
            *token = token.trim().to_string();
        }
        tokens.sort();
        tokens.dedup();
        tokens_string = tokens.join("\n");
    }

    write(output, tokens_string)?;
    Ok(())
}

pub(crate) fn build_lexicon_forms(treebank: &Treebank, output: &str, count: bool) -> Result<()> {
    let mut tokens: Vec<String> = vec![];
    let tokens_string: String;
    for sentence in treebank.sentences() {
        for word in sentence.words() {
            let form_string = word.form().to_string();
            if word.is_word() {
                tokens.push(form_string)
            }
        }
    }

    if count {
        log::info!("Writing list of tokens with counts in {}", &output);
        let mut m: HashMap<String, usize> = HashMap::new();
        for x in tokens.iter() {
            *m.entry(x.trim().to_string()).or_default() += 1;
        }

        let mut hash_vec: Vec<(&String, &usize)> = m.iter().collect();
        hash_vec.sort_by(|a, b| b.1.cmp(a.1));

        let mut tokens: Vec<String> = vec![];
        for i in hash_vec {
            tokens.push(format!("\"{}\",{}", i.0, i.1));
        }
        tokens_string = tokens.join("\n");
    } else {
        log::info!("Writing list of tokens in {}", &output);
        for token in tokens.iter_mut() {
            *token = token.trim().to_string();
        }
        tokens.sort();
        tokens.dedup();
        tokens_string = tokens.join("\n");
    }
    write(output, tokens_string)?;
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
    let tokens_string: String;
    for sentence in treebank.sentences() {
        for word in sentence.words() {
            let form_string = word.form().to_string();
            if word.is_word() {
                tokens.push(form_string)
            }
        }
    }

    tokens.dedup();

    let mut m: HashMap<bool, usize> = HashMap::new();
    for x in tokens.iter() {
        *m.entry(is_nfkc(x)).or_default() += 1;
    }

    let mut hash_vec: Vec<(&bool, &usize)> = m.iter().collect();
    hash_vec.sort_by(|a, b| b.1.cmp(a.1));

    let mut tokens: Vec<String> = vec![];
    for i in hash_vec {
        tokens.push(format!("\"{}\",{}", i.0, i.1));
    }
    tokens_string = tokens.join("\n");

    write("report.txt", tokens_string)?;

    Ok(())
}
