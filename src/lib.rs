use std::{error::Error, fs, process};

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

    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        // Check for help flags
        if args.len() > 1 {
            let arg: &str = args[1].as_str();

            if arg.starts_with('-') {
                if Self::HELP_FLAGS.contains(&arg) {
                    println!("{}", Self::HELP_INFORMATION);
                    process::exit(0);
                } else {
                    return Err("Invalid flag: expected '-h' or '--help'");
                }
            }
        }

        if args.len() < 3 {
            return Err("Not enough arguments");
        }

        let query: String = args[1].clone();
        let file_path: String = args[2].clone();
        let ignore_case: bool = match args.get(3) {
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
    let mut results: Vec<&str> = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    return results;
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query: String = query.to_lowercase();
    let mut results: Vec<&str> = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }

    return results;
}

// Test
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
