// 130. Multiple producers
//
// clone the Sender to have several producers. The Receiver stays unique. Each send is a
// move of the value. for msg in rx iterates until all senders are gone.
//
// Run: cargo run --bin 130_multi_tx

use std::sync::mpsc;
use std::thread;

fn main() {
    let (tx, rx) = mpsc::channel();
    for i in 0..3 {
        let tx = tx.clone();
        thread::spawn(move || tx.send(i).unwrap());
    }
    drop(tx);
    for n in rx {
        println!("{n}");
    }
}
