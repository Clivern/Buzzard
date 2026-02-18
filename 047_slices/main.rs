// 047. Slices
//
// A slice &[T] is a view: a pointer and a length. &v[1..4] borrows that span. &str is a
// slice of UTF-8 bytes. Slices do not own the data. Indexing a slice panics if the range
// is out of bounds or, for &str, not on a char boundary.
//
//
// A slice's length is stored next to the pointer. That is why &[T] is a fat pointer,
// twice the size of a thin pointer on 64-bit.
//
// Run: cargo run --bin 047_slices

fn sum(nums: &[i32]) -> i32 {
    nums.iter().sum()
}

fn main() {
    let v = vec![1, 2, 3, 4, 5];
    println!("{} {:?}", sum(&v[1..4]), &v[..2]);
    let s = "rustacean";
    println!("{}", &s[0..4]);
    println!("fat pointer bytes {}", std::mem::size_of::<&[i32]>());
}
