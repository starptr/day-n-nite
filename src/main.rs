mod os;
mod dnn;
mod util;

use dnn::*;
use clap::{Parser, Subcommand, ArgEnum};

const DNN_FILE_EXTENSION: &str = "day-n-nite";
const _DNN_FILE_EXTENSION_PAIR_LIGHT: &str = "day-n-nite-light";
const _DNN_FILE_EXTENSION_PAIR_DARK: &str = "day-n-nite-dark";

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    #[clap(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Toggle day-n-nite theme scheme
    Toggle,
    /// Install default configs
    Install,
    /// Add or update a script or day-n-nite config and register its hash
    Add {
        #[clap(arg_enum)]
        kind: DnnEntryKind,

        filename: String
    },
    /// Remove a script or day-n-nite config
    Rm { filename: String },
}

fn main() {
    let cli = Cli::parse();

    let res = match cli.command {
        None | Some(Commands::Toggle) => cmd_toggle(),
        Some(Commands::Add{ kind, filename }) => cmd_add(filename, kind),
        Some(Commands::Rm{ filename }) => cmd_rm(filename),
        Some(Commands::Install) => panic!("Not implemented"),
    };
    if res.is_err() {
        eprintln!("{}", res.unwrap_err());
    }
}
