/**
 * cargo run --bin chapter12 -- needle haystack
 */

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    dbg!(args);
}
