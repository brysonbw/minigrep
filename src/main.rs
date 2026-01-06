use std::{env, process};

use minigrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config: Config = Config::build(&args).unwrap_or_else(|error: &'static str| {
        eprintln!("Problem parsing arguments: {error}");
        process::exit(1)
    });

    if let Err(error) = minigrep::run(config) {
        eprintln!("Application error: {error}");
        process::exit(1);
    }
}
