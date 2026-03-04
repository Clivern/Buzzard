// 105. Vec extras
//
// extend adds from an iterator. retain keeps items matching a predicate. split_off
// splits at an index. drain removes a range and yields the items. truncate shortens.
// clear drops all. swap_remove is O(1) and unordered.
//
// Run: cargo run --bin 105_vec_extra

fn main() {
    let mut v = vec![1, 2, 3, 4, 5];
    v.retain(|n| *n % 2 == 1);
    println!("{v:?}");
    let rest = v.split_off(1);
    println!("{v:?} {rest:?}");
    println!("{:?}", vec![10, 20, 30].swap_remove(0));
}
