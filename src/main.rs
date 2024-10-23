mod zsh;

use core::panic;
use std::env;
use std::fs::File;
use std::path::Path;

// TODO: Read a file, print it's contents

fn main() {
    // creates a file_path
    let env_path = env::var("HISTFILE").unwrap_or(format!(
        "{}/.zsh_history",
        env::var("HOME").expect("HOME var is not set")
    ));
    let file_path = Path::new(&env_path);

    // Open file connector
    let file = match File::open(&file_path) {
        Err(err) => panic!("Couldn't open {}: {}", file_path.display(), err),
        Ok(file) => file,
    };
}
