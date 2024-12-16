mod outputs;
mod test;
mod zsh;

use clap::{Args, Parser, Subcommand};
use core::panic;
use outputs::{bot, get, print, rand, top};
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

    /// Prints the sum of the values from a query
    #[arg(long, short)]
    length: bool,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Shows the N most used commands
    Top(Int),
    /// Shows the N least used commands
    Bot(Int),
    /// Shows N random commands
    Rand(Int),
    /// Gets the stats for PROG
    Get(Name),
    /// Prints all PROG names
    All,
}

#[derive(Args, Debug)]
struct Int {
    num: Option<usize>,
}
#[derive(Args, Debug)]
struct Name {
    name: Option<String>,
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

    match &cli.command {
        Commands::Top(arg) => {
            let n = arg.num.unwrap_or_else(|| MIN);
            if n < 1 {
                println!("N must be bigger than 0");
                exit(1);
            }
            let map: HashMap<String, usize> = zsh::map(file);
            let values = top(map, n);
            if cli.length {
                println!(
                    "Length of query:{}",
                    values.iter().fold(0, |mut sum, val| {
                        sum += val.1;
                        sum
                    })
                );
            }

            outputs::print(values)
        }
        Commands::Bot(arg) => {
            let n = arg.num.unwrap_or_else(|| MIN);
            if n < 1 {
                println!("N must be bigger than 0");
                exit(1);
            }
            let map: HashMap<String, usize> = zsh::map(file);
            let values = bot(map, n);
            if cli.length {
                println!(
                    "Length of query:{}",
                    values.iter().fold(0, |mut sum, val| {
                        sum += val.1;
                        sum
                    })
                );
            }
            outputs::print(values)
        }
        Commands::Rand(arg) => {
            let n = arg.num.unwrap_or_else(|| MIN);
            if n < 1 {
                println!("N must be bigger than 0");
                exit(1);
            }
            let map = zsh::map(file);
            let values = rand(map, n);
            if cli.length {
                println!(
                    "Length of query:{}",
                    values.iter().fold(0, |mut sum, val| {
                        sum += val.1;
                        sum
                    })
                );
            }
            outputs::print(values)
        }
        Commands::Get(arg) => {
            let n = arg.name.clone().unwrap_or_else(|| {
                eprintln!("Failed to retrieve blank PROG name");
                exit(1)
            });
            let map = zsh::map(file);
            let values = get(map, n);
            if cli.length {
                println!("Length of query:{}", values.1);
            }
            println!("{}:{}", values.0, values.1)
        }
        Commands::All => {
            let map = zsh::map(file);
            let n = map.len();
            let values = top(map, n);
            if cli.length {
                println!(
                    "Length of query:{}",
                    values.iter().fold(0, |mut sum, val| {
                        sum += val.1;
                        sum
                    })
                );
            }
            outputs::print(values);
        }
    }
}
