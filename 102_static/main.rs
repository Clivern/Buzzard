// 102. 'static
//
// 'static means the data can live for the rest of the program. String literals are
// &'static str. Owned data can be 'static too: a String stored in a lazy static, or T:
// 'static meaning T does not contain short borrows. Thread spawn requires 'static
// because the thread may outlive the caller.
//
// Run: cargo run --bin 102_static

fn describe() -> &'static str {
    "ok"
}

fn main() {
    let s: &'static str = "hello";
    println!("{s} {}", describe());
}
