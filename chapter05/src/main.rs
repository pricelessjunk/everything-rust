fn main() {
    let user = User {
        active: true,
        name: "Kaustuv".to_string(),
        age: 20,
    };

    println!("{0}", user.name);

    // user.age = 37 Will fail since the whole struct is immutable

    let mut mutable_user = User {
        active: true,
        name: String::from("Kaustuv"),
        age: 20,
    };

    mutable_user.age = 36;  // Works

    /* Field init shorthand  */
    let name = String::from("Kaustuv");
    let active = true;
    let age = 20;
    let init_shorthand_user = User {
        active,
        name,
        age,
    };

    /* Struct update syntax   */
    // Takes the rest from user.
    let user_copy = User {
        age: 37,
        ..user
    };
    // println!("{}", user.name);  value borrowed here after move. Compilation error

    // println!("{}", mutable_user);    Error `std::fmt:Display` not implemented by `User`.
    println!("{:?}", mutable_user);    // Works only with debug opt-in #derive[Debug] 
    println!("{:#?}", mutable_user);   // Pretty. Works only with debug opt-in #derive[Debug] 

    /* Methods in struct  */
    let rect = Rectangle {
        width: 2,
        height: 3,
    };
    println!("{}", rect.area());
    

    println!("{}", Rectangle::square(5).area());
}

#[derive(Debug)]
struct User {
    active: bool,
    name: String,
    age: u32,
}

struct Rectangle {
    width: i32,
    height: i32,
}

impl Rectangle {
    // This is an assiciated function with the type Rectangle.
    fn area(&self) -> i32 {
        return self.width * self.height;
    }

    // Associated function without &self in paramter. Like String::from()
    fn square(size: i32) -> Self{
        Self {
            width: size,
            height: size,
        }
    }
}

