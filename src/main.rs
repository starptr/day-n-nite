mod os;
mod dnn;
mod util;

use dnn::*;
use clap::{Parser, Subcommand};

use crate::util::get_pathstr;

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
    Add { filename: String },
    /// Remove a script or day-n-nite config
    Rm { filename: String },
}

fn main() {
    let cli = Cli::parse();

    let res = match &cli.command {
        None | Some(Commands::Toggle) => {
            let target_theme = get_theme().invert();
            set_theme(target_theme)
        }
        Some(Commands::Add{ filename }) => {
            Ok(())
        }
        Some(_) => {
            panic!("Command not implemented")
        }
    };
    println!("{:?}", get_pathstr("yeah".to_string()));
}
