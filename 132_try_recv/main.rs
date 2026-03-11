// 132. try_recv
//
// try_recv returns immediately: Ok, Empty, or Disconnected. It is the closest std
// equivalent of a non-blocking select on one channel. recv_timeout waits a while. For
// many channels, a crate such as crossbeam-channel adds select.
//
// Run: cargo run --bin 132_try_recv

use std::sync::mpsc;
use std::time::Duration;

fn main() {
    let (tx, rx) = mpsc::channel();
    println!("{:?}", rx.try_recv());
    tx.send("hi").unwrap();
    println!("{:?}", rx.recv_timeout(Duration::from_millis(10)));
}
