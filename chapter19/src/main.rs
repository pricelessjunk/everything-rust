fn main() {
    let x = Some(1);
    match x {
        None => None,
        Some(i) if i % 2 == 0 => Some(i + 2), // Extra conditions
        Some(i) => Some(i + 1),
    };

    // If let
    let fav_color = Some("Red");
    if let Some(color) = fav_color {
        println!("Directly matching in the if command: {color}");
    }

    // Same with while
    //  while let Ok(value) = rx.recv() {
    //     println!("{value}");
    // }

    // for loops with destructurers
    let v = vec!['a', 'b'];
    for (index, value) in v.iter().enumerate() {
        println!("{index} -> {value}");
    }

    // More destructure
    #[allow(unused_variables)]
    let (x, y, z) = (1, 2, 3);
    let p = Point { x: 0, y: 7, z: 6 };
    let Point { x, y, z } = p;
    assert_eq!(0, x);
    assert_eq!(7, y);

    // In functions it would be
    // let point = (3, 5);
    // print_coordinates(&point);
    // fn print_coordinates(&(x, y): &(i32, i32)) {
    //      println!("Current location: ({x}, {y})");
    // }

    // multiple patterns
    let x = 5;
    match x {
        1 | 2 => println!("one or two"),
        1..=5 => println!("one through five"),
        _ => println!("something else"),
    }

    // matching structs with partial matches
    let p = Point { x: 0, y: 10, z: 1 };
    #[allow(unused_variables, unreachable_patterns)]
    match p {
        Point {
            x: inner_x @ 0..10,
            y,
            z,
        } => println!("Binding to an inner value {inner_x}"),
        Point { x, y: 0, z } => println!("Only when y is 0"),
        Point { x: 0, y, z } => println!("Only when x is 0"),
        Point { x, .. } => println!("Ignoring all but x"),
        Point { x, y, z } => println!("Normal match"),
    }

    // Ignoring
    let _x = 5;

    // More matches
    let numbers = (2, 4, 8, 16, 32);
    match numbers {
        (first, .., last) => {
            println!("Some numbers: {first}, {last}");
        }
    }
}

struct Point<T> {
    x: T,
    y: T,
    z: T,
}
