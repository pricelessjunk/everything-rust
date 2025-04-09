/* Future Traits */

use std::pin::{self, Pin};
use std::task::{Context, Poll};
/// Definition of Future trait
pub trait Future {
    type Output;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output>;
}

#[test]
fn future_trait() {

}


/* Pin and Unpin trait */
// https://doc.rust-lang.org/book/ch17-05-traits-for-async.html#the-pin-and-unpin-traits

#[test]
fn pin_trait() {
    let mut s = String::from("Hello!");
    let mut pinned = Pin::new(&mut s);
    pinned.push_str(" Won't be moved!");
    println!("{pinned}");
}

/* Stream Trait */

trait Stream {
    type Item;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>>;
}