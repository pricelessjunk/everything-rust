use std::env;

fn main() {
    let s = true;
    let a = Try { s } ;
}

struct Try {
    something: bool
}