/**
 * # Commands
 * cargo test
 * cargo test -- --test-threads=1   // running not in parallel
 * cargo test -- --show-output      // Shows output
 * cargo test <test function name>/<prefix>
 * cargo test -- --ignored          // Running ignored tests
 *
 * Typical test structure
 * ├── Cargo.lock
 * ├── Cargo.toml
 * ├── src
 * │   └── lib.rs
 * └── tests
 *    ├── common
 *    │   └── mod.rs
 *    └── integration_test.rs
 *
 * Putting common code as module doesn't print output of that file.
 * It will not be considered as an output file.
 */

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub fn add_two(a: usize) -> usize {
    add(a, 2)
}

pub fn function_that_panics() {
    panic!("I am panicking");
}

// cfg tells the compiler not to compile the module under for name test
// Alternatively move to tests directory
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4, "result is suppose to be 4"); // asserting equality
        assert!(result == 4); // asserting bool
        assert_ne!(result, 5) // asserting ne
    }

    #[test]
    #[should_panic(expected = "panicking")] // Expected substing
    fn should_panic() {
        function_that_panics();
    }

    // Tests with results
    #[test]
    fn test_with_result() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("Two plus two is four"))
        }
    }

    #[test]
    #[ignore]
    fn ignoring_this_test() {}
}
