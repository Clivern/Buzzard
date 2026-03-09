// 126. Threads
//
// thread::spawn starts a thread. The closure must be 'static + Send because the thread
// may outlive the caller. join waits and returns the closure's result (or a panic
// payload). main exiting terminates other threads; join them if you need their work to
// finish.
//
// Run: cargo run --bin 126_threads

use std::thread;

fn main() {
    let h = thread::spawn(|| {
        println!("in a thread");
        7
    });
    println!("in main");
    println!("joined {}", h.join().unwrap());
}
