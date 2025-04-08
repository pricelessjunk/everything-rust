#![allow(dead_code, unused_imports, unused_variables)]

use std::cell::Ref;
use std::rc::{Rc, Weak};
use std::{cell::RefCell, fmt::Debug};

fn main() {
    println!("Hello, world!");
}

///
/// Box smart pointer
///
/// - Data is actually stored in a heap
/// - Used when a the size is not known at compile time
/// - Can be treated as regular references since they implement traits `deref` and `drop`
/// - These pointers implicitly have Deref Coercions. Deref Coercion converts a reference to a type that implements the Deref trait into a reference to another type.
/// - Deref Coercion can also interact with mutability. The DerefMut trait is necessary.

#[derive(Debug)]
enum List {
    Cons(i8, Box<List>),
    Nil,
}

use List::{Cons, Nil};

#[test]
fn smart_pointer_box() {
    let list = Box::new(Cons(1, Box::new(Cons(2, Box::new(Nil)))));
    println!("{:?}", list);
}

#[test]
fn using_box_as_regular_pointer() {
    let x = 5;
    let y = Box::new(x);

    assert_eq!(5, *y);
}

/// Example of drop trait implementation
#[derive(Debug)]
struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!(
            "Closehandler: Dropping CustomSmartPointer with data `{}`!",
            self.data
        );
    }
}

#[test]
fn drop_trait_helps_create_closehandler() {
    let c = CustomSmartPointer {
        data: String::from("some value"),
    };
    let d = CustomSmartPointer {
        data: String::from("This will be dropped early"),
    };

    // dropping d early
    std::mem::drop(d);
    println!("no more d");

    println!("the close handler should be called after this line")
}

/// Rc<T> implementation
///
/// - Useful for counting how many people borrowed
/// - Only the pointers get borrowed. You can't move the value
/// - Reference stays alive till all borrows are completed
#[test]
fn exploring_rc() {
    let a = CustomSmartPointer {
        data: "abc".to_string(),
    };

    // let b = a;
    // let c = a; This won't work

    // let aBox = Box::new(a);
    // let b = Box::clone(&aBox);  // Clone is not implemented

    let a_rc = Rc::new(a);
    let b = Rc::clone(&a_rc); // immutable borrow  
    let c = Rc::clone(&a_rc);
    println!("{:?}", *c);

    assert_eq!(3, Rc::strong_count(&a_rc)); // 2 borrows + 1 main owner
}

/// RefCell<T>
///
/// - Same as Box<T>, just borrowing rules checked at runtime
/// - If rule breaks, then the code panics
/// - Interior Mutability - It can be used to mutate struct when it is non-mutable.

#[test]
fn exploring_ref_cell() {
    let a = 5;
    let a_ref_cel = RefCell::new(&a);
    let b = a_ref_cel.borrow_mut();
    // This is wrong. It will fail in runtime
    // already mutably borrowed: BorrowError
    // But since this is refcell, compilation doesn't fail.
    // let c = a_ref_cel.borrow();
}

/// Combining RefCell<T> and Rc<T>
#[test]
fn combining_refcell_rc() {
    let a = 5;
    let a_refcell_rc = Rc::new(RefCell::new(a)); // a is copied. so original a is untouched
    let b = Rc::clone(&a_refcell_rc);
    let c = Rc::clone(&a_refcell_rc);
    assert_eq!(3, Rc::strong_count(&a_refcell_rc));

    // Both the statements update the same memory value, which is a_refcell
    *(b.borrow_mut()) += 1; // incrementing first borrow 1
    *(c.borrow_mut()) += 1; // incrementing second borrow 1 

    println!("{a}"); // 5
    println!("{}", (*b).borrow()); // 7
    println!("{}", (*c).borrow()); // 7
    assert_eq!(3, Rc::strong_count(&a_refcell_rc));
}

/// Reference cycle and weak pointers
///
/// Happens for cyclic pointers where two Rc hold a reference of one another.
/// When both are dropped at the end, the counter would still count the references held by each other.
/// This leads to a memory leak.
/// To avoid this, one needs to be a weak pointer.
///
/// Best example a parent and child node. When the parent expires, the child reference should also expire but not the other way around.
struct Node {
    value: i8,
    parent: RefCell<Weak<Node>>,
    child: RefCell<Option<Rc<Node>>>,
}

#[test]
fn weak_references() {
    // Creating a leaf with empty parent and child
    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        child: RefCell::new(None),
    });

    // Creating a parent with the leaf as child
    let branch = Rc::new(Node {
        value: 5,
        parent: RefCell::new(Weak::new()),
        child: RefCell::new(Some(Rc::clone(&leaf))),
    });

    // We assign a downgraded version of the branch to the parent. So, if parent dies, nobody cares for this parent Rc that the child holds.
    *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

    // upgrade to make it strong again. Need to upgrade to access the value
    println!(
        "leaf parent {}",
        (*leaf.parent.borrow()).upgrade().unwrap().value
    );
}
