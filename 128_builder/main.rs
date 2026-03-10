// 128. Thread builder
//
// Builder sets a name (shown in panic messages) and a stack size before spawn. spawn
// returns io::Result because OS thread creation can fail. Use named threads when you
// read traces.
//
// Run: cargo run --bin 128_builder

use std::thread::Builder;

fn main() {
    let h = Builder::new()
        .name(String::from("worker"))
        .spawn(|| {
            println!("{}", std::thread::current().name().unwrap());
        })
        .unwrap();
    h.join().unwrap();
}
