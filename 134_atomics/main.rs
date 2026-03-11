// 134. Atomics
//
// AtomicUsize and friends are lock-free shared integers. load, store, fetch_add take an
// Ordering. SeqCst is the strongest and the simplest to reason about. Relaxed is fine
// for counters where you only need the final total. Atomics are Sync, so
// Arc<AtomicUsize> is a common shared counter.
//
// Run: cargo run --bin 134_atomics

use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::thread;

fn main() {
    let n = Arc::new(AtomicUsize::new(0));
    thread::scope(|s| {
        for _ in 0..4 {
            let n = Arc::clone(&n);
            s.spawn(move || {
                n.fetch_add(1, Ordering::SeqCst);
            });
        }
    });
    println!("{}", n.load(Ordering::SeqCst));
}
