fn main() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    let home = IpAddr::V4(String::from("127.0.0.1"));
    let loopback = IpAddr::V6(String::from("::1"));
    loopback.print_it();

    /* Option enum  */
    let x: Option<i8> = Some(6);
    let y: i8 = 3;

    /* match control flow */
    println!("{}", value_in_numbers(IpAddrKind::V4)); // prints 4

    /* match control flow example with Option  */
    let optVal: Option<u8> = Some(5);
    match optVal {
        Some(i) => {
            println!("In some {0}", i);
        }
        None => {
            println!("In none");
        }
    }

    /* if let instead of match  */
    if let Some(val) = optVal {
        println!("Inside if let with {0}", val);
    }
}

fn value_in_numbers(ip_kind: IpAddrKind) -> u8 {
    match ip_kind {
        IpAddrKind::V4 => 4,
        IpAddrKind::V6 => 6,
        other => 0,
        _ => 0,
    }
}

enum IpAddrKind {
    V4,
    V6,
}

enum IpAddr {
    V4(String),
    V6(String),
}

impl IpAddr {
    fn print_it(&self) {
        // println!("{:?}", self);  derive debug not opted in
    }
}

/*
 * Standard lib
 *
 * enum Option<T> {
 *      None,
 *      Some(T)
 *  }
 */
