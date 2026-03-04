// 108. HashSet
//
// HashSet<T> is a HashMap with no useful values. insert returns whether the value was
// new. contains tests membership. union, intersection, difference are iterators. Use it
// to drop duplicates or test membership in average O(1).
//
// Run: cargo run --bin 108_hashset

use std::collections::HashSet;

fn main() {
    let a: HashSet<_> = [1, 2, 3].into_iter().collect();
    let b: HashSet<_> = [3, 4].into_iter().collect();
    println!("{}", a.contains(&2));
    println!("{:?}", a.intersection(&b).collect::<Vec<_>>());
}
