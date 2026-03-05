// 114. Sorting
//
// sort on a Vec requires Ord. sort_by takes a comparator. sort_by_key projects a key.
// sort_unstable is faster and may reorder equal items. binary_search finds an index in a
// sorted slice. Reverse wraps a key for descending order.
//
// Run: cargo run --bin 114_sort

use std::cmp::Reverse;

fn main() {
    let mut names = vec!["b", "c", "a"];
    names.sort();
    println!("{names:?}");
    let mut pairs = vec![("ke", 80), ("joe", 20)];
    pairs.sort_by_key(|p| p.1);
    println!("{pairs:?}");
    pairs.sort_by_key(|p| Reverse(p.1));
    println!("{pairs:?}");
}
