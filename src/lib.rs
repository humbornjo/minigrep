use std::error::Error;
use std::fs;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    for line in search(&config.query, &contents) {
        println!("{line}");
    }
    Ok(())
}

pub struct Config {
    pub query: String,
    pub file_path: String,
}

impl Config {
    pub fn from(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        let query = args[1].clone();
        let file_path = args[2].clone();
        Ok(Config { query, file_path })
    }
}

// in lib.rs
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut ret = Vec::new();
    for line in contents.lines() {
        if line.contains(query) {
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
}
