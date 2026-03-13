// 129. Channels
//
// mpsc is multiple producer, single consumer. channel() is unbounded. send waits only if
// you use a sync_channel with a full buffer. recv blocks until a value arrives. When all
// Senders are dropped, recv returns Err. This is a rendezvous for passing ownership
// between threads.
//
//
// send moves the value. After send, the producer no longer has it. That is how channels
// transfer ownership across threads.
//
// Run: cargo run --bin 129_mpsc

use std::sync::mpsc;
use std::thread;

fn main() {
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        tx.send("ping").unwrap();
    });
    println!("{}", rx.recv().unwrap());
    // recv blocks until ping arrives.
}
