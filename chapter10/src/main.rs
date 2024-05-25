mod traits;

fn main() {
    let list = vec![1,2,3,4];
    let num = largest_with_generics(&list);
    println!("{num}");

    let p = Point {
        x: 1,
        y: 2,
    };

    println!("Point.x is {}", p.x());

    // Traits
    traits::traits_example();
}

// In functions
fn largest_with_generics<T: std::cmp::PartialOrd>(list: &[T]) -> &T{
    let mut largest = &list[0];

    for l in list {
        if l > largest {
            largest = l;
        }
    }

    largest
}

// In structs and functions of struct
struct Point<T> {
    x: T,
    y: T,
}

// We can also implement methods for generic structs with specific types
// impl Point<f32> {
impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}




