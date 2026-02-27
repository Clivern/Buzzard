// 087. Supertraits
//
// trait Print: Display means Print can only be implemented for types that already
// implement Display. Inside Print methods you can use Display. It is a bound on Self,
// not inheritance of fields.
//
// Run: cargo run --bin 087_supertraits

use std::fmt::Display;

trait Label: Display {
    fn label(&self) -> String {
        format!("item:{self}")
    }
}

impl Label for i32 {}

fn main() {
    println!("{}", 7.label());
}
