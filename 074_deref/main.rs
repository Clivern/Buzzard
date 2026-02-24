// 074. Deref
//
// Deref lets a type behave like a reference to an inner value. *x on a String is a str
// because String implements Deref<Target = str>. Method resolution also follows Deref,
// which is why s.len() works on String. DerefMut is the mutable version. Smart pointers
// (Box, Rc, Arc) all implement it.
//
// Run: cargo run --bin 074_deref

use std::ops::Deref;

fn needs_str(s: &str) {
    println!("{s}");
}

fn main() {
    let s = String::from("hello");
    needs_str(&s);
    let r: &str = s.deref();
    println!("{r}");
}
