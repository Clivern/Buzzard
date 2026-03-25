// 194. ManuallyDrop
//
// ManuallyDrop<T> suppresses Drop. into_inner takes T out. This is how you implement
// custom Drop that moves a field out, or a union. Forgetting to drop leaks; dropping
// twice is undefined if T has Drop. Use it inside unsafe abstractions, not in ordinary
// code.
//
// Run: cargo run --bin 194_manuallydrop

use std::mem::ManuallyDrop;

fn main() {
    let s = ManuallyDrop::new(String::from("keep"));
    let inner = ManuallyDrop::into_inner(s);
    println!("{inner}");
}
