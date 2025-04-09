use std::fmt;

/* Advanced Traits  */

// Preferred way to write traits over generics
pub trait Iterator {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;
}

// Operator overloading
#[derive(Debug, Copy, Clone, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

// can be done for traits in the `std::ops` crate
impl std::ops::Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

// Traits that take a parameter, so has a defined type

/*
Here Add is defined as

trait Add<Rhs=Self> {
    type Output;

    fn add(self, rhs: Rhs) -> Self::Output;
}
*/

struct Millimeters(u32);
struct Meters(u32);

impl std::ops::Add<Meters> for Millimeters {
    type Output = Millimeters;

    fn add(self, other: Meters) -> Millimeters {
        Millimeters(self.0 + (other.0 * 1000))
    }
}

// Traits with same named functions
trait Trait1 {
    fn func_name(&self);
}

trait Trait2 {
    fn func_name(&self);
    fn no_self_func();
}

struct Child;

impl Trait1 for Child {
    fn func_name(&self) {
        println!("Trait 1");
    }
}

impl Trait2 for Child {
    fn func_name(&self) {
        println!("Trait 2");
    }

    fn no_self_func() {
        println!("In no self func trait 2");
    }
}

impl Child {
    fn func_name(&self) {
        println!("In child");
    }

    fn no_self_func() {
        println!("In no self func Child")
    }
}

#[test]
fn multi_traits_impl() {
    let c = Child;
    c.func_name(); // Child's own method
    Trait1::func_name(&c);
    Trait2::func_name(&c);
    Child::no_self_func(); // prints the child
    <Child as Trait2>::no_self_func(); // prints from the trait
}

// Supertrait
trait OutlinePrint: fmt::Display {}

// implementing the functions of Display in case not implemeted.
impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({} {})", self.x, self.y)
    }
}

// Newtype pattern. So, implement the properties on the wrapper instead of the other struct.
struct Wrapper(Vec<String>);

impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}]", self.0.join(", "))
    }
}
// Usage
// fn main() {
//     let w = Wrapper(vec![String::from("hello"), String::from("world")]);
//     println!("w = {w}");
// }
