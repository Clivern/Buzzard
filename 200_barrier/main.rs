// 200. Barrier
//
// Barrier waits until n threads have called wait. Then they all proceed. Use it to start
// a race fairly or to split a parallel algorithm into phases. The last thread to arrive
// gets BarrierWaitResult that is_leader.
//
// Run: cargo run --bin 200_barrier

use std::sync::{Arc, Barrier};
use std::thread;

fn main() {
    let bar = Arc::new(Barrier::new(3));
    thread::scope(|s| {
        for i in 0..3 {
            let bar = Arc::clone(&bar);
            s.spawn(move || {
                println!("arrive {i}");
                bar.wait();
                println!("go {i}");
            });
        }
    });
}
