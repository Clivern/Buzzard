// 122. Arc
//
// Arc<T> is an atomically reference-counted pointer. clone is safe across threads.
// Combined with Mutex or RwLock you get shared mutable state. Prefer channels when you
// can; Arc<Mutex<T>> when many threads must see the same data.
//
// Run: cargo run --bin 122_arc

use std::sync::Arc;
use std::thread;

fn main() {
    let a = Arc::new(5);
    let b = Arc::clone(&a);
    let h = thread::spawn(move || println!("thread {b}"));
    println!("main {a}");
    h.join().unwrap();
}
