// 125. OnceLock
//
// OnceLock<T> is a thread-safe cell you write once. get_or_init runs a closure the first
// time. Later calls return the same value. It replaced the lazy_static pattern for many
// globals. OnceLock is in std as of Rust 1.70.
//
// Run: cargo run --bin 125_once_lock

use std::sync::OnceLock;

static START: OnceLock<String> = OnceLock::new();

fn main() {
    let a = START.get_or_init(|| String::from("init"));
    let b = START.get_or_init(|| String::from("ignored"));
    println!("{} {}", a, b);
}
