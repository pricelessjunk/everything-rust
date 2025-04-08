fn main() {
    println!("Hello, world!");
}

#[test]
fn unsafe_raw_pointers() {
    let mut num = 5;

    let r1 = &raw const num;
    let r2 = &raw mut num;

    // We can dereference a raw pointer only with an unsafe block
    unsafe {
        println!("r1 is: {}", *r1);
        println!("r2 is: {}", *r2);
    }
}

// Unsafe methods - can be only called from within an unsafe block
unsafe fn dangerous() {}

#[test]
fn unsafe_functions() {
    unsafe {
        dangerous();
    }
}
