use std::thread;

fn main() {
    // Closures with options (Some or None)
    let value_option = Some(8);
    println!("{}", run_closure(value_option)); // prints 8
    let empty_option = None;
    println!("{}", run_closure(empty_option)); // prints 0

    // Different versions of add_one_v1()
    add_one_v1(1);
    let add_one_v2 = |x: u32| -> u32 { x + 1 };
    add_one_v2(1);
    let add_one_v3 = |x| x + 1;
    add_one_v3(1);

    // closure borrowing immutably
    {
        let immutable_list = vec![1, 2, 3];
        let borrow_immutably = || println!("{immutable_list:?}");
        borrow_immutably();
        println!("After borrow immutably {immutable_list:?}");
    }

    // closure borrowing mutably
    {
        let mut mutable_list = vec![1, 2, 3];
        let mut borrow_mutably = || mutable_list.push(7);
        borrow_mutably();
        println!("After borrow immutably {mutable_list:?}"); // this works since the closure is not used again
        // borrow_mutably();   // Calling this line would break previous line since cannot mutable borrow after println
    }

    // taking ownership with move
    {
        let immutable_list = vec![1, 2, 3];
        // the move keyword is necessary since the other thread may outlive the current thread and needs ownership of immutable_list
        thread
            ::spawn(move || println!("From thread: {immutable_list:?}"))
            .join()
            .unwrap();
    }

    // Fn - Can be called multiple times since doesn't mutate or consume.
    let greeting = "Hello";
    let say_hello = || println!("{}", greeting); // Immutable borrow
    call_fn(say_hello);

    // FnMut - Use this when the closure mutates something it captures.
    let mut count = 0;
    let mut increment = || {
        count += 1; // Mutates outer variable
        println!("Count: {}", count);
    };
    call_fn_mut(&mut increment);

    //FnOnce - Consumes captured variables (takes ownership)
    let name = String::from("Kaustuv");
    let say_name = || {
        println!("Name: {}", name);
        drop(name); // Moves ownership
    };
    call_fn_once(say_name);
}

fn run_closure(input: Option<u8>) -> u8 {
    input.unwrap_or_else(|| closure_default())
}

fn closure_default() -> u8 {
    0
}

/// Add one
///
/// returns the sum
///
/// Panics here
fn add_one_v1(x: u32) -> u32 {
    x + 1
}

fn call_fn<F: Fn()>(f: F) {
    f();
    f(); // Can call again
}

fn call_fn_mut<F: FnMut()>(mut f: F) {
    f();
    f();
}

fn call_fn_once<F>(f: F) where F: FnOnce() {
    f();
    // f(); // ❌ Can't call again
}

#[test]
fn iterator_sum() {
    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();
    let total: i32 = v1_iter.sum();
    assert_eq!(total, 6);
}

#[test]
fn iterator_map() {
    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();
    // Iterators are lazy and don't do anything unless consumed.
    let _: Vec<_> = v1_iter
        .clone()
        .map(|val| println!("Got {val}"))
        .collect();
    let plus_2: Vec<i32> = v1_iter.map(|i| i + 2).collect();
}
