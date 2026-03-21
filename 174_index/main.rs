// 174. Index
//
// v[i] is Index. For Vec it panics on a bad index. HashMap does not implement Index with
// a panic-on-miss in older code; it does Index and panics if missing, so prefer get.
// Range indexing on slices returns a subslice.
//
// Run: cargo run --bin 174_index

use std::ops::Index;

fn main() {
    let v = vec![10, 20, 30];
    println!("{} {:?}", v[1], v.index(1));
    println!("{:?}", &v[1..]);
}
