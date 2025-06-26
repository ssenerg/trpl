use std::env;
use std::error::Error;
use std::fs;
use std::io::{Error as IError, ErrorKind};

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;
    let res = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else {
        search_case_sensitive(&config.query, &contents)
    };
    for line in res {
        println!("{line}")
    }
    Ok(())
}

pub struct Config {
    query: String,
    file_path: String,
    ignore_case: bool,
}

impl Config {
    pub fn load2() -> Result<Self, Box<dyn Error>> {
        let args: Vec<String> = env::args().collect();
        if args.len() != 3 {
            return Err(Box::new(IError::new(
                ErrorKind::InvalidInput,
                String::from("query and file path must be specified"),
            )));
        }
        let query = args[1].clone();
        let file_path = args[2].clone();
        let ignore_case = env::var("IGNORE_CASE").or_else(|error| match error {
            env::VarError::NotPresent => Ok(String::from("0")),
            error => Err(error),
        })?;
        let ignore_case = if &ignore_case == "1" { true } else { false };
        return Ok(Self {
            query,
            file_path,
            ignore_case,
        });
    }

    pub fn load(mut args: impl Iterator<Item = String>) -> Result<Self, Box<dyn Error>> {
        args.next();
        let query = match args.next() {
            Some(arg) => Ok(arg),
            None => Err(Box::new(IError::new(
                ErrorKind::InvalidInput,
                String::from("query and file path must be specified"),
            ))),
        }?;
        let file_path = match args.next() {
            Some(arg) => Ok(arg),
            None => Err(Box::new(IError::new(
                ErrorKind::InvalidInput,
                String::from("file path must be specified"),
            ))),
        }?;
        let ignore_case = env::var("IGNORE_CASE").or_else(|error| match error {
            env::VarError::NotPresent => Ok(String::from("0")),
            error => Err(error),
        })?;
        let ignore_case = if &ignore_case == "1" { true } else { false };
        return Ok(Self {
            query,
            file_path,
            ignore_case,
        });
    }

    pub fn query(&self) -> &str {
        return &self.query;
    }

    pub fn file_path(&self) -> &str {
        return &self.file_path;
    }
}

fn search_case_sensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }
    results
}

#[cfg(test)]
mod tests {
    use super::*;

    const CONTENTS: &str = "\
Rust:
safe, fast, productive.
Pick three.
--- HELPERS
St
";

    #[test]
    fn sensitive_zero_result() {
        let query = "ST";
        let res = search_case_sensitive(query, CONTENTS);
        assert_eq!(Vec::<&str>::new(), res);
    }

    #[test]
    fn sensitive_one_result() {
        let query = "ductive";
        assert_eq!(
            vec!["safe, fast, productive."],
            search_case_sensitive(query, CONTENTS)
        );
    }

    #[test]
    fn sensitive_multiple_results() {
        let query = "st";
        assert_eq!(
            vec!["Rust:", "safe, fast, productive."],
            search_case_sensitive(query, CONTENTS)
        );
    }

    #[test]
    fn insensitive_zero_result() {
        let query = "##@";
        let res = search_case_insensitive(query, CONTENTS);
        assert_eq!(Vec::<&str>::new(), res);
    }

    #[test]
    fn insensitive_one_result() {
        let query = "PROductive";
        let res = search_case_insensitive(query, CONTENTS);
        assert_eq!(vec!["safe, fast, productive."], res);
    }

    #[test]
    fn insensitive_multiple_results() {
        let query = "ST";
        let res = search_case_insensitive(query, CONTENTS);
        assert_eq!(vec!["Rust:", "safe, fast, productive.", "St"], res);
    }
}
