// 123. Mutex
//
// Mutex<T> is mutual exclusion. lock() blocks until you get a MutexGuard, which derefs
// to T and unlocks on drop. If a thread panics while holding the lock, the mutex is
// poisoned: lock() returns Err. unwrap that unless you have a recovery plan.
//
//
// The guard's lifetime is the critical section. Drop it before join if you do not want
// to hold the lock while waiting.
//
// Run: cargo run --bin 123_mutex

use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let n = Arc::new(Mutex::new(0));
    let mut handles = vec![];
    for _ in 0..4 {
        let n = Arc::clone(&n);
        handles.push(thread::spawn(move || {
            *n.lock().unwrap() += 1;
        }));
    }
    for h in handles {
        h.join().unwrap();
    }
    println!("{}", n.lock().unwrap());
    println!("done");
}
