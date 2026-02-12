// 039. Ownership
//
// Each value has one owner. When the owner goes out of scope, the value is dropped.
// Assigning a String moves it: the old name cannot be used. That prevents a double free.
// Types that implement Copy, such as i32, are duplicated instead of moved.
//
// Run: cargo run --bin 039_ownership

fn main() {
    let a = String::from("hello");
    let b = a;
    println!("{b}");
    let n = 3;
    let m = n;
    println!("{n} {m}");
}
