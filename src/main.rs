use std::{env, process};

use minigrep::Config;

fn main() {
    let config: Config =
        Config::build(env::args().peekable()).unwrap_or_else(|error: &'static str| {
            eprintln!("Problem parsing arguments: {error}");
            process::exit(1);
        });

    if let Err(error) = minigrep::run(config) {
        eprintln!("Application error: {error}");
        process::exit(1);
    }
}
