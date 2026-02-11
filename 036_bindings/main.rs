// 036. Match bindings and @
//
// @ binds a name to the value that matched a subpattern. n @ 1..=5 captures the number
// while still testing the range. You can also bind with a simple name in a tuple or
// struct pattern: Point { x, y }.
//
// Run: cargo run --bin 036_bindings

fn main() {
    let n = 3;
    match n {
        k @ 1..=5 => println!("small {k}"),
        k => println!("other {k}"),
    }
}
