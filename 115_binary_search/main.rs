// 115. Binary search
//
// binary_search returns Ok(index) or Err(insert_at) on a sorted slice.
// binary_search_by_key searches with a projection. The slice must already be sorted by
// the same order or the result is meaningless.
//
// Run: cargo run --bin 115_binary_search

fn main() {
    let v = [1, 3, 5, 7];
    println!("{:?} {:?}", v.binary_search(&5), v.binary_search(&4));
}
