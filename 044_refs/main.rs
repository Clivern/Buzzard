// 044. References
//
// &T is a shared reference. You can have many of them at once. *r reads through the
// reference; field access auto-dereferences. The referent cannot be mutated while any
// shared reference exists. References never dangle: the compiler tracks how long they
// may live.
//
// Run: cargo run --bin 044_refs

fn len(s: &String) -> usize {
    s.len()
}

fn main() {
    let s = String::from("rust");
    let n = len(&s);
    println!("{s} len {n}");
}
