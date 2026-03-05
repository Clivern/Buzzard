// 110. BTreeSet
//
// BTreeSet is a sorted set. first and last are O(log n). range walks a span. It is the
// ordered counterpart of HashSet.
//
// Run: cargo run --bin 110_btreeset

use std::collections::BTreeSet;

fn main() {
    let mut s = BTreeSet::new();
    s.insert(3);
    s.insert(1);
    s.insert(2);
    println!("{:?} {:?}", s.first(), s.range(2..));
}
