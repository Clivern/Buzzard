// 109. BTreeMap
//
// BTreeMap keeps keys sorted. Iteration is in key order. It needs Ord, not Hash. range()
// walks a key range. Use it when you need order, or when keys are not Hash, or when you
// want deterministic output.
//
// Run: cargo run --bin 109_btreemap

use std::collections::BTreeMap;

fn main() {
    let mut m = BTreeMap::new();
    m.insert(2, "b");
    m.insert(1, "a");
    m.insert(3, "c");
    println!("{:?}", m.keys().collect::<Vec<_>>());
    println!("{:?}", m.range(2..).collect::<Vec<_>>());
}
