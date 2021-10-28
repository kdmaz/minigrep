use std::{error::Error, fs};

#[derive(PartialEq, Debug)]
pub struct Config {
    pub query: String,
    pub filename: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let filename = args[2].clone();

        Ok(Config { query, filename })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    for line in search(&config.query, &contents) {
        println!("{}", line);
    }

    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }
    results
}

#[cfg(test)]
mod new {
    use super::*;

    #[test]
    fn not_enough_args() {
        let args = vec![String::new(), String::from("the")];
        assert_eq!(Err("not enough arguments"), Config::new(&args));
    }

    #[test]
    fn creates_config() {
        let config = Config {
            query: String::from("the"),
            filename: String::from("poem.txt"),
        };
        let args = vec![String::new(), String::from("the"), String::from("poem.txt")];
        assert_eq!(Ok(config), Config::new(&args));
    }
}

#[cfg(test)]
mod search {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }
}
