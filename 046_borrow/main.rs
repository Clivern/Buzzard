// 046. Borrow rules
//
// You may have either one mutable reference, or any number of shared references, not
// both. The compiler enforces this at compile time. That is the aliasing rule that makes
// data races impossible in safe Rust. NLL (non-lexical lifetimes) ends a borrow when
// last used, not at the end of the scope.
//
// Run: cargo run --bin 046_borrow

fn main() {
    let mut s = String::from("hello");
    let r1 = &s;
    let r2 = &s;
    println!("{r1} {r2}");
    let r3 = &mut s;
    r3.push_str(" rust");
    println!("{r3}");
}
