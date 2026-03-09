// 127. move closures in threads
//
// spawn needs ownership of captured data. move on the closure takes it. Clone an Arc
// before spawn so both sides share. Do not try to borrow a local across spawn without
// scoped threads; the borrow might dangle.
//
// Run: cargo run --bin 127_move_thread

use std::thread;

fn main() {
    let s = String::from("hello");
    let h = thread::spawn(move || {
        println!("{s}");
    });
    h.join().unwrap();
}
