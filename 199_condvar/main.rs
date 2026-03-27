// 199. Condvar
//
// Condvar waits for a notification. You hold a MutexGuard, call wait, and the lock is
// released until notify_one or notify_all. Always wait in a loop that checks the actual
// condition: wakeups can be spurious.
//
// Run: cargo run --bin 199_condvar

use std::sync::{Arc, Condvar, Mutex};
use std::thread;

fn main() {
    let pair = Arc::new((Mutex::new(false), Condvar::new()));
    let pair2 = Arc::clone(&pair);
    thread::spawn(move || {
        let (lock, cv) = &*pair2;
        let mut ready = lock.lock().unwrap();
        *ready = true;
        cv.notify_one();
    });
    let (lock, cv) = &*pair;
    let mut ready = lock.lock().unwrap();
    while !*ready {
        ready = cv.wait(ready).unwrap();
    }
    println!("ready");
}
