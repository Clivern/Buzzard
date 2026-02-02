// 003. Mutability
//
// let mut marks a binding you can assign to later. The name stays the same; the value
// stored there changes. Prefer immutable bindings, and add mut only when the later
// assignment is the point.
//
// Run: cargo run --bin 003_mutability

fn main() {
    let mut y = 23;
    println!("The value of y is {y}");
    y = 50;
    println!("The value of y is now {y}");
}
