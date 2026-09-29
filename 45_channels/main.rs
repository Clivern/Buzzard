// 45. Channels: mpsc, multiple producers, sync_channel, try_recv
//
// std::sync::mpsc is a multiple-producer, single-consumer channel. channel() returns a
// (Sender, Receiver) pair with an unbounded buffer. send moves the value to the other
// side, transferring ownership between threads. recv blocks until a value arrives, and
// returns Err once every Sender has been dropped and the buffer is empty.
//
// Clone the Sender to get several producers. Iterating over the Receiver (for msg in
// rx) runs until all senders are gone, so drop the original Sender when you are done.
//
// sync_channel(n) has a bounded buffer: send blocks when it is full, which gives a fast
// producer backpressure. sync_channel(0) is a rendezvous where every send waits for a
// recv. try_recv and recv_timeout never block forever.
//
// Run: cargo run --bin 45_channels

use std::sync::mpsc::{self, RecvTimeoutError, TryRecvError};
use std::thread;
use std::time::Duration;

// Example 1: one producer, one consumer.
fn demo_basic() {
    println!("-- basic --");

    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        tx.send("ping").unwrap();
    });

    println!("{}", rx.recv().unwrap()); // blocks until ping arrives
    println!("after sender dropped: {:?}", rx.recv());

    let (tx, rx) = mpsc::channel();
    let s = String::from("moved through the channel");
    tx.send(s).unwrap();

    // println!("{s}"); // error[E0382]: borrow of moved value: `s`
    println!("{}", rx.recv().unwrap());
}

// Example 2: a stream of values, consumed with a for loop.
fn demo_stream() {
    println!("-- stream --");

    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        for i in 1..=3 {
            tx.send(i * 10).unwrap();
            thread::sleep(Duration::from_millis(1));
        }
    }); // tx dropped here, which ends the loop below
    for n in rx {
        print!("{n} ");
    }

    println!();
}

// Example 3: multiple producers.
fn demo_multi_producer() {
    println!("-- multiple producers --");

    let (tx, rx) = mpsc::channel();
    for id in 0..3 {
        let tx = tx.clone();
        thread::spawn(move || {
            for j in 0..2 {
                tx.send(format!("worker {id} msg {j}")).unwrap();
            }
        });
    }

    drop(tx); // without this, the loop below would wait forever
    let mut got: Vec<String> = rx.iter().collect();
    got.sort();
    println!("{} messages: {got:?}", got.len());
}

// Example 4: bounded channels and backpressure.
fn demo_sync_channel() {
    println!("-- sync_channel --");

    let (tx, rx) = mpsc::sync_channel(1);
    let producer = thread::spawn(move || {
        for i in 0..3 {
            tx.send(i).unwrap(); // blocks while the buffer is full
            println!("  sent {i}");
        }
    });

    thread::sleep(Duration::from_millis(10));
    for _ in 0..3 {
        println!("  received {}", rx.recv().unwrap());
    }

    producer.join().unwrap();

    let (tx, rx) = mpsc::sync_channel::<i32>(1);
    tx.try_send(1).unwrap();
    println!("try_send into full buffer: {:?}", tx.try_send(2));
    println!("drain {:?}", rx.try_iter().collect::<Vec<_>>());
}

// Example 5: try_recv and recv_timeout.
fn demo_non_blocking() {
    println!("-- try_recv / recv_timeout --");

    let (tx, rx) = mpsc::channel();
    println!("try_recv empty: {:?}", rx.try_recv() == Err(TryRecvError::Empty));
    tx.send("hi").unwrap();
    println!("try_recv {:?}", rx.try_recv());
    println!(
        "timeout {:?}",
        rx.recv_timeout(Duration::from_millis(5)) == Err(RecvTimeoutError::Timeout)
    );
    drop(tx);
    println!("disconnected {:?}", rx.try_recv() == Err(TryRecvError::Disconnected));
}

// Example 6: a worker pool with a job channel and a result channel.
enum Job {
    Square(u64),
    Stop,
}

fn demo_worker_pool() {
    println!("-- worker pool --");

    let (job_tx, job_rx) = mpsc::channel::<Job>();
    let (res_tx, res_rx) = mpsc::channel::<(usize, u64)>();

    // Receiver is not Clone; share it behind a Mutex.
    let job_rx = std::sync::Arc::new(std::sync::Mutex::new(job_rx));
    let workers: Vec<_> = (0..3)
        .map(|id| {
            let job_rx = std::sync::Arc::clone(&job_rx);
            let res_tx = res_tx.clone();
            thread::spawn(move || {
                loop {
                    let job = job_rx.lock().unwrap().recv();
                    match job {
                        Ok(Job::Square(n)) => res_tx.send((id, n * n)).unwrap(),
                        Ok(Job::Stop) | Err(_) => break,
                    }
                }
            })
        })
        .collect();

    drop(res_tx);
    for n in 1..=6 {
        job_tx.send(Job::Square(n)).unwrap();
    }

    for _ in &workers {
        job_tx.send(Job::Stop).unwrap();
    }

    let mut results: Vec<u64> = res_rx.iter().map(|(_, v)| v).collect();
    results.sort();

    for w in workers {
        w.join().unwrap();
    }

    println!("squares {results:?}");
}

// Example 7: a request/response pattern: send a reply channel with the request.
fn demo_request_reply() {
    println!("-- request / reply --");

    let (req_tx, req_rx) = mpsc::channel::<(String, mpsc::Sender<usize>)>();
    let server = thread::spawn(move || {
        for (text, reply) in req_rx {
            reply.send(text.len()).unwrap();
        }
    });

    for word in ["a", "hello"] {
        let (reply_tx, reply_rx) = mpsc::channel();
        req_tx.send((word.to_string(), reply_tx)).unwrap();
        println!("{word} -> {}", reply_rx.recv().unwrap());
    }

    drop(req_tx);
    server.join().unwrap();
}

fn main() {
    demo_basic();
    demo_stream();
    demo_multi_producer();
    demo_sync_channel();
    demo_non_blocking();
    demo_worker_pool();
    demo_request_reply();
}
