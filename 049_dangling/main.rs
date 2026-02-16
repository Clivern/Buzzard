// 049. No dangling references
//
// A function cannot return a reference to a local value: the value would be dropped
// before the caller used the reference. Return an owned value, or take a reference that
// the caller already owns and pass that borrow back.
//
// Run: cargo run --bin 049_dangling

fn longest<'a>(a: &'a str, b: &'a str) -> &'a str {
    if a.len() >= b.len() {
        a
    } else {
        b
    }
}

fn main() {
    let a = String::from("long");
    let b = String::from("tiny");
    println!("{}", longest(&a, &b));
}
