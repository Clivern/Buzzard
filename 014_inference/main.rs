// 014. Type inference
//
// The compiler fills in types from how a value is used. parse needs a type on the left,
// or a turbofish on the right, because many types implement FromStr. Once one use pins
// the type, the rest follow. _ in a type asks inference to fill that part.
//
// Run: cargo run --bin 014_inference

fn main() {
    let n: i32 = "12".parse().unwrap();
    let m = "12".parse::<i64>().unwrap();
    let nums = vec![1, 2, 3];
    let first: Option<_> = nums.into_iter().next();
    println!("{n} {m} {first:?}");
}
