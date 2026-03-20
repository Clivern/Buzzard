// 169. Range patterns
//
// 1..=5 in a match arm is inclusive. Exclusive range patterns 1..5 are allowed for
// numbers. char ranges work for ASCII classes. The range must be compile-time constant.
//
// Run: cargo run --bin 169_range_patterns

fn main() {
    let c = 'm';
    match c {
        'a'..='z' => println!("lower"),
        'A'..='Z' => println!("upper"),
        _ => println!("other"),
    }
}
