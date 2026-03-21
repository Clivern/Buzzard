// 176. once, repeat, empty
//
// once(v) yields one value. empty() yields none. repeat(v) is infinite clones. chain
// concatenates. zip stops at the shorter. These are the small building blocks of
// iterator pipelines.
//
// Run: cargo run --bin 176_once_repeat

use std::iter::{empty, once, repeat};

fn main() {
    let v: Vec<_> = once(1).chain(once(2)).chain(empty()).collect();
    println!("{v:?}");
    println!("{:?}", repeat("x").take(3).collect::<Vec<_>>());
}
