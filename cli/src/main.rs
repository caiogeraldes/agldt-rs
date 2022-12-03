use agldt::parser::*;
use anyhow::Result;
use clap::*;
use dotenv;
use log;
use pretty_env_logger;
use std::collections::HashMap;
use std::fs::{read_to_string, write};

const HELP_TEMPLATE: &'static str = "\
{before-help}{name} {version}
{about-section}{author-section}
{usage-heading} {usage}

{all-args}{after-help}
";

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None, help_template = HELP_TEMPLATE)]
struct Cli {
    /// AGLDT Treebank file
    #[clap(value_name = "TREEBANK")]
    treebank_file: String,

    /// Commands for running on the treebank
    #[clap(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Builds a lexicon for the treebank
    Lexicon {
        /// File where to save the treebank
        #[clap(short, long, default_value = "output.txt")]
        output: String,
        /// Lists forms and not lemmata
        #[clap(short, long, action)]
        forms: bool,
        /// Counts entries
        #[clap(short, long, action)]
        count: bool,
    },
    /// Describes treebank
    Describe,
}

fn build_lexicon_lemmata(treebank: &Treebank, output: &str, count: bool) -> Result<()> {
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
            *token = token.trim().to_string();
        }
        tokens.sort();
        tokens.dedup();
        tokens_string = tokens.join("\n");
    }

    write(output, tokens_string)?;
    Ok(())
}

fn build_lexicon_forms(treebank: &Treebank, output: &str, count: bool) -> Result<()> {
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

fn print_info(treebank: &Treebank) -> Result<()> {
    println!("{}", treebank);
    Ok(())
}

fn main() -> anyhow::Result<()> {
    dotenv::dotenv().ok();
    pretty_env_logger::init();

    let args = Cli::parse();
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
                    build_lexicon_forms(&src, &output, count)?;
                } else {
                    build_lexicon_lemmata(&src, &output, count)?;
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
