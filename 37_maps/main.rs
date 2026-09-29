// 37. HashMap, the entry API, and BTreeMap
//
// HashMap<K, V> maps keys to values with average O(1) lookups. Keys need Eq + Hash.
// insert overwrites and returns the old value; get returns Option<&V>; remove returns
// the old value. Iteration order is unspecified and can change between runs.
//
// The entry API looks a key up once and lets you act on the slot: or_insert writes a
// default when it is missing, or_insert_with builds the default lazily, or_default uses
// Default, and and_modify updates an existing value. It replaces the get-then-insert
// double lookup.
//
// BTreeMap keeps keys sorted. It needs Ord instead of Hash, iterates in key order, and
// can walk a key range. Reach for it when you need ordering or deterministic output.
//
// Run: cargo run --bin 37_maps

use std::collections::{BTreeMap, HashMap};

// Example 1: inserting, reading, removing.
fn demo_hashmap_basics() {
    println!("-- HashMap basics --");

    let mut scores = HashMap::new();
    scores.insert("ann", 10);
    scores.insert("ben", 8);
    println!("get {:?} {:?}", scores.get("ann"), scores.get("cy"));
    println!("insert returns old {:?}", scores.insert("ann", 11));
    println!("contains ben? {} len {}", scores.contains_key("ben"), scores.len());

    if let Some(s) = scores.get_mut("ben") {
        *s *= 2;
    }

    println!("remove {:?} then {:?}", scores.remove("ben"), scores.remove("ben"));

    // scores["cy"] panics on a missing key; get is the safe form.
    println!("indexing an existing key {}", scores["ann"]);
}

// Example 2: iteration (sorted here so the output is stable).
fn demo_iteration() {
    println!("-- iteration --");

    let m: HashMap<&str, i32> = [("b", 2), ("a", 1), ("c", 3)].into();
    let mut pairs: Vec<_> = m.iter().collect();
    pairs.sort();
    println!("{pairs:?}");

    let mut keys: Vec<_> = m.keys().copied().collect();
    keys.sort();

    let total: i32 = m.values().sum();
    println!("keys {keys:?} total {total}");

    let mut m = m;
    for v in m.values_mut() {
        *v *= 10;
    }

    m.retain(|_, v| *v > 10);
    let mut left: Vec<_> = m.into_iter().collect();
    left.sort();
    println!("after values_mut and retain {left:?}");
}

// Example 3: the entry API.
fn demo_entry() {
    println!("-- entry --");

    let text = "the cat and the hat and the bat";
    let mut counts: HashMap<&str, usize> = HashMap::new();
    for w in text.split_whitespace() {
        *counts.entry(w).or_insert(0) += 1;
    }

    let mut top: Vec<_> = counts.into_iter().collect();
    top.sort_by(|a, b| b.1.cmp(&a.1).then(a.0.cmp(b.0)));
    println!("word counts {top:?}");

    let mut words = HashMap::new();
    for w in ["a", "b", "a"] {
        words.entry(w).and_modify(|c| *c += 1).or_insert(1);
    }

    println!("and_modify a={} b={}", words["a"], words["b"]);

    // Group values under a key with or_default.
    let mut by_len: HashMap<usize, Vec<&str>> = HashMap::new();
    for w in ["fig", "kiwi", "pear", "plum", "date"] {
        by_len.entry(w.len()).or_default().push(w);
    }

    let mut groups: Vec<_> = by_len.into_iter().collect();
    groups.sort();
    println!("grouped {groups:?}");

    let mut cache: HashMap<u64, String> = HashMap::new();
    let v = cache.entry(7).or_insert_with(|| format!("computed {}", 7 * 6));
    println!("or_insert_with -> {v}");
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Pos {
    x: i32,
    y: i32,
}

// Example 4: your own types as keys, and maps of structs.
fn demo_custom_keys() {
    println!("-- custom keys --");

    let mut board: HashMap<Pos, char> = HashMap::new();
    board.insert(Pos { x: 0, y: 0 }, 'X');
    board.insert(Pos { x: 1, y: 1 }, 'O');
    println!(
        "{:?} {:?}",
        board.get(&Pos { x: 0, y: 0 }),
        board.get(&Pos { x: 2, y: 2 })
    );

    let tuple_keys: HashMap<(i32, i32), &str> = [((0, 1), "north")].into();
    println!("{:?}", tuple_keys.get(&(0, 1)));

    // HashMap<f64, _> does not work: f64 is not Eq or Hash.
}

// Example 5: BTreeMap ordering and ranges.
fn demo_btreemap() {
    println!("-- BTreeMap --");

    let mut m = BTreeMap::new();
    m.insert(2, "b");
    m.insert(1, "a");
    m.insert(5, "e");
    m.insert(3, "c");
    println!("in order {m:?}");
    println!("range 2.. {:?}", m.range(2..).collect::<Vec<_>>());
    println!("range ..=3 {:?}", m.range(..=3).map(|(k, _)| k).collect::<Vec<_>>());
    println!("first {:?} last {:?}", m.first_key_value(), m.last_key_value());
    println!("pop_first {:?} now {m:?}", m.pop_first());

    // Floor lookup: the greatest key <= 4.
    println!("floor(4) {:?}", m.range(..=4).next_back());
}

// Example 6: a time series with BTreeMap.
fn demo_time_series() {
    println!("-- time series --");

    let mut temps: BTreeMap<u32, f64> = BTreeMap::new();
    for (hour, t) in [(9, 14.5), (12, 19.0), (15, 21.5), (18, 17.0), (21, 13.0)] {
        temps.insert(hour, t);
    }

    let afternoon: Vec<_> =
        temps.range(12..=18).map(|(h, t)| format!("{h}h:{t}")).collect();
    println!("12..=18 {afternoon:?}");

    let avg = temps.values().sum::<f64>() / temps.len() as f64;
    println!("avg {avg:.1}");

    let split = temps.split_off(&15);
    println!("before 15 {temps:?} from 15 {split:?}");
}

// Example 7: merging maps and building them from iterators.
fn demo_merge() {
    println!("-- merge --");

    let mut a: BTreeMap<&str, i32> = [("x", 1), ("y", 2)].into();
    let b: BTreeMap<&str, i32> = [("y", 10), ("z", 3)].into();
    for (k, v) in b {
        *a.entry(k).or_insert(0) += v;
    }

    println!("{a:?}");
    let inverted: BTreeMap<i32, &str> = a.iter().map(|(k, v)| (*v, *k)).collect();
    println!("inverted {inverted:?}");
}

fn main() {
    demo_hashmap_basics();
    demo_iteration();
    demo_entry();
    demo_custom_keys();
    demo_btreemap();
    demo_time_series();
    demo_merge();
}
