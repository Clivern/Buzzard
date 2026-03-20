// 168. Or patterns
//
// | in a pattern is or: 1 | 2 | 3. Bindings must be the same on every branch. Combined
// with .. and if guards, match stays flat instead of nested.
//
// Run: cargo run --bin 168_or_patterns

fn main() {
    for n in [1, 2, 8] {
        match n {
            1 | 2 => println!("{n} small"),
            _ => println!("{n} other"),
        }
    }
}
