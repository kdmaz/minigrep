use std::{env, error::Error, fs};

#[derive(PartialEq, Debug)]
pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let filename = args[2].clone();

        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        Ok(Config {
            query,
            filename,
            case_sensitive,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    let results = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };

    for line in results {
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

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&*query.to_lowercase()) {
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
    fn creates_config_case_sensitive() {
        let config = Config {
            query: String::from("the"),
            filename: String::from("poem.txt"),
            case_sensitive: true,
        };
        let args = vec![String::new(), String::from("the"), String::from("poem.txt")];
        assert_eq!(Ok(config), Config::new(&args));
    }

    #[test]
    fn creates_config_case_insensitive() {
        let config = Config {
            query: String::from("the"),
            filename: String::from("poem.txt"),
            case_sensitive: false,
        };
        env::set_var("CASE_INSENSITIVE", "1");
        let args = vec![String::new(), String::from("the"), String::from("poem.txt")];
        assert_eq!(Ok(config), Config::new(&args));
        env::remove_var("CASE_INSENSITIVE");
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

#[cfg(test)]
mod casing {
    use super::*;

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
