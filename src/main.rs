mod lib;
mod zsh;

use clap::{Args, Parser, Subcommand};
use core::panic;
use lib::top;
use std::alloc::System;
use std::collections::HashMap;
use std::fs::File;
use std::path::{Path, PathBuf};
use std::{env, usize};

// TODO: Refactor test code to test.rs
// TODO: Parse cli arguments
// TODO: Display results

#[derive(Parser, Debug)]
#[command(version,about,long_about=None)]
struct Cli {
    /// Use file instead of HISTFILE or $HOME/.zsh_histfile
    #[arg(long, value_name = "FILE")]
    file: Option<PathBuf>,

    /// Toggles minimal graph
    #[arg(short, long)]
    min: bool,

    /// Counts sudo as a seperate command
    #[arg(short, long)]
    sudo: bool,

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

    // Specs:
    // --file PATH read from file
    // -m create minimal graph
    // -s don't count sudo as seperate command
    // -r recursively evaluate expressions (so eval $(ssh-agent) would return both eval and ssh-agent)
    // top N returns the N top used results
    // bot N returns the N least used results
    // Needs more options for Filter, and sorting

    // creates a file_path
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
                std::process::exit(1);
            }
            if !cli.sudo {
                let map: HashMap<String, usize> = zsh::gen_hash_map(file);
                if !cli.min {
                    lib::print_result(top(map, n));
                }
            }
        }
        Commands::Bot(arg) => {
            let n = arg.num.unwrap_or_else(|| MIN);
            if n < 1 {
                println!("N must be bigger than 0");
                std::process::exit(1);
            }
            if !cli.sudo {
                let map: HashMap<String, usize> = zsh::gen_hash_map(file);
                if !cli.min {
                    lib::print_result(top(map, n));
                }
            }
        }
        _ => {}
    }
}
