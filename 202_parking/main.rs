// 202. park and unpark
//
// thread::park pauses until unpark or a timeout. Each thread has a token: unpark before
// park does not deadlock, it just makes the next park return immediately. This is a low-
// level building block; Condvar and channels are the usual API.
//
// Run: cargo run --bin 202_parking

use std::thread;
use std::time::Duration;

fn main() {
    let t = thread::current();
    thread::spawn(move || {
        thread::sleep(Duration::from_millis(5));
        t.unpark();
    });
    thread::park();
    println!("unparked");
}
