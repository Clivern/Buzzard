// 096. Closures
//
// A closure is an anonymous function that can capture its environment. |n| n + 1 is the
// short form. The compiler infers parameter and return types when it can. Closures can
// be stored, passed, and returned (with impl Fn).
//
// Run: cargo run --bin 096_closures

fn main() {
    let add = |a, b| a + b;
    println!("{}", add(2, 3));
    let factor = 10;
    let scale = |n| n * factor;
    println!("{}", scale(4));
}
