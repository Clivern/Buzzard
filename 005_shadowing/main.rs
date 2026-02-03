// 005. Shadowing
//
// A second let with the same name hides the earlier binding. The new binding can even
// have a different type. That is not mutation: the old value is still there, just
// unreachable by that name. Shadowing is common when you transform a value in steps,
// such as parsing a string into a number.
//
// Run: cargo run --bin 005_shadowing

fn main() {
    let n = 1;
    let n = n + 1;
    println!("as int {n}");
    let n = "two";
    println!("shadowed {n}");

    let spaces = "   ";
    let spaces = spaces.len();
    println!("spaces {spaces}");
}
