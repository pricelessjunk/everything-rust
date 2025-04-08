use std::error::Error;
use std::fs::File;
use std::io::{ErrorKind, Read};

/**
 * The main in general has a return type of (). This means, using ? on main will cause an error.
 * To fix this problem, the return type of main needs to be changed to
 *
 * fn main() -> Result<(), <dyn Error>> {
 *      Ok(());
 * }
 *
 * This is a trait object. More on this later.
 *
 */
fn main() {
    let file_result = File::open("something.txt");

    let the_file = match file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => panic!("Problem opening file {:?}", error),
            other_error => panic!("Problem opening file {:?}", other_error),
        },
    };
}

fn manual_panic() {
    panic!("lala");
}

// unwrap returns the file if Ok, else panics.
fn unwrap_method() {
    let another_file = File::open("something.txt").unwrap();
}

// expect same as unwrap but lets you choose an error message
fn expect_method() {
    let third_file = File::open("lla").expect("Custom error message");
}

/** The ? at the end of the result behaves as match. If it is error, it propagates to the calling method,
 * else returns the value. The return type also changed.
 */
fn read_from_file() -> Result<String, Box<dyn Error>> {
    let mut username_file = File::open("hello.txt")?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username) // semicolon not added to return the value
}
