// 196. Wrapping arithmetic types
//
// Wrapping<T> makes + wrap in both debug and release. Saturating<T> saturates. They
// implement arithmetic operators so you do not sprinkle wrapping_add on every call.
// Useful in checksums, graphics, and size calculations.
//
// Run: cargo run --bin 196_wrapping

use std::num::Wrapping;

fn main() {
    let a = Wrapping(250u8);
    let b = Wrapping(10u8);
    println!("{}", (a + b).0);
}
