use std::os::raw::c_int;

/* Unsafe Rust */

fn main() {
    unsafe {
        println!("Hello, world! {}", *(&raw const HELLO_WORLD)); // raw pointer
        hello_to_universe();
        println!("Hello, world! {}", *(&raw const HELLO_WORLD));
    }
}

#[test]
fn unsafe_raw_pointers() {
    let mut num = 5;

    let r1 = &raw const num;
    let r2 = &raw mut num;

    // We can dereference a raw pointer only within an unsafe block
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

/// Creating slices from raw pointers
///
/// slice::from_raw_parts_mut(ptr, mid)
///
#[allow(dead_code)]
fn split_at_mut(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = values.len();
    let ptr = values.as_mut_ptr();

    assert!(mid <= len);

    unsafe {
        (
            std::slice::from_raw_parts_mut(ptr, mid),
            std::slice::from_raw_parts_mut(ptr.add(mid), len - mid),
        )
    }
}

// Using extern keyword
unsafe extern "C" {
    fn abs(input: c_int) -> c_int;
}

#[test]
fn extern_test() {
    unsafe {
        assert_eq!(3, abs(-3));
    }
}

/// Calling rust from other languages
/// no_mangle tells the compiler not to change the name for performance
#[unsafe(no_mangle)]
pub extern "C" fn call_from_c() {
    println!("Just called a Rust function from C!");
}

/// Accessing and modifying mutable static
static mut HELLO_WORLD: &str = "Hello World";
// see main

/// SAFETY: Calling this from more than a single thread at a time is undefined
/// behavior, so you *must* guarantee you only call it from a single thread at
/// a time.
fn hello_to_universe() {
    unsafe { HELLO_WORLD = "hello universe!" }
}

mod advanced_traits;
mod macros;
