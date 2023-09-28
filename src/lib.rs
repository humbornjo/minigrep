use std::error::Error;
use std::{env, fs, process};

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    let res = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };

    for line in res {
        println!("{line}");
    }
    Ok(())
}

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {
    pub fn from(args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        let mut ignore_case = env::var("IGNORE_CASE").map_or(false, |x| x == "1");

        let mut args = args.peekable();
        args.next();

        while let Some(s) = args.next_if(|x: &String| x.starts_with("-")) {
            match s.as_str() {
                "-i" | "--ignore_case" => ignore_case = true,
                _ => {
                    eprintln!("invalid parameter");
                    process::exit(1);
                }
            }
        }

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("no parameter query"),
        };

        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err("no parameter file_path"),
        };

        Ok(Config {
            query,
            file_path,
            ignore_case,
        })
    }
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut ret = Vec::new();
    for line in contents.lines() {
        if line.contains(query) {
            ret.push(line);
        }
    }
    ret
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut ret = Vec::new();
    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            ret.push(line);
        }
    }
    ret
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let mut contents = String::new();
        contents.push_str("Rust:\n");
        contents.push_str("safe, fast, productive.\n");
        contents.push_str("Pick three.");
        assert_eq!(
            vec!["safe, fast, productive."],
            search(query, contents.as_str())
        );
    }

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
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
