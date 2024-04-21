fn main() {
    let mut s = String::from("Hello");

    let r = &mut s;
    // let r2 = &mut s;   Second borrow here for &mut results in error when used
    // println!("{r}{r2}");
    // This can be solved with scopes but no scope can have both
    //
    let r_immut = &s;
    let r_immut_2 = &s;
    // let r_mut = &mut s; Combining is also erroraneous
    // println!("{r_immut}, {r_immut_2}, {r_mut}");


    /* String slices */
    let ss = "hello world";
    let str_len = ss.len();
    let first: &str = &ss[0..5];
    let second: &str = &ss[6..str_len]; // same as &ss[6..]
    println!("{first}");
    println!("{second}");

    let s2 = String::from("hello");
    let slice = slice_string(&s2);
    println!("{slice}");

    /* array slices  */
    let a = [1,2,3,4,5];
    let a_slice = &a[2..4];
    assert_eq!(a_slice, [3, 4])

}

// The following throws an error 'expected named lifetime parameter'.
// This is because s is dropped when it reaches the end of scope
// Expects the scope to be static
fn dangling_reference() -> /*&*/String {
    let s=String::from("abc");
    return /*&*/s;
}

fn slice_string(s: &String) -> &str {
    return &s[0..1];
}
