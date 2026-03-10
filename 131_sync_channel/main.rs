// 131. sync_channel
//
// sync_channel(n) has a buffer of n. send blocks when the buffer is full.
// sync_channel(0) is a true rendezvous: send waits for recv. Use it to apply
// backpressure so a fast producer cannot grow memory without bound.
//
// Run: cargo run --bin 131_sync_channel

use std::sync::mpsc;
use std::thread;

fn main() {
    let (tx, rx) = mpsc::sync_channel(1);
    thread::spawn(move || {
        tx.send(1).unwrap();
        tx.send(2).unwrap();
    });
    println!("{} {}", rx.recv().unwrap(), rx.recv().unwrap());
}
