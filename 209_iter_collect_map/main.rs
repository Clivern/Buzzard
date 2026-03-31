// 209. Collecting HashMap
//
// An iterator of pairs can collect into a HashMap. Duplicate keys: the later one wins.
// unzip splits a pair iterator into two collections. FromIterator for HashMap is the
// trait that makes this work.
//
// Run: cargo run --bin 209_iter_collect_map

use std::collections::HashMap;

fn main() {
    let m: HashMap<_, _> = [("a", 1), ("b", 2), ("a", 3)].into_iter().collect();
    println!("{m:?}");
    let (keys, vals): (Vec<_>, Vec<_>) = m.into_iter().unzip();
    println!("{keys:?} {vals:?}");
}
