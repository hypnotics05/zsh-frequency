mod outputs;
mod test;
mod zsh;

use clap::{Args, Parser, Subcommand};
use core::panic;
use outputs::{bot, top};
use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::path::{Path, PathBuf};
use std::process::exit;

#[derive(Parser, Debug)]
#[command(version,about,long_about=None)]
struct Cli {
    /// Use file instead of HISTFILE or $HOME/.zsh_histfile
    #[arg(long, value_name = "FILE")]
    file: Option<PathBuf>,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Shows the N most used commands
    Top(AddArgs),
    /// Shows the N least used commands
    Bot(AddArgs),
}

#[derive(Args, Debug)]
struct AddArgs {
    num: Option<usize>,
}

fn main() {
    let file: File;
    const MIN: usize = 5;

    let cli = Cli::parse();

    let env_path = env::var("HISTFILE").unwrap_or(format!(
        "{}/.zsh_history",
        env::var("HOME").expect("HOME var is not set")
    ));
    let file_path = Path::new(&env_path);

    let final_path = cli.file.unwrap_or(PathBuf::from(&file_path));
    file = match File::open(&final_path) {
        Err(err) => panic!("Couldn't open {}: {}", final_path.display(), err),
        Ok(file) => file,
    };

    println!("{:?}", final_path.as_os_str());
    match &cli.command {
        Commands::Top(arg) => {
            let n = arg.num.unwrap_or_else(|| MIN);
            if n < 1 {
                println!("N must be bigger than 0");
                exit(1);
            }
            let map: HashMap<String, usize> = zsh::map(file);
            top(map, n).iter().for_each(|i| println!("{}:{}", i.0, i.1))
        }
        Commands::Bot(arg) => {
            let n = arg.num.unwrap_or_else(|| MIN);
            if n < 1 {
                println!("N must be bigger than 0");
                exit(1);
            }
            let map: HashMap<String, usize> = zsh::map(file);
            bot(map, n).iter().for_each(|i| println!("{}:{}", i.0, i.1))
        }
    }
}
