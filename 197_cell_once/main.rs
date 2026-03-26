// 197. Once
//
// Once runs a closure at most once, even if many threads call call_once. OnceLock stores
// a value. Once is for side-effect initialization (logging, C libraries). is_completed
// reports whether it already ran.
//
// Run: cargo run --bin 197_cell_once

use std::sync::Once;

static INIT: Once = Once::new();

fn main() {
    INIT.call_once(|| println!("init"));
    INIT.call_once(|| println!("skipped"));
    println!("{}", INIT.is_completed());
}
