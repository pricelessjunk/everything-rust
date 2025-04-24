use std::{collections::HashMap, fmt};

/*
 * To note, the get function borrows the value. Which means, once the value is borrowed, a
 * simutaneous edit on the list is not possible as long as the borrowed value has a pending usage
 */

fn main() {
    /* Vectors  */
    // Creation
    let mut v: Vec<i32> = Vec::new();
    let v2 = vec![1, 2, 3]; // At the end of the scope this contents of the elements will also
    // drop
    v.push(4);
    v.push(5);
    v.push(6);

    // Access
    let third: &i32 = &v[2];

    // Access with macros
    let third: Option<&i32> = v.get(2);
    match third {
        Some(i) => println!("{}", i),
        None => println!("There is no third"),
    }

    // Iterating over vectors
    let v4 = vec![100, 32, 57];
    for i in v4 {
        println!("{i}");
    }

    // Iterating over mutable vectors
    let mut v5 = vec![1, 2, 3];
    for i in &mut v5 {
        println!("{i}");
        *i += 2 // To change the value of a mutable reference, * is needed
    }

    // removing
    let mut v6 = vec!["first", "second"];
    v6.remove(0);
    println!("{:?}",v6);

    // Using enum to store different values
    // https://doc.rust-lang.org/book/ch08-01-vectors.html#using-an-enum-to-store-multiple-types
    // Not doing this since this looks terrible

    // Strings editing
    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2); // s2 is a string slice

    let mut s3 = String::from("lo");
    s3.push('l'); // Single character pushing

    let s4 = String::from("Hello, ");
    let s5 = String::from("World");
    // The + here uses the add method
    // fn add(self, s:&str) -> String
    // self ownership is taken, s ownership is not taken.
    let s6 = s4 + &s5; // s4 has been moved. Cannot be used here anymore. 
    println!("{s6}");

    // String char access
    let hindiStr = String::from("नमस्ते");
    for s in hindiStr.chars() {
        println!("{s}");
    }

    // Byte access
    for s in hindiStr.bytes() {
        println!("{s}");
    }

    // Slicing strings
    println!("{}", &hindiStr[0..6]); // The letters here are 3 bytes each

    // Hashmaps
    let mut map = HashMap::new();
    map.insert(String::from("ten"), 10);
    map.insert(String::from("fifty"), 50); // reinserting replaces an older value
    map.entry(String::from("twenty")).or_insert(20); // inserting if not present                                           

    // copied is used to convert the Option<V> to Option<i32>. unwrap_or returns 0 if not found
    println!("{}", map.get(&String::from("ten")).copied().unwrap_or(0));

    // Looping over hashmap
    for (key, value) in map {
        println!("{key},{value}");
    }

    let text = "heelo worls dll ed";
    text.split_whitespace();
}
