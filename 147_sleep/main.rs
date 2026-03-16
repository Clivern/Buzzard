// 147. sleep
//
// thread::sleep pauses this thread. It is not a high-resolution timer. In async code,
// use the runtime's sleep so the executor can run other tasks. sleep(0) is not a
// reliable yield; yield_now is.
//
// Run: cargo run --bin 147_sleep

use std::thread;
use std::time::Duration;

fn main() {
    thread::sleep(Duration::from_millis(1));
    thread::yield_now();
    println!("awake");
}
