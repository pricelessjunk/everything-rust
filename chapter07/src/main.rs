use crate::garden::vegetables::Asparagus;

mod garden;

/* module for resturant */
// Child modules can see the outer context, but the outer context can't see the child module.
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
        fn seat_at_table() {
            // Used to call a function from a outer context
            super::outer_func();
        }
    }

    mod serving {
        fn take_order() {}
        fn serve_order() {}
        fn take_payment() {}
    }
    
    fn outer_func() {}
}

use crate::front_of_house::hosting as host;

fn main() {
    let plant = Asparagus {};
    println!("{:?}", plant);


    /* Using the resturant system  */
    // absolute path
    crate::front_of_house::hosting::add_to_waitlist();
    // Relative path
    front_of_house::hosting::add_to_waitlist();
    // Using use
    hos::add_to_wishlist();
}
