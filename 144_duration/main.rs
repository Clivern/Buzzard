// 144. Duration
//
// Duration is a span of time. from_secs, from_millis, from_nanos build one. You can add
// durations, divide them, and convert to seconds as f64. It cannot be negative; a signed
// span is not in std.
//
// Run: cargo run --bin 144_duration

use std::time::Duration;

fn main() {
    let d = Duration::from_millis(1500);
    println!("{}s {}ms", d.as_secs(), d.as_millis());
    println!("{:?}", d + Duration::from_secs(1));
}
