// 146. SystemTime
//
// SystemTime is wall-clock time. now() can jump backwards if the OS clock changes.
// duration_since fails if the other time is later. UNIX_EPOCH is 1970. For clocks you
// display to users, a crate such as chrono or time is nicer.
//
// Run: cargo run --bin 146_systemtime

use std::time::{SystemTime, UNIX_EPOCH};

fn main() {
    let now = SystemTime::now();
    let secs = now.duration_since(UNIX_EPOCH).unwrap().as_secs();
    println!("unix {secs}");
}
