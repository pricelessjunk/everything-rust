/*
Code extracted to lib from main file to reduce main file size
*/

use std::error::Error;
use std::fs;

pub struct LibConfig {
  pub query: String,
  pub file_path: String,
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

    let query = args[1].clone();
    let file_path = args[2].clone();

    Ok(LibConfig { query, file_path })
  }
}

// Called in main
pub fn run(config: LibConfig) -> Result<(), Box<dyn Error>> {
  // ? here also replaces the expect. Basically it sends back the error if it gets it instead of panicking.
  let contents = fs::read_to_string(config.file_path)?;
    println!("With text:\n{contents}");
    Ok(())  // Nothing to return, so empty return
}

// 'a is the lifetime specifier. Without it, this code will not compile
pub fn search<'a>(query:&'a str, contents: &'a str) -> Vec<&'a str> {
  let mut results = Vec::new();

  for line in contents.lines() {
      if line.contains(query) {
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
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }
}