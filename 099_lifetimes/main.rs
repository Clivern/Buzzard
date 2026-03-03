// 099. Lifetime annotations
//
// A lifetime 'a is a name for how long a reference is valid. fn longest<'a>(x: &'a str,
// y: &'a str) -> &'a str says the result cannot outlive either input. The compiler
// already checks this; annotations describe the relationship to the type system, they do
// not change how long values live.
//
// Run: cargo run --bin 099_lifetimes

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() >= y.len() {
        x
    } else {
        y
    }
}

fn main() {
    let a = String::from("abcd");
    let b = "xyz";
    println!("{}", longest(&a, b));
}
