/**
 * cargo run --bin chapter12 -- arg1 arg2
 *
 * Eg:
 *
 * > cargo run --bin chapter12 -- chapter12/src/poem.txt to
 * ["Are you nobody, too?", "How dreary to be somebody!"]
 *
 * > IGNORE_CASE=true cargo run --bin chapter12 -- chapter12/src/poem.txt to
 * ["Are you nobody, too?", "How dreary to be somebody!", "To tell your name the livelong day", "To an admiring bog!"]
 */
use std::env;
use std::fs;
use std::process::exit;

use chapter12::LibConfig;

fn main() {
    // Capture command line arguments
    // Use std::env::args_os to accept invalid Unicode
    let args: Vec<String> = env::args().collect();
    let path = &args[1];
    println!("{path}");

    read_file(path);

    // Calling the search from lib
    let file_path = args[1].clone();
    let query = args[2].clone(); // to create a copy, but this is a bit inefficient
    let ignore_case = env::var("IGNORE_CASE").is_ok();

    let lib_config = LibConfig {
        query,
        file_path,
        ignore_case,
    };
    if let Err(e) = chapter12::run(lib_config) {
        eprintln!("Application error: {e}");
        exit(1);
    }
}

fn read_file(filename: &String) {
    println!("---- In read_file ----");
    let contents = fs::read_to_string(filename).expect("Should have read the value, but error");
    println!("{contents}");
    println!("---- DONE ----");
}

/*
Suggested uses
*/

// Example to return groups of variables as abstraction/struct
struct Config {
    query: String,
    file_path: String,
}

fn parse_config(args: &[String]) -> Config {
    let query = args[1].clone(); // to create a copy, but this is a bit inefficient
    let file_path = args[2].clone();

    Config { query, file_path }
}

/*
Example of doing this here. The right place is in lib.rs. See that file to see how it looks there
*/

// or event better than parse_config fun
// then can be called with
// let config = Config::new(&args);
impl Config {
    fn new(args: &[String]) -> Result<Config, &'static str> {
        // Error handling
        if args.len() < 3 {
            // One alternative is to panic as follows
            // panic!("not enough arguments");
            // Another is to return an error. This changes the return type for `Config` to `Result<Config, &'static str>`
            return Err("Not enough arguments");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();

        Ok(Config { query, file_path })
    }
}
