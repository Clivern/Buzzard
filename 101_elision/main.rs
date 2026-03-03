// 101. Lifetime elision
//
// Many functions can omit lifetimes because of elision rules. One input lifetime is
// copied to all output references. &self gives its lifetime to outputs. When those rules
// are not enough, the compiler asks you to write the names. Elision never changes
// meaning; it only hides the obvious cases.
//
// Run: cargo run --bin 101_elision

fn first_word(s: &str) -> &str {
    s.split_whitespace().next().unwrap_or(s)
}

fn main() {
    println!("{}", first_word("hello rust"));
}
