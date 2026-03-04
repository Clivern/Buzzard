// 107. HashMap entry
//
// entry(key) returns an Entry. or_insert writes a default if vacant. or_insert_with
// lazily builds the default. and_modify updates an occupied slot. This avoids a double
// lookup compared to get-then-insert.
//
// Run: cargo run --bin 107_entry

use std::collections::HashMap;

fn main() {
    let mut words = HashMap::new();
    for w in ["a", "b", "a"] {
        words.entry(w).and_modify(|c| *c += 1).or_insert(1);
    }
    println!("{words:?}");
}
