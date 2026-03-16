// 145. Instant
//
// Instant is a monotonic timestamp. elapsed() is a Duration since it was taken. Use it
// to measure speed. It is not wall-clock time and should not be stored across process
// restarts. Instant::now() - start also works.
//
// Run: cargo run --bin 145_instant

use std::time::{Duration, Instant};

fn main() {
    let start = Instant::now();
    std::thread::sleep(Duration::from_millis(5));
    println!("elapsed {:?}", start.elapsed());
}
