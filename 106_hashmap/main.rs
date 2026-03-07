// 106. HashMap
//
// HashMap<K, V> associates keys with values. insert overwrites and returns the old
// value. get returns Option<&V>. entry API inserts only when missing. Keys must be Eq +
// Hash. Iteration order is not insertion order.
//
//
// len counts keys. is_empty is clearer than len() == 0. remove returns the old value if
// the key was present.
//
// Run: cargo run --bin 106_hashmap

use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();
    scores.insert("ann", 10);
    scores.insert("ben", 8);
    println!("{:?} {:?}", scores.get("ann"), scores.get("cy"));
    scores.entry("cy").or_insert(0);
    *scores.entry("ann").or_insert(0) += 1;
    println!("{scores:?}");
    println!("len {} {:?}", scores.len(), scores.remove("ben"));
}
