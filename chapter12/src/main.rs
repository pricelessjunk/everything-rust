/**
 * cargo run --bin chapter12 -- arg1 arg2
 */

use std::env;
use std::fs;
use std::process::exit;

use chapter12::LibConfig;

fn main() {
    // Capture command line arguments
    // Use std::env::args_os to accept invalid Unicode
    let args: Vec<String> = env::args().collect();
    // dbg!(args);

    let path = &args[1];
    print!("{path}");

    read_file(path);

    // Calling the same thing in lib
    let query = args[1].clone();  // to create a copy, but this is a bit inefficient
    let file_path = args[2].clone();
    let lib_config = LibConfig {query, file_path};
    // if let says if there is an error, then only run the block of code
    if let Err(e) = chapter12::run(lib_config) {
            print!("Application error: {e}");
            exit(1);
    }
}


fn read_file(filename: &String){
    let contents = fs::read_to_string(filename).expect("Should have read the value, but error");

    print!("{contents}");
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
    let query = args[1].clone();  // to create a copy, but this is a bit inefficient
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