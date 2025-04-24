use std::env;

fn main() {
    let s = true;
    // let a = Try { s } ;
}

struct Try {
    something: bool,
}


#[test]
fn try_something() {
    assert_eq!(100usize.saturating_add(1), 101);
}