//https://doc.rust-lang.org/book/ch03-01-variables-and-mutability.html
fn main() {
    let x = 5;  // immutable i32
    let mut y = 4; //changing to mut

    const THREE: u32 = 3;

    /*
    Variable Shadowing
     */
    let x1=5;
    let x1 = x1 + 1;
    {
        let x1 = x1 * 2;
        println!("{x1}"); // 12
    }
    println!("{x1}") // 6


}
