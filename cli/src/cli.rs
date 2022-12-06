use crate::tools::*;
use anyhow::Result;
use clap::*;

const HELP_TEMPLATE: &'static str = "\
{before-help}{name} {version}
{about-section}{author-section}
{usage-heading} {usage}

{all-args}{after-help}
";
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None, help_template = HELP_TEMPLATE)]
pub(crate) struct Cli {
    /// Commands for running on the treebank
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Builds a lexicon for the treebank
    Lexicon {
        /// AGLDT Treebank file
        #[clap(value_name = "TREEBANK")]
        treebank_file: String,
        /// File where to save the treebank
        #[clap(short, long, default_value = "OUT")]
        output: String,
        /// Lists forms and not lemmata
        #[clap(short, long, action)]
        forms: bool,
        /// Counts entries
        #[clap(short, long, action)]
        count: bool,
    },
    /// Describes treebank
    Describe {
        /// AGLDT Treebank file
        #[clap(value_name = "TREEBANK")]
        treebank_file: String,
    },
    /// Checks unicode normalization
    UniCheck {
        /// AGLDT Treebank file
        #[clap(value_name = "TREEBANK(S)")]
        treebank_files: Vec<String>,
    },
}

pub(crate) fn run_command(cli: Cli) -> Result<()> {
    match cli.command {
        Commands::Describe { treebank_file } => {
            let src = pick_treebank_file(&treebank_file)?;
            print_info(&src)?;
        }
        Commands::Lexicon {
            treebank_file,
            output,
            forms,
            count,
        } => {
            let src = pick_treebank_file(&treebank_file)?;
            let output_file: String;
            if &output != "OUT" {
                output_file = output;
            } else {
                output_file = format!("lexicon-{}", &treebank_file.replace(".xml", ".csv"));
            }
            if forms {
                build_lexicon_forms(&src, &output_file, count)?;
            } else {
                build_lexicon_lemmata(&src, &output_file, count)?;
            }
        }
        Commands::UniCheck { treebank_files } => {
            for treebank_file in treebank_files {
                let src = pick_treebank_file(&treebank_file)?;
                check_unicode(&src)?;
            }
        }
        #[allow(unreachable_patterns)]
        _ => unimplemented!(),
    }
    Ok(())
}
