// 023. Returning values
//
// A return type of -> T means every path must produce a T. diverging with panic! or loop
// {} can stand in for a T because they never come back. Option and Result are the usual
// way to report absence or failure instead of sentinel values.
//
// Run: cargo run --bin 023_returns

fn first(nums: &[i32]) -> Option<i32> {
    if nums.is_empty() {
        return None;
    }
    Some(nums[0])
}

fn main() {
    println!("{:?} {:?}", first(&[8, 1]), first(&[]));
}
