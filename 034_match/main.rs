// 034. match
//
// match compares a value against patterns. It must be exhaustive: every possible value
// has an arm, or you use _. The first match wins. match is an expression, so each arm
// produces the same type. Use it instead of a long if-else chain on enums.
//
// Run: cargo run --bin 034_match

fn main() {
    let n = 2;
    let word = match n {
        1 => "one",
        2 => "two",
        3 => "three",
        _ => "other",
    };
    println!("{word}");
}
