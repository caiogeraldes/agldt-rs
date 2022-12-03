use agldt::parser::*;
use anyhow::Result;
use clap::{Parser, Subcommand};
use dotenv;
use log;
use pretty_env_logger;
use std::collections::HashMap;
use std::fs::{read_to_string, write};

#[derive(Parser, Debug)]
#[command(author = "Caio Geraldes <caio.geraldes@usp.br>", version, about, long_about = None)]
struct Args {
    /// AGLDT Treebank file
    #[arg(value_name = "TREEBANK")]
    treebank_file: String,

    /// Commands for running on the treebank
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Builds a lexicon for the treebank
    Lexicon {
        /// File where to save the treebank
        #[arg(short, long, default_value = "output.txt")]
        output: String,
        /// Lists forms and not lemmata
        #[arg(short, long, default_value = "false")]
        forms: bool,
        /// Counts entries
        #[arg(short, long, default_value = "false")]
        count: bool,
    },
    /// Describes treebank
    Describe,
}

fn build_lemmata_lexicon_count(treebank: &Treebank, output: &str) -> Result<()> {
    let mut tokens: Vec<String> = vec![];
    for sentence in treebank.sentences() {
        for word in sentence.words() {
            if let Some(form_string) = word.lemma() {
                if word.is_word() {
                    tokens.push(form_string.into())
                } else {
                    continue;
                }
            } else {
                continue;
            }
        }
    }

    let mut m: HashMap<String, usize> = HashMap::new();
    for x in tokens.iter() {
        *m.entry(x.clone()).or_default() += 1;
    }

    let mut hash_vec: Vec<(&String, &usize)> = m.iter().collect();
    hash_vec.sort_by(|a, b| b.1.cmp(a.1));

    let mut tokens: Vec<String> = vec![];
    for i in hash_vec {
        tokens.push(format!("\"{}\",{}", i.0, i.1));
    }
    let tokens_string = tokens.join("\n");

    write(output, tokens_string)?;

    Ok(())
}

fn build_form_lexicon(treebank: &Treebank, output: &str) -> Result<()> {
    let mut tokens: Vec<String> = vec![];
    for sentence in treebank.sentences() {
        for word in sentence.words() {
            let form_string = word.form().to_string();
            if !tokens.contains(&form_string) & word.is_word() {
                tokens.push(form_string)
            }
        }
    }
    tokens.sort();
    let tokens_string = tokens.join("\n");
    write(output, tokens_string)?;
    Ok(())
}

fn build_lemmata_lexicon(treebank: &Treebank, output: &str) -> Result<()> {
    let mut tokens: Vec<String> = vec![];
    for sentence in treebank.sentences() {
        for word in sentence.words() {
            if let Some(form_string) = word.lemma() {
                if !tokens.contains(&form_string.into()) & word.is_word() {
                    tokens.push(form_string.into())
                }
            } else {
                continue;
            }
        }
    }
    tokens.sort();
    let tokens_string = tokens.join("\n");
    write(output, tokens_string)?;
    Ok(())
}

fn print_info(treebank: &Treebank) -> Result<()> {
    println!("{}", treebank);
    Ok(())
}

fn main() -> anyhow::Result<()> {
    dotenv::dotenv().ok();
    pretty_env_logger::init();

    let args = Args::parse();

    let src: String;

    if let Ok(agldt_path) = dotenv::var("AGLDT_PATH") {
        let agldt_file = std::path::Path::new(&agldt_path).join(&args.treebank_file);
        src = match read_to_string(&agldt_file) {
            Ok(string) => {
                log::info!("Using path {:?}", agldt_file);
                string
            }
            Err(_) => {
                log::info!("Using path {}", &args.treebank_file);
                read_to_string(args.treebank_file)?
            }
        };
    } else {
        src = read_to_string(args.treebank_file)?;
    }

    let src = Treebank::from_str(&src)?;

    if let Some(cmd) = args.command {
        match cmd {
            Commands::Lexicon {
                output,
                forms,
                count,
            } => {
                if forms {
                    log::info!("Writing list of forms in {}", &output);
                    build_form_lexicon(&src, &output)?;
                } else if count {
                    log::info!("Writing list of lemmata with counts in {}", &output);
                    build_lemmata_lexicon_count(&src, &output)?;
                } else {
                    log::info!("Writing list of lemmata in {}", &output);
                    build_lemmata_lexicon(&src, &output)?;
                }
            }
            Commands::Describe => {
                print_info(&src)?;
            }
        }
    } else {
        print_info(&src)?;
    }
    log::info!("Done!");

    Ok(())
}
