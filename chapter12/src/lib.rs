/*
Code extracted to lib from main file to reduce main file size

cargo test --lib chapter12
*/

use std::error::Error;
use std::fs;

pub struct LibConfig {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl LibConfig {
    pub fn build(args: &[String]) -> Result<LibConfig, &'static str> {
        // Error handling
        if args.len() < 3 {
            // One alternative is to panic as follows
            // panic!("not enough arguments");
            // Another is to return an error. This changes the return type for `LibConfig` to `Result<LibConfig, &'static str>`
            return Err("Not enough arguments");
        }

        let file_path = args[1].clone();
        let query = args[2].clone();

        Ok(LibConfig {
            query,
            file_path,
            ignore_case: false,
        })
    }
}

// Called in main
pub fn run(config: LibConfig) -> Result<(), Box<dyn Error>> {
    println!("---- In lib run ----");
    // ? here also replaces the expect. Basically it sends back the error if it gets it instead of panicking.
    let contents = fs::read_to_string(config.file_path)?;
    // let contents = fs::read_to_string(config.file_path).expect("Should have read the value, but error");
    let res;
    if config.ignore_case {
        res = search_case_insensitive(config.query.as_str(), contents.as_str());
    } else {
        res = search(config.query.as_str(), contents.as_str())
    }

    println!("{:?}", res);
    println!("---- DONE ----");

    Ok(())
}

// 'a is the lifetime specifier. Without it, this code will not compile. It signifies that the lifetime
// of the return variable is the same as the contents or query variable.
pub fn search<'a>(query: &'a str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    results
}

pub fn search_case_insensitive<'a>(query: &'a str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(query) {
            results.push(line);
        }
    }

    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let query2 = "fast";
        let contents = "\
Rust:
safe, Fast, productive.
Pick three.";

        assert_eq!(vec!["safe, Fast, productive."], search(query, contents));
        assert_eq!(
            vec!["safe, Fast, productive."],
            search_case_insensitive(query2, contents)
        );
    }
}
