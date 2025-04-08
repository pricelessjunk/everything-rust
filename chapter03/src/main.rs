use std::{io, result};

//https://doc.rust-lang.org/book/ch03-01-variables-and-mutability.html
fn main() {
    let x = 5; // immutable i32
    let mut y = 4; //changing to mut

    const THREE: u32 = 3;

    /*
    Variable Shadowing
     */
    let x1 = 5;
    let x1 = x1 + 1;
    {
        let x1 = x1 * 2;
        println!("{x1}"); // 12
    }
    println!("{x1}"); // 6

    /*
    Shadowing with different type not possible
     */
    let mut spaces = "   ";
    //spaces = spaces.len(); We cannot change type unless we redeclare them
    let mut spaces = spaces.len();

    /*
    Data types
     */
    let decimal: i32 = 98_222;
    let hex = 0xfff;
    let octal = 0o77;
    let bin = 0b11110000;
    let byte: u8 = b'A';

    let x = 2.0; //f64
    let y: f32 = 2.0; //f32

    let t = true;
    let f = false;

    let heart_eyed_cat = '😻';

    /*
    Compound types
     */
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (a, b, c) = tup;

    /*
    Arrays
     */
    let arr: [i32; 5] = [2, 3, 4, 5, 6];
    let months: [&str; 2] = ["Jan", "Feb"];
    let threeArr = [3; 5]; // [3, 3, 3, 3, 3]

    /* Reading line from console */
    let mut index = String::new();
    print!("Enter a number (0-4): ");
    io::stdin()
        .read_line(&mut index)
        .expect("Error reading line");
    let index: usize = index.trim().parse().expect("Index not a number");
    let element = arr[index]; // Throws error at runtime if size greater

    /* Functions */
    another_function(5);
    let five = five();
    println!("{five}");

    /* if */
    let number = 3;
    if number < 5 {
        println!("true")
    } else {
        println!("false");
    }

    let absNumber = if number < 0 { number * -1 } else { number };

    /* loop  */
    /*loop {
        println!("infinite loop");
    }*/

    // Returns a value on break
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };

    // Or break by label
    'label_for_loop: loop {
        break 'label_for_loop;
    }

    //for element in a {}
    for number in (1..4).rev() {
        println!("{number}");
    }
}

fn another_function(x: i32) {
    println!("Another function: {x}");
}

fn five() -> i32 {
    5
}
