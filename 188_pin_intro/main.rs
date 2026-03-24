// 188. Pin
//
// Pin<&mut T> promises T will not be moved in memory. async state machines can hold
// references to their own fields, which would dangle if the future moved. Unpin is auto-
// implemented for most types and means Pin does not add extra rules. Box::pin and pin!
// create pinned futures.
//
// Run: cargo run --bin 188_pin_intro

use std::pin::Pin;

fn main() {
    let mut n = 1;
    let mut p = Pin::new(&mut n);
    *p = 2;
    println!("{n}");
}
