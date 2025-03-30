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
    let add_one_v4 = |x| x + 1;
    add_one_v4(1);

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
        thread::spawn(move || println!("From thread: {immutable_list:?}"))
            .join()
            .unwrap();
    }

    // Skipped fn, fnOnce, fnMut
}

fn run_closure(input: Option<u8>) -> u8 {
    input.unwrap_or_else(|| closure_default())
}

fn closure_default() -> u8 {
    0
}

fn add_one_v1(x: u32) -> u32 {
    x + 1
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
    let _: Vec<_> = v1_iter.map(|val| println!("Got {val}")).collect();
}

