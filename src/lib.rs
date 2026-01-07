use std::{error::Error, fs, iter::Peekable, process};

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {
    const IGNORE_CASE_FLAG: &'static str = "-i";
    const HELP_FLAGS: [&str; 2] = ["-h", "--help"];
    const HELP_INFORMATION: &'static str = "Usage: cargo run -- <query_string> <file_path> [OPTIONS]\n\n\
                 Options:\n\
                 \t-i\tIgnore case distinctions in both the query and the file contents.\n\
                 \t-h, --help\tDisplay this help information.";

    pub fn build(mut args: Peekable<impl Iterator<Item = String>>) -> Result<Config, &'static str> {
        args.next(); // Skip program name argument

        // Peek at the next argument, determine if it's a help flag before consuming it
        if let Some(flag) = args.peek() {
            let flag: &str = flag.as_str();
            if flag.starts_with('-') {
                if Self::HELP_FLAGS.contains(&flag) {
                    println!("{}", Self::HELP_INFORMATION);
                    process::exit(0);
                } else {
                    return Err("Invalid flag: expected '-h' or '--help'");
                }
            }
        }

        let query: String = match args.next() {
            Some(arg) => arg,
            None => return Err("Expected a query string"),
        };

        let file_path: String = match args.next() {
            Some(arg) => arg,
            None => return Err("Expected a file path"),
        };

        let ignore_case: bool = match args.next() {
            Some(flag) if flag.to_ascii_lowercase() == Self::IGNORE_CASE_FLAG => true,
            Some(_) => return Err("Invalid flag: expected '-i' [ignore case]"),
            None => false,
        };

        return Ok(Config {
            query,
            file_path,
            ignore_case,
        });
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents: String = fs::read_to_string(config.file_path)?;

    let results: Vec<&str> = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };

    for line in results {
        println!("{line}");
    }

    return Ok(());
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    return contents
        .lines()
        .filter(|line: &&str| line.contains(query))
        .collect();
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    return contents
        .lines()
        .filter(|line: &&str| line.to_lowercase().contains(&query.to_lowercase()))
        .collect();
}

// Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query: &str = "duct";
        let contents: &str = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query: &str = "rUsT";
        let contents: &str = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}
