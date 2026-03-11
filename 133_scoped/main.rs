// 133. Scoped threads
//
// thread::scope lets threads borrow local data. The scope waits for all spawned threads
// before it ends, so the borrows are valid. Use it when you want parallelism without
// 'static + Arc.
//
// Run: cargo run --bin 133_scoped

use std::thread;

fn main() {
    let v = vec![1, 2, 3];
    thread::scope(|s| {
        s.spawn(|| println!("{:?}", &v));
        s.spawn(|| println!("{}", v.len()));
    });
}
