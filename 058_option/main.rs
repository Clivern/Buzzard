// 058. Option
//
// Option<T> is Some(T) or None. It replaces null. You cannot read the inner value
// without handling None. unwrap panics on None; expect does too, with your message.
// Prefer match, if let, or combinators in library code.
//
// Run: cargo run --bin 058_option

fn find(nums: &[i32], target: i32) -> Option<usize> {
    nums.iter().position(|n| *n == target)
}

fn main() {
    println!("{:?} {:?}", find(&[1, 2, 3], 2), find(&[1, 2, 3], 9));
}
