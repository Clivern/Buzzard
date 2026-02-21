// 069. Display
//
// Display is the user-facing format, used by {}. Implement it with write!(f, "...").
// ToString is implemented for every Display type via to_string(). Do not implement
// ToString yourself. Debug and Display can differ.
//
// Run: cargo run --bin 069_display

use std::fmt;

struct Meter(i32);

impl fmt::Display for Meter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}m", self.0)
    }
}

fn main() {
    let m = Meter(12);
    println!("{m}");
    println!("{}", m.to_string());
}
