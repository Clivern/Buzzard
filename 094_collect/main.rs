// 094. collect
//
// collect turns an iterator into a collection. The target type is often annotated:
// collect::<Vec<_>>(). It works for Vec, HashMap, String, Result (stops at first Err),
// and Option (stops at first None). FromIterator is the trait behind it.
//
// Run: cargo run --bin 094_collect

use std::collections::HashSet;

fn main() {
    let set: HashSet<_> = ["a", "a", "b"].into_iter().collect();
    println!("{set:?}");
    let s: String = ['r', 'u', 's', 't'].into_iter().collect();
    println!("{s}");
    let r: Result<Vec<_>, _> = ["1", "2", "x"].iter().map(|t| t.parse::<i32>()).collect();
    println!("{r:?}");
}
