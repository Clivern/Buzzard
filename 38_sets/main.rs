// 38. HashSet and BTreeSet
//
// A set stores each value at most once. HashSet<T> is a HashMap with no values: average
// O(1) insert and contains, no ordering, and T: Eq + Hash. BTreeSet<T> keeps values
// sorted, needs T: Ord, and supports first, last, and range.
//
// insert returns true when the value was new, which makes "have I seen this?" a one
// liner. union, intersection, difference, and symmetric_difference return lazy
// iterators; the operators |, &, -, and ^ on references build new sets.
//
// Run: cargo run --bin 38_sets

use std::collections::{BTreeSet, HashSet};

// Example 1: insert, contains, remove.
fn demo_basics() {
    println!("-- basics --");

    let mut s = HashSet::new();
    println!("insert 1 new? {}", s.insert(1));
    println!("insert 1 again new? {}", s.insert(1));
    s.insert(2);
    println!("contains 2? {} len {}", s.contains(&2), s.len());
    println!("remove 2 {} remove 2 again {}", s.remove(&2), s.remove(&2));

    let from_array: HashSet<char> = "hello".chars().collect();
    println!("unique letters in hello: {}", from_array.len());
}

// Example 2: removing duplicates.
fn demo_dedup() {
    println!("-- dedup --");

    let words = ["b", "a", "b", "c", "a"];

    // Keep first occurrences, in order.
    let mut seen = HashSet::new();
    let first_seen: Vec<_> = words.iter().filter(|w| seen.insert(**w)).collect();
    println!("order kept {first_seen:?}");

    // Or let a BTreeSet sort them.
    let sorted: BTreeSet<_> = words.iter().collect();
    println!("sorted unique {sorted:?}");

    let has_dupes = words.len() != words.iter().collect::<HashSet<_>>().len();
    println!("has duplicates? {has_dupes}");
}

// Example 3: set algebra.
fn demo_algebra() {
    println!("-- set algebra --");

    let a: BTreeSet<_> = [1, 2, 3, 4].into_iter().collect();
    let b: BTreeSet<_> = [3, 4, 5].into_iter().collect();
    println!("union {:?}", a.union(&b).collect::<Vec<_>>());
    println!("intersection {:?}", a.intersection(&b).collect::<Vec<_>>());
    println!("difference a-b {:?}", a.difference(&b).collect::<Vec<_>>());
    println!("symmetric {:?}", a.symmetric_difference(&b).collect::<Vec<_>>());
    println!("operators: {:?} {:?}", &a | &b, &a & &b);

    let small: BTreeSet<_> = [3, 4].into_iter().collect();
    println!(
        "subset {} superset {} disjoint {}",
        small.is_subset(&a),
        a.is_superset(&small),
        small.is_disjoint(&[9].into_iter().collect())
    );
}

// Example 4: BTreeSet ordering and ranges.
fn demo_btreeset() {
    println!("-- BTreeSet --");

    let mut s = BTreeSet::new();
    for n in [30, 10, 50, 20, 40] {
        s.insert(n);
    }

    println!("{s:?} first {:?} last {:?}", s.first(), s.last());
    println!("range 20..=40 {:?}", s.range(20..=40).collect::<Vec<_>>());
    println!("next after 25: {:?}", s.range(26..).next());
    println!("pop_last {:?} now {s:?}", s.pop_last());
}

// Example 5: a practical use: tags and permissions.
fn demo_tags() {
    println!("-- tags --");

    let required: HashSet<&str> = ["read", "write"].into();
    let users = [("ann", vec!["read", "write", "admin"]), ("ben", vec!["read"])];
    for (name, perms) in &users {
        let have: HashSet<&str> = perms.iter().copied().collect();
        let missing: BTreeSet<_> = required.difference(&have).collect();
        if missing.is_empty() {
            println!("{name} ok");
        } else {
            println!("{name} missing {missing:?}");
        }
    }
}

// Example 6: sets of tuples and structs.
fn demo_compound() {
    println!("-- compound values --");

    let mut visited: HashSet<(i32, i32)> = HashSet::new();
    let moves = [(0, 1), (1, 0), (0, -1), (-1, 0), (0, 1)];
    let mut pos = (0, 0);
    visited.insert(pos);

    for (dx, dy) in moves {
        pos = (pos.0 + dx, pos.1 + dy);
        visited.insert(pos);
    }

    println!("visited {} distinct cells", visited.len());
    #[derive(Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
    enum Color {
        Red,
        Blue,
    }

    let colors: BTreeSet<Color> =
        [Color::Blue, Color::Red, Color::Blue].into_iter().collect();
    println!("{colors:?}");
}

fn main() {
    demo_basics();
    demo_dedup();
    demo_algebra();
    demo_btreeset();
    demo_tags();
    demo_compound();
}
